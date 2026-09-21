// Execute only the unchanged, import-free upstream question builder. No inference.
import fs from 'node:fs';
import { createHash } from 'node:crypto';
import { pathToFileURL } from 'node:url';
import assert from 'node:assert/strict';
const path = process.argv[2];
assert.ok(path, 'provide pinned questions.ts');
const data = fs.readFileSync(path);
assert.equal(createHash('sha1').update(`blob ${data.length}\0`).update(data).digest('hex'), '59f7aa60227dd5b40ff2d639a53581ed0d3757b2');
globalThis.fetch = () => { throw Error('NETWORK_FORBIDDEN'); };
const { QUESTIONS, questionsFor, questionsOf, JUDGE_STATE } = await import(pathToFileURL(path));
assert.equal(QUESTIONS.length, 34);
assert.equal(QUESTIONS.filter(q => q.type === 'noul').length, 31);
assert.equal(QUESTIONS.filter(q => q.type === 'score').length, 3);
const scopes = {};
for (const [name, file, expected, test, patch] of [
  ['whole', {path:'src/change.ts'},32,false,false],
  ['test', {path:'tests/change.test.ts'},33,true,false],
  ['patch', {path:'src/change.ts',patch:true},33,false,true],
  ['test-patch', {path:'tests/change.test.ts',patch:true},34,true,true],
]) {
  const rows = questionsFor(file), native = questionsOf(rows), ids = rows.map(q=>q.id);
  assert.equal(rows.length, expected);
  assert.equal(ids.includes('unclear_tests'), test);
  assert.equal(ids.includes('leaves_it_worse'), patch);
  assert.equal(native.too_many_arguments.type, 'boolean');
  assert.equal(native.verdict.type, 'score');
  assert.equal(native.verdict.criteria.length, 5);
  for (const id of ['too_many_arguments','flag_or_output_arguments','commented_out_code','swallowed_errors']) assert.ok(ids.includes(id));
  scopes[name]={ids, noul_count:rows.filter(q=>q.type==='noul').length, score_count:3, request:native};
}
for(const path of ['a/specs/code.ts','src/a.test.ts','src/test_code.py']) assert.ok(questionsFor({path}).some(q=>q.id==='unclear_tests'));
assert.equal(JUDGE_STATE.whole.code,'');
assert.equal(JUDGE_STATE.patch.code_after_change,'');
console.log(JSON.stringify({...scopes, origin:'pinned-native-question-builder',live_model_calls:0, source_git_blob:'59f7aa60227dd5b40ff2d639a53581ed0d3757b2'}));
