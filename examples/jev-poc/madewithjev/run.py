#!/usr/bin/env python3
"""Replay pinned Made with Jev evidence through the actual Validator. No inference.

Public reference labels stay attributed to their upstream sources. Synthetic
code-review replies are only protocol fixtures and cannot establish accuracy.
"""
from __future__ import annotations
import argparse
from collections import Counter
import csv
import hashlib
import io
import json
import math
from pathlib import Path
import re
import shutil
import subprocess
import sys

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
from adapters import ContractError, binary, categorical, class_out, labels_out, number
from poc import command, digest, encoded, episode_id, native_run, read_json, strict_json, write_bundle, write_private
from oracle import require, transition_ids, verify_single, verify_multi

PINS = read_json(HERE / 'pins.json')
LABELS = ['legitimate', 'spam', 'phishing']
GOLD_MAP = {'ham': 'legitimate', 'spam': 'spam', 'phish': 'phishing'}
CLEAN_LABELS = ['too_many_arguments', 'flag_or_output_arguments', 'commented_out_code', 'swallowed_errors']


def jsonl(data):
    return [strict_json(line) for line in data.splitlines() if line.strip()]


def checked_sources(root, key):
    pin = PINS['sources'][key]
    data = {}
    for name, wanted in pin['files'].items():
        path = root / key / name
        if path.is_symlink():
            raise ContractError(f'symlink source is not allowed: {key}/{name}')
        value = path.read_bytes()
        if digest(value) != wanted['sha256']:
            raise ContractError(f'pinned source bytes changed: {key}/{name}')
        data[name] = value
    return data


def unique(rows, key):
    result = {}
    for r in rows:
        k = key(r)
        if k in result:
            raise ContractError(f'duplicate source key: {k}')
        result[k] = r
    return result


def origin(key):
    pin = PINS['sources'][key]
    return {'repository': pin['repository'], 'commit': pin['commit']}


def case(cid, model_input, target):
    return {'case_id': cid, 'family_id': cid, 'partition': 'development',
            'model_input': model_input, 'proposed_expected': target,
            'review': {'state': 'upstream_author_fixture', 'reviewer': None}}


def book(name, source, labels, cases, kind='single_label', **configuration):
    return {'schema_version': 1, 'example': name, 'reference_authority': 'upstream_author_fixture',
            'source': source, 'task': {'kind': kind, 'labels': labels}, 'cases': cases,
            'configuration': {'assessment': 'retrospective diagnostic; not a new untouched holdout', **configuration}}


def capture(b, arm, records, *, fixture=False):
    return {'schema_version': 1, 'example': b['example'],
            'origin': 'synthetic_fixture' if fixture else 'published_replay',
            'source': {**b['source'], 'model': 'synthetic-fixture' if fixture else 'jev-1.13.0'},
            'arm': arm, 'complete': True, 'records': records}


def choice(answer, labels):
    if not isinstance(answer, dict):
        raise ContractError('missing Choice answer')
    result = {'outcome': class_out(answer.get('choice'), labels), 'observations': {}}
    categorical(result, answer.get('probabilities'), labels)
    result['confidence'] = number(answer.get('confidence'), probability=True)
    return result


def spam_binary(data):
    records = jsonl(data['results/results_criteria_all.jsonl'])
    indexed = unique(records, lambda r: r['file'])
    labels = ['ham', 'spam']
    cases = []
    for cid, r in indexed.items():
        require(r['model'] == 'jev-1.13.0', 'unexpected binary producer')
        cases.append(case(cid, {'source_file': cid, 'content_available': False}, class_out(r['label'], labels)))
    b = book('made-spam-binary', origin('spam'), labels, cases,
             population='all saved rows; source-email deduplication cannot be reconstructed', threshold=.5)
    arms = {}
    for name, q in [('plain', 'spam_plain'), ('criteria', 'spam_structured_criteria')]:
        rows = [{'case_id': cid, 'prediction': binary(r['nouls'][q], labels, .5)} for cid, r in indexed.items()]
        raw = {cid: labels[0] if r['nouls'][q] < .5 else labels[1] for cid, r in indexed.items()}
        arms[name] = (rows, raw, capture(b, name, records))
    return b, arms, {cid: r['label'] for cid, r in indexed.items()}


