#!/usr/bin/env python3
"""Apply explicit task priorities to completed replay metrics. Never select a winner
across tasks or alter a reference. This is deterministic interpretation, not a
model-generated causal explanation or a production release authorization.
"""
import argparse
import json
from pathlib import Path


def assessments(result):
    tasks = result['tasks']
    review = {}
    if 'spam-binary' in tasks:
        a,b = [tasks['spam-binary']['arms'][x]['per_class']['spam'] for x in ('plain','criteria')]
        review['spam-binary'] = {'candidate':'criteria','false_positive_change': b['fp']-a['fp'],
            'false_negative_change': b['fn']-a['fn'],
            'rubric_status': 'meets_both_error_guards' if b['fp']<=a['fp'] and b['fn']<=a['fn'] else 'tradeoff_or_regression',
            'recommendation':'Choose an explicit allowed missed-spam cost before accepting a reduction in false alarms. Do not call this the deduplicated headline population.'}
    for key in ('email-main','email-fresh','email-recent'):
        if key not in tasks:continue
        task=tasks[key]; entries=[]
        for baseline,candidate in [('text','enriched'),('enriched','framing')]:
            a,b=[task['arms'][x] for x in (baseline,candidate)]
            old,new=a['per_class']['phishing'],b['per_class']['phishing']
            # Fixed native label order legitimate/spam/phishing. The matrix is
            # verified independently before reaching this review.
            false_alarm_change=b['matrix'][0][2]-a['matrix'][0][2]
            rule=new['fn']<=old['fn'] and false_alarm_change<=0
            if key!='email-recent':rule=rule and b['correct']>=a['correct']
            entries.append({'baseline':baseline,'candidate':candidate,
              'phishing_missed_change':new['fn']-old['fn'],'legitimate_to_phishing_change':false_alarm_change if key!='email-recent' else None,
              'rubric_status':'meets_declared_cohort_guards' if rule else 'tradeoff_or_regression',
              'recommendation':'Preserve the metadata-rich input and test wording changes on a new locked mixed-label corpus; do not trade away recent phishing recall unnoticed.'})
        review[key]=entries
    if 'banking77' in tasks:
        arm=tasks['banking77']['arms']['confidence-0.8']
        accuracy=arm['selective_accuracy'];coverage=arm['coverage']
        review['banking77']={'candidate':'confidence-0.8','conditional_accuracy':accuracy,'coverage':coverage,
            'accuracy_at_least_0_95': accuracy is not None and accuracy>=.95,
            'coverage_at_least_0_75':coverage is not None and coverage>=.75,
            'rubric_status':'meets_both_guards' if accuracy is not None and accuracy>=.95 and coverage>=.75 else 'does_not_meet_both_guards',
            'recommendation':'Review high-confidence confusion pairs and refine intent criteria on separate development data; do not equate abstaining with fixing predictions.'}
    if 'clean-code' in tasks:
        review['clean-code']={'rubric_status':'semantic_performance_unmeasured',
            'recommendation':'Human-review the eight proposed code cases, capture real per-question answers, and preserve applicability. Do not score final worst-window signals as calibrated marginals.'}
    return review


def render(result):
    lines=['# Made with Jev: measured feedback examples','',
           '**Authority: Reference.** Retrospective replay and synthetic software tests, not a production release approval.','',
           'All measurements below were produced by the actual Validator executable from case-level data. There were no new inference calls and no fabricated human reference approvals.','',
           '## Published-response replay','',
           '| Task | Arm | Cases | Correct | Answered | Accuracy | Accuracy among answered |','|---|---|---:|---:|---:|---:|---:|']
    for key,task in result['tasks'].items():
        if task.get('origin')!='published_replay':continue
        for arm,s in task['arms'].items():
            pct=lambda v:'not applicable' if v is None else f'{100*v:.2f}%'
            lines.append(f"| {key} | {arm} | {s['total']} | {s['correct']} | {s['answered']} | {pct(s['accuracy'])} | {pct(s['selective_accuracy'])} |")
    lines+=['','The recent email cohort contains phishing only. Its displayed accuracy is numerically recall; it cannot establish useful precision or a false-positive rate. Binary email uses all 19,528 saved records, not the upstream deduplicated 18,514-message headline sample.','',
            '## Paired changes','', '| Task | Comparison | Recovered | Regressed | Changed decisions |','|---|---|---:|---:|---:|']
    for key,task in result['tasks'].items():
        for name,s in task.get('comparisons',{}).items():
            lines.append(f"| {key} | {name} | {s['recovered']} | {s['regressed']} | {s['changed_final_outcomes']} |")
    lines+=['','The reference data and vocabulary are byte-identical within each comparison. Existing source labels are retained; `fresh` is a source cohort name, not a new held-out claim.','',
            '## Task-rubric findings','']
    for key,value in assessments(result).items():
        lines+=['### '+key,'','```json',json.dumps(value,indent=2),'```','']
    lines+=['## Interpretation limits','',
      '- The published email capture manifest names a runner hash different from the sole published runner revision. This replay explicitly acknowledges that gap; it verifies recorded-output accounting, not exact reproduction of the original producing code.',
      '- Email source bodies are not committed upstream. Case IDs, original labels, model outputs and available state digests are preserved, but this replay cannot review the missing message contents or reconstruct every request.',
      '- Banking77 requests and labels were joined to the independently frozen corpus by source ID. 145 recorded vectors sum to 0.99; they remain observations. They are not normalized to force probability-scoring admission. Native confidence remains available for its own gate.',
      '- Where complete scored vectors are admissible, their probability metrics remain separate from hard-label accuracy. Saved zero probabilities for reviewed classes yield an explicit positive_infinity log-loss status; they are not silently clipped. This does not recover hidden pre-rounding model probabilities.',
      '- Upstream ASSAY uses different calibration conventions, including a relaxed sum tolerance and right-closed bins. This replay does not claim to reproduce its ECE by changing Validator\'s semantics.',
      '- Clean Code Review testing executes the original pure question builder and our output adapter with synthetic replies. It does not execute the complete AI SDK, windowing/comment-stripping runtime or live inference.',
      '- Clean Code Review combines per-window answers using maximum Noul/minimum Score values. Aggregated signals are kept as observations. The four-label task is deliberately smaller than the full 31-finding schema.',
      '- Eight new code-reference proposals require human review. Reported fixture transitions establish software behavior, not Jev accuracy.',
      '- None of these small/retrospective evaluations supplies a deployment guarantee or proves representative production performance.','']
    return '\n'.join(lines)


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--outcomes',type=Path,required=True);p.add_argument('--out',type=Path,required=True)
    a=p.parse_args();result=json.loads(a.outcomes.read_text())
    if result.get('status')!='passed':raise ValueError('only completed verified outcomes can be reviewed')
    with a.out.open('x') as f:f.write(render(result))
    print(a.out)


if __name__=='__main__':main()
