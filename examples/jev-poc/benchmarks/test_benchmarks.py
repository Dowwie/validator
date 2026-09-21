"""Contract and native arithmetic checks for the published-evidence adapters."""
import copy
import math
import os
from pathlib import Path
import sys
import tempfile
import unittest

HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE))
import readers as r
import run as workflow
sys.path.insert(0,str(HERE.parent))
from poc import encoded, native_run, read_json, write_bundle
from madewithjev.oracle import verify_multi


class Contracts(unittest.TestCase):
    def test_unknown_is_not_negative(self):
        self.assertIsNone(r.known_or({'H':0},['H','HR','V']))

    def test_positive_resolves_unknown_or(self):
        self.assertTrue(r.known_or({'H':1},['H','HR','V']))

    def test_negative_requires_all_operands(self):
        self.assertFalse(r.known_or({'H':0,'HR':0,'V':0},['H','HR','V']))

    def test_reference_boolean_rejected(self):
        with self.assertRaises(r.ContractError):r.known_or({'S':True},['S'])

    def test_reference_fraction_rejected(self):
        with self.assertRaises(r.ContractError):r.known_or({'S':.3},['S'])

    def test_reference_null_rejected(self):
        with self.assertRaises(r.ContractError):r.known_or({'S':None},['S'])

    def test_index_is_id_based(self):
        rows=[{'id':'b','value':1},{'id':'a','value':2}]
        self.assertEqual(r.indexed(rows,'id'),r.indexed(rows[::-1],'id'))

    def test_duplicate_id_rejected(self):
        with self.assertRaises(r.ContractError):r.indexed([{'id':'a'},{'id':'a'}],'id')

    def test_missing_id_rejected(self):
        with self.assertRaises(r.ContractError):r.indexed([{}],'id')

    def test_complete_vector_scored(self):
        got=r.choice_prediction('a',{'a':.6,'b':.4},.9,['a','b'])
        self.assertEqual(got['probabilities']['values'],{'a':.6,'b':.4})
        self.assertEqual(got['confidence'],.9)

    def test_rounded_vector_not_normalized(self):
        got=r.choice_prediction('a',{'a':.6,'b':.39},None,['a','b'])
        self.assertNotIn('probabilities',got)
        self.assertEqual(got['observations']['unscored_categorical']['values']['b'],.39)

    def test_missing_probability_key_rejected(self):
        with self.assertRaises(r.ContractError):r.choice_prediction('a',{'a':1},None,['a','b'])

    def test_nonfinite_probability_rejected(self):
        with self.assertRaises(r.ContractError):r.choice_prediction('a',{'a':math.nan,'b':0},None,['a','b'])

    def test_out_of_range_confidence_rejected(self):
        with self.assertRaises(r.ContractError):r.choice_prediction('a',{'a':1,'b':0},2,['a','b'])

    def test_unknown_prediction_rejected(self):
        with self.assertRaises(r.ContractError):r.choice_prediction('z',{'a':1,'b':0},None,['a','b'])

    def test_moderation_requires_explicit_identity_exception(self):
        with self.assertRaises(r.ContractError):r.moderation(Path('/not/read'),False)

    def pair(self,order='ab'):
        gold={'a':'doc-1','b':'doc-2','gold':'a','source':'extreme'}
        row={'pair_id':'p1','ok':True,'kind':'choice','order':order,'gold':'a','model':'jev-1.13.0','source':'extreme',
             'id_A':'doc-1' if order=='ab' else 'doc-2','id_B':'doc-2' if order=='ab' else 'doc-1',
             'winner':'A','p_A':.6,'p_B':.4,'probabilities':{'A':.6,'B':.4},'confidence':.8}
        return row,gold

    def test_position_reversal_remaps_selected_document(self):
        ab,g=self.pair();ba,_=self.pair('ba')
        self.assertEqual(r.orient_pair(ab,g)['outcome']['label'],'a')
        out=r.orient_pair(ba,g)
        self.assertEqual(out['outcome']['label'],'b')
        self.assertEqual(out['probabilities']['values'],{'b':.6,'a':.4})
        self.assertEqual(out['confidence'],.8)

    def test_same_document_choice_is_order_invariant(self):
        ab,g=self.pair();ba,_=self.pair('ba');ba.update(winner='B',p_A=.4,p_B=.6,probabilities={'A':.4,'B':.6})
        self.assertEqual(r.orient_pair(ab,g),r.orient_pair(ba,g))

    def test_wrong_document_assignment_rejected(self):
        row,g=self.pair();row['id_A']='unrelated'
        with self.assertRaises(r.ContractError):r.orient_pair(row,g)

    def test_bad_order_rejected(self):
        row,g=self.pair();row['order']='unknown'
        with self.assertRaises(r.ContractError):r.orient_pair(row,g)

    def test_scalar_probability_mismatch_rejected(self):
        row,g=self.pair();row['p_A']=.8
        with self.assertRaises(r.ContractError):r.orient_pair(row,g)

    def test_repeated_pair_values_must_agree(self):
        row,_=self.pair();other=copy.deepcopy(row);other['confidence']=.2
        with self.assertRaises(r.ContractError):r.select_pair_answers([row,other])

    def test_repeated_pair_metadata_preserved_last(self):
        row,_=self.pair();other=copy.deepcopy(row);other['hash_A']='updated-effective-input'
        selected,n=r.select_pair_answers([row,other])
        self.assertEqual(n,1);self.assertEqual(selected['p1','ab']['hash_A'],'updated-effective-input')

    def test_failed_pair_is_not_a_negative(self):
        row,_=self.pair();row['ok']=False
        with self.assertRaises(r.ContractError):r.select_pair_answers([row])


