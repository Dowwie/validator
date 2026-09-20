#!/usr/bin/env node
// Full pinned Sift classify + provider path, with in-memory HTTP responses only.
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import { execFileSync } from 'node:child_process';

const opts = {};
for (let i = 2; i < process.argv.length; i += 2) {
  const flag = process.argv[i];
  if (!['--upstream','--book','--out'].includes(flag) || !process.argv[i+1] || opts[flag]) throw Error('Use --upstream DIR --book FILE --out NEW_FILE');
  opts[flag] = process.argv[i+1];
}
for (const k of ['--upstream','--book','--out']) if (!opts[k]) throw Error(`${k} required`);
const root = path.resolve(opts['--upstream']);
const pin = '966de12e2bb5f94d47886ee51f30a07ec8ef1607';
assert.equal(execFileSync('git',['-C',root,'rev-parse','HEAD'],{encoding:'utf8'}).trim(), pin);
assert.equal(execFileSync('git',['-C',root,'diff','--name-only','HEAD','--','src'],{encoding:'utf8'}).trim(), '');
globalThis.fetch = () => { throw Error('NETWORK_FORBIDDEN: this is a transport fixture'); };
const {classify, inputSchema} = await import(pathToFileURL(path.join(root,'src/classify.js')));
const {createProvider, fromJevAnswers} = await import(pathToFileURL(path.join(root,'src/provider.js')));
const tests = [];
async function check(name, fn) { await fn(); tests.push(name); }
const response = (answers, extra={}) => new Response(JSON.stringify({model:'synthetic-fixture',answers,usage:{input_tokens:17},...extra}), {headers:{'content-type':'application/json'}});
const noul = p => ({relevant:{type:'noul',noul:p}});
const query = text => ({query:'Evidence of software sold to hospitals?',items:[{id:'one',text}]});
const provider = fetcher => createProvider({apiKey:'fixture-only-never-transmitted',timeoutMs:1000},fetcher);
await check('all-80-draft-inputs-accepted-by-real-schema', () => {
  const book = JSON.parse(fs.readFileSync(opts['--book'],'utf8'));
  assert.equal(book.cases.length,80);
  for (const c of book.cases) inputSchema.parse({query:c.model_input.query,items:[{id:c.case_id,text:c.model_input.text}]});
});
await check('native-noul-request-and-model-usage-preserved', async () => {
  let seen;
  const out = await classify(query('Vendor text'), {evaluate:provider(async (url,init) => {
    seen = JSON.parse(init.body); assert.equal(url,'https://api.typesafe.ai/v1/systemone');
    assert.equal(init.redirect,'error'); return response(noul(.5));
  })});
  assert.equal(seen.state,'Vendor text'); assert.equal(seen.questions.relevant.type,'noul');
  assert.equal(out.results[0].answers.relevant.probability,.5);
  assert.equal(out.results[0].model,'synthetic-fixture'); assert.equal(out.usage.inputTokens,17);
});
await check('choice-confidence-distinct-from-selected-probability', async () => {
  const out=await classify({items:[{id:'one',text:'text'}],questions:{category:{type:'choice',instructions:'Choose',criteria:{a:'a',b:'b'}}}},
    {evaluate:provider(async()=>response({category:{type:'choice',choice:'a',confidence:.9,probabilities:{a:.6,b:.4}}}))});
  assert.equal(out.results[0].answers.category.confidence,.9); assert.equal(out.results[0].answers.category.probabilities.a,.6);
});
await check('score-scale-and-distribution-preserved', async () => {
  const out=await classify({items:[{id:'one',text:'text'}],questions:{rating:{type:'score',instructions:'Grade',criteria:['low','medium','high']}}},
    {evaluate:provider(async()=>response({rating:{type:'score',score:1.4,confidence:.8,probabilities:{'0':.1,'1':.4,'2':.5},legend:{'0':'low','1':'medium','2':'high'}}}))});
  assert.equal(out.scales.rating,2); assert.equal(out.results[0].answers.rating.score,1.4);
});
await check('concurrent-results-retain-input-order', async () => {
  const out=await classify({query:'Relevant?',items:[{id:'slow',text:'slow'},{id:'fast',text:'fast'}]}, {concurrency:2,
    evaluate:provider(async(_url,init)=>{if(JSON.parse(init.body).state==='slow')await new Promise(r=>setTimeout(r,15));return response(noul(.4));})});
  assert.deepEqual(out.results.map(x=>x.id),['slow','fast']);
});
for(const [name,answers] of [['missing-answer',{}],['out-of-range',noul(1.1)],['wrong-type',{relevant:{type:'invented'}}]]) {
  await check(`${name}-becomes-explicit-item-error`,async()=>{
    const out=await classify(query('text'),{evaluate:provider(async()=>response(answers))});
    assert.equal(out.ok,false);assert.equal(typeof out.results[0].error,'string');assert.equal(out.results[0].answers,undefined);
  });
}
for(const status of [401,429,500]) await check(`http-${status}-no-hidden-retry-or-secret-echo`,async()=>{
  let n=0;
  const out=await classify(query('text'),{evaluate:provider(async()=>{n++;return new Response('fixture-only-never-transmitted',{status});})});
  assert.equal(n,1);assert.equal(out.ok,false);assert.ok(!JSON.stringify(out).includes('fixture-only-never-transmitted'));
});
await check('invalid-json-is-explicit-error',async()=>{
  const out=await classify(query('text'),{evaluate:provider(async()=>new Response('{bad'))}); assert.equal(out.ok,false);
});
await check('truncation-preserves-flag-and-bounds-effective-state',async()=>{
  let seen;
  const out=await classify({query:'Relevant?',items:[{id:'one',path:'/fictional.txt'}]},
    {readText:async()=> 'x'.repeat(60001), evaluate:provider(async(_url,init)=>{seen=JSON.parse(init.body).state;return response(noul(.6));})});
  assert.equal(seen.length,60000);assert.equal(out.results[0].truncated,true);
});
await check('missing-file-reader-is-not-irrelevant',async()=>{
  const out=await classify({query:'Relevant?',items:[{id:'one',path:'/fictional.txt'}]},{evaluate:provider(async()=>{throw Error('must not call')})});
  assert.equal(out.ok,false);assert.equal(out.results[0].answers,undefined);
});
await check('pre-aborted-request-stops-before-transport',async()=>{
  const c=new AbortController();c.abort(new Error('fixture-cancelled'));let n=0;
  await assert.rejects(()=>classify(query('text'),{signal:c.signal,evaluate:provider(async()=>{n++;return response(noul(.4));})}),/fixture-cancelled/);
  assert.equal(n,0);
});
await check('duplicate-ids-rejected',async()=>{
  await assert.rejects(()=>classify({query:'Relevant?',items:[{id:'a',text:'x'},{id:'a',text:'y'}]},{evaluate:()=>{throw Error('must not call')}}));
});
await check('mixed-success-and-error-retains-both-ids',async()=>{
  const out=await classify({query:'Relevant?',items:[{id:'good',text:'good'},{id:'bad',text:'bad'}]},
    {evaluate:provider(async(_url,init)=>JSON.parse(init.body).state==='bad'?new Response('',{status:500}):response(noul(.8)))});
  assert.equal(out.ok,true);assert.equal(out.results.length,2);assert.equal(typeof out.results[1].error,'string');
});
await check('invalid-answer-container-rejected',()=>{assert.throws(()=>fromJevAnswers([]));});
const book=JSON.parse(fs.readFileSync(opts['--book'],'utf8'));
const cases=book.cases.filter(x=>x.partition==='development').slice(0,3);
const records=[];
for (const [i,c] of cases.entries()) {
  let request;
  const nativeResponse={model:'synthetic-fixture',answers:noul([.2,.5,.8][i]),usage:{input_tokens:17}};
  const out=await classify({query:c.model_input.query,items:[{id:c.case_id,text:c.model_input.text}]},
    {evaluate:provider(async(_url,init)=>{request=JSON.parse(init.body);return new Response(JSON.stringify(nativeResponse));})});
  assert.ok(!['proposed_expected','review','rationale','partition'].some(k=>k in request));
  records.push({case_id:c.case_id,output:out.results[0],request,metadata:{response:nativeResponse,transport:'in-memory-fixture'}});
}
const result={schema_version:1,example:'sift',origin:'synthetic_fixture',complete:true,live_model_calls:0,
  source:{model:'synthetic-fixture',repository:'kbhuw/jev-sift',commit:pin},records,tests,
  scope:'Unmodified classify/provider/config and installed lockfile dependencies; only HTTP transport replaced. This is not live inference or approved semantic gold.'};
const fd=fs.openSync(opts['--out'],'wx',0o600);try{fs.writeFileSync(fd,JSON.stringify(result,null,2)+'\n');}finally{fs.closeSync(fd);}
console.log(JSON.stringify({passed:tests.length,live_model_calls:0,capture:opts['--out']}));
