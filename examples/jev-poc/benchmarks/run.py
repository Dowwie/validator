#!/usr/bin/env python3
"""Execute published bilingual, moderation, and pairwise evidence with Validator.

This command is offline. Acquisition is separate. It does not call a provider,
approve references, run a bot, or recommend financial actions.
"""
from __future__ import annotations

import argparse
import copy
from fractions import Fraction
import hashlib
import json
import math
from pathlib import Path
import shutil
import sys

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import readers
sys.path.insert(0, str(HERE.parent))
from poc import command, digest, encoded, episode_id, native_run, read_json, write_bundle, write_private
from madewithjev.oracle import close, multilabel, require, single, transition_ids, verify_single


def value(outcome):
    kind = outcome['type']
    if kind == 'abstention':
        return None
    return outcome['label'] if kind == 'class' else outcome['labels']


def verify_multi_family(report, gold, predictions, labels, family):
    wanted = multilabel(gold, predictions, labels)
    actual = report[family]
    for key in ('total', 'answered', 'abstained', 'exact_matches', 'wrong_sets'):
        require(actual[key] == wanted[key], f'{family}.{key}')
    require({r['label'] for r in actual['labels']} == set(labels), 'label identity')
    for row in actual['labels']:
        for key, n in wanted['labels'][row['label']].items():
            require(row[key] == n, f'{family}.{row["label"]}.{key}')
        require(sum(wanted['labels'][row['label']].values()) == wanted['answered'], 'count conservation')
    for key in ('exact_match_accuracy', 'answered_hamming_loss'):
        close(actual[key], wanted[key], key)
    return wanted


def summarize_single(result):
    return {k: v for k, v in result.items() if k != 'matrix'}


def verify_transitions(comparison, slug, gold, before, after):
    wanted = transition_ids(gold, before, after)
    result = {}
    for key, source_ids in wanted.items():
        expected = sorted(episode_id(slug, cid) for cid in source_ids)
        actual = comparison['transitions'][key]
        require(actual['count'] == len(expected), f'{key} count mismatch')
        require(sorted(actual['ids']) == expected, f'{key} ID mismatch')
        result[key] = {'count': len(expected), 'source_ids': source_ids}
    return result


def probability_audit(report, prepared):
    """Separate standard-library arithmetic on the canonical evidence, no production scorer.

Checks selected population, Brier/log loss or the explicit missing-family status.
No clipping, renormalization, or invented missing probability is permitted.
"""
    g = read_json(prepared/'golden.json'); p = read_json(prepared/'predictions.json')
    labels = g['task']['labels']; multi = g['task']['kind']=='multi_label'
    targets = {r['id']: r['expected'] for r in g['episodes']}
    rows = p['predictions']
    family = report['probability']
    available = bool(rows) and all('probabilities' in r for r in rows)
    if not available:
        keys = ('mean_binary_brier', 'mean_binary_log_loss') if multi else ('brier_score', 'log_loss')
        for key in keys:
            require(family[key]['status'] == 'not_applicable' and family[key]['value'] is None
                    and family[key]['population_count'] == 0, 'unavailable probability family must be explicit')
        return {'status': 'not_applicable', 'checked': True}
    terms_brier, terms_loss = [], []
    for r in rows:
        probs = r['probabilities']['values']; target = targets[r['id']]
        if multi:
            for label in labels:
                y = int(label in target['labels']); prob = probs[label]
                terms_brier.append((prob-y)**2)
                q = prob if y else 1-prob
                terms_loss.append(-math.log(q) if q else math.inf)
        else:
            terms_brier.append(sum((probs[label]-int(label==target['label']))**2 for label in labels))
            q = probs[target['label']]
            terms_loss.append(-math.log(q) if q else math.inf)
    brier = math.fsum(terms_brier)/len(terms_brier)
    loss = math.inf if any(math.isinf(x) for x in terms_loss) else math.fsum(terms_loss)/len(terms_loss)
    brier_key, loss_key = ('mean_binary_brier', 'mean_binary_log_loss') if multi else ('brier_score', 'log_loss')
    close(family[brier_key], brier, brier_key)
    require(family[brier_key]['status'] == 'defined', 'Brier status')
    if math.isinf(loss):
        require(family[loss_key]['status'] == 'positive_infinity' and family[loss_key]['value'] is None, 'infinite log-loss status')
    else:
        close(family[loss_key], loss, loss_key)
        require(family[loss_key]['status'] == 'defined', 'log-loss status')
    require(family[brier_key]['population_count'] == len(terms_brier), 'probability denominator')
    if multi:
        for item in family['labels']:
            label = item['label']
            probs = [r['probabilities']['values'][label] for r in rows]
            bits = [int(label in targets[r['id']]['labels']) for r in rows]
            close(item['binary_brier'], math.fsum((p-y)**2 for p,y in zip(probs,bits))/len(rows), label+' Brier')
    return {'status': 'available', 'checked': True, 'independent_brier': brier,
            'independent_log_loss': 'positive_infinity' if math.isinf(loss) else loss}


