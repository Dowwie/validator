#!/usr/bin/env node
// Original sponsor controller and moderation processor, explicit in-memory replies.
// No browser, speech service, agent runtime, actual user content or model calls.
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {fileURLToPath, pathToFileURL} from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const args = {};
for (let i=2;i<process.argv.length;i+=2) {
  assert.ok(['--sources','--out'].includes(process.argv[i]) && process.argv[i+1] && !args[process.argv[i]], 'Use --sources DIR --out NEW_FILE');
  args[process.argv[i]]=process.argv[i+1];
}
assert.ok(args['--sources'] && args['--out']);
assert.ok(!fs.existsSync(args['--out']), 'never replace capture output');
const pins=JSON.parse(fs.readFileSync(path.join(here,'sources.json')));
for (const slug of ['sponsor','moderation']) for (const [name,identity] of Object.entries(pins[slug].files)) {
  const p=path.join(args['--sources'],slug,name);
  assert.ok(!fs.lstatSync(p).isSymbolicLink());
  assert.equal(createHash('sha256').update(fs.readFileSync(p)).digest('hex'),identity.sha256,`${slug}/${name}`);
}
const fixture=JSON.parse(fs.readFileSync(path.join(here,'native-cases.json')));
assert.equal(fixture.origin,'synthetic_fixture');
globalThis.fetch=()=>{throw Error('NETWORK_FORBIDDEN: fixture only');};
const live=await import(pathToFileURL(path.join(args['--sources'],'sponsor/src/live.js')));
const mod=await import(pathToFileURL(path.join(args['--sources'],'moderation/jev-moderation.ts')));
const checks=[];
async function check(name,fn) {await fn();checks.push(name);}
const utterance=(text,at=0,until=7000)=>({text,heardAt:at,heardUntil:until});
const long='These words form a sufficiently long neutral transcript so that the actual controller has enough evidence to request classification.';
const source=slug=>({repository:pins[slug].repository,commit:pins[slug].commit});
const tasks={sponsor:{source:source('sponsor'),cases:fixture.sponsor,thresholds:{baseline:.7,candidate:.9}},
             moderation:{source:source('moderation'),cases:fixture.moderation,thresholds:{baseline:.7,candidate:.9}}};

function sponsorCase(c, threshold) {
  const controller=live.createLiveController({threshold,...c.input.options});
  const trace=[];
  for (const e of c.input.events) {
    let result=null;
    if(e.op==='hear') controller.hear(utterance(e.text,e.at,e.until));
    else if(e.op==='skipped') controller.skipped(e.at);
    else throw Error('unknown fixture event');
    trace.push({event:e,phase:controller.phase,result});
  }
  const request=controller.next(c.input.now);
  const response=request ? {answers:Object.fromEntries(Object.keys(request.questions).map(k=>[k,{type:'noul',noul:c.reply}]))} : null;
  if(c.reply===null && response) response.answers={};
  const action=request ? controller.answer(response,c.input.now) : null;
  return {id:c.id,input:c.input,action:action?'skip':'keep',probability:response && c.reply!==null?c.reply:null,
          request,response,phase:controller.phase,trace,log:controller.log,
          status:request?(c.reply===null?'missing_model_answer':'answered'):'not_requested'};
}
for(const arm of ['baseline','candidate']) tasks.sponsor[arm]=fixture.sponsor.map(c=>{
  const r=sponsorCase(c,tasks.sponsor.thresholds[arm]);assert.equal(r.action,c.expected_policy[arm],c.id);return r;
});
checks.push('sponsor-12-scenarios-both-thresholds-exact-actions');
await check('sponsor-noul-not-native-confidence',()=>{
  const r=tasks.sponsor.baseline.find(x=>x.id==='s02');assert.equal(r.request.questions.sponsor_now.type,'noul');assert.equal(r.probability,.7);
});
await check('sponsor-internal-utterance-order-preserved',()=>{
  const lines=[utterance('first'),utterance('second')];
  assert.equal(live.renderHeard(lines),'L001| first\nL002| second');
  assert.notEqual(live.renderHeard(lines),live.renderHeard([...lines].reverse()));
});
await check('sponsor-no-two-outstanding-requests',()=>{
  const c=live.createLiveController();c.hear(utterance(long));assert.ok(c.next(7000));assert.equal(c.next(9000),null);
});
await check('sponsor-no-unrequested-answer-action',()=>assert.equal(live.createLiveController().answer({answers:{sponsor_now:{noul:1}}},1000),null));
await check('sponsor-preseek-late-transcript-is-not-new-evidence',()=>{
  const c=live.createLiveController();c.skipped(10000);c.hear(utterance(long,9999,17000));assert.equal(c.heard.length,0);
  c.hear(utterance(long,10000,17000));assert.equal(c.heard.length,1);assert.ok(c.next(17000));
});
await check('sponsor-silence-timeout-resets-without-skip',()=>{
  const c=live.createLiveController();c.skipped(10000);assert.equal(c.next(49999),null);assert.equal(c.phase,'verifying');
  assert.equal(c.next(50000),null);assert.equal(c.phase,'listening');assert.equal(c.log.at(-1).kind,'timeout');
});
await check('sponsor-skip-limit-is-policy-not-negative-noul',()=>{
  const r=tasks.sponsor.baseline.find(x=>x.id==='s07');assert.equal(r.probability,.99);assert.equal(r.action,'keep');assert.equal(r.log.at(-1).kind,'limit');
});

