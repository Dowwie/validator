"""Six explicit portfolio tasks. No inference, semantic labeling, or automatic repair.

Atlas answers are published observations. Sponsor/moderation replies are named
software fixtures produced through the original application code. Do not mix them.
"""
from __future__ import annotations

import ast
import copy
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
from adapters import ContractError, abstain, categorical, class_out, number  # noqa: E402
from poc import digest, encoded, read_json, strict_json  # noqa: E402


def require(condition, message):
    if not condition:
        raise ContractError(message)


def indexed(rows):
    require(isinstance(rows, list), "records must be a list")
    result = {}
    for row in rows:
        require(isinstance(row, dict) and isinstance(row.get('id'), str) and row['id'], "nonempty source id required")
        require(row['id'] not in result, "duplicate source id")
        result[row['id']] = row
    return result


def checked_sources(root: Path):
    """Admit exact pinned tracked bytes; npm-added dependency files are separate."""
    pins = read_json(HERE / 'sources.json')
    evidence = {'portfolio-pins.json': encoded(pins), 'portfolio-adapter.py': Path(__file__).read_bytes()}
    for slug, spec in pins.items():
        directory = root / slug
        head = subprocess.check_output(['git', '-C', str(directory), 'rev-parse', 'HEAD'], text=True).strip()
        require(head == spec['commit'], f'{slug}: wrong source revision')
        for relative, identity in spec['files'].items():
            p = directory / relative
            require(not p.is_symlink(), f'{slug}: symlink source refused')
            data = p.read_bytes()
            require(digest(data) == identity['sha256'], f'{slug}: changed source {relative}')
            if relative.endswith('.json'):
                strict_json(data)
            evidence[slug + '-' + relative.replace('/', '__')] = data
    lock = root / 'moderation' / 'package-lock.json'
    require(digest(lock.read_bytes()) == digest((HERE / 'moderation.package-lock.json').read_bytes()), 'moderation dependency lock differs')
    evidence['moderation-runtime-lock.json'] = lock.read_bytes()
    return pins, evidence


def literal_assignment(path: Path, name: str):
    """Read a literal constant; never execute the upstream model runner."""
    for stmt in ast.parse(path.read_text()).body:
        if isinstance(stmt, ast.Assign) and any(isinstance(t, ast.Name) and t.id == name for t in stmt.targets):
            return ast.literal_eval(stmt.value)
    raise ContractError(f'missing literal {name}')


def make_case(cid, state, expected, labels, rationale):
    require(isinstance(state, dict), 'canonical case input must be an object')
    return {'case_id': cid, 'family_id': cid, 'partition': 'development',
            'model_input': state, 'proposed_expected': class_out(expected, labels),
            'rationale': rationale, 'review': {'state': 'pending', 'reviewer': None}}


def book(slug, labels, cases, source, *, historical=False):
    return {'schema_version': 1, 'example': slug,
            'task': {'kind': 'single_label', 'labels': labels}, 'cases': cases, 'source': source,
            'reference_authority': 'upstream_author_fixture',
            'configuration': {'assessment': 'historical_label_agreement' if historical else 'author_diagnostic_agreement',
                              'new_human_approval': False, 'held_out': False}}


def choice_cases(cases, records, labels, state_fn, instruction):
    gold = indexed(cases); actual = indexed(records)
    require(set(gold) == set(actual), 'reference/response IDs do not match')
    rows, authored, withheld = [], [], []
    for cid, c in gold.items():
        r = actual[cid]
        require('expected' in c and r.get('expected', object()) == c['expected'], 'reference identity mismatch')
        prediction = {'outcome': class_out(r.get('choice'), labels), 'observations': {}}
        categorical(prediction, r.get('probabilities'), labels)
        prediction['confidence'] = number(r.get('confidence'), probability=True)
        if c['expected'] is None:
            withheld.append({'id': cid, 'reason': 'upstream_reference_is_explicitly_unresolved',
                             'observed_choice': r['choice'], 'confidence': r['confidence'],
                             'probabilities': r['probabilities']})
            continue
        require(c['expected'] in labels, 'invalid reference label')
        state = state_fn(c)
        authored.append(make_case(cid, state, c['expected'], labels, c.get('note', 'Upstream declared reference.')))
        rows.append({'case_id': cid, 'prediction': prediction,
                     'source_configuration': {'question_instruction': instruction}})
    require(authored, 'no settled references')
    return authored, rows, withheld


def triage_cases(data, recorded, threshold):
    number(threshold, probability=True)
    cases = indexed(data['cases']); actual = indexed(recorded['results'])
    require(set(cases) == set(actual), 'triage reference/response IDs do not match')
    labels = ['active', 'frozen', 'rejected']
    authored, rows = [], []
    for cid, c in cases.items():
        r = actual[cid]
        for key in ('historical_label', 'historical_score', 'title', 'topic'):
            require(c[key] == r[key], f'triage source mismatch: {key}')
        pa, pr = number(r['p_active'], probability=True), number(r['p_reject'], probability=True)
        def route(t):
            return 'active' if pa >= t else 'rejected' if pr >= t else 'batch_review'
        require(route(recorded['threshold']) == r['routed'], 'saved policy does not match saved signals')
        state = (f"Captured item metadata:\nTitle: {c['title']}\nURL: {c['url']}\nTopic tag: {c['topic']}\n\n"
                 f"Why this topic is watched: {data['watch_reasons'][c['topic']]}\n\n"
                 f"Content summary: {c['content']}\n\n{data['roster_note']}")
        authored.append(make_case(cid, {'state': state}, c['historical_label'], labels,
                                   'Historical disposition is not independently adjudicated truth.'))
        action = route(threshold)
        p = abstain('batch_review_not_a_final_class') if action == 'batch_review' else {'outcome': class_out(action, labels), 'observations': {}}
        p['observations'] = {'active_trigger': {'kind': 'bernoulli', 'value': pa},
                             'reject_trigger': {'kind': 'bernoulli', 'value': pr}}
        # Two independent signals are not a three-way final-action distribution.
        rows.append({'case_id': cid, 'prediction': p,
                     'source_configuration': {'priority': ['active', 'rejected', 'batch_review'], 'threshold': threshold}})
    return authored, rows


