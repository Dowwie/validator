#!/usr/bin/env python3
"""Acquire exact public evidence revisions. No upstream code or provider call runs.

Needs git and pyarrow==21.0.0. Acquisition is the only network-enabled command.
No corpus is committed by this command; keep the destination outside the repo.
"""
from __future__ import annotations
import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile

from readers import PINS, verify_sources


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out',type=Path,required=True)
    args=parser.parse_args()
    import pyarrow
    import pyarrow.parquet as pq
    if pyarrow.__version__ != '21.0.0':
        parser.error('Install pyarrow==21.0.0 for the frozen metadata export.')
    args.out.mkdir(parents=True,exist_ok=False,mode=0o700)
    env=dict(os.environ,GIT_TERMINAL_PROMPT='0')
    for name in list(env):
        if any(token in name.upper() for token in ('TOKEN','SECRET','API_KEY')):
            env.pop(name)
    with tempfile.TemporaryDirectory(prefix='validator-public-evidence-') as tmp:
        for name,(repo,pin) in PINS.items():
            checkout=Path(tmp)/name;checkout.mkdir()
            subprocess.run(['git','init','-q',str(checkout)],check=True,env=env)
            command=['git','-c','credential.helper=','-C',str(checkout)]
            subprocess.run(command+['fetch','--quiet','--depth=1',f'https://github.com/{repo}.git',pin],check=True,env=env,timeout=180)
            actual=subprocess.check_output(command+['rev-parse','FETCH_HEAD'],env=env,text=True).strip()
            if actual!=pin:raise ValueError('source revision mismatch')
            data=subprocess.check_output(command+['archive','FETCH_HEAD'],env=env)
            target=args.out/name;target.mkdir(mode=0o700)
            with tarfile.open(fileobj=io.BytesIO(data)) as archive:
                archive.extractall(target,filter='data')
            (target/'.source-revision').write_text(pin+'\n')
    p=args.out/'acento/data/items.parquet'
    rows=pq.read_table(p).to_pylist()
    data=json.dumps(rows,sort_keys=True,ensure_ascii=False,allow_nan=False,default=str).encode()
    p.with_suffix('.json').write_bytes(data)
    p.with_suffix('.export.json').write_text(json.dumps({
        'source_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),
        'json_sha256':hashlib.sha256(data).hexdigest(),
        'rows':len(rows),
        'exporter':'pyarrow-21.0.0 to_pylist; json sort_keys ensure_ascii=False default=str'},indent=2))
    verify_sources(args.out,list(PINS))
    print(json.dumps({'status':'verified','pins':PINS,'inference_calls':0}))


if __name__=='__main__':main()
