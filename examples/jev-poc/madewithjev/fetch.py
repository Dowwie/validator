#!/usr/bin/env python3
"""Download only pinned public source archives. Never execute upstream code.

Separate network acquisition from offline measurement. Does not fetch email links,
provider keys, or dataset URLs found inside the downloaded sources.
"""
import argparse
import hashlib
import io
import json
from pathlib import Path
import shutil
import tarfile
import tempfile
import urllib.request

HERE = Path(__file__).resolve().parent
MAX_ARCHIVE = 100 * 1024 * 1024
MAX_EXPANDED = 300 * 1024 * 1024


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out', type=Path, required=True)
    args = p.parse_args()
    destination = args.out.resolve()
    if destination.exists():
        p.error('destination must not exist')
    destination.parent.mkdir(parents=True, exist_ok=True)
    pins = json.loads((HERE/'pins.json').read_text())
    with tempfile.TemporaryDirectory(prefix='.jev-download-', dir=destination.parent) as temp:
        root = Path(temp)
        manifest = {}
        for name, spec in pins['sources'].items():
            url = f'https://codeload.github.com/{spec["repository"]}/tar.gz/{spec["commit"]}'
            with urllib.request.urlopen(url, timeout=90) as response:
                data = response.read(MAX_ARCHIVE + 1)
            if len(data) > MAX_ARCHIVE:
                raise ValueError('archive exceeds declared limit')
            folder = root/name
            folder.mkdir()
            with tarfile.open(fileobj=io.BytesIO(data), mode='r:gz') as archive:
                members = archive.getmembers()
                if sum(m.size for m in members) > MAX_EXPANDED:
                    raise ValueError('expanded archive exceeds declared limit')
                for member in members:
                    parts = Path(member.name).parts
                    if not parts or '..' in parts or Path(member.name).is_absolute():
                        raise ValueError('unsafe archive path')
                    if len(parts) == 1:
                        continue
                    relative = Path(*parts[1:])
                    target = folder/relative
                    if member.isdir():
                        target.mkdir(parents=True, exist_ok=True)
                    elif member.isfile():
                        target.parent.mkdir(parents=True, exist_ok=True)
                        stream = archive.extractfile(member)
                        if stream is None:
                            raise ValueError('missing member body')
                        with target.open('xb') as handle:
                            shutil.copyfileobj(stream, handle)
                    else:
                        # Links/devices are never necessary for these data replays.
                        raise ValueError(f'unsupported archive member: {member.name}')
            for path, wanted in spec['files'].items():
                actual = hashlib.sha256((folder/path).read_bytes()).hexdigest()
                if actual != wanted['sha256']:
                    raise ValueError(f'wrong pinned file: {name}/{path}')
            manifest[name] = {'url': url, 'archive_sha256': hashlib.sha256(data).hexdigest()}
        (root/'acquisition.json').write_text(json.dumps(manifest, indent=2)+'\n')
        # Refuse replacement even after a concurrent directory creation.
        destination.mkdir(exist_ok=False, mode=0o700)
        try:
            for item in root.iterdir():
                shutil.move(str(item), str(destination/item.name))
        except BaseException:
            shutil.rmtree(destination)
            raise
    print(json.dumps({'out': str(destination), 'sources': list(manifest), 'inference_calls': 0}))


if __name__ == '__main__':
    main()
