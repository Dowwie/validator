#!/usr/bin/env ruby
# frozen_string_literal: true

require 'json'
require 'digest'
require 'pathname'

ROOT = File.expand_path('../../..', __dir__)
PLAN = File.join(ROOT, 'docs/plans/validator')
REQUIRED = %w[id title authority phase effort_hours_estimate context outcome dependencies artifacts interfaces acceptance_criteria verification handoff_contract implementation_role verification_role requirement_ids input_artifacts verification_responsibility common_completion_checks].freeze

def assert_plan(condition, message)
  raise message unless condition
end

def read_json(path)
  JSON.parse(File.read(path, encoding: 'UTF-8'))
end

def verify_task(task, ids)
  assert_plan((REQUIRED - task.keys).empty?, "Missing task fields: #{task['id']}")
  %w[context artifacts interfaces acceptance_criteria verification].each do |key|
    assert_plan(!task.fetch(key).empty?, "Empty #{key}: #{task['id']}")
  end
  task.fetch('context').each do |context|
    path = File.join(ROOT, context.fetch('source').split('#', 2).first)
    assert_plan(File.file?(path), "Missing task context: #{task['id']}: #{path}")
  end
  task.fetch('dependencies').each do |dep|
    assert_plan(ids.include?(dep['task']), "Unknown dependency: #{dep}")
    assert_plan(!dep.fetch('requires').empty?, "Dependency lacks reason: #{task['id']}")
  end
  task.fetch('verification').each do |proof|
    assert_plan(proof['command'] || proof['procedure'], "Missing verification procedure: #{proof['id']}")
    assert_plan(!proof.fetch('expected').empty?, "Missing expected result: #{proof['id']}")
  end
end

def verify_graph(tasks, graph)
  assert_plan(graph['nodes'].map { |n| n['id'] }.sort == tasks.map { |t| t['id'] }.sort, 'Graph nodes mismatch')
  expected = tasks.flat_map do |task|
    task['dependencies'].map { |dep| [dep['task'], task['id'], dep['requires']] }
  end
  actual = graph['edges'].map { |edge| [edge['from'], edge['to'], edge['requires']] }
  assert_plan(expected.sort == actual.sort, 'Graph disagrees with task dependencies')
  order = graph.fetch('topological_order')
  assert_plan(order.sort == tasks.map { |t| t['id'] }.sort, 'Topological nodes mismatch')
  actual.each do |from, to, _|
    assert_plan(order.index(from) < order.index(to), "Cycle or invalid order: #{from} -> #{to}")
  end
end

def verify_source_block(entry, lines)
  source = entry.fetch('source')
  slice = lines.fetch(source['path'])[(source['line_start'] - 1)..(source['line_end'] - 1)].join
  assert_plan(Digest::SHA256.hexdigest(slice) == source['sha256'], "Source block drift: #{entry['id']}")
end

def verify_test_owner(entry, tasks)
  return unless entry['planned_test']

  task = tasks.find { |t| t['id'] == entry['test_owner'] }
  assert_plan(task, "Unknown test owner: #{entry['id']}")
  proof = task['verification'].find { |v| v['id'] == entry['planned_test'] }
  assert_plan(proof, "Unassigned case test: #{entry['id']}")
end

def verify_source_coverage(coverage, tasks)
  lines = {}
  coverage['baseline_sources'].each do |source|
    path = File.join(ROOT, source['path'])
    assert_plan(Digest::SHA256.file(path).hexdigest == source['sha256'], "Baseline changed: #{path}")
    lines[source['path']] = File.readlines(path, encoding: 'UTF-8')
  end
  ids = tasks.map { |t| t['id'] }
  entries = coverage.fetch('entries')
  assert_plan(entries.map { |e| e['id'] }.uniq.length == entries.length, 'Duplicate coverage IDs')
  entries.each do |entry|
    verify_source_block(entry, lines)
    %w[implementation_tasks verification_tasks].each do |key|
      assert_plan(!entry[key].empty? && (entry[key] - ids).empty?, "Unowned #{key}: #{entry['id']}")
    end
    verify_test_owner(entry, tasks)
  end
  verify_line_coverage(lines, entries)
