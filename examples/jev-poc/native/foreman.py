#!/usr/bin/env python3
"""Run actual pinned Foreman schema/assessment code with an injected fake SDK client.

No workers, repository observation builder, subprocess tools, or provider calls run.
This is an application integration test, not a model-quality experiment.
"""
from __future__ import annotations

import argparse
import asyncio
import copy
import json
import math
import os
import socket
import subprocess
import sys
from pathlib import Path
from types import SimpleNamespace

PIN = "a7d21d18d306a0cb9f3e15acefbdb5663521405c"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--upstream", type=Path, required=True)
    parser.add_argument("--book", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    actual = subprocess.check_output(["git", "-C", str(args.upstream), "rev-parse", "HEAD"], text=True).strip()
    if actual != PIN:
        raise ValueError("wrong upstream pin")
    if subprocess.check_output(["git", "-C", str(args.upstream), "diff", "--name-only", "HEAD", "--", "src"], text=True).strip():
        raise ValueError("dirty upstream source")
    # Import the real upstream package. Its optional-client fallback is implemented
    # by upstream itself; do not replace any Foreman module or Pydantic type.
    sys.path.insert(0, str(args.upstream.resolve() / "src"))
    from pydantic import ValidationError, __version__ as pydantic_version
    from foreman.observation import FactoryObservation
    from foreman.foreman.jev import ASSESSMENT_QUESTIONS, JevForemanModel, parse_jev_response
    from foreman.foreman.base import ForemanModelError

    original_connect = socket.socket.connect
    def deny(*_args, **_kwargs):
        raise RuntimeError("NETWORK_FORBIDDEN: fixture-only")
    socket.socket.connect = deny
    tests = []
    def rejects(name, exc_types, fn):
        try:
            fn()
        except exc_types:
            tests.append(name)
        else:
            raise AssertionError(f"{name}: did not reject")
    book = json.loads(args.book.read_text())
    if book["example"] != "foreman" or len(book["cases"]) != 60:
        raise ValueError("expected 60-case Foreman proposal book")
    names = list(ASSESSMENT_QUESTIONS)
    for case in book["cases"]:
        FactoryObservation.model_validate(case["model_input"])
    tests.append("all-60-snapshots-accepted-by-real-FactoryObservation")
    state = book["cases"][0]["model_input"]
    rejects("unknown-observation-field", ValidationError,
            lambda: FactoryObservation.model_validate(dict(state, unexpected=True)))
    rejects("negative-observation-elapsed", ValidationError,
            lambda: FactoryObservation.model_validate(dict(state, elapsed_factory_seconds=-1)))
    rejects("wrong-worker-map-shape", ValidationError,
            lambda: FactoryObservation.model_validate(dict(state, worker_exit_status=[])))
    rejects("nonstring-text", (ValidationError, TypeError),
            lambda: FactoryObservation.model_validate(dict(state, git_diff=3)))
    def raw(value=.2):
        return {"model": "synthetic-fixture", "answers": {n: {"type": "noul", "noul": value} for n in names}}
    complete = raw()
    for name, value in [("boolean", True), ("nan", math.nan), ("infinity", math.inf), ("string", "0.5")]:
        bad = copy.deepcopy(complete); bad["answers"][names[0]]["noul"] = value
        rejects(f"reject-{name}-probability", ForemanModelError, lambda b=bad: parse_jev_response(b))
    missing = copy.deepcopy(complete); del missing["answers"][names[0]]
    rejects("missing-assessment-head", ForemanModelError, lambda: parse_jev_response(missing))
    sdk_shape = SimpleNamespace(nouls={n: SimpleNamespace(noul=.2) for n in names})
    assert parse_jev_response(sdk_shape).needs_human == .2
    tests.append("real-parser-accepts-SDK-object-shape")
    overshoot = raw(); overshoot["answers"][names[0]]["noul"] = 8.0
    clamped = parse_jev_response(overshoot)
    assert getattr(clamped, names[0]) == 1.0 and overshoot["answers"][names[0]]["noul"] == 8.0
    tests.append("raw-overshoot-remains-visible-despite-upstream-clamping")

    class Client:
        def __init__(self, response):
            self.response, self.requests, self.closed = response, [], False
        async def system_one(self, **kwargs):
            self.requests.append({"state": kwargs["state"], "model": kwargs["model"],
                                  "questions": {n: {"type": "noul", "instructions": q.instructions} for n, q in kwargs["questions"].items()}})
            if isinstance(self.response, Exception):
                raise self.response
            return self.response
        async def aclose(self):
            self.closed = True

    async def exercise():
        records = []
        selected = [c for c in book["cases"] if c["partition"] == "development"][:3]
        for i, case in enumerate(selected):
            response = raw()
            active = [["needs_human"], ["meaningful_progress"], ["implementation_complete", "meaningful_progress", "needs_verification"]][i]
            for name in active:
                response["answers"][name]["noul"] = .9
            response["answers"]["ready_to_finish"]["noul"] = [.2, .5, .8][i]
            client = Client(response)
            model = JevForemanModel(client=client, model="synthetic-fixture", timeout_seconds=.1)
            assessment = await model.assess(FactoryObservation.model_validate(case["model_input"]))
            assert len(client.requests) == 1 and list(client.requests[0]["questions"]) == names
            assert client.requests[0]["state"] == FactoryObservation.model_validate(case["model_input"]).model_dump(mode="json")
            assert not set(client.requests[0]["state"]) & {"review", "rationale", "proposed_expected", "partition"}
            await model.close()
            assert not client.closed  # injected client ownership remains with caller
            records.append({"case_id": case["case_id"], "output": assessment.model_dump(mode="json"),
                            "request": client.requests[0], "model": "synthetic-fixture",
                            "metadata": {"response": response, "transport": "in-memory-fixture"}})
        tests.extend(["ten-native-questions-one-call-per-snapshot", "reference-metadata-absent-from-state", "injected-client-not-closed-by-model"])
        broken = JevForemanModel(client=Client(RuntimeError("fixture service failure")))
        try:
            await broken.assess(FactoryObservation.model_validate(state))
        except ForemanModelError:
            tests.append("client-failure-translated-by-native-adapter")
        else:
            raise AssertionError("upstream failure not translated")
        return records
    try:
        records = asyncio.run(exercise())
    finally:
        socket.socket.connect = original_connect
    result = {"schema_version": 1, "example": "foreman", "origin": "synthetic_fixture", "complete": True,
              "live_model_calls": 0, "source": {"model": "synthetic-fixture", "repository": "thruwire/foreman", "commit": PIN},
              "records": records, "tests": tests, "pydantic": pydantic_version,
              "canary": {"raw": 8.0, "normalized": 1.0, "meaning": "upstream clamps any finite overshoot, not only a small one"},
              "scope": "Actual upstream schema, questions, parser, assessment adapter and injected-client ownership; fake SDK responses. No worker or provider call."}
    fd = os.fdopen(os.open(args.out, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600), "w", encoding="utf8")
    with fd:
        json.dump(result, fd, indent=2, allow_nan=False); fd.write("\n")
    print(json.dumps({"passed": len(tests), "live_model_calls": 0, "capture": str(args.out)}))


if __name__ == "__main__":
    main()
