#!/usr/bin/env python3
"""Offline portfolio replay: explicit original schemas, native evidence, task rubrics.

Acquire the pinned sources separately. This command makes no inference call.
Every result keeps its evidence origin. Unknown references are not false labels.
"""
from __future__ import annotations

import argparse
import copy
import json
import math
import os
from pathlib import Path
import shutil
import subprocess
import sys

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
sys.path.insert(0, str(HERE))
from poc import command, digest, encoded, episode_id, native_run, read_json, write_bundle, write_private  # noqa: E402
from tasks import atlas_tasks, checked_sources, fixture_tasks, require  # noqa: E402
from madewithjev.oracle import transition_ids, verify_single  # noqa: E402


def reference_map(task):
    b = task['book']
    return {episode_id(b['example'], c['case_id']): c['proposed_expected']['label'] for c in b['cases']}


def outcome_map(task, rows, policy):
    result = {}
    for r in rows:
        outcome = r['prediction']['outcome']
        value = outcome.get('label') if outcome['type'] == 'class' else None
        if 'minimum' in policy and r['prediction']['confidence'] < policy['minimum']:
            value = None
        result[episode_id(task['book']['example'], r['case_id'])] = value
    return result


def probability_oracle(gold, rows, labels, slug):
    """Separate direct arithmetic, including zero reference probability and absence."""
    values = []
    for row in rows:
        p = row['prediction'].get('probabilities')
        if p is None:
            return None
        require(p['kind'] == 'categorical', 'this oracle is categorical only')
        mapping = p['values']
        y = gold[episode_id(slug, row['case_id'])]
        loss = math.inf if mapping[y] == 0 else -math.log(mapping[y])
        brier = sum((mapping[k] - int(k == y)) ** 2 for k in labels)
        values.append((loss, brier))
    return (sum(v[0] for v in values) / len(values), sum(v[1] for v in values) / len(values))


def execute_task(task, binary, out, evidence, rubric):
    b = task['book']; slug = b['example']; destination = out / slug
    destination.mkdir(mode=0o700)
    write_private(destination/'reference-book.json', encoded(b))
    write_private(destination/'withheld.json', encoded(task['withheld']))
    summary = {'origin': task['origin'], 'source': b['source'], 'reference_meaning': b['configuration']['assessment'],
               'withheld': task['withheld'], 'cases': len(b['cases']), 'arms': {}, 'rubric': rubric}
    gold = reference_map(task)
    ids = {episode_id(slug,c['case_id']): c['case_id'] for c in b['cases']}
    arm_records = {}
    gold_digest = None
    for arm, rows, policy in task['arms']:
        cap = copy.deepcopy(task['capture'])
        cap.update(schema_version=1, example=slug, origin=task['origin'], complete=True)
        selected_evidence = {name: value for name,value in evidence.items()
                             if name.startswith('portfolio-') or name.startswith(('atlas-' if task['origin']=='published_replay' else slug.split('-')[0]+'-'))}
        receipt = write_bundle(b, cap, rows, destination/f'{arm}-prepared', 'development', policy,
                               fixture=task['origin']=='synthetic_fixture', accept_author_fixture=task['origin']=='published_replay',
                               extra_evidence=selected_evidence)
        if gold_digest is None:
            gold_digest = receipt['dataset_sha256']
        require(gold_digest == receipt['dataset_sha256'], 'gold changed across candidate arms')
        prepared = destination/f'{arm}-prepared'
        if 'minimum' in policy:
            cfg = read_json(prepared/'config.json')
            cfg['decision'] = {'type':'reject_below','signal':'confidence','minimum':policy['minimum']}
            (prepared/'config.json').write_bytes(encoded(cfg))
        report, native_receipt = native_run(binary, prepared, destination/arm)
        raw = outcome_map(task, rows, {})
        final = outcome_map(task, rows, policy)
        verify_single(report, gold, raw, b['task']['labels'], 'raw')
        counts = verify_single(report, gold, final, b['task']['labels'], 'final')
        expected_probability = probability_oracle(gold, rows, b['task']['labels'], slug)
        # The actual report metrics are audited below using the report's typed fields.
        # Both arms retain identical probability results on confidence-only changes.
        arm_records[arm] = {'raw':raw, 'final':final, 'report':report}
        summary['arms'][arm] = {'report_sha256':native_receipt['result_sha256'], 'dataset_sha256':gold_digest,
                               'count_oracle_passed':True, 'probability_oracle': None if expected_probability is None else
                                   {'log_loss': expected_probability[0] if math.isfinite(expected_probability[0]) else 'positive_infinity',
                                    'brier':expected_probability[1]},
                               'model_performance_claim': None,
                               'counts': {k:counts[k] for k in ['total','answered','abstained','correct','wrong','matrix']},
                               'wrong_case_ids':[ids[i] for i in sorted(gold) if final[i] is not None and final[i]!=gold[i]],
                               'abstained_case_ids':[ids[i] for i in sorted(gold) if final[i] is None]}
        if task['origin']=='published_replay':
            summary['arms'][arm]['reference_agreement'] = {k:counts[k] for k in ['accuracy','selective_accuracy','coverage']}
        # Fixture metrics deliberately remain counts for code correctness, not model quality.
    base, candidate = [v[0] for v in task['arms']]
    receipt = command(binary,['compare','--baseline',destination/base,'--candidate',destination/candidate,'--out',destination/'comparison'])
    comparison_file = Path(receipt['result_path'])
    require(digest(comparison_file.read_bytes()) == receipt['result_sha256'], 'comparison receipt changed')
    comparison = read_json(comparison_file)
    expected = transition_ids(gold, arm_records[base]['final'], arm_records[candidate]['final'])
    for key, values in expected.items():
        require(comparison['transitions'][key]['count'] == len(values), f'{slug}: wrong {key} count')
        require(comparison['transitions'][key]['ids'] == values, f'{slug}: wrong {key} IDs')
    summary['comparison'] = {'sha256':receipt['result_sha256'], 'baseline':base,'candidate':candidate,
                             **{key:[ids[i] for i in values] for key,values in expected.items()}}
    # Inspect every affected case, with evidence kept in the result directory.
    inspections = destination/'inspections'; inspections.mkdir(mode=0o700)
    for arm in [base,candidate]:
        for i in expected['changed_final_outcomes']:
            result = command(binary,['inspect','--run',destination/arm,'--episode',i])
            write_private(inspections/f'{arm}-{i}.json',encoded(result))
    relocated = destination/'relocated'
    shutil.copytree(destination/base,relocated)
    command(binary,['inspect','--run',relocated,'--episode',next(iter(gold))])
    (relocated/'evidence/0.bin').write_bytes(b'intentional portfolio tamper')
    failure = command(binary,['inspect','--run',relocated,'--episode',next(iter(gold))],expected_exit=2)
    require(failure['code'] == 'E_PROVENANCE', 'altered evidence was accepted')
    shutil.rmtree(relocated)
    summary['verification'] = ['all_raw_and_final_matrix_cells','per_class_rates','exact_transition_ids','affected_case_inspection','relocation','tamper_rejection']
    # Apply numeric task-rubric guards in code, never using model confidence as a quality score.
    first = summary['arms'][base]['counts']; second = summary['arms'][candidate]['counts']
    summary['rubric_result'] = {
        'candidate_does_not_increase_wrong_count':second['wrong']<=first['wrong'],
        'candidate_preserves_correct_count':second['correct']>=first['correct'],
        'candidate_has_no_regressed_cases':not expected['regressed'],
        'candidate_preserves_answered_population':second['answered']>=first['answered'],
        'semantic_acceptance':'not_assessed' if task['origin']=='synthetic_fixture' else 'retrospective_reference_agreement_only'}
    return summary, arm_records