end

def verify_line_coverage(lines, entries)
  limits = {'docs/specs/validator-v1.md' => 1350, 'docs/specs/validator-data-model.md' => 276}
  limits.each do |path, limit|
    covered = entries.select { |e| e['source']['path'] == path }.flat_map do |entry|
      (entry['source']['line_start']..entry['source']['line_end']).to_a
    end
    lines.fetch(path).first(limit).each_with_index do |line, index|
      next if line.strip.empty? || line.start_with?('#') || line.match?(/^\|[- :|]+\|\s*$/)
      assert_plan(covered.include?(index + 1), "Unmapped source line: #{path}:#{index + 1}")
    end
  end
end

def verify_reverse_maps(tasks, coverage, physical)
  tasks.each do |task|
    wanted = coverage['entries'].select { |e| e['implementation_tasks'].include?(task['id']) }.map { |e| e['id'] }
    assert_plan(task['requirement_ids'].sort == wanted.sort, "Task coverage mismatch: #{task['id']}")
    expected_inputs = task['dependencies'].flat_map do |dep|
      tasks.find { |t| t['id'] == dep['task'] }['artifacts'].map { |a| {'from'=>dep['task'], 'path'=>a['path']} }
    end.uniq
    assert_plan(task['input_artifacts'] == expected_inputs, "Dependency inputs drifted: #{task['id']}")
    task['artifacts'].each do |artifact|
      file = physical['artifacts'].find { |f| f['path'] == artifact['path'] }
      assert_plan(file && file['writers'].include?(task['id']), "Missing physical owner: #{artifact['path']}")
    end
  end
end

def verify_document_links
  index = File.read(File.join(ROOT, 'docs/artifact-index.md'), encoding: 'UTF-8')
  docs = Dir[File.join(PLAN, '**', '*')].select { |f| File.file?(f) }
  docs << File.join(ROOT, 'docs/plans/build-validator.md')
  docs.each do |path|
    relative = Pathname.new(path).relative_path_from(Pathname.new(File.join(ROOT, 'docs'))).to_s
    assert_plan(index.include?("(#{relative})"), "Unindexed plan artifact: #{relative}")
  end
  markdown = docs.select { |p| p.end_with?('.md') } + [File.join(ROOT, 'docs/artifact-index.md')]
  markdown.each do |path|
    File.read(path, encoding: 'UTF-8').scan(/\]\(([^)]+)\)/).flatten.each do |link|
      next if link.start_with?('http://', 'https://', '#')
      target = File.expand_path(link.split('#', 2).first, File.dirname(path))
      assert_plan(File.exist?(target), "Broken local link: #{path}: #{link}")
    end
  end
end

tasks = Dir[File.join(PLAN, 'tasks/*.json')].sort.map { |p| read_json(p) }
ids = tasks.map { |t| t['id'] }
assert_plan(ids.length == ids.uniq.length, 'Duplicate task IDs')
tasks.each { |task| verify_task(task, ids) }
verify_graph(tasks, read_json(File.join(PLAN, 'dependency-graph.json')))
coverage = read_json(File.join(PLAN, 'coverage.json'))
verify_source_coverage(coverage, tasks)
verify_reverse_maps(tasks, coverage, read_json(File.join(PLAN, 'physical-map.json')))
assert_plan(coverage['case_counts'] == {'S'=>29, 'M'=>21, 'E'=>13, 'AC'=>8, 'DOD'=>5, 'DM'=>12}, 'Case counts differ from reviewed source tables')
verify_document_links
puts "PASS: #{tasks.length} task contracts, acyclic dependencies, #{coverage['entries'].length} source blocks, 63 conformance cases, 12 structure checks, 8 ACs, 5 DoD clauses, physical owners, local links and artifact index."
puts 'This verifies planning structure and source identity, not implementation correctness.'
