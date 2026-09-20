#!/usr/bin/env node
// Bounded capture of the *pinned application's* real classify/provider path.
// Dry-run is the default. No reference fields or credentials enter saved requests.
import fs from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { execFileSync } from 'node:child_process';

const PIN = '966de12e2bb5f94d47886ee51f30a07ec8ef1607';
const args = process.argv.slice(2);
const options = { partition: 'development', limit: '5', execute: false };
for (let i = 0; i < args.length; i++) {
  if (args[i] === '--execute') options.execute = true;
  else if (['--book', '--out', '--upstream', '--partition', '--limit'].includes(args[i]) && args[i + 1]) {
    const key = args[i].slice(2);
    if (!['partition', 'limit'].includes(key) && options[key]) throw new Error(`Duplicate ${args[i]}`);
    options[key] = args[++i];
  } else throw new Error(`Unknown or incomplete argument: ${args[i]}`);
}
if (!options.book || !['development', 'held_out'].includes(options.partition)) {
  throw new Error('Use --book PATH [--partition development|held_out] [--limit 1..80]; add --execute --upstream PINNED_CHECKOUT --out NEW_FILE for a live capture.');
}
const limit = Number(options.limit);
if (!Number.isInteger(limit) || limit < 1 || limit > 80) throw new Error('limit must be 1..80');
const book = JSON.parse(await fs.readFile(options.book, 'utf8'));
if (book.example !== 'sift' || book.schema_version !== 1) throw new Error('Expected a Sift authoring casebook');
const eligible = book.cases.filter(c => c.partition === options.partition);
const selected = eligible.slice(0, limit);
if (!selected.length) throw new Error('No selected cases');
const ids = new Set();
for (const c of selected) {
  if (typeof c.case_id !== 'string' || ids.has(c.case_id)) throw new Error('Missing/duplicate case identity');
  ids.add(c.case_id);
  if (typeof c.model_input.query !== 'string' || typeof c.model_input.text !== 'string' || c.model_input.text.length > 10_000) {
    throw new Error('Expected query/text; this bounded capture accepts at most 10,000 text characters per case');
  }
}
const projections = selected.map(c => ({ query: c.model_input.query, items: [{ id: c.case_id, text: c.model_input.text }] }));
if (!options.execute) {
  console.log(JSON.stringify({ dry_run: true, model_calls: 0, selected: selected.length,
    total_in_partition: eligible.length, pending_reviews: selected.filter(c => c.review?.state !== 'approved').length,
    application_inputs: projections }, null, 2));
} else {
  if (!options.out || !options.upstream) throw new Error('Live capture needs --out and --upstream');
  if (!selected.every(c => c.review?.state === 'approved' && c.review?.method === 'human' && typeof c.review?.reviewer === 'string' && c.review.reviewer.trim())) {
    throw new Error('Human reference approval is required before a target-model call');
  }
  const key = process.env.TYPESAFE_API_KEY || process.env.JEV_API_KEY;
  if (!key) throw new Error('Set TYPESAFE_API_KEY or JEV_API_KEY; never pass a key on the command line');
  const source = path.resolve(options.upstream);
  if (execFileSync('git', ['-C', source, 'rev-parse', 'HEAD'], { encoding: 'utf8' }).trim() !== PIN) throw new Error('Upstream checkout is not at the reviewed pin');
  if (execFileSync('git', ['-C', source, 'diff', '--name-only', 'HEAD', '--', 'src'], { encoding: 'utf8' }).trim()) throw new Error('Upstream source has local changes');
  // Reserve private destinations before any paid call; never replace an old capture.
  const output = await fs.open(options.out, 'wx', 0o600);
  let subset;
  try { subset = await fs.open(`${options.out}.book.json`, 'wx', 0o600); }
  catch (error) { await output.close(); await fs.unlink(options.out); throw error; }
  const records = [];
  const signal = AbortSignal.timeout(120_000);
  const capture = { schema_version: 1, example: 'sift', origin: 'observed', complete: false,
    source: { model: 'jev-latest', requested_model: 'jev-latest', repository: 'kbhuw/jev-sift', commit: PIN },
    records, selection: { partition: options.partition, selected: selected.length, total_in_partition: eligible.length },
    limits: { requests: limit, retries: 0, request_timeout_ms: 15_000, overall_timeout_ms: 120_000 } };
  try {
    const { classify } = await import(pathToFileURL(path.join(source, 'src/classify.js')));
    const { createProvider } = await import(pathToFileURL(path.join(source, 'src/provider.js')));
    for (let i = 0; i < selected.length; i++) {
      signal.throwIfAborted();
      let request, responseBody, status;
      const evaluate = createProvider({ apiKey: key, timeoutMs: 15_000 }, async (url, init) => {
        request = JSON.parse(init.body); // Deliberately omit all HTTP headers.
        const response = await fetch(url, init);
        status = response.status;
        if (response.ok) responseBody = await response.clone().json();
        return response;
      });
      const result = await classify(projections[i], { evaluate, concurrency: 1, signal });
      const native = result.results[0];
      records.push({ case_id: selected[i].case_id, output: native, request,
        model: native.model || 'jev-latest', configuration: { query: projections[i].query },
        metadata: { http_status: status, response: responseBody,
          resolved_model_supplied: typeof responseBody?.model === 'string' } });
    }
    capture.complete = true;
  } catch (error) {
    // Persist an explicitly incomplete record without exception text that could
    // contain environment paths or transport details; preparation refuses it.
    capture.failure = error.name || 'CaptureError';
    process.exitCode = 2;
  } finally {
    capture.not_attempted_ids = selected.slice(records.length).map(c => c.case_id);
    await output.writeFile(`${JSON.stringify(capture, null, 2)}\n`);
    await subset.writeFile(`${JSON.stringify({ ...book, cases: selected }, null, 2)}\n`);
    await output.close(); await subset.close();
  }
  console.log(JSON.stringify({ complete: capture.complete, recorded: records.length, output: options.out }));
}