def verify_probability_metrics(report, expected):
    """Use known public report fields; never recursively guess metric names."""
    p = report['probability']
    if expected is None:
        require(p['log_loss']['status']=='not_applicable' and p['brier_score']['status']=='not_applicable','unavailable probabilities were scored')
    else:
        for key,value in zip(['log_loss','brier_score'],expected):
            observed = p[key]
            if math.isinf(value):
                require(observed['status']=='positive_infinity', 'zero reference probability was clipped')
            else:
                require(observed['status']=='defined' and math.isclose(observed['value'],value,rel_tol=1e-10,abs_tol=1e-12),f'{key}: independent probability audit failed')


def run(sources:Path,binary:Path,out:Path):
    require(binary.is_file(), 'an actual Validator executable is required')
    pins,evidence = checked_sources(sources)
    rubric = read_json(HERE/'rubric.json')
    out.mkdir(mode=0o700,parents=True,exist_ok=False)
    environment = {k:v for k,v in os.environ.items() if not any(s in k.upper() for s in ['TOKEN','API_KEY','SECRET'])}
    child = subprocess.run(['node','--experimental-strip-types',str(HERE/'native.mjs'),'--sources',str(sources),'--out',str(out/'native-capture.json')],
                           capture_output=True,text=True,timeout=90,env=environment)
    write_private(out/'native.log',(child.stdout+child.stderr).encode())
    require(child.returncode==0,'native application probe failed; see native.log')
    capture = read_json(out/'native-capture.json')
    evidence.update({'portfolio-native.mjs':(HERE/'native.mjs').read_bytes(),'portfolio-native-cases.json':(HERE/'native-cases.json').read_bytes(),
                     'portfolio-run.py':Path(__file__).read_bytes(),'portfolio-rubric.json':encoded(rubric),
                     'portfolio-oracle.py':(HERE.parent/'madewithjev/oracle.py').read_bytes()})
    tasks = atlas_tasks(sources,pins)+fixture_tasks(capture,pins)
    results = {}
    for task in tasks:
        slug=task['book']['example']
        summary,records=execute_task(task,binary,out,evidence,rubric[slug])
        for arm,rows,policy in task['arms']:
            verify_probability_metrics(records[arm]['report'],probability_oracle(reference_map(task),rows,task['book']['task']['labels'],slug))
            summary['arms'][arm]['probability_audit_passed']=True
        results[slug]=summary
    summary={'schema_version':1,'status':'passed','live_model_calls':0,'human_approvals_created':0,
             'binary_sha256':digest(binary.read_bytes()),'native_checks':capture['checks'],
             'results':results,'source_pins':{k:{i:v[i] for i in ['repository','commit']} for k,v in pins.items()},
             'new_tasks':len(tasks),'evaluations':sum(len(t['arms']) for t in tasks),'comparisons':len(tasks),
             'limitations':['Published responses are upstream claims, not signed provenance or fresh inference.',
                            'Atlas publishes jev-latest, not the resolved model revision; retain that uncertainty.',
                            'Two explicit-null sarcasm references are withheld, not relabeled.',
                            'Historical triage agreement is not semantic accuracy.',
                            'Synthetic replies prove native software behavior only.',
                            'These small author diagnostics are neither population estimates nor held-out optimization.']}
    write_private(out/'summary.json',encoded(summary))
    print(json.dumps({'status':'passed','tasks':len(tasks),'evaluations':summary['evaluations'],'comparisons':len(tasks),'live_model_calls':0}))
    return summary


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--sources',required=True,type=Path)
    parser.add_argument('--validator',required=True,type=Path)
    parser.add_argument('--out',required=True,type=Path)
    args=parser.parse_args()
    run(args.sources.resolve(),args.validator.resolve(),args.out.resolve())
