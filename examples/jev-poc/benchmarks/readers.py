"""Pinned published-response adapters. No provider calls or upstream code execution.

References are author/dataset declarations, not labels approved by this adapter.
Missing reference labels remain unknown. Predictions and references join by ID.
"""
from __future__ import annotations

import gzip
import hashlib
import json
import math
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
from adapters import ContractError, categorical, class_out, labels_out, number
from poc import encoded, read_json, strict_json

PINS = {
    'acento': ('marcosmartinez/jev-acento', '7e007b4c2bd552471b56eff035f9a3df7593f9fe'),
    'moderation': ('ohernandezdev/jevmod', '01063f3e927ddbc3c7de923b73f56dcc33034ca9'),
    'pairwise': ('maybern-tripp-smith/fedjev-bench', '7911b5da1dee9947c75784a6962ee5d4fd3d315f'),
    'reference': ('openai/moderation-api-release', 'f4ab51b5edd3bfbcb349a56324274235b674e0e4'),
}
MOD_MAP = {'harassment': ['H', 'HR', 'V'], 'nsfw': ['S', 'V2'], 'selfharm': ['SH'], 'minors': ['S3']}
DEFAULTS = {'harassment': .75, 'nsfw': .8, 'selfharm': .8, 'minors': .7}


def require(ok, message):
    if not ok:
        raise ContractError(message)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def rows(path):
    data = path.read_bytes()
    if path.suffix == '.gz':
        data = gzip.decompress(data)
    return [strict_json(line) for line in data.splitlines() if line.strip()]


def indexed(records, key):
    out = {}
    for row in records:
        value = row.get(key)
        require(isinstance(value, str) and value and value not in out, f'missing/duplicate {key}')
        out[value] = row
    return out


def verify_sources(root, wanted):
    lock = read_json(HERE / 'sources.lock.json')
    for name in wanted:
        require((root / name / '.source-revision').read_text().strip() == PINS[name][1], f'{name}: wrong revision')
        for rel, expected in lock['files'][name].items():
            path = root / name / rel
            require(path.is_file() and not path.is_symlink(), f'{name}/{rel}: absent or symlink')
            require(sha(path.read_bytes()) == expected, f'{name}/{rel}: digest mismatch')
    return lock


def book(name, source, labels, cases, *, multi=False, details=None):
    return {'schema_version': 1, 'example': name,
            'source': dict(zip(('repository', 'commit'), PINS[source])),
            'reference_authority': 'upstream_author_fixture',
            'task': {'kind': 'multi_label' if multi else 'single_label', 'labels': labels},
            'configuration': details or {}, 'cases': cases}


def case(cid, expected, inputs, *, multi=False):
    return {'case_id': cid, 'family_id': cid, 'partition': 'development',
            'model_input': inputs,
            'proposed_expected': labels_out(expected, expected) if multi else {'type': 'class', 'label': expected},
            'review': {'state': 'upstream_reference', 'reviewer': None}}


def choice_prediction(pred, probs, confidence, labels):
    result = {'outcome': class_out(pred, labels), 'observations': {}}
    categorical(result, probs, labels)
    if confidence is not None:
        result['confidence'] = number(confidence, probability=True)
    return result


def arm(name, model, data, predicted, records, *, decision=None, limitations=None, evidence=None):
    return {'name': name, 'model': model, 'rows': data, 'expected_predictions': predicted,
            'records': records, 'decision': decision or {'type': 'as_recorded'},
            'limitations': limitations or [], 'evidence': evidence or {}}


