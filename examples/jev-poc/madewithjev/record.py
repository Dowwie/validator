#!/usr/bin/env python3
"""Create a compact durable summary from completed, checked replay outputs.

All case IDs remain in the native run bundles. Committed summaries retain exact
transition counts, selected example IDs, and a hash of the complete ID arrays.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import review
from audit_probabilities import audit


def encoded(value):
    return (json.dumps(value, sort_keys=True, indent=2, ensure_ascii=False, allow_nan=False)+'\n').encode()


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--run',type=Path,required=True)
    p.add_argument('--summary',type=Path,required=True)
    p.add_argument('--report',type=Path,required=True)
    args=p.parse_args()
    original=json.loads((args.run/'outcomes.json').read_bytes())
    if original.get('status')!='passed':raise ValueError('replay has not completed')
    result=copy.deepcopy(original)
    result['probability_audit']=audit(args.run)
    result['raw_outcomes_sha256']=hashlib.sha256((args.run/'outcomes.json').read_bytes()).hexdigest()
    result['task_rubric_assessments']=review.assessments(original)
    result['script_sha256']={name:hashlib.sha256((Path(__file__).parent/name).read_bytes()).hexdigest()
                             for name in ['run.py','oracle.py','clean-questions.mjs','rubric.json','pins.json','audit_probabilities.py','record.py','review.py']}
    for task in result['tasks'].values():
        comparisons=list(task.get('comparisons',{}).values())
        if 'comparison' in task:comparisons.append(task['comparison'])
        for comparison in comparisons:
            full=comparison.pop('source_ids')
            comparison['source_ids_sha256']=hashlib.sha256(encoded(full)).hexdigest()
            comparison['example_source_ids']={k:ids[:5] for k,ids in full.items() if k in ('recovered','regressed')}
        if 'scope_contract' in task:
            task['scope_contract']={name:{k:v for k,v in scope.items() if k!='request'} if isinstance(scope,dict) else scope
                                    for name,scope in task['scope_contract'].items()}
    with args.summary.open('xb') as f:f.write(encoded(result))
    with args.report.open('x') as f:f.write(review.render(original))
    print(json.dumps({'summary':str(args.summary),'report':str(args.report),'status':'recorded'}))


if __name__=='__main__':main()
