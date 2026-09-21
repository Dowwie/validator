"""Discriminating adapter/accounting tests. All constructed responses are fixtures."""
import copy
import csv
import io
import json
from pathlib import Path
import tempfile
import unittest
import sys
sys.path.insert(0, str(Path(__file__).resolve().parent))
import run as made
from oracle import single, multilabel, transition_ids


def lines(rows):
    return b'\n'.join(json.dumps(r).encode() for r in rows) + b'\n'


def email_data():
    refs=[{'file':'a','label':'ham'},{'file':'b','label':'phish'}]
    rows=[]
    for ref in refs:
        answer={'choice':made.GOLD_MAP[ref['label']], 'probabilities':{k:float(k==made.GOLD_MAP[ref['label']]) for k in made.LABELS}, 'confidence':.9}
        for arm in ['text','enriched']:
            rows.append(dict(ref,set='main',arm=arm,model='jev-1.13.0',state_sha256='a'*64,choices={'category':answer,'evidence_framing':answer}))
    manifest={'source_hash':made.digest(b'fixture-runner'),'model':'jev-1.13.0','definitions':{},'framing':'candidate','original_instruction':'baseline'}
    return {'results/phish_main_all.jsonl':lines(refs), 'experiments/jev-context/predictions.jsonl':lines(rows),
            'experiments/jev-context/evaluate.py':b'fixture-runner','experiments/jev-context/manifest.json':json.dumps(manifest).encode()}


def bank_data():
    labels=['card_arrival','card_delivery_estimate']
    corpus='text,category\nWhere is my card?,card_arrival\nWhen will delivery happen?,card_delivery_estimate\n'
    refs=list(csv.DictReader(io.StringIO(corpus)))
    rows=[]
    for i,ref in enumerate(refs):
        rows.append({'id':f'b77-{i:04d}','text':ref['text'],'label':ref['category'],
          'request':{'state':ref['text'],'model':'jev-latest','questions':{'intent':{'type':'choice','criteria':dict.fromkeys(labels)}}},
          'response':{'status':200,'body':{'model':'jev-1.13.0','answers':{'intent':{'type':'choice','choice':ref['category'],
             'probabilities':{label:.9 if label==ref['category'] else .09 for label in labels},'confidence':.8 if i==0 else .799999}}}}})
    raw=lines(rows)
    return {'raw/run/banking77/responses.jsonl':raw,'raw/run/banking77/responses.sha256':made.digest(raw).encode(),
            'corpus/banking77/categories.json':json.dumps(labels).encode(),'corpus/banking77/test.csv':corpus.encode()}


