// Actual capture CLI implementation with explicitly synthetic transport, never live inference.
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import {fileURLToPath} from 'node:url';
import {runCapture,parseOptions} from '../capture_sift.mjs';
const source=process.argv[2], out=process.argv[3];
if(!source || !out) throw Error('Use node test-capture.mjs UPSTREAM NEW_EVIDENCE_DIR');
await fs.mkdir(out,{mode:0o700});
const temp=await fs.mkdtemp(path.join(os.tmpdir(),'sift-capture-test-'));
const book=fileURLToPath(new URL('development.proposed.json',import.meta.url));
const tests=[];
const response=p=>new Response(JSON.stringify({model:'synthetic-fixture',answers:{relevant:{type:'noul',noul:p}},usage:{input_tokens:12}}));
const base={book,upstream:source,partition:'development',limit:'6',execute:true};
async function check(name,fn) {await fn();tests.push(name);}
try {
 await check('duplicate-flags-rejected',()=>assert.throws(()=>parseOptions(['--partition','development','--partition','held_out']),/Duplicate/));
 await check('human-gate-before-credentials-and-files',()=>assert.rejects(()=>runCapture({...base,out:path.join(temp,'no-approval.json')}),/Human reference approval/));
 await check('dry-run-still-no-approval',async()=>{const r=await runCapture({...base,execute:false});assert.equal(r.pending_reviews,6);assert.equal(r.model_calls,0);});
 let n=0;
 const result=await runCapture({...base,out:path.join(out,'capture.json')},{fixtureFetcher:async()=>++n===6?new Response('',{status:500}):response([.8,.6,.5,.7,.1][n-1])});
 await check('actual-pipeline-all-six-attempts',()=>{assert.equal(n,6);assert.equal(result.complete,true);assert.equal(result.origin,'synthetic_fixture');});
 const capture=JSON.parse(await fs.readFile(path.join(out,'capture.json'),'utf8'));
 await check('raw-response-retained',()=>assert.equal(JSON.parse(capture.records[0].metadata.response_text).answers.relevant.noul,.8));
 await check('operational-error-not-negative',()=>{assert.equal(capture.records[5].output.answers,undefined);assert.equal(typeof capture.records[5].output.error,'string');});
 await check('headers-and-reference-metadata-excluded',()=>{
   assert.ok(!JSON.stringify(capture).includes('synthetic-transport-key'));
   for(const r of capture.records) assert.deepEqual(Object.keys(r.request).sort(),['model','questions','state']);
 });
 await check('private-no-replace-destinations',async()=>{
   assert.equal((await fs.stat(path.join(out,'capture.json'))).mode & 0o777,0o600);
   await assert.rejects(()=>runCapture({...base,out:path.join(out,'capture.json')},{fixtureFetcher:async()=>{throw Error('must not run');}}),/EEXIST/);
 });
 await check('authentication-fails-fast',async()=>{
   let calls=0;const dest=path.join(temp,'auth.json');
   const r=await runCapture({...base,out:dest},{fixtureFetcher:async()=>{calls++;return new Response('secret echoed only in discarded error body',{status:401});}});
   assert.equal(calls,1);assert.equal(r.complete,false);assert.equal(r.not_attempted,5);
   assert.equal(JSON.parse(await fs.readFile(dest)).failure,'AuthenticationRejected');
 });
 await check('malformed-json-reaches-provider-error-path',async()=>{
   const dest=path.join(temp,'bad-json.json');
   const r=await runCapture({...base,out:dest,limit:'1'},{fixtureFetcher:async()=>new Response('{bad-json')});
   const c=JSON.parse(await fs.readFile(dest));assert.equal(r.complete,true);
   assert.match(c.records[0].output.error,/invalid or incomplete JSON/);
   assert.equal(c.records[0].metadata.response_text,'{bad-json');
 });
 await check('interrupted-case-is-attempted-not-never-started',async()=>{
   const ctrl=new AbortController(),dest=path.join(temp,'cancel.json');
   const r=await runCapture({...base,out:dest},{fixtureSignal:ctrl.signal,fixtureFetcher:async(_url,init)=>{
     ctrl.abort(new Error('test-cancellation'));throw init.signal.reason;
   }});
   const c=JSON.parse(await fs.readFile(dest));assert.equal(r.complete,false);assert.equal(r.attempted,1);assert.equal(r.not_attempted,5);
   assert.equal(c.attempts[0].status,'interrupted');assert.equal(c.records[0].output.error,'CaptureInterrupted');
   assert.ok(!c.not_attempted_ids.includes(c.records[0].case_id));
 });
 await check('pre-abort-leaves-every-case-unattempted',async()=>{
   const ctrl=new AbortController();ctrl.abort();
   const r=await runCapture({...base,out:path.join(temp,'preabort.json')},{fixtureSignal:ctrl.signal,fixtureFetcher:async()=>{throw Error('must not run');}});
   assert.equal(r.attempted,0);assert.equal(r.not_attempted,6);assert.equal(r.complete,false);
 });
 await check('book-snapshot-contains-no-human-approval',async()=>{
   const b=JSON.parse(await fs.readFile(path.join(out,'capture.json.book.json')));
   assert.equal(b.cases.length,6);assert.ok(b.cases.every(c=>c.review.state==='pending'));
 });
 await fs.writeFile(path.join(out,'tests.json'),JSON.stringify({status:'passed',tests,live_model_calls:0,human_approvals_created:0},null,2)+'\n',{mode:0o600});
 console.log(JSON.stringify({passed:tests.length,live_model_calls:0,evidence:out}));
} finally {await fs.rm(temp,{recursive:true,force:true});}