def acento(root):
    p = root / 'acento'
    export = read_json(p / 'data/items.export.json')
    require(export['source_sha256'] == sha((p / 'data/items.parquet').read_bytes()), 'parquet/export identity')
    require(export['json_sha256'] == sha((p / 'data/items.json').read_bytes()), 'reference export identity')
    metadata = read_json(p / 'data/items.json')
    require(len(metadata) == export['rows'] == 3200, 'acento item count')
    for hashfile in ('prompts.sha256', 'PREREG.sha256'):
        for line in (p / hashfile).read_text().splitlines():
            if not line.strip():
                continue
            digest, rel = line.split(maxsplit=1)
            rel = rel.lstrip('*')
            require(not Path(rel).is_absolute() and '..' not in Path(rel).parts, 'unsafe freeze path')
            require(sha((p / rel).read_bytes()) == digest, 'upstream freeze mismatch')
    manifest = read_json(p / 'runs/manifest.json')['20260921-es-v1']
    require(manifest['config']['dry_run'] is False and manifest['summary']['calls'] == 19200 and not manifest['summary']['errors'], 'not complete observed run')
    tasks = []
    for dataset, n in [('xnli', 1000), ('pawsx', 1000), ('massive', 600), ('belebele', 600)]:
        reference = indexed([r for r in metadata if r['dataset'] == dataset], 'item_id')
        require(len(reference) == n, f'{dataset}: reference count')
        prompt_en = read_json(p / f'prompts/{dataset}.en.json')
        prompt_es = read_json(p / f'prompts/{dataset}.es.json')
        labels = ['false', 'true'] if dataset == 'pawsx' else sorted(prompt_en['criteria'])
        if dataset != 'pawsx':
            require(set(prompt_es['criteria']) == set(labels), 'instruction translation changed options')
        cases = [case(cid, r['gold'], {'item_id': cid, 'dataset': dataset,
                                     'state_hash_en': r['state_hash_en'], 'state_hash_es': r['state_hash_es'],
                                     'source_text_available': False}) for cid, r in sorted(reference.items())]
        b = book('bench-acento-' + dataset, 'acento', labels, cases, details={
            'reference': 'Published parallel dataset item metadata, checked against every recorded gold and state hash.',
            'decision': 'Upstream derived prediction: lexical argmax for Choice; Noul strictly >0.5. Native Choice itself is not retained.',
            'population': 'Published diagnostic; not a newly sealed holdout.'})
        arms = []
        for a in 'ABC':
            for repeat in (0, 1):
                path = p / f'runs/20260921-es-v1-{dataset}-{a}-pass{repeat}.jsonl'
                rr = indexed(rows(path), 'item_id')
                require(set(rr) == set(reference), 'acento incomplete/extra records')
                output, predictions = [], {}
                for cid, row in sorted(rr.items()):
                    require(row['dataset'] == dataset and row['arm'] == a and row['pass'] == repeat, 'cell identity')
                    require(row['gold'] == reference[cid]['gold'], 'changed gold between arms')
                    require(row['state_hash'] == reference[cid]['state_hash_en' if a == 'A' else 'state_hash_es'], 'effective input hash mismatch')
                    require(row['model'] == 'jev-1.13.0' and not row['generation_id'].startswith('fake'), 'model identity')
                    require(set(row['probs']) == set(labels), 'incomplete probability vocabulary')
                    for v in row['probs'].values():
                        number(v, probability=True)
                    expected = ('true' if row['probs']['true'] > .5 else 'false') if dataset == 'pawsx' else min(labels, key=lambda k: (-row['probs'][k], k))
                    require(row['pred'] == expected, 'recorded derived decision differs from frozen parser rule')
                    require(math.isclose(row['p_max'], max(row['probs'].values()), abs_tol=1e-12), 'p_max mismatch')
                    if dataset == 'pawsx':
                        require(row['confidence'] is None, 'Noul has no native confidence')
                    output.append({'case_id': cid, 'prediction': choice_prediction(row['pred'], row['probs'], row['confidence'], labels)})
                    predictions[cid] = expected
                arms.append(arm(f'{a}-pass{repeat}', 'jev-1.13.0', output, predictions, list(rr.values()),
                                limitations=['Derived predictions, not the unretained native Choice selection.',
                                             'Only input hashes are published; no new inference or translation adjudication.'],
                                evidence={'recorded.jsonl': path.read_bytes(), 'question.json': encoded(prompt_es if a == 'C' else prompt_en),
                                          'reference-items.json': encoded(list(reference.values()))}))
        comparisons = [('A-pass0', 'B-pass0', 'language'), ('B-pass0', 'C-pass0', 'instruction-language')]
        comparisons += [(f'{a}-pass0', f'{a}-pass1', 'repeat') for a in 'ABC']
        tasks.append({'book': b, 'arms': arms, 'comparisons': comparisons,
                      'disclosures': {'cases': n, 'classes': len(labels), 'passes': 2, 'state_text_retained': False,
                                      'native_choice_differs_from_derived': {x['name']: sum(r['choice_disagrees_with_argmax'] for r in x['records']) for x in arms}}})
    return tasks