class MadeTests(unittest.TestCase):
    def test_confusion_orientation_and_abstention_column(self):
        s=single({'1':'A','2':'B','3':'A'},{'1':'B','2':'B','3':None},['A','B'])
        self.assertEqual(s['matrix'],[[0,1,1],[0,1,0]])
        self.assertEqual((s['correct'],s['wrong'],s['abstained']),(1,1,1))
    def test_rates_recomputed_not_averaged(self):
        s=single({'1':'A','2':'A','3':'B'},{'1':'A','2':'B','3':'B'},['A','B'])
        self.assertAlmostEqual(s['accuracy'],2/3)
        self.assertEqual(sum(map(sum,s['matrix'])),3)
    def test_equal_per_label_counts_different_exact_sets(self):
        g={'1':['A','B'],'2':['A','B']}
        x=multilabel(g,{'1':['A','B'],'2':[]},['A','B'])
        y=multilabel(g,{'1':['A'],'2':['B']},['A','B'])
        self.assertEqual(x['labels'],y['labels']);self.assertEqual((x['exact_matches'],y['exact_matches']),(1,0))
    def test_multilabel_order_does_not_create_transition(self):
        t=transition_ids({'x':['A','B']},{'x':['B','A']},{'x':['A','B']})
        self.assertEqual(t['both_correct'],['x']);self.assertFalse(t['changed_final_outcomes'])
    def test_multilabel_abstention_not_empty_set(self):
        s=multilabel({'1':[],'2':['A']},{'1':[],'2':None},['A','B'])
        self.assertEqual((s['answered'],s['exact_matches'],s['abstained']),(1,1,1))
        self.assertEqual(sum(s['labels']['A'].values()),1)
    def test_duplicate_json_fields_rejected(self):
        with self.assertRaises(made.ContractError):made.jsonl(b'{"x":1,"x":2}\n')
    def test_duplicate_source_id_rejected(self):
        with self.assertRaises(made.ContractError):made.unique([{'id':'a'},{'id':'a'}],lambda r:r['id'])
    def test_binary_threshold_equality(self):
        for p,label in [(.49999,'ham'),(.5,'spam'),(.50001,'spam')]:
            self.assertEqual(made.binary(p,['ham','spam'],.5)['outcome']['label'],label)
    def test_choice_not_replaced_by_argmax(self):
        a={'choice':'A','probabilities':{'A':.4,'B':.6},'confidence':.9}
        r=made.choice(a,['A','B']);self.assertEqual(r['outcome']['label'],'A');self.assertEqual(r['confidence'],.9)
    def test_non_unit_vectors_observed_not_normalized(self):
        r=made.choice({'choice':'A','probabilities':{'A':.9,'B':.09},'confidence':.8},['A','B'])
        self.assertNotIn('probabilities',r);self.assertEqual(r['observations']['unscored_categorical']['values'],{'A':.9,'B':.09})
    def test_missing_or_invalid_choice_values_rejected(self):
        good={'choice':'A','probabilities':{'A':.9,'B':.1},'confidence':.8}
        for a in [dict(good,choice='C'),dict(good,probabilities={'A':1}),dict(good,confidence=True),dict(good,confidence=1.1)]:
            with self.subTest(a=a),self.assertRaises(made.ContractError):made.choice(a,['A','B'])
    def test_email_order_independent_by_id_and_arm(self):
        d=email_data();b,arms,g=made.spam_multiclass(d,'main')
        d['experiments/jev-context/predictions.jsonl']=lines(list(reversed(made.jsonl(d['experiments/jev-context/predictions.jsonl']))))
        b2,a2,g2=made.spam_multiclass(d,'main');self.assertEqual(g,g2)
        for name in arms:self.assertEqual(arms[name][1],a2[name][1])
    def test_email_missing_or_duplicate_arm_stops(self):
        d=email_data();rows=made.jsonl(d['experiments/jev-context/predictions.jsonl'])
        for bad in (rows[:-1],rows+[rows[0]]):
            dd=dict(d);dd['experiments/jev-context/predictions.jsonl']=lines(bad)
            with self.assertRaises(ValueError):made.spam_multiclass(dd,'main')
    def test_email_reference_disagreement_stops(self):
        d=email_data();rows=made.jsonl(d['experiments/jev-context/predictions.jsonl']);rows[0]['label']='spam';d['experiments/jev-context/predictions.jsonl']=lines(rows)
        with self.assertRaises(ValueError):made.spam_multiclass(d,'main')
    def test_email_runner_digest_binding(self):
        d=email_data();d['experiments/jev-context/evaluate.py']=b'changed'
        with self.assertRaises(ValueError):made.spam_multiclass(d,'main')
    def test_unverified_runner_requires_explicit_qualified_replay(self):
        d=email_data();d['experiments/jev-context/evaluate.py']=b'changed'
        b,arms,g=made.spam_multiclass(d,'main',allow_unverified_runner=True)
        self.assertFalse(b['configuration']['producer_source_identity']['verified'])
        self.assertFalse(arms['text'][2]['producer_source_identity']['verified'])
    def test_bank_confidence_equality_and_nonunit_counts(self):
        b,a,g=made.banking(bank_data());self.assertEqual(b['configuration']['non_unit_probability_rows'],2)
        self.assertEqual(a['confidence-0.8'][1],{'b77-0000':'card_arrival','b77-0001':None})
    def test_bank_state_disagreement_rejected(self):
        d=bank_data();rows=made.jsonl(d['raw/run/banking77/responses.jsonl']);rows[0]['request']['state']='altered'
        d['raw/run/banking77/responses.jsonl']=lines(rows);d['raw/run/banking77/responses.sha256']=made.digest(lines(rows)).encode()
        with self.assertRaises(ValueError):made.banking(d)
    def test_bank_seal_rejected(self):
        d=bank_data();d['raw/run/banking77/responses.sha256']=b'0'*64
        with self.assertRaises(ValueError):made.banking(d)
    def test_clean_missing_head_never_means_negative(self):
        f=made.read_json(made.HERE/'clean-fixtures.json')['records'][0]['output'];del f['answers'][made.CLEAN_LABELS[0]]
        with self.assertRaises(made.ContractError):made.decode_clean(f,made.CLEAN_LABELS)
    def test_clean_scope_and_partial_file_rejected(self):
        f=made.read_json(made.HERE/'clean-fixtures.json')['records'][0]['output']
        with self.assertRaises(made.ContractError):made.decode_clean(f,made.CLEAN_LABELS[1:])
        for key,value in [('cut',True),('complete',False)]:
            with self.assertRaises(made.ContractError):made.decode_clean(dict(f,**{key:value}),made.CLEAN_LABELS)
    def test_clean_never_scores_aggregated_signals_or_scores(self):
        f=made.read_json(made.HERE/'clean-fixtures.json')['records'][0]['output'];r=made.decode_clean(f,made.CLEAN_LABELS)
        self.assertNotIn('probabilities',r);self.assertNotIn('confidence',r)
        self.assertEqual(r['observations']['verdict']['value'],3.4)
        self.assertEqual(r['observations']['verdict_confidence']['value'],.8)
    def test_clean_order_invariant_and_threshold_equal(self):
        f=made.read_json(made.HERE/'clean-fixtures.json')['records'][0]['output'];f['answers'][made.CLEAN_LABELS[0]]['noul']=.7
        a=made.decode_clean(f,made.CLEAN_LABELS,threshold=.7);f['answers']=dict(reversed(list(f['answers'].items())))
        self.assertEqual(a,made.decode_clean(f,list(reversed(made.CLEAN_LABELS)),threshold=.7))
        self.assertEqual(a['outcome']['labels'],[made.CLEAN_LABELS[0]])
    def test_no_reference_approval_in_proposals(self):
        b=made.read_json(made.HERE/'clean-reference-proposals.json');self.assertEqual(len(b['cases']),8)
        self.assertTrue(all(c['review']['state']=='pending' for c in b['cases']))
    def test_changes_detect_both_recovery_and_regression(self):
        g={'a':'A','b':'B'};b={'a':'B','b':'B'};c={'a':'A','b':'A'}
        t=transition_ids(g,b,c);self.assertEqual(t['recovered'],['a']);self.assertEqual(t['regressed'],['b'])


if __name__=='__main__':unittest.main()