def spam_multiclass(data, group, *, allow_unverified_runner=False):
    manifest = strict_json(data['experiments/jev-context/manifest.json'])
    runner_identity = {'manifest_sha256': manifest['source_hash'],
                       'published_sha256': digest(data['experiments/jev-context/evaluate.py'])}
    runner_identity['verified'] = runner_identity['manifest_sha256'] == runner_identity['published_sha256']
    require(runner_identity['verified'] or allow_unverified_runner,
            'upstream runner binding changed; use --allow-unverified-email-runner only for explicitly qualified recorded-output accounting')
    refs = unique(jsonl(data[f'results/phish_{"main_all" if group == "main" else group}.jsonl']), lambda r: r['file'])
    all_rows = jsonl(data['experiments/jev-context/predictions.jsonl'])
    records = [r for r in all_rows if r['set'] == group]
    indexed = unique(records, lambda r: (r['file'], r['arm']))
    require(set(indexed) == {(cid, arm) for cid in refs for arm in ('text', 'enriched')}, 'missing/extra paired email capture')
    cases = []
    for cid, r in refs.items():
        for arm in ('text', 'enriched'):
            row = indexed[cid, arm]
            require(row['label'] == r['label'] and row['model'] == manifest['model'], 'email label/model mismatch')
            require(re.fullmatch('[0-9a-f]{64}', row['state_sha256']) is not None, 'missing effective-state digest')
        cases.append(case(cid, {'corpus': group, 'source_file': cid, 'content_available': False},
                          class_out(GOLD_MAP[r['label']], LABELS)))
    b = book('made-email-' + group, origin('spam'), LABELS, cases,
             producer_source_identity=runner_identity, category_definitions=manifest['definitions'], source_content='not published; IDs and request digests retained')
    arms = {}
    for name, a, q in [('text', 'text', 'category'), ('enriched', 'enriched', 'category'), ('framing', 'enriched', 'evidence_framing')]:
        rows = [{'case_id': cid, 'prediction': choice(indexed[cid, a]['choices'][q], LABELS)} for cid in refs]
        original = {cid: indexed[cid, a]['choices'][q]['choice'] for cid in refs}
        c = capture(b, name, [indexed[cid, a] for cid in refs])
        c['producer_source_identity'] = runner_identity
        c['question'] = manifest['framing'] if q == 'evidence_framing' else manifest['original_instruction']
        arms[name] = (rows, original, c)
    return b, arms, {cid: GOLD_MAP[r['label']] for cid, r in refs.items()}


def banking(data):
    raw_bytes = data['raw/run/banking77/responses.jsonl']
    require(digest(raw_bytes) == data['raw/run/banking77/responses.sha256'].decode().split()[0], 'raw response seal mismatch')
    labels = strict_json(data['corpus/banking77/categories.json'])
    refs = list(csv.DictReader(io.StringIO(data['corpus/banking77/test.csv'].decode())))
    records = jsonl(raw_bytes)
    indexed = unique(records, lambda r: r['id'])
    require(set(indexed) == {f'b77-{i:04d}' for i in range(len(refs))}, 'Banking77 identity mismatch')
    cases, rows, selected, gated = [], [], {}, {}
    non_unit = 0
    for i, ref in enumerate(refs):
        cid = f'b77-{i:04d}'
        r = indexed[cid]
        require(r['text'] == ref['text'] and r['label'] == ref['category'], 'corpus/reference disagreement')
        require(r['request']['state'] == ref['text'], 'provider state differs from source input')
        q = r['request']['questions']['intent']
        require(q['type'] == 'choice' and set(q['criteria']) == set(labels), 'wrong requested vocabulary')
        require(all(v is None for v in q['criteria'].values()), 'unexpected Choice criteria')
        require(r['response']['status'] == 200, 'unexpected unsuccessful Banking77 capture; no silent omission')
        body = r['response']['body']
        require(body['model'] == 'jev-1.13.0', 'resolved banking model mismatch')
        answer = body['answers']['intent']
        require(answer['type'] == 'choice', 'wrong banking primitive')
        prediction = choice(answer, labels)
        non_unit += int('probabilities' not in prediction)
        cases.append(case(cid, {'text': ref['text']}, class_out(ref['category'], labels)))
        rows.append({'case_id': cid, 'prediction': prediction})
        selected[cid] = answer['choice']
        gated[cid] = answer['choice'] if answer['confidence'] >= .8 else None
    b = book('made-banking77', origin('assay'), labels, cases,
             task='77-way banking intent', non_unit_probability_rows=non_unit,
             gate={'signal': 'native reported confidence', 'minimum': .8},
             reference='frozen upstream Banking77 corpus, not newly human-approved here')
    c = capture(b, 'recorded', records)
    return b, {'recorded': (rows, selected, c), 'confidence-0.8': (rows, gated, c)}, {f'b77-{i:04d}': r['category'] for i, r in enumerate(refs)}