def known_or(row, keys):
    """Partial references: known positive dominates; negative requires every operand."""
    for k in keys:
        require(k not in row or type(row[k]) is int and row[k] in (0, 1), f'invalid reference bit {k}')
    if any(row.get(k) == 1 for k in keys):
        return True
    if all(row.get(k) == 0 for k in keys):
        return False
    return None


def moderation(root, allow_index_join=False):
    require(allow_index_join, 'Moderation has no original input digest: require --allow-unverified-moderation-join for qualified reference agreement.')
    p = root / 'moderation'
    refs = rows(root / 'reference/data/samples-1680.jsonl.gz')
    require(len(refs) == 1680, 'unexpected reference population')
    raw = indexed(rows(p / 'benchmark/results/jevmod.jsonl'), 'id')
    oai = {k: v for k, v in raw.items() if k.startswith('oai')}
    require(set(oai) == {f'oai{i}' for i in range(1680)}, 'source ID join is incomplete')
    truth = {f'oai{i}': {label: known_or(r, keys) for label, keys in MOD_MAP.items()} for i, r in enumerate(refs)}
    labels = list(MOD_MAP)
    full_ids = sorted(cid for cid, values in truth.items() if all(v is not None for v in values.values()))
    groups = [('complete-four-label', full_ids, labels)] + [(label, sorted(cid for cid in truth if truth[cid][label] is not None), [label]) for label in labels]
    tasks = []
    for suffix, ids, dimensions in groups:
        multi = len(dimensions) > 1
        vocabulary = dimensions if multi else ['negative', 'positive']
        gold = {cid: ([l for l in dimensions if truth[cid][l]] if multi else ('positive' if truth[cid][dimensions[0]] else 'negative')) for cid in ids}
        cases = [case(cid, gold[cid], {'original_row_index': int(cid[3:]),
                     'text_sha256': sha(refs[int(cid[3:])]['prompt'].encode()), 'source_text_available': False}, multi=multi) for cid in ids]
        b = book('bench-moderation-' + suffix, 'moderation', vocabulary, cases, multi=multi, details={
            'reference_mapping': {l: MOD_MAP[l] for l in dimensions},
            'unknown_rule': 'OR of reviewed bits: positive if any one is 1; negative only if all operands explicitly 0; otherwise excluded.',
            'identity_limitation': 'Index-based reconstruction from a separate frozen public dataset. Original producer did not save input hashes; this join is not authenticated.',
            'semantic_limitation': 'Upstream combined label taxonomy differs from application questions. Measures agreement with this projection, not universal moderation correctness.'})
        arms = []
        for policy_name, thresholds in [('shipped', DEFAULTS), ('uniform-0.5', {l: .5 for l in labels})]:
            output, predicted = [], {}
            for cid in ids:
                r = oai[cid]
                if r.get('judged') is not True or r.get('reason') not in ('jev', 'cache'):
                    prediction = {'outcome': {'type': 'abstention', 'reason': 'upstream-no-model-decision'}}
                    value = None
                else:
                    require(all(l in r.get('scores', {}) for l in dimensions), 'missing model head')
                    scores = {l: number(r['scores'][l], probability=True) for l in dimensions}
                    if multi:
                        # Raw reproduces shipped sets. Final thresholds belong to the core.
                        raw_labels = [l for l in dimensions if scores[l] >= DEFAULTS[l]]
                        prediction = {'outcome': labels_out(raw_labels, vocabulary), 'probabilities': {'kind': 'label_marginals', 'values': scores}}
                        value = [l for l in dimensions if scores[l] >= thresholds[l]]
                    else:
                        label = dimensions[0]; prob = scores[label]
                        value = 'positive' if prob >= thresholds[label] else 'negative'
                        prediction = choice_prediction(value, {'negative': 1-prob, 'positive': prob}, None, vocabulary)
                output.append({'case_id': cid, 'prediction': prediction}); predicted[cid] = value
            projection = [{'id': cid, 'known_labels': truth[cid], 'reference_source_row': int(cid[3:]),
                           'text_sha256': sha(refs[int(cid[3:])]['prompt'].encode())} for cid in ids]
            arms.append(arm(policy_name, 'jev-requested-alias-unresolved', output, predicted, [oai[cid] for cid in ids],
                            decision={'type': 'label_thresholds', 'thresholds': {l: thresholds[l] for l in dimensions}} if multi else None,
                            limitations=['No response-level model version saved.', 'Unverified original-input identity; explicit index-join exception enabled.',
                                         'Unknown labels excluded, not invented negatives. No fresh review of semantic mapping.'],
                            evidence={'reference-projection.json': encoded(projection), 'prepare-upstream.py': (p/'benchmark/prepare.py').read_bytes(),
                                      'producer.py': (p/'benchmark/run_jevmod.py').read_bytes(), 'category-questions.json': (p/'jevmod/categories.json').read_bytes()}))
        tasks.append({'book': b, 'arms': arms, 'comparisons': [('shipped', 'uniform-0.5', 'threshold-sensitivity')],
                      'disclosures': {'source_cases': 1680, 'scored_cases': len(ids), 'excluded_unknown': 1680-len(ids),
                                      'input_identity_verified': False, 'resolved_model_available': False,
                                      'complete_reference_population': multi}})
    return tasks