def task_run(binary, task, out, rubric):
    b = task['book']; slug = b['example']; labels = b['task']['labels']; multi = b['task']['kind']=='multi_label'
    gold = {c['case_id']: c['proposed_expected']['labels' if multi else 'label'] for c in b['cases']}
    out.mkdir(parents=True, exist_ok=False, mode=0o700)
    results, raw_results, artifacts = {}, {}, {}
    raw_reference = {}
    for a in task['arms']:
        name = a['name']; prepared = out/(name+'-prepared'); destination = out/name
        capture = {'schema_version':1, 'example':slug, 'origin':'published_replay', 'complete':True,
                   'source':{**b['source'], 'model':a['model']}, 'arm':name,
                   'limitations':a['limitations'], 'records':a['records']}
        extra = {'benchmark-reader.py': (HERE/'readers.py').read_bytes(), 'benchmark-runner.py': Path(__file__).read_bytes(),
                 'source-lock.json': (HERE/'sources.lock.json').read_bytes(), 'task-rubric.json':encoded(rubric)}
        extra.update(a['evidence'])
        receipt = write_bundle(b, capture, a['rows'], prepared, 'development', {'arm':name, 'decision':a['decision']},
                               accept_author_fixture=True, extra_evidence=extra)
        config = read_json(prepared/'config.json'); config['decision'] = a['decision']
        (prepared/'config.json').write_bytes(encoded(config))
        report, native_receipt = native_run(binary, prepared, destination)
        raw = {r['case_id']: value(r['prediction']['outcome']) for r in a['rows']}
        final = a['expected_predictions']
        if multi:
            raw_result = verify_multi_family(report, gold, raw, labels, 'raw')
            result = verify_multi_family(report, gold, final, labels, 'final')
        else:
            raw_result = summarize_single(verify_single(report, gold, raw, labels, 'raw'))
            result = summarize_single(verify_single(report, gold, final, labels))
        result['probability_audit'] = probability_audit(report, prepared)
        result['demoted_scoring_families'] = receipt['demoted_partial_scoring_families']
        results[name] = result; raw_results[name] = raw_result; raw_reference[name] = final
        artifacts[name] = {'dataset_sha256': receipt['dataset_sha256'], 'report_sha256': native_receipt['result_sha256'],
                           'record_count':len(a['records']), 'records_sha256':digest(encoded(a['records']))}
    require(len({a['dataset_sha256'] for a in artifacts.values()})==1, 'comparison golden bytes differ')
    comparisons = {}
    for before, after, purpose in task['comparisons']:
        name = before+'--'+after
        receipt = command(binary, ['compare','--baseline',out/before,'--candidate',out/after,'--out',out/name])
        path = Path(receipt['result_path']); require(digest(path.read_bytes()) == receipt['result_sha256'], 'comparison receipt')
        comparison = read_json(path)
        transitions = verify_transitions(comparison, slug, gold, raw_reference[before], raw_reference[after])
        inspected = {}
        for key in ('recovered','regressed'):
            ids = transitions[key]['source_ids'][:2]
            for cid in ids:
                for which in (before,after):
                    result = command(binary,['inspect','--run',out/which,'--episode',episode_id(slug,cid)])
                    require(result['episode_id']==episode_id(slug,cid),'selected inspection identity')
                inspected[cid] = key
        comparisons[name] = {'purpose':purpose, 'transitions':transitions,
                            'comparison_sha256':receipt['result_sha256'], 'inspected':inspected}
    # Verify portable bundle and corruption rejection on a disposable copy.
    first = task['arms'][0]['name']; copy_path=out/'relocation-proof'
    shutil.copytree(out/first,copy_path)
    eid=episode_id(slug,next(iter(gold)))
    command(binary,['inspect','--run',copy_path,'--episode',eid])
    (copy_path/'evidence/0.bin').write_bytes(b'Intentional negative-test corruption')
    error=command(binary,['inspect','--run',copy_path,'--episode',eid],expected_exit=2)
    require(error['code']=='E_PROVENANCE','tampering must fail')
    shutil.rmtree(copy_path)
    return {'task':b['task'], 'source':b['source'], 'origin':'published_replay', 'reference_status':'upstream_reference_not_new_approval',
            'disclosures':task['disclosures'], 'arms':results, 'comparisons':comparisons, 'artifacts':artifacts,
            'checks':['input admission','native evaluation','exact raw/final accounting','all transition IDs','selected inspection','relocation','tamper rejection']}


