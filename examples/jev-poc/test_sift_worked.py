"""Request/reference integrity tests; every constructed response is synthetic."""
import copy
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
from poc import ContractError, digest, encoded, read_json

ROOT=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('sift_example', ROOT/'sift/run.py')
sift=importlib.util.module_from_spec(spec);spec.loader.exec_module(sift)

class SiftWorkedTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.addCleanup(self.tmp.cleanup)
        self.root=Path(self.tmp.name)
        self.book=read_json(ROOT/'sift/development.proposed.json')
        self.book['cases']=self.book['cases'][:2]
        self.bpath=self.root/'book.json';self.cpath=self.root/'capture.json'
        self.bpath.write_bytes(encoded(self.book))
        self.capture={'schema_version':1,'example':'sift','origin':'synthetic_fixture','complete':True,
          'source':{'model':'synthetic-fixture',**self.book['source']},
          'reference_binding':{'book_sha256':digest(self.bpath.read_bytes()),'selected_ids':[c['case_id'] for c in self.book['cases']]},'records':[]}
        for c in self.book['cases']:
            self.capture['records'].append({'case_id':c['case_id'],
              'request':{'model':'jev-latest','state':c['model_input']['text'],'questions':{'relevant':{'type':'noul','instructions':sift.PREFIX+c['model_input']['query']}}},
              'output':{'id':c['case_id'],'model':'synthetic-fixture','answers':{'relevant':{'type':'boolean','probability':.6}}},
              'metadata':{'response':{'model':'synthetic-fixture','answers':{'relevant':{'type':'noul','noul':.6}}}}})
    def admit(self,fixture=True):
        self.cpath.write_bytes(encoded(self.capture));return sift.admit_capture(self.bpath,self.cpath,fixture)
    def test_pending_packet_and_disjoint_family_splits(self):
        dev,_=sift.load_book(ROOT/'sift/development.proposed.json');held,_=sift.load_book(ROOT/'sift/held_out.proposed.json')
        self.assertEqual(len(dev['cases']),16);self.assertEqual(len(held['cases']),8)
        self.assertFalse({c['family_id'] for c in dev['cases']} & {c['family_id'] for c in held['cases']})
        self.assertTrue(all(c['review']['state']=='pending' for c in dev['cases']+held['cases']))
    def test_preflight_names_real_barriers_without_secret(self):
        with patch.dict(os.environ,{},clear=True):r=sift.preflight(self.bpath,None,None)
        self.assertEqual(r['status'],'blocked');self.assertEqual(len(r['pending_case_ids']),2)
        self.assertFalse(r['checks']['provider_key_present']);self.assertEqual(r['model_calls'],0)
    def test_good_fixture_allowed_only_in_fixture_mode(self):
        self.admit()
        with self.assertRaises(ContractError):self.admit(False)
    def test_observed_never_uses_pending_gold_or_fixture_bypass(self):
        self.capture['origin']='observed'
        for mode in (True,False):
            with self.subTest(mode=mode),self.assertRaises(ContractError):self.admit(mode)
    def test_edited_label_invalidates_snapshot_binding(self):
        self.book['cases'][0]['proposed_expected']['label']='irrelevant';self.bpath.write_bytes(encoded(self.book))
        with self.assertRaisesRegex(ContractError,'snapshot mismatch'):self.admit()
    def test_changed_input_rejected_even_with_updated_book_hash(self):
        self.book['cases'][0]['model_input']['text']='different text';self.bpath.write_bytes(encoded(self.book))
        self.capture['reference_binding']['book_sha256']=digest(self.bpath.read_bytes())
        with self.assertRaisesRegex(ContractError,'Effective request'):self.admit()
    def test_changed_question_rejected(self):
        self.capture['records'][0]['request']['questions']['relevant']['instructions']='different task'
        with self.assertRaisesRegex(ContractError,'Effective request'):self.admit()
    def test_probability_translation_mismatch_rejected(self):
        self.capture['records'][0]['output']['answers']['relevant']['probability']=.9
        with self.assertRaisesRegex(ContractError,'probability'):self.admit()
    def test_native_model_mismatch_rejected(self):
        self.capture['records'][0]['metadata']['response']['model']='not-the-producing-model'
        with self.assertRaisesRegex(ContractError,'model identity'):self.admit()
    def test_incomplete_capture_rejected_before_publication(self):
        self.capture['complete']=False
        with self.assertRaisesRegex(ContractError,'complete Sift'):self.admit()
        self.assertFalse((self.root/'out').exists())
    def test_missing_duplicate_unknown_ids_rejected(self):
        original=copy.deepcopy(self.capture)
        for mutation in ('missing','duplicate','unknown'):
            self.capture=copy.deepcopy(original)
            if mutation=='missing':self.capture['records'].pop()
            elif mutation=='duplicate':self.capture['records'][1]=copy.deepcopy(self.capture['records'][0])
            else:self.capture['records'][0]['case_id']='wrong'
            with self.subTest(mutation=mutation),self.assertRaises(ContractError):self.admit()
    def test_unexpected_truncation_rejected(self):
        self.capture['records'][0]['output']['truncated']=True
        with self.assertRaisesRegex(ContractError,'truncated'):self.admit()
    def test_single_partition_required(self):
        held=read_json(ROOT/'sift/held_out.proposed.json');self.book['cases']+=held['cases'];self.bpath.write_bytes(encoded(self.book))
        with self.assertRaisesRegex(ContractError,'partition-specific'):self.admit()
    def test_error_is_explicit_no_decision_not_probability(self):
        self.capture['records'][0]['output']={'id':self.book['cases'][0]['case_id'],'error':'transport error'}
        self.capture['records'][0]['metadata']={};self.admit()
    def test_native_threshold_comparison_known_fixture(self):
        binary=Path(os.environ.get('VALIDATOR_BINARY',ROOT.parents[1]/'target/debug/validator'))
        if not binary.is_file():self.skipTest('Native executable is supplied by CI build or VALIDATOR_BINARY')
        self.admit()
        summary=sift.compare(self.bpath,self.cpath,binary.resolve(),self.root/'out',fixture=True)
        self.assertEqual(summary['recovered_case_ids'],[self.book['cases'][1]['case_id']])
        self.assertEqual(summary['regressed_case_ids'],[self.book['cases'][0]['case_id']])
        self.assertIsNone(summary['quality_metrics']);self.assertIsNone(summary['model_accuracy_claim'])
        self.assertEqual(summary['comparison_inference_calls'],0)
        self.assertIn('not Jev accuracy',(self.root/'out/review.md').read_text())
        manifest=read_json(self.root/'out/evidence-manifest.json')
        self.assertTrue(all(digest((self.root/'out'/p).read_bytes())==h for p,h in manifest.items()))
        with self.assertRaises(FileExistsError):sift.compare(self.bpath,self.cpath,binary.resolve(),self.root/'out',fixture=True)