def atlas_tasks(root: Path, pins):
    source = {k: pins['atlas'][k] for k in ('repository', 'commit')}
    suite_root = root / 'atlas' / 'suites'
    specifications = [
        ('citation-support', 'citation-support-check', 'cases', 'results', ['supports', 'contradicts', 'says_nothing'],
         lambda c: {'claim': c['claim'], 'quote': c['quote']},
         'Does the quote support, contradict, or fail to address the claim?'),
        ('sarcasm-local', 'sarcasm-vs-sincere-praise', 'same-clause', 'same_clause', ['sincere', 'sarcastic'],
         lambda c: {'state': c['state']},
         "Is the speaker's positive-sounding statement sincere, or sarcastic/ironic?"),
        ('sarcasm-context', 'sarcasm-vs-sincere-praise', 'cross-turn', 'cross_turn', ['sincere', 'sarcastic'],
         lambda c: {'earlier_context': c['context'], 'statement_to_judge': c['statement']},
         'Given the earlier context, is the positive-sounding statement_to_judge sincere, or sarcastic/ironic given what the context establishes?'),
    ]
    tasks = []
    for slug, suite, data_name, key, labels, state_fn, question in specifications:
        folder = suite_root / suite
        records = read_json(folder / 'runs/2026-09-19.json')
        require(records['suite'] == suite and records['model'] == 'jev-latest', 'unexpected source suite/model')
        c, rows, withheld = choice_cases(read_json(folder / f'data/{data_name}.json'), records[key], labels, state_fn, question)
        b = book(slug, labels, c, source)
        b['configuration'].update(source_case_count=len(records[key]), scored_case_count=len(c),
                                  withheld_reference_ids=[v['id'] for v in withheld])
        tasks.append({'book': b, 'origin': 'published_replay', 'withheld': withheld,
                      'capture': {'source': dict(source, model=records['model'], resolved_model=None),
                                  'published_rows': records[key], 'request_reconstruction': 'from_pinned_runner_not_original_request_log'},
                      'arms': [('recorded', rows, {}), ('confidence-080', copy.deepcopy(rows), {'minimum': .8})]})
    folder = suite_root / 'stage1-triage-filter'
    data = read_json(folder/'data/cases.json'); records = read_json(folder/'runs/2026-09-20.json')
    require(records['suite'] == 'stage1-triage-filter' and records['model'] == 'jev-latest' and records['threshold'] == .75,
            'unrecognized triage run')
    c, rows = triage_cases(data, records, .75)
    _, alternative = triage_cases(data, records, .9)
    b = book('feed-triage', ['active', 'frozen', 'rejected'], c, source, historical=True)
    b['configuration']['questions'] = {n: literal_assignment(folder/'run.py', n) for n in ['ACTIVE_Q', 'REJECT_Q']}
    tasks.append({'book': b, 'origin': 'published_replay', 'withheld': [],
                  'capture': {'source': dict(source, model=records['model'], resolved_model=None), 'published_rows': records['results'],
                              'reference_meaning': 'historical_disposition_not_new_gold'},
                  'arms': [('threshold-075', rows, {'threshold': .75}), ('threshold-090', alternative, {'threshold': .9})]})
    return tasks


def fixture_tasks(capture, pins):
    require(capture.get('origin') == 'synthetic_fixture' and capture.get('live_model_calls') == 0, 'native reply origin must be synthetic')
    require(set(capture['tasks']) == {'sponsor', 'moderation'}, 'native task set changed')
    tasks = []
    for slug, task in capture['tasks'].items():
        source = {k: pins[slug][k] for k in ('repository', 'commit')}
        require(task['source'] == source, 'native source pin mismatch')
        labels = ['keep', 'skip'] if slug == 'sponsor' else ['allow', 'block']
        refs = indexed(task['cases']); cases = []; arms = []
        for cid, row in refs.items():
            cases.append(make_case(cid, row['input'], row['expected'], labels,
                                   'Synthetic software fixture, not approved semantic gold.'))
        for name in ('baseline', 'candidate'):
            results = indexed(task[name]); require(set(results) == set(refs), 'incomplete native capture')
            rows = []
            for cid, r in results.items():
                require(r['input'] == refs[cid]['input'], 'native request input changed across arms')
                p = {'outcome': class_out(r['action'], labels), 'observations': {}}
                if r.get('probability') is not None:
                    p['observations']['native_noul'] = {'kind': 'bernoulli', 'value': number(r['probability'], probability=True)}
                # An application fail-open action is still an observed action. Its
                # absent model verdict is disclosed, not invented as p=0.
                p['observations']['model_answered'] = {'kind': 'scalar', 'value': int(r.get('probability') is not None)}
                rows.append({'case_id': cid, 'prediction': p, 'source_configuration': {'threshold': task['thresholds'][name]}})
            arms.append((name, rows, {'threshold': task['thresholds'][name]}))
        b = book(slug + '-action', labels, cases, source)
        b['reference_authority'] = 'synthetic_test_only'
        b['configuration']['assessment'] = 'synthetic_software_test'
        tasks.append({'book': b, 'origin': 'synthetic_fixture', 'withheld': [],
                      'capture': {'source': dict(source, model='synthetic-fixture-no-model-call'), 'native_trace': task}, 'arms': arms})
    return tasks