def select_pair_answers(records):
    """Deterministic last log occurrence, only when repeated decision evidence agrees.

The source duplicates 40 extreme pairs in both orders after changing inputs. Do
not pretend these are identical repeats. Keep the last record's actual metadata.
"""
    seen, duplicates = {}, 0
    for r in records:
        if r.get('kind') != 'choice':
            continue
        require(r.get('ok') is True and r.get('order') in ('ab', 'ba'), 'failed/malformed pair capture')
        key = (r['pair_id'], r['order'])
        if key in seen:
            duplicate = seen[key]
            require(all(duplicate[k] == r[k] for k in ('winner', 'probabilities', 'confidence', 'model')), 'conflicting repeated pair answer; needs separate arm')
            duplicates += 1
        seen[key] = r
    return seen, duplicates


def orient_pair(r, gold_row):
    require(r['model'] == 'jev-1.13.0' and r['gold'] == gold_row['gold'], 'pair producer/reference mismatch')
    order = r['order']; require(order in ('ab', 'ba'), 'unknown pair order')
    expected_a = gold_row['a' if order == 'ab' else 'b']
    expected_b = gold_row['b' if order == 'ab' else 'a']
    require((r['id_A'], r['id_B']) == (expected_a, expected_b), 'pair IDs do not match orientation')
    require(r['winner'] in ('A', 'B') and set(r['probabilities']) == {'A', 'B'}, 'bad native pair choice')
    for k in ('A', 'B'):
        require(number(r['probabilities'][k], probability=True) == number(r['p_' + k], probability=True), 'pair scalar/map mismatch')
    remap = {'A': 'a', 'B': 'b'} if order == 'ab' else {'A': 'b', 'B': 'a'}
    prediction = choice_prediction(remap[r['winner']], {remap[k]: v for k, v in r['probabilities'].items()}, r['confidence'], ['a', 'b'])
    return prediction