def decode_clean(file, declared_ids, *, threshold=.5):
    """Decode FINAL file findings. Worst-window signals are NOT calibrated marginals.

The task covers exactly four reviewed all-scope findings, not every question. The
other answers remain raw evidence. Omitted applicable findings must not become TN.
"""
    if not set(CLEAN_LABELS) <= set(declared_ids):
        raise ContractError('requested scope does not contain the fixed reviewed task')
    if file.get('cut') or file.get('complete') is False:
        raise ContractError('partial file cannot stand in for whole-file references')
    answers = file.get('answers', {})
    values = {}
    observations = {}
    for label in CLEAN_LABELS:
        a = answers.get(label, {})
        if a.get('type') != 'noul':
            raise ContractError(f'missing/wrong applicable Noul: {label}')
        values[label] = number(a.get('noul'), probability=True)
        observations[label] = {'kind': 'bernoulli', 'value': values[label]}
    for label in ('function_size', 'nesting', 'verdict'):
        a = answers.get(label)
        if a is not None:
            if a.get('type') != 'score' or not 0 <= number(a.get('score')) <= 4:
                raise ContractError('wrong Score type/range')
            observations[label] = {'kind': 'scalar', 'value': a['score']}
            if 'confidence' in a:
                observations[label + '_confidence'] = {'kind': 'reported_confidence', 'value': number(a['confidence'], probability=True)}
            if 'probabilities' in a:
                if not isinstance(a['probabilities'], dict) or set(a['probabilities']) != set('01234'):
                    raise ContractError('Score distribution needs explicit five levels')
                observations[label + '_distribution'] = {'kind': 'categorical', 'values': {k: number(v, probability=True) for k, v in a['probabilities'].items()}}
    return {'outcome': labels_out([k for k in CLEAN_LABELS if values[k] >= threshold], CLEAN_LABELS), 'observations': observations}


def run_clean(sources, out):
    completed = subprocess.run(['node', '--experimental-strip-types', str(HERE / 'clean-questions.mjs'),
                                str(sources / 'clean-code' / 'agent/lib/judging/questions.ts')],
                               capture_output=True, text=True, timeout=30)
    if completed.returncode:
        raise ContractError(completed.stderr)
    return strict_json(completed.stdout.encode())


def compare(binary_path, out, b, gold, before_name, after_name, before, after):
    name = before_name + '-vs-' + after_name
    receipt = command(binary_path, ['compare', '--baseline', out/before_name/'run', '--candidate', out/after_name/'run', '--out', out/name])
    path = Path(receipt['result_path'])
    require(digest(path.read_bytes()) == receipt['result_sha256'], 'comparison receipt mismatch')
    report = read_json(path)
    expected = transition_ids(gold, before, after)
    for k, ids in expected.items():
        actual = report['transitions'][k]
        require(actual['count'] == len(ids), f'{name}.{k} count')
        require(set(actual['ids']) == {episode_id(b['example'], i) for i in ids}, f'{name}.{k} IDs')
    return {**{k: len(v) for k, v in expected.items()},
            'source_ids': expected, 'comparison_sha256': receipt['result_sha256']}


