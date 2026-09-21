"""Independent source-to-artifact checks plus explicit malformed-input regressions.

Run with PORTFOLIO_SOURCES, VALIDATOR_BIN and PORTFOLIO_RESULTS set. The result
folder is produced by run.py, not invented golden model output.
"""
import copy
import json
import math
import os
from pathlib import Path
import sys
import unittest

HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE))
sys.path.insert(0,str(HERE.parent))
from tasks import atlas_tasks, checked_sources, choice_cases, fixture_tasks, indexed, triage_cases
from poc import ContractError, episode_id, read_json, strict_json
from madewithjev.oracle import verify_single
from run import probability_oracle, reference_map, verify_probability_metrics


class PortfolioTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        # Required inputs deliberately fail instead of silently skipping native evidence.
        cls.root=Path(os.environ['PORTFOLIO_SOURCES']).resolve()
        cls.results=Path(os.environ['PORTFOLIO_RESULTS']).resolve()
        cls.pins,_=checked_sources(cls.root)
        cls.tasks={t['book']['example']:t for t in atlas_tasks(cls.root,cls.pins)}
        cls.capture=read_json(cls.results/'native-capture.json')
        cls.summary=read_json(cls.results/'summary.json')
        cls.cases=read_json(cls.root/'atlas/suites/citation-support-check/data/cases.json')
        cls.records=read_json(cls.root/'atlas/suites/citation-support-check/runs/2026-09-19.json')['results']
        cls.labels=['supports','contradicts','says_nothing']
        cls.state=staticmethod(lambda c:{'claim':c['claim'],'quote':c['quote']})

    def parse_choice(self, cases=None, records=None):
        return choice_cases(cases or self.cases,records or self.records,self.labels,self.state,'specific test question')

    def test_complete_six_task_inventory(self):
        self.assertEqual(set(self.summary['results']),{'citation-support','sarcasm-local','sarcasm-context','feed-triage','sponsor-action','moderation-action'})
        self.assertEqual((self.summary['evaluations'],self.summary['comparisons']),(12,6))
        self.assertEqual(self.summary['live_model_calls'],0)

    def test_independent_reordering_does_not_change_mapping(self):
        a=self.parse_choice();b=self.parse_choice(list(reversed(self.cases)),self.records[2:]+self.records[:2])
        self.assertEqual({r['case_id']:r['prediction'] for r in a[1]}, {r['case_id']:r['prediction'] for r in b[1]})
        self.assertEqual({c['case_id']:c['proposed_expected'] for c in a[0]}, {c['case_id']:c['proposed_expected'] for c in b[0]})

    def test_duplicate_reference_id_rejected(self):
        with self.assertRaises(ContractError):self.parse_choice(self.cases+[self.cases[0]])

    def test_duplicate_prediction_id_rejected(self):
        with self.assertRaises(ContractError):self.parse_choice(records=self.records+[self.records[0]])

    def test_missing_prediction_rejected(self):
        with self.assertRaises(ContractError):self.parse_choice(records=self.records[:-1])

    def test_unknown_prediction_id_rejected(self):
        rows=copy.deepcopy(self.records);rows[0]['id']='other'
        with self.assertRaises(ContractError):self.parse_choice(records=rows)

    def test_reference_disagreement_rejected(self):
        rows=copy.deepcopy(self.records);rows[0]['expected']='supports'
        with self.assertRaises(ContractError):self.parse_choice(records=rows)

    def test_invalid_probability_forms_rejected(self):
        for value in [True,'0.9',-0.1,1.1,math.nan,math.inf]:
            with self.subTest(value=value):
                rows=copy.deepcopy(self.records);rows[0]['probabilities']['supports']=value
                with self.assertRaises(ContractError):self.parse_choice(records=rows)

    def test_probability_label_universe_is_fixed(self):
        for alter in ['extra','missing']:
            rows=copy.deepcopy(self.records)
            if alter=='extra':rows[0]['probabilities']['new']=0
            else:del rows[0]['probabilities']['supports']
            with self.assertRaises(ContractError):self.parse_choice(records=rows)

    def test_unknown_selected_class_rejected(self):
        rows=copy.deepcopy(self.records);rows[0]['choice']='new'
        with self.assertRaises(ContractError):self.parse_choice(records=rows)

    def test_rounded_nonunit_vector_not_silently_normalized(self):
        rows=copy.deepcopy(self.records);rows[0]['probabilities']['contradicts']=.99
        _,preds,_=self.parse_choice(records=rows)
        self.assertNotIn('probabilities',preds[0]['prediction'])
        self.assertEqual(preds[0]['prediction']['observations']['unscored_categorical']['values']['contradicts'],.99)

    def test_native_confidence_is_not_selected_probability(self):
        row=next(r for r in self.tasks['citation-support']['arms'][0][1] if r['case_id']=='subtle_thin_support')['prediction']
        self.assertEqual(row['confidence'],.45)
        self.assertEqual(row['probabilities']['values']['says_nothing'],.63)

    def test_explicit_unknowns_stay_withheld_despite_confidence_one(self):
        t=self.tasks['sarcasm-context']
        self.assertEqual({c['id'] for c in t['withheld']},{'AMB1','AMB2'})
        self.assertEqual(len(t['book']['cases']),10)
        self.assertEqual(next(c for c in t['withheld'] if c['id']=='AMB2')['confidence'],1)
        self.assertNotIn('AMB2',{c['case_id'] for c in t['book']['cases']})

    def test_reference_authority_not_new_human_approval(self):
        for task in self.tasks.values():
            self.assertFalse(task['book']['configuration']['new_human_approval'])
            self.assertTrue(all(c['review']['state']=='pending' for c in task['book']['cases']))
            self.assertEqual(task['capture']['source']['model'],'jev-latest')
            self.assertIsNone(task['capture']['source']['resolved_model'])

    def test_historical_triage_is_not_three_way_probability(self):
        t=self.tasks['feed-triage']
        self.assertEqual(t['book']['configuration']['assessment'],'historical_label_agreement')
        for r in t['arms'][0][1]:
            self.assertNotIn('probabilities',r['prediction'])
            self.assertEqual(set(r['prediction']['observations']),{'active_trigger','reject_trigger'})

    def test_triage_priority_and_threshold_equality(self):
        d={'watch_reasons':{'t':'watch'},'roster_note':'none','cases':[{'id':'x','topic':'t','title':'Title','url':'example.invalid','content':'text','historical_label':'frozen','historical_score':4}]}
        r={'threshold':.75,'results':[{'id':'x','topic':'t','title':'Title','historical_label':'frozen','historical_score':4,'p_active':.75,'p_reject':.99,'routed':'active'}]}
        _,rows=triage_cases(d,r,.75);self.assertEqual(rows[0]['prediction']['outcome']['label'],'active')
        _,rows=triage_cases(d,r,.9);self.assertEqual(rows[0]['prediction']['outcome']['label'],'rejected')

    def test_triage_saved_routing_must_match_signals(self):
        d=read_json(self.root/'atlas/suites/stage1-triage-filter/data/cases.json')
        r=read_json(self.root/'atlas/suites/stage1-triage-filter/runs/2026-09-20.json')
        r['results'][0]['routed']='active'
        with self.assertRaises(ContractError):triage_cases(d,r,.75)

    def test_fixture_cannot_be_presented_as_observed(self):
        c=copy.deepcopy(self.capture);c['origin']='observed'
        with self.assertRaises(ContractError):fixture_tasks(c,self.pins)

    def test_moderation_fail_open_retains_application_answer_without_probability(self):
        t=next(t for t in fixture_tasks(self.capture,self.pins) if t['book']['example']=='moderation-action')
        r=next(r for r in t['arms'][0][1] if r['case_id']=='m08')['prediction']
        self.assertEqual(r['outcome'],{'type':'class','label':'allow'})
        self.assertNotIn('native_noul',r['observations'])
        self.assertEqual(r['observations']['model_answered']['value'],0)

    def test_native_captures_reject_invalid_probability(self):
        for value in [False,2,'0.5']:
            c=copy.deepcopy(self.capture);c['tasks']['moderation']['baseline'][0]['probability']=value
            with self.assertRaises(ContractError):fixture_tasks(c,self.pins)

    def test_native_candidate_keeps_same_case_inputs(self):
        c=copy.deepcopy(self.capture);c['tasks']['sponsor']['candidate'][0]['input']={'events':[]}
        with self.assertRaises(ContractError):fixture_tasks(c,self.pins)

    def test_original_reference_and_decision_mapping_independent_of_adapter(self):
        # Independently read originals and compare every canonical expected/predicted label.
        specs=[('citation-support','citation-support-check','cases','results'),('sarcasm-local','sarcasm-vs-sincere-praise','same-clause','same_clause'),('sarcasm-context','sarcasm-vs-sincere-praise','cross-turn','cross_turn')]
        for slug,suite,data_name,key in specs:
            raw_gold=read_json(self.root/f'atlas/suites/{suite}/data/{data_name}.json')
            raw_results=read_json(self.root/f'atlas/suites/{suite}/runs/2026-09-19.json')[key]
            expected={episode_id(slug,c['id']):c['expected'] for c in raw_gold if c['expected'] is not None}
            predictions={episode_id(slug,c['id']):c['choice'] for c in raw_results if c['expected'] is not None}
            canonical=read_json(self.results/slug/'recorded-prepared/golden.json')
            self.assertEqual({c['id']:c['expected']['label'] for c in canonical['episodes']},expected)
            p=read_json(self.results/slug/'recorded-prepared/predictions.json')
            self.assertEqual({c['id']:c['outcome']['label'] for c in p['predictions']},predictions)
            report=read_json(self.results/slug/'recorded/report.json')
            verify_single(report,expected,predictions,canonical['task']['labels'])


    def test_choice_signals_match_original_saved_values(self):
        for slug,suite,key in [('citation-support','citation-support-check','results'),('sarcasm-local','sarcasm-vs-sincere-praise','same_clause'),('sarcasm-context','sarcasm-vs-sincere-praise','cross_turn')]:
            source=read_json(self.root/f'atlas/suites/{suite}/runs/2026-09-19.json')[key]
            source={episode_id(slug,r['id']):r for r in source if r['expected'] is not None}
            predictions=read_json(self.results/slug/'recorded-prepared/predictions.json')['predictions']
            for row in predictions:
                self.assertEqual(row['probabilities']['values'],source[row['id']]['probabilities'])
                self.assertEqual(row['confidence'],source[row['id']]['confidence'])

    def test_known_fixture_recovery_and_regression_ids(self):
        checks={'sponsor-action':({'s11'},{'s02','s12'}),'moderation-action':({'m10'},{'m02'}),'citation-support':(set(),{'subtle_thin_support'}),'feed-triage':(set(),set())}
        for slug,(recovered,regressed) in checks.items():
            c=self.summary['results'][slug]['comparison']
            self.assertEqual(set(c['recovered']),recovered);self.assertEqual(set(c['regressed']),regressed)

    def test_confidence_gate_cannot_rewrite_probability_results(self):
        for slug in ['citation-support','sarcasm-local','sarcasm-context']:
            a=read_json(self.results/slug/'recorded/report.json');b=read_json(self.results/slug/'confidence-080/report.json')
            self.assertEqual(a['raw'],b['raw']);self.assertEqual(a['probability'],b['probability'])
            self.assertEqual((self.results/slug/'recorded/golden.json').read_bytes(),(self.results/slug/'confidence-080/golden.json').read_bytes())

    def test_no_model_accuracy_claim_from_native_fixtures(self):
        for slug in ['moderation-action','sponsor-action']:
            for arm in self.summary['results'][slug]['arms'].values():
                self.assertNotIn('reference_agreement',arm)
                self.assertIsNone(arm['model_performance_claim'])
                self.assertIsNone(arm['probability_oracle'])

    def test_strict_json_duplicate_and_nonfinite(self):
        for b in [b'{"id":1,"id":2}',b'{"p":NaN}']:
            with self.assertRaises(ContractError):strict_json(b)

    def test_zero_probability_oracle_never_clips(self):
        task=self.tasks['citation-support'];rows=copy.deepcopy(task['arms'][0][1])
        cid=rows[0]['case_id'];labels=task['book']['task']['labels'];gold=reference_map(task)
        correct=gold[episode_id('citation-support',cid)]
        incorrect=next(l for l in labels if l!=correct)
        rows[0]['prediction']['probabilities']['values']={l:float(l==incorrect) for l in labels}
        loss,brier=probability_oracle(gold,rows,labels,'citation-support')
        self.assertTrue(math.isinf(loss));self.assertGreater(brier,0)
        with self.assertRaises(ContractError):verify_probability_metrics({'probability':{'log_loss':{'status':'defined','value':30},'brier_score':{'status':'defined','value':brier}}},(loss,brier))

    def test_all_44_native_action_assertions_executed(self):
        self.assertEqual(len(self.capture['tasks']['sponsor']['baseline']),12)
        self.assertEqual(len(self.capture['tasks']['sponsor']['candidate']),12)
        self.assertEqual(len(self.capture['tasks']['moderation']['baseline']),10)
        self.assertEqual(len(self.capture['tasks']['moderation']['candidate']),10)
        self.assertEqual(len(self.capture['checks']),18)


if __name__=='__main__':unittest.main()
