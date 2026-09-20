#!/usr/bin/env node
// Capture the pinned application's real path. The CLI has no mock/live bypass.
// The explicitly injected test transport always produces synthetic_fixture evidence.
import fs from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';

const PIN = '966de12e2bb5f94d47886ee51f30a07ec8ef1607';
const sha = value => createHash('sha256').update(value).digest('hex');
export function parseOptions(args) {
  const options = { partition: 'development', limit: '5', execute: false };
  const seen = new Set();
  for (let i = 0; i < args.length; i++) {
    const flag = args[i];
    if (seen.has(flag)) throw new Error(`Duplicate ${flag}`);
    seen.add(flag);
    if (flag === '--execute') options.execute = true;
    else if (['--book', '--out', '--upstream', '--partition', '--limit'].includes(flag) && args[i+1] && !args[i+1].startsWith('--')) options[flag.slice(2)] = args[++i];
    else throw new Error(`Unknown or incomplete argument: ${flag}`);
  }
  return options;
}
export async function runCapture(options, { fixtureFetcher, fixtureSignal } = {}) {
  if (!options.book || !['development','held_out'].includes(options.partition)) throw new Error('Use --book PATH [--partition development|held_out] [--limit 1..80]; --execute additionally requires --upstream and --out.');
  const limit = Number(options.limit);
  if (!Number.isInteger(limit) || limit < 1 || limit > 80) throw new Error('limit must be 1..80');
  const bytes = await fs.readFile(options.book);
  const book = JSON.parse(bytes);
  if (book.example !== 'sift' || book.schema_version !== 1 || book.task?.kind !== 'single_label' || JSON.stringify(book.task.labels) !== '["irrelevant","relevant"]') throw new Error('Expected the fixed Sift binary casebook');
  if (!Array.isArray(book.cases)) throw new Error('cases must be an array');
  const ids = new Set(), families = new Map();
  for (const c of book.cases) {
    if (typeof c.case_id !== 'string' || !c.case_id.trim() || ids.has(c.case_id)) throw new Error('Missing/duplicate case identity');
    ids.add(c.case_id);
    if (!['development','held_out'].includes(c.partition) || typeof c.family_id !== 'string' || !c.family_id.trim()) throw new Error('Missing family/partition');
    if (families.has(c.family_id) && families.get(c.family_id) !== c.partition) throw new Error('Family leaks across partitions');
    families.set(c.family_id,c.partition);
    if (typeof c.model_input?.query !== 'string' || !c.model_input.query.trim() || c.model_input.query.length > 2000 || typeof c.model_input.text !== 'string' || c.model_input.text.length > 10000) throw new Error('Expected bounded query/text inputs');
    if (c.proposed_expected?.type !== 'class' || !book.task.labels.includes(c.proposed_expected.label)) throw new Error('Invalid reference target');
  }
  const eligible = book.cases.filter(c=>c.partition===options.partition), selected = eligible.slice(0,limit);
  if (!selected.length) throw new Error('No selected cases');
  const projections = selected.map(c=>({query:c.model_input.query,items:[{id:c.case_id,text:c.model_input.text}]}));
  const approved = c=>c.review?.state==='approved' && c.review?.method==='human' && typeof c.review?.reviewer==='string' && c.review.reviewer.trim();
  if (!options.execute) return {dry_run:true,model_calls:0,selected:selected.length,total_in_partition:eligible.length,pending_reviews:selected.filter(c=>!approved(c)).length,application_inputs:projections};
  if (!options.out || !options.upstream) throw new Error('Capture needs --out and --upstream');
  const fixture = typeof fixtureFetcher === 'function';
  if (fixtureSignal && !fixture) throw new Error('Test signal needs a test transport');
  if (!fixture && !selected.every(approved)) throw new Error('Human reference approval is required before a target-model call');
  const key = fixture ? 'synthetic-transport-key' : process.env.TYPESAFE_API_KEY || process.env.JEV_API_KEY;
  if (!key) throw new Error('Set TYPESAFE_API_KEY or JEV_API_KEY securely; never pass a key on the command line');
  const source = path.resolve(options.upstream);
  if (execFileSync('git',['-C',source,'rev-parse','HEAD'],{encoding:'utf8'}).trim()!==PIN) throw new Error('Wrong upstream revision');
  if (execFileSync('git',['-C',source,'status','--porcelain','--untracked-files=all','--','src'],{encoding:'utf8'}).trim()) throw new Error('Upstream source has local changes');
  const {classify} = await import(pathToFileURL(path.join(source,'src/classify.js')));
  const {createProvider} = await import(pathToFileURL(path.join(source,'src/provider.js')));
  const subsetBytes = `${JSON.stringify({...book,cases:selected},null,2)}\n`;
  const output = await fs.open(options.out,'wx',0o600);
  let subset;
  try {subset=await fs.open(`${options.out}.book.json`,'wx',0o600);}
  catch(error) {await output.close();await fs.unlink(options.out);throw error;}
  const records=[], attempts=[];
  const signal = fixtureSignal || AbortSignal.timeout(120000);
  const capture = {schema_version:1,example:'sift',origin:fixture?'synthetic_fixture':'observed',complete:false,
    source:{model:fixture?'synthetic-fixture':'jev-latest',requested_model:'jev-latest',repository:'kbhuw/jev-sift',commit:PIN},records,attempts,
    reference_binding:{book_sha256:sha(subsetBytes),source_book_sha256:sha(bytes),selected_ids:selected.map(c=>c.case_id)},
    selection:{partition:options.partition,selected:selected.length,total_in_partition:eligible.length},
    limits:{requests:selected.length,retries:0,request_timeout_ms:15000,overall_timeout_ms:120000}};
  let stopped=false;
  try {
    await subset.writeFile(subsetBytes);
    for (let i=0;i<selected.length;i++) {
      signal.throwIfAborted();
      let request,responseBody,status,rawText;
      const attempt={case_id:selected[i].case_id,status:'started'};attempts.push(attempt);
      try {
        const evaluate=createProvider({apiKey:key,timeoutMs:15000},async(url,init)=>{
          request=JSON.parse(init.body); // Authentication headers are never retained.
          const response=await (fixtureFetcher || fetch)(url,init);
          status=response.status;
          if (response.ok) {
            rawText=await response.clone().text();
            try {responseBody=JSON.parse(rawText);} catch { /* let the original provider classify its JSON error */ }
          }
          return response;
        });
        const result=await classify(projections[i],{evaluate,concurrency:1,signal});
        const native=result.results[0];
        records.push({case_id:selected[i].case_id,output:native,request,
          model:native.model || capture.source.model,configuration:{query:projections[i].query},
          metadata:{http_status:status,response:responseBody,response_text:rawText,
            resolved_model_supplied:typeof responseBody?.model==='string'}});
        attempt.status=native.error?'error':'answered';
        if (status===401 || status===403) {capture.failure='AuthenticationRejected';stopped=true;break;}
      } catch(error) {
        attempt.status='interrupted';
        records.push({case_id:selected[i].case_id,output:{id:selected[i].case_id,error:'CaptureInterrupted'},request,
          model:capture.source.model,metadata:{http_status:status,response:responseBody,response_text:rawText}});
        throw error;
      }
    }
    capture.complete=!stopped && records.length===selected.length;
  } catch(error) {capture.failure=error?.name || 'CaptureError';}
  finally {
    capture.not_attempted_ids=selected.slice(attempts.length).map(c=>c.case_id);
    let text=`${JSON.stringify(capture,null,2)}\n`;
    if (!fixture && text.includes(key)) {
      // Never persist a provider echo of the credential. Do not call this a complete capture.
      capture.complete=false;capture.failure='CredentialEchoSuppressed';capture.records=[];
      text=`${JSON.stringify(capture,null,2)}\n`;
    }
    try {await output.writeFile(text);} finally {await output.close();await subset.close();}
  }
  return {complete:capture.complete,origin:capture.origin,recorded:capture.records.length,attempted:attempts.length,
    not_attempted:capture.not_attempted_ids.length,output:options.out};
}
if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  try {const result=await runCapture(parseOptions(process.argv.slice(2)));console.log(JSON.stringify(result,null,2));if(result.complete===false)process.exitCode=2;}
  catch(error) {console.error(JSON.stringify({status:'error',message:error.message}));process.exitCode=2;}
}