def pairwise(root):
    p = root / 'pairwise'; refs = indexed(rows(p/'data/pairs/gold_pairs.jsonl'), 'pair_id')
    require(len(refs) == 262, 'pair population')
    selected, duplicates = select_pair_answers(rows(p/'runs/jev/answers.jsonl'))
    require(set(selected) == {(cid, order) for cid in refs for order in ('ab', 'ba')}, 'unpaired/missing records')
    require(duplicates == 80, 'unexpected repeated captures')
    for cid in refs:
        forward, reverse = selected[cid, 'ab'], selected[cid, 'ba']
        require((forward['hash_A'], forward['hash_B']) == (reverse['hash_B'], reverse['hash_A']),
                'position comparison changed document content, not just order')
    tasks = []
    for group, count in [('extreme', 40), ('shah', 200), ('adjacent', 22)]:
        group_refs = {cid: r for cid, r in refs.items() if r['source'] == group}
        require(len(group_refs) == count, 'pair reference stratum count')
        cases = [case(cid, r['gold'], {'document_a': r['a'], 'document_b': r['b'], 'reference_stratum': group,
                                     'evidence': 'Selected effective input hashes retained per capture; original document text is not in this bundle.'}) for cid, r in sorted(group_refs.items())]
        b = book('bench-pair-' + group, 'pairwise', ['a','b'], cases, details={
            'criterion': 'more hawkish about inflation', 'reference_scope': group,
            'reference_warning': 'All source pairs put reference winner at a. No balanced-class accuracy claim. Adjacent/extreme references use rate-change proxies; Shah uses source sentence labels.',
            'duplicate_policy': 'Last log occurrence, requiring identical repeated decision/probability/confidence/model fields. Effective input hashes may differ.',
            'not_supported': 'No tournament ranking, forecasting, investment or policy-causality conclusions.'})
        arms = []
        for order in ('ab', 'ba'):
            rr = [selected[cid, order] for cid in sorted(group_refs)]
            output = [{'case_id': r['pair_id'], 'prediction': orient_pair(r, group_refs[r['pair_id']])} for r in rr]
            # Independent ID-based winner mapping, rather than reading canonical outcome.
            predicted = {r['pair_id']: ('a' if (r['id_A'] if r['winner']=='A' else r['id_B']) == group_refs[r['pair_id']]['a'] else 'b') for r in rr}
            arms.append(arm(order, 'jev-1.13.0', output, predicted, rr, limitations=['Author reference pairs; source documents are correlated.', 'Position reversal is a real input change, not a file-order shuffle.'],
                            evidence={'reference-pairs.json': encoded(list(group_refs.values())), 'pair-manifest.md': (p/'data/pairs/PAIR_MANIFEST.md').read_bytes(),
                                      'published-answer-log.jsonl': (p/'runs/jev/answers.jsonl').read_bytes()}))
        agreement, predictions = [], {}
        for cid in sorted(group_refs):
            lhs = arms[0]['expected_predictions'][cid]; rhs = arms[1]['expected_predictions'][cid]
            value = lhs if lhs == rhs else None
            predictions[cid] = value
            agreement.append({'case_id': cid, 'prediction': {'outcome': class_out(value, ['a','b']) if value else {'type':'abstention','reason':'order-dependent-choices'},
                'observations': {'ab_confidence': {'kind':'reported_confidence','value':number(selected[cid,'ab']['confidence'], probability=True)},
                                 'ba_confidence': {'kind':'reported_confidence','value':number(selected[cid,'ba']['confidence'], probability=True)}}}})
        arms.append(arm('order-agreement', 'jev-1.13.0', agreement, predictions,
                        [selected[cid, order] for cid in sorted(group_refs) for order in ('ab','ba')],
                        limitations=['Agreement does not establish correctness; this is a conservative abstaining policy compared to external declared references.',
                                     'No invented probability/confidence for the composed outcome.'],
                        evidence={'reference-pairs.json': encoded(list(group_refs.values()))}))
        tasks.append({'book':b, 'arms':arms, 'comparisons':[('ab','ba','order-reversal'),('ab','order-agreement','abstaining-policy')],
                      'disclosures':{'reference_cases':count,'all_gold_is_a':all(r['gold']=='a' for r in group_refs.values()),
                                     'duplicate_log_rows_selected_last': 80 if group=='extreme' else 0,
                                     'ranking_is_not_scored':True}})
    return tasks
