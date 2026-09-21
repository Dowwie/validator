"""Small independent hard-accounting oracle, not a replacement product evaluator.

Consumes original case/reference/decision triples. Does not import preparation or
Validator. Integer assertions are exact; fractions use a declared tolerance.
"""
from collections import Counter
import math


def require(condition, message):
    if not condition:
        raise ValueError(message)


def close(actual, expected, name):
    value = actual.get('value')
    require((value is None and expected is None) or
            (value is not None and expected is not None and math.isclose(value, expected, rel_tol=1e-10, abs_tol=1e-12)),
            f'{name}: got {value!r}, expected {expected!r}')


def single(gold, predictions, labels):
    require(set(gold) == set(predictions), 'oracle population mismatch')
    counts = Counter((gold[i], predictions[i]) for i in gold)
    require(set(gold.values()) <= set(labels), 'oracle unknown gold')
    require(set(predictions.values()) <= set(labels) | {None}, 'oracle unknown prediction')
    total = len(gold)
    answered = sum(p is not None for p in predictions.values())
    correct = sum(gold[i] == predictions[i] for i in gold)
    matrix = [[counts[y, p] for p in [*labels, None]] for y in labels]
    per_class = {}
    for label in labels:
        tp = counts[label, label]
        support = sum(y == label for y in gold.values())
        predicted = sum(p == label for p in predictions.values())
        fp, fn = predicted - tp, support - tp
        per_class[label] = {'tp': tp, 'fp': fp, 'fn': fn, 'support': support,
                            'precision': tp / predicted if predicted else None,
                            'recall': tp / support if support else None,
                            'f1': 2 * tp / (2 * tp + fp + fn) if 2 * tp + fp + fn else None}
    return {'total': total, 'answered': answered, 'correct': correct,
            'abstained': total - answered, 'wrong': answered - correct,
            'accuracy': correct / total if total else None,
            'selective_accuracy': correct / answered if answered else None,
            'coverage': answered / total if total else None,
            'macro_f1': sum(c['f1'] or 0 for c in per_class.values()) / len(labels) if total else None,
            'matrix': matrix, 'per_class': per_class}


def verify_single(report, gold, predicted, labels, family='final'):
    expected = single(gold, predicted, labels)
    hard = report[family]
    for key in ('total', 'correct', 'answered', 'wrong', 'abstained'):
        require(hard[key] == expected[key], f'{family}.{key}')
    require(hard['matrix']['rows'] == expected['matrix'], f'{family}: matrix cells')
    for key in ('accuracy', 'selective_accuracy', 'coverage'):
        close(hard[key], expected[key], f'{family}.{key}')
    close(hard['macro_f1']['metric'], expected['macro_f1'], f'{family}.macro_f1')
    for row in hard['classes']:
        e = expected['per_class'][row['label']]
        for key, source in [('true_positive', 'tp'), ('false_positive', 'fp'), ('false_negative', 'fn'), ('support', 'support')]:
            require(row[key] == e[source], f'{family}.{row["label"]}.{key}')
        for key in ('precision', 'recall', 'f1'):
            close(row[key], e[key], f'{family}.{row["label"]}.{key}')
    return expected


def transition_ids(gold, before, after):
    require(set(gold) == set(before) == set(after), 'comparison population mismatch')
    gold = {k: frozenset(v) if isinstance(v, list) else v for k, v in gold.items()}
    before = {k: frozenset(v) if isinstance(v, list) else v for k, v in before.items()}
    after = {k: frozenset(v) if isinstance(v, list) else v for k, v in after.items()}
    return {'both_correct': sorted(i for i in gold if before[i] == gold[i] and after[i] == gold[i]),
            'neither_correct': sorted(i for i in gold if before[i] != gold[i] and after[i] != gold[i]),
            'recovered': sorted(i for i in gold if before[i] != gold[i] and after[i] == gold[i]),
            'regressed': sorted(i for i in gold if before[i] == gold[i] and after[i] != gold[i]),
            'changed_final_outcomes': sorted(i for i in gold if before[i] != after[i])}


def multilabel(gold, predictions, labels):
    require(set(gold) == set(predictions), 'multilabel population mismatch')
    answered = [i for i in gold if predictions[i] is not None]
    exact = sum(set(gold[i]) == set(predictions[i]) for i in answered)
    counts = {}
    for label in labels:
        cells = Counter((label in gold[i], label in predictions[i]) for i in answered)
        counts[label] = {'true_positive': cells[True, True], 'false_positive': cells[False, True],
                         'false_negative': cells[True, False], 'true_negative': cells[False, False]}
        require(sum(counts[label].values()) == len(answered), 'label conservation')
    errors = sum(c['false_positive'] + c['false_negative'] for c in counts.values())
    return {'total': len(gold), 'answered': len(answered), 'abstained': len(gold) - len(answered),
            'exact_matches': exact, 'wrong_sets': len(answered) - exact,
            'exact_match_accuracy': exact / len(gold) if gold else None,
            'answered_hamming_loss': errors / (len(answered) * len(labels)) if answered else None,
            'labels': counts}


def verify_multi(report, gold, predictions, labels):
    expected = multilabel(gold, predictions, labels)
    for family in ('raw', 'final'):
        hard = report[family]
        for key in ('total', 'answered', 'abstained', 'exact_matches', 'wrong_sets'):
            require(hard[key] == expected[key], f'multi.{family}.{key}')
        for row in hard['labels']:
            for key, value in expected['labels'][row['label']].items():
                require(row[key] == value, f'multi.{family}.{row["label"]}.{key}')
        for key in ('exact_match_accuracy', 'answered_hamming_loss'):
            close(hard[key], expected[key], f'multi.{family}.{key}')
    return expected
