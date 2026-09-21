#!/usr/bin/env python3
"""Independently check categorical arithmetic from prepared, not raw, inputs.

No imports from the preparation module or production evaluator. This verifies
metric arithmetic and explicit non-applicability, not raw-provider calibration.
"""
import argparse
import json
import math
from pathlib import Path


def need(value, message):
    if not value:raise ValueError(message)


def audit(root):
    result={}
    for path in sorted(root.glob('*/*/run/report.json')):
        report=json.loads(path.read_bytes())
        if report['task']['kind']!='single_label':continue
        prepared=path.parent.parent/'prepared'
        gold=json.loads((prepared/'golden.json').read_bytes())
        rows=json.loads((prepared/'predictions.json').read_bytes())['predictions']
        labels=gold['task']['labels'];expected={r['id']:r['expected']['label'] for r in gold['episodes']}
        metrics=report['probability'];name=str(path.relative_to(root))
        available=all('probabilities' in r for r in rows)
        if not available:
            need(all('probabilities' not in r for r in rows),'partial scoring reached the core')
            need(metrics['log_loss']['status']=='not_applicable','missing probabilities silently scored')
            result[name]={'categorical_scoring':'not_applicable','rows':len(rows)}
            continue
        losses=[];briers=[];zeros=0
        for row in rows:
            values=row['probabilities']['values'];mass=math.fsum(values.values())
            need(abs(mass-1)<=1e-9,'inadmissible scored distribution')
            working={k:values[k]/mass for k in labels};truth=expected[row['id']]
            if working[truth]==0:zeros+=1
            else:losses.append(-math.log(working[truth]))
            briers.append(math.fsum((working[k]-(1 if k==truth else 0))**2 for k in labels))
        brier=math.fsum(briers)/len(rows)
        need(math.isclose(brier,metrics['brier_score']['value'],rel_tol=1e-10,abs_tol=1e-12),'Brier arithmetic differs')
        if zeros:
            need(metrics['log_loss']['status']=='positive_infinity' and metrics['log_loss']['value'] is None,'zero truth probability must have explicit infinite-loss status')
        else:
            need(math.isclose(math.fsum(losses)/len(rows),metrics['log_loss']['value'],rel_tol=1e-10,abs_tol=1e-12),'log-loss arithmetic differs')
        result[name]={'categorical_scoring':'independently_checked','zero_probability_for_reference_count':zeros,
                      'brier':brier,'log_loss_status':metrics['log_loss']['status'],'rows':len(rows)}
    need(bool(result),'no single-label reports were audited')
    return {'status':'passed','scope':'Independent arithmetic on prepared canonical distributions, not a raw-provider calibration audit.', 'reports':result}


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--run',type=Path,required=True);p.add_argument('--out',type=Path,required=True)
    args=p.parse_args();result=audit(args.run)
    with args.out.open('x') as handle:json.dump(result,handle,indent=2);handle.write('\n')
    print(json.dumps({'status':'passed','reports':len(result['reports'])}))


if __name__=='__main__':main()