class Native(unittest.TestCase):
    def test_multilabel_ids_counts_empty_sets_abstention_and_probability(self):
        binary=Path(os.environ.get('VALIDATOR_BIN',str(HERE.parents[2]/'target/debug/validator')))
        self.assertTrue(binary.is_file(),'Set VALIDATOR_BIN to the real native executable; this test must not silently skip.')
        labels=['A','B'];gold={'c1':['A'],'c2':['A','B'],'c3':[],'c4':['B']}
        predicted={'c1':['A'],'c2':['B'],'c3':[],'c4':None}
        cases=[r.case(cid,g,{'id':cid},multi=True) for cid,g in gold.items()]
        b=r.book('benchmark-test','moderation',labels,cases,multi=True)
        records=[{'case_id':cid,'prediction':{'outcome':r.labels_out(v,labels) if v is not None else {'type':'abstention','reason':'operational-fixture'}}} for cid,v in predicted.items()]
        for row, (a,bprob) in zip(records,[(.9,.1),(.5,.8),(0.,0.),(0.,0.)]):
            row['prediction']['probabilities']={'kind':'label_marginals','values':{'A':a,'B':bprob}}
        capture={'schema_version':1,'example':b['example'],'origin':'synthetic_fixture',
                 'source':{**b['source'],'model':'synthetic-no-inference'},'records':[]}
        with tempfile.TemporaryDirectory() as temp:
            path=Path(temp);write_bundle(b,capture,records[::-1],path/'prepared','development',{},fixture=True)
            report,_=native_run(binary.resolve(),path/'prepared',path/'run')
            expected=verify_multi(report,gold,predicted,labels)
            audit=workflow.probability_audit(report,path/'prepared')
            self.assertTrue(audit['checked'])
            self.assertEqual(audit['independent_log_loss'],'positive_infinity')
            self.assertEqual(expected['exact_matches'],2);self.assertEqual(expected['answered'],3)
            self.assertEqual(expected['labels']['A'],{'true_positive':1,'false_positive':0,'false_negative':1,'true_negative':1})
            self.assertEqual(expected['labels']['B'],{'true_positive':1,'false_positive':0,'false_negative':0,'true_negative':2})
            # A faulty denominator or FP/FN swap must be detected by the oracle.
            broken=copy.deepcopy(report);broken['final']['labels'][0]['false_positive']=1
            with self.assertRaises(ValueError):verify_multi(broken,gold,predicted,labels)
            second=copy.deepcopy(b);second['cases'].reverse()
            for c in second['cases']:c['proposed_expected']['labels'].reverse()
            write_bundle(second,capture,records,path/'reordered','development',{},fixture=True)
            replay,_=native_run(binary.resolve(),path/'reordered',path/'run-reordered')
            self.assertEqual(report['final'],replay['final'])


if __name__=='__main__':unittest.main()