def run_task(binary_path, task_out, b, arms, gold, comparisons, extra):
    task_out.mkdir(parents=True, exist_ok=False, mode=0o700)
    reports, summary = {}, {'population': len(gold), 'label_count': len(b['task']['labels']),
                            'source': b['source'], 'origin': 'published_replay',
                            'producer_source_identity': b['configuration'].get('producer_source_identity'),
                            'arms': {}, 'comparisons': {}}
    for arm, (rows, original, cap) in arms.items():
        parent = task_out / arm
        parent.mkdir(mode=0o700)
        prep = parent/'prepared'
        receipt = write_bundle(b, cap, rows, prep, 'development', {'arm': arm}, accept_author_fixture=True,
                               extra_evidence=extra)
        if arm == 'confidence-0.8':
            cfg = read_json(prep/'config.json')
            cfg['decision'] = {'type': 'reject_below', 'signal': 'confidence', 'minimum': .8}
            (prep/'config.json').write_bytes(encoded(cfg))
        print(f"Evaluating {b['example']}/{arm}: {len(gold)} cases", flush=True)
        report, native_receipt = native_run(binary_path, prep, parent/'run')
        expected = verify_single(report, gold, original, b['task']['labels'])
        raw_original = arms['recorded'][1] if arm == 'confidence-0.8' else original
        verify_single(report, gold, raw_original, b['task']['labels'], family='raw')
        reports[arm] = report
        full = dict(expected, report_sha256=native_receipt['result_sha256'],
                    demoted_families=receipt['demoted_partial_scoring_families'],
                    probability_status=report['probability']['log_loss']['status'])
        summary['arms'][arm] = full
    datasets = [digest((task_out/arm/'prepared/golden.json').read_bytes()) for arm in arms]
    require(len(set(datasets)) == 1, 'gold changed between arms')
    for before, after in comparisons:
        result = compare(binary_path, task_out, b, gold, before, after, arms[before][1], arms[after][1])
        summary['comparisons'][before + '-vs-' + after] = result
    return summary


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--sources', type=Path, required=True)
    parser.add_argument('--validator', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--tasks', nargs='+', choices=['spam-binary','email','banking','clean'], default=['spam-binary','email','banking','clean'])
    parser.add_argument('--allow-unverified-email-runner', action='store_true',
                        help='Acknowledge the published email runner differs from its manifest hash; replay recorded decisions only, not verified request construction.')
    args = parser.parse_args()
    root, out, binary_path = args.sources.resolve(), args.out.resolve(), args.validator.resolve()
    out.mkdir(parents=True, exist_ok=False, mode=0o700)
    result = {'schema_version': 1, 'status': 'running', 'live_model_calls': 0, 'new_human_approvals': 0,
              'validator_sha256': digest(binary_path.read_bytes()), 'source_pins': PINS,
              'evidence_classes': {'published_replay': [], 'synthetic_fixture': []}, 'tasks': {}}
    extra = {'made-adapter.py': Path(__file__).read_bytes(), 'independent-oracle.py': (HERE/'oracle.py').read_bytes(),
             'made-pins.json': encoded(PINS), 'task-rubric.json': (HERE/'rubric.json').read_bytes()}
    if {'spam-binary','email'} & set(args.tasks):
        data = checked_sources(root, 'spam')
        if 'spam-binary' in args.tasks:
            b, arms, gold = spam_binary(data)
            result['tasks']['spam-binary'] = run_task(binary_path, out/'spam-binary', b, arms, gold,
                                [('plain','criteria')], dict(extra, **{'native-binary.jsonl': data['results/results_criteria_all.jsonl']}))
        if 'email' in args.tasks:
            for group in ('main','fresh','recent'):
                b, arms, gold = spam_multiclass(data, group, allow_unverified_runner=args.allow_unverified_email_runner)
                key = 'email-' + group
                result['tasks'][key] = run_task(binary_path, out/key, b, arms, gold,
                           [('text','enriched'),('enriched','framing')], dict(extra, **{'context-manifest.json': data['experiments/jev-context/manifest.json']}))
        for key in result['tasks']:
            result['evidence_classes']['published_replay'].append(key)
    if 'banking' in args.tasks:
        data = checked_sources(root, 'assay')
        b, arms, gold = banking(data)
        result['tasks']['banking77'] = run_task(binary_path, out/'banking77', b, arms, gold,
                      [('recorded','confidence-0.8')], dict(extra, **{'native-banking.jsonl': data['raw/run/banking77/responses.jsonl'],
                        'reference-corpus.csv': data['corpus/banking77/test.csv'], 'categories.json': data['corpus/banking77/categories.json']}))
        result['tasks']['banking77']['non_unit_probability_rows'] = b['configuration']['non_unit_probability_rows']
        result['evidence_classes']['published_replay'].append('banking77')
    if 'clean' in args.tasks:
        checked_sources(root, 'clean-code')
        scopes = run_clean(root, out)
        (out/'clean-question-contract.json').write_bytes(encoded(scopes))
        fixtures = read_json(HERE/'clean-fixtures.json')
        b = book('made-clean-code', origin('clean-code'), CLEAN_LABELS, fixtures['cases'], kind='multi_label',
                 measurement='four named all-scope findings; other findings not silently labeled absent',
                 signal='final worst-window aggregation; no scored marginal probabilities')
        cap = {'schema_version':1,'example':b['example'],'origin':'synthetic_fixture',
               'source':{**b['source'],'model':'synthetic-fixture'},'records':fixtures['records']}
        gold = {c['case_id']: c['proposed_expected']['labels'] for c in b['cases']}
        decisions, arms = {}, {}
        task_out = out/'clean'
        for name, threshold in [('threshold-0.5',.5),('threshold-0.7',.7)]:
            rows = [{'case_id':r['case_id'],'prediction':decode_clean(r['output'],scopes['whole']['ids'],threshold=threshold)} for r in fixtures['records']]
            # Independent policy oracle from fixture values; not the decoder result.
            original = {r['case_id']: sorted(k for k in CLEAN_LABELS if r['output']['answers'][k]['noul'] >= threshold) for r in fixtures['records']}
            prepared = task_out/name/'prepared'
            receipt=write_bundle(b,cap,rows,prepared,'development',{'threshold':threshold},fixture=True,extra_evidence=extra)
            report, native_receipt = native_run(binary_path,prepared,task_out/name/'run')
            verify_multi(report,gold,original,CLEAN_LABELS)
            require(report['probability']['mean_binary_log_loss']['status'] == 'not_applicable','aggregated signals were incorrectly scored')
            decisions[name]=original
            arms[name]={'fixture_cases':len(gold),'report_sha256':native_receipt['result_sha256'],'model_accuracy_claim':None}
        comparison=compare(binary_path,task_out,b,gold,'threshold-0.5','threshold-0.7',decisions['threshold-0.5'],decisions['threshold-0.7'])
        result['tasks']['clean-code']={'origin':'synthetic_fixture','model_accuracy_claim':None,
            'scope_contract':scopes,'arms':arms,'comparison':comparison,
            'limits':['Only the actual pure question builder was executed; full AI SDK, windows and comment stripping were not run.',
                      'Every model reply here is a fixture. Final aggregated Noul signals remain observations, not calibrated marginals.']}
        result['evidence_classes']['synthetic_fixture'].append('clean-code')
    result['status']='passed'
    (out/'outcomes.json').write_bytes(encoded(result))
    print(json.dumps({'status':'passed','tasks':list(result['tasks']),'out':str(out),'live_model_calls':0},indent=2))


if __name__ == '__main__':
    main()
