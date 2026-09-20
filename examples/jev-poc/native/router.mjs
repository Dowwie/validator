#!/usr/bin/env node
// Run exact, hash-checked upstream policy/config code with its lockfile SDK.
// Model answers are explicit fixtures; no proxy or network calls are permitted.
import fs from 'node:fs';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { execFileSync } from 'node:child_process';
import assert from 'node:assert/strict';

const PIN = '38da6b84ea01241bfc41fbddc0928d0f40a703f0';
const BLOBS = {
  'config.mjs': '266683bc52e2bffb4ae1f59d72b0d2781c1396a2',
  'policy.mjs': '16adaa80b61cbec286e1fcbe1c829f57fd290f0d',
};
const opts = {};
for (let i = 2; i < process.argv.length; i += 2) {
  const flag = process.argv[i];
  if (!['--upstream', '--cases', '--out'].includes(flag) || !process.argv[i + 1] || opts[flag]) {
    throw new Error('Use --upstream DIR --out NEW_FILE [--cases FILE]');
  }
  opts[flag] = process.argv[i + 1];
}
if (!opts['--upstream'] || !opts['--out']) throw new Error('--upstream and --out are required');
const source = path.resolve(opts['--upstream'], 'src');
assert.equal(execFileSync('git',['-C',opts['--upstream'],'rev-parse','HEAD'],{encoding:'utf8'}).trim(),PIN);
assert.equal(execFileSync('git',['-C',opts['--upstream'],'diff','--name-only','HEAD','--','src'],{encoding:'utf8'}).trim(),'');
const identities = {};
for (const [name, wanted] of Object.entries(BLOBS)) {
  const bytes = fs.readFileSync(path.join(source, name));
  const actual = createHash('sha1').update(`blob ${bytes.length}\0`).update(bytes).digest('hex');
  assert.equal(actual, wanted, `${name}: different upstream source, including line endings`);
  identities[name] = { git_blob: actual, sha256: createHash('sha256').update(bytes).digest('hex') };
}
globalThis.fetch = () => { throw Error('NETWORK_FORBIDDEN: policy-only fixture'); };
const { decide } = await import(pathToFileURL(path.join(source,'policy.mjs')));
const { THRESHOLDS } = await import(pathToFileURL(path.join(source,'config.mjs')));
const fixture = JSON.parse(fs.readFileSync(opts['--cases'] || fileURLToPath(new URL('router-cases.json', import.meta.url)), 'utf8'));
assert.equal(fixture.origin, 'synthetic_fixture');
assert.equal(fixture.cases.length, 24);
const result = {
  schema_version: 1, origin: 'synthetic_fixture', model_accuracy_claim: null,
  live_model_calls: 0, repository: 'gargpratyush/jev-router', commit: PIN,
  source_files: identities, node: process.version,
  scope: 'Exact upstream policy/config with installed lockfile SDK; synthetic model answers. No proxy or live inference tested.',
  cases: [], canaries: [],
};
const plain = value => JSON.parse(JSON.stringify(value));
for (const row of fixture.cases) {
  THRESHOLDS.minConfidence = 0.3;
  const baseline = plain(decide(structuredClone(row.input)));
  assert.deepEqual(baseline, row.expected, row.id);
  THRESHOLDS.minConfidence = 0.5;
  const candidate = plain(decide(structuredClone(row.input)));
  result.cases.push({ id: row.id, input: row.input, expected: row.expected, baseline, candidate });
}
THRESHOLDS.minConfidence = 0.3;
for (const row of fixture.canaries) {
  const output = plain(decide(structuredClone(row.input)));
  result.canaries.push({ ...row, output, kept_current: output.tier === row.expected_safe_tier });
}
result.passed = result.cases.length;
result.changed_at_0_5 = result.cases.filter(x => x.baseline.tier !== x.candidate.tier).map(x => x.id);
result.canaries_not_keeping_current = result.canaries.filter(x => !x.kept_current).map(x => x.id);
const fd = fs.openSync(opts['--out'], 'wx', 0o600);
try { fs.writeFileSync(fd, `${JSON.stringify(result, null, 2)}\n`); } finally { fs.closeSync(fd); }
console.log(JSON.stringify({ passed: result.passed, changed_at_0_5: result.changed_at_0_5,
  malformed_input_findings: result.canaries_not_keeping_current, live_model_calls: 0, output: opts['--out'] }));