def feedback(results, rubric):
    out = {}
    for slug, task in results.items():
        arms=task['arms']
        if slug.startswith('bench-acento-'):
            def acc(name):
                x=arms[name]; return Fraction(x['correct'],x['total'])
            delta=acc('B-pass0')-acc('A-pass0'); wording=acc('C-pass0')-acc('B-pass0')
            drift=max(abs(acc(f'{a}-pass1')-acc(f'{a}-pass0')) for a in 'ABC')
            limit=Fraction(str(rubric['bilingual']['maximum_accuracy_drop']))
            out[slug]={'language_delta':float(delta),'instruction_delta':float(wording),'largest_repeat_accuracy_change':float(drift),
                       'language_guard_met':delta>=-limit,
                       'recommendation':'Review language-specific regressions under a fixed instruction. Treat the two repeats as observed stability, not a confidence interval or general guarantee.'}
        elif slug.startswith('bench-moderation-'):
            if task['task']['kind']=='multi_label':
                changes={l: {k: arms['uniform-0.5']['labels'][l][k]-arms['shipped']['labels'][l][k] for k in ('false_positive','false_negative')} for l in task['task']['labels']}
            else:
                changes={'positive':{k:arms['uniform-0.5']['per_class']['positive'][v]-arms['shipped']['per_class']['positive'][v] for k,v in [('false_positive','fp'),('false_negative','fn')]}}
            out[slug]={'error_count_changes':changes,
                       'no_more_false_positives_or_misses':all(v<=0 for row in changes.values() for v in row.values()),
                       'recommendation':'Inspect errors per label. Do not adopt a uniform threshold from this diagnostic; validate the reference mapping and original input identity first.'}
        else:
            x=arms['order-agreement']; a=Fraction(x['correct'],x['answered']) if x['answered'] else Fraction(0)
            cov=Fraction(x['answered'],x['total'])
            out[slug]={'agreement_policy_selective_accuracy':float(a),'agreement_policy_coverage':float(cov),
                       'illustrative_gate_met':a>=Fraction(str(rubric['pairwise']['minimum_reference_agreement'])) and cov>=Fraction(str(rubric['pairwise']['minimum_coverage'])),
                       'recommendation':'Investigate order-dependent cases against their reference source. Agreement is a consistency filter, not an independent correctness judgment or ranking validation.'}
    return out



def compact(summary):
    """Keep complete IDs in run bundles; commit hashes, counts and bounded previews."""
    output=copy.deepcopy(summary)
    output['detail_policy']='Full transition ID lists remain in outcomes.full.json and native comparison bundles. Previews below are bounded; their source_ids_sha256 binds each full list.'
    for task in output['tasks'].values():
        for result in task['arms'].values():
            if 'per_class' in result:
                classes=result.pop('per_class')
                result['per_class_sha256']=digest(encoded(classes))
                if len(classes)<=4:
                    result['per_class']=classes
        for comparison in task['comparisons'].values():
            for transition in comparison['transitions'].values():
                ids=transition.pop('source_ids')
                transition['source_ids_sha256']=digest(encoded(ids))
                transition['source_ids_preview']=ids[:12]
    return output


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--sources',type=Path,required=True)
    parser.add_argument('--validator',type=Path,required=True)
    parser.add_argument('--out',type=Path,required=True)
    parser.add_argument('--example',choices=['all','acento','moderation','pairwise'],default='all')
    parser.add_argument('--allow-unverified-moderation-join',action='store_true')
    args=parser.parse_args()
    wanted=['acento','moderation','pairwise'] if args.example=='all' else [args.example]
    readers.verify_sources(args.sources, wanted+(['reference'] if 'moderation' in wanted else []))
    tasks=[]
    for name in wanted:
        tasks += readers.moderation(args.sources,args.allow_unverified_moderation_join) if name=='moderation' else getattr(readers,name)(args.sources)
    require(args.validator.is_file(),'native executable required')
    args.out.mkdir(parents=True,exist_ok=False,mode=0o700)
    rubric=read_json(HERE/'rubric.json');results={}
    for task in tasks:
        slug=task['book']['example'];print('Executing '+slug,flush=True)
        results[slug]=task_run(args.validator.resolve(),task,args.out/slug,rubric)
        write_private(args.out/(slug+'.summary.json'),encoded(results[slug]))
    summary={'schema_version':1,'status':'passed','live_inference_calls':0,'new_human_approvals':0,
             'validator_sha256':digest(args.validator.read_bytes()),'source_lock_sha256':digest((HERE/'sources.lock.json').read_bytes()),
             'evaluations':sum(len(v['arms']) for v in results.values()),'comparisons':sum(len(v['comparisons']) for v in results.values()),
             'tasks':results,'rubric':rubric,'feedback':feedback(results,rubric),
             'limits':['Retrospective source-reference agreement, not production performance or a fresh holdout.',
                       'No raw source text is copied into run inputs. Hashes identify available source bytes, not human approval.',
                       'Moderation input linkage is unverified and must be explicitly authorized; missing reference labels are never negatives.']}
    write_private(args.out/'outcomes.full.json',encoded(summary))
    write_private(args.out/'outcomes.json',encoded(compact(summary)))
    print(json.dumps({'tasks':len(results),'evaluations':summary['evaluations'],'comparisons':summary['comparisons'],'status':'passed'}))


if __name__=='__main__':
    main()