let queue=[],requests=[];
globalThis.fetch=async(url,init)=>{
  assert.equal(url,mod.JEV_ENDPOINT);assert.equal(init.method,'POST');
  requests.push(JSON.parse(init.body));
  const q=queue.shift();assert.ok(q,'unplanned call');
  if(q.error) throw Error('fixture transport failure');
  return new Response(q.badJSON?'{invalid':JSON.stringify(q.body??{}),{status:q.status??200,headers:{'content-type':'application/json'}});
};
function response(probability, category='none') {
  const answers={blocking:{noul:probability}};
  if(category!==null) answers.category={choice:category};
  return {body:{answers,model:'synthetic-fixture',usage:{input_tokens:10}}};
}
function messages(text, legacy=false) {
  return [{role:'user',content:legacy?{content:text,parts:[]}:{parts:[{type:'text',text}]}}];
}
async function moderationCase(c,threshold) {
  queue=[];requests=[];let verdict=null,blocked=false;const logs=[];
  if(c.reply.type==='answer') queue.push(response(c.reply.probability,c.reply.category));
  else if(c.reply.type==='http_error') queue.push({status:c.reply.status});
  else if(c.reply.type!=='not_requested') throw Error('unknown fixture reply');
  const p=mod.createJevModerationProcessor({apiKey:'fixture-not-a-key',threshold,maxChars:c.input.maxChars??8000,
    logger:{warn:(fields,text)=>logs.push({fields:{...fields,err:fields.err?String(fields.err):undefined},text})},
    onVerdict:v=>{verdict=v;}});
  const input=messages(c.input.text,c.input.legacy);
  const output=await p.processInput({messages:input,abort:reason=>{blocked=true;assert.equal(reason,'MESSAGE_BLOCKED');return {aborted:true};}});
  assert.equal(queue.length,0);
  if(!blocked) assert.equal(output,input,'fail-open and pass paths preserve messages');
  return {id:c.id,input:c.input,action:blocked?'block':'allow',probability:verdict?.score??null,
          request:requests[0]??null,verdict,logs,status:verdict?'answered':requests.length?'fail_open':'unchecked_empty'};
}
for(const arm of ['baseline','candidate']) {
  tasks.moderation[arm]=[];
  for(const c of fixture.moderation) {
    const r=await moderationCase(c,tasks.moderation.thresholds[arm]);
    assert.equal(r.action,c.expected_policy[arm],c.id);tasks.moderation[arm].push(r);
  }
}
checks.push('moderation-10-scenarios-both-thresholds-exact-actions');
await check('moderation-actual-two-native-questions',()=>{
  const r=tasks.moderation.baseline[0];assert.equal(r.request.questions.blocking.type,'noul');assert.equal(r.request.questions.category.type,'choice');
  assert.equal(r.request.state.message,r.input.text);assert.ok(!('expected' in r.request));
});
await check('moderation-category-is-not-the-block-gate',()=>{
  assert.equal(tasks.moderation.baseline.find(x=>x.id==='m05').action,'block');
  assert.equal(tasks.moderation.baseline.find(x=>x.id==='m06').action,'allow');
});
await check('moderation-missing-category-cannot-bypass-block',()=>{
  const r=tasks.moderation.baseline.find(x=>x.id==='m07');assert.equal(r.verdict.category,'none');assert.equal(r.action,'block');
});
await check('moderation-http-failure-remains-action-not-model-no',()=>{
  const r=tasks.moderation.baseline.find(x=>x.id==='m08');assert.equal(r.action,'allow');assert.equal(r.probability,null);assert.equal(r.status,'fail_open');
});
await check('moderation-last-message-only-and-truncation',async()=>{
  queue=[response(.1)];requests=[];
  const p=mod.createJevModerationProcessor({apiKey:'fixture-not-a-key',maxChars:5});
  await p.processInput({messages:[...messages('do not forward history'),...messages('abcdefghi')],abort:()=>{throw Error('unexpected block');}});
  assert.deepEqual(requests[0].state,{message:'abcde'});
});
await check('moderation-invalid-json-fails-open',async()=>{
  queue=[{badJSON:true}];requests=[];
  const p=mod.createJevModerationProcessor({apiKey:'fixture-not-a-key',logger:{warn:()=>{}}});
  const m=messages('neutral input');assert.equal(await p.processInput({messages:m,abort:()=>{throw Error('unexpected block');}}),m);
});
await check('moderation-missing-gate-is-not-p0',async()=>{
  queue=[{body:{answers:{category:{choice:'none'}}}}];requests=[];
  await assert.rejects(()=>mod.moderateWithJev('neutral',{apiKey:'fixture-not-a-key',signal:new AbortController().signal}),/no parsable verdict/);
});
await check('moderation-circuit-open-cooldown-and-recovery',async()=>{
  let now=0,calls=0,fail=true,opened=0,recovered=0;
  const f=mod.withResilience(async()=>{calls++;if(fail)throw Error('fixture');return 1;},
    {now:()=>now,timeoutMs:1000,breakerThreshold:2,breakerCooldownMs:100,onBreakerOpen:()=>opened++,onBreakerRecovered:()=>recovered++});
  await assert.rejects(()=>f('x'));await assert.rejects(()=>f('x'));await assert.rejects(()=>f('x'),/breaker open/);assert.equal(calls,2);assert.equal(opened,1);
  now=100;fail=false;assert.equal(await f('x'),1);assert.equal(calls,3);assert.equal(recovered,1);
});
await check('moderation-tripwire-reason-distinct',()=>{
  assert.ok(mod.isModerationBlock({reason:'MESSAGE_BLOCKED'}));assert.ok(!mod.isModerationBlock({reason:'ordinary stop'}));
});
const dependency=JSON.parse(fs.readFileSync(path.join(args['--sources'],'moderation/node_modules/zod/package.json')));
assert.equal(dependency.version,'3.25.76');
const result={schema_version:1,origin:'synthetic_fixture',live_model_calls:0,human_approvals_created:0,
  tasks,checks,node:process.version,zod:dependency.version,
  limitations:['Synthetic replies test software paths, not model accuracy.','Sponsor tests the live controller, not browser/audio or recorded-transcript localization.',
    'Moderation executes original processor and HTTP parsing; no actual Mastra agent session.','No arbitrary-score-to-probability conversion; no real media or private messages used.']};
const fd=fs.openSync(args['--out'],'wx',0o600);
try{fs.writeFileSync(fd,JSON.stringify(result,null,2)+'\n');}finally{fs.closeSync(fd);}
console.log(JSON.stringify({checks:checks.length,fixture_cases:fixture.sponsor.length+fixture.moderation.length,live_model_calls:0}));
