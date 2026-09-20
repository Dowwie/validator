"""Application-output adapters. No provider calls, labeling, or metric calculations.

The selected label always comes from the application or a documented policy;
probabilities and native confidence retain their different meanings.
"""
from __future__ import annotations

import math
from typing import Any

FOREMAN_LABELS = (
    "implementation_complete", "tests_sufficient", "requirements_satisfied",
    "needs_verification", "meaningful_progress", "worker_stuck", "work_off_track",
    "agents_md_drift", "ready_to_finish", "needs_human",
)


class ContractError(ValueError):
    """An input cannot be represented without changing its declared meaning."""


def number(value: Any, *, probability: bool = False) -> float:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise ContractError("expected a finite number, not a boolean or numeric string")
    if not math.isfinite(value) or (probability and not 0 <= value <= 1):
        raise ContractError("number is outside its declared range")
    return float(value)


def vector(value: Any, labels: list[str]) -> dict[str, float]:
    if not isinstance(value, dict) or set(value) != set(labels):
        raise ContractError("probability keys must exactly match the frozen vocabulary")
    return {label: number(value[label], probability=True) for label in labels}


def class_out(label: Any, labels: list[str]) -> dict:
    if not isinstance(label, str) or label not in labels:
        raise ContractError("unknown class; do not grow the vocabulary from predictions")
    return {"type": "class", "label": label}


def labels_out(selected: Any, labels: list[str]) -> dict:
    if not isinstance(selected, list) or not all(isinstance(x, str) for x in selected):
        raise ContractError("expected an explicit label list")
    if len(set(selected)) != len(selected) or not set(selected) <= set(labels):
        raise ContractError("duplicate or unknown selected label")
    return {"type": "labels", "labels": [x for x in labels if x in selected]}


def abstain(reason: str) -> dict:
    return {"outcome": {"type": "abstention", "reason": reason}, "observations": {}}


def scalar_observations(output: dict, names: tuple[str, ...]) -> dict:
    return {name: {"kind": "scalar", "value": number(output[name])}
            for name in names if output.get(name) is not None}


def categorical(result: dict, values: Any, labels: list[str]) -> None:
    values = vector(values, labels)
    # Rounding by a wrapper can invalidate scoring without destroying observations.
    if abs(math.fsum(values.values()) - 1.0) <= 1e-9:
        result["probabilities"] = {"kind": "categorical", "values": values}
    else:
        result["observations"]["unscored_categorical"] = {
            "kind": "categorical", "values": values}


def binary(probability: Any, labels: list[str], threshold: float) -> dict:
    if len(labels) != 2:
        raise ContractError("binary adapters require ordered [negative, positive] labels")
    p = number(probability, probability=True)
    result = {"outcome": class_out(labels[int(p >= threshold)], labels),
              "observations": {"p_yes": {"kind": "bernoulli", "value": p}}}
    categorical(result, {labels[0]: 1.0 - p, labels[1]: p}, labels)
    return result


def decode(example: str, output: dict, task: dict, policy: dict | None = None) -> dict:
    """Decode one native application result, not a model-generated gold label.

    policy is explicit experimental configuration and is preserved by preparation.
    Unknown/malformed values stop admission; an explicit operational error is a
    whole-episode no-decision, never an ordinary negative or answered empty set.
    """
    if not isinstance(output, dict):
        raise ContractError("native output must be an object")
    policy = policy or {}
    labels = task["labels"]
    allowed = {
        "sift": {"threshold"}, "foreman": {"threshold", "thresholds"},
        "upwork": set(), "tax": {"minimum"},
        "classifier-dev": {"threshold"}, "filing": {"minimum", "disposition"},
        "compaction": {"threshold"}, "router": set(), "jev-align": {"threshold"},
    }
    if example not in allowed or set(policy) - allowed[example]:
        raise ContractError("unknown adapter or policy field")
    if "disposition" in policy and not isinstance(policy["disposition"], bool):
        raise ContractError("disposition must be a boolean")
    for key in ("threshold", "minimum"):
        if key in policy:
            number(policy[key], probability=True)
    if output.get("error"):
        return abstain("upstream_error")  # error text remains only in capture evidence
    result = {"observations": {}}
    if example == "sift":
        answer = output.get("answers", {}).get("relevant", {})
        if answer.get("type") != "boolean":
            raise ContractError("Sift requires its native relevant boolean answer")
        result = binary(answer.get("probability"), labels, policy.get("threshold", 0.5))
        if "truncated" in output:
            if not isinstance(output["truncated"], bool):
                raise ContractError("truncated must be boolean")
            result["observations"]["truncated"] = {
                "kind": "scalar", "value": int(output["truncated"])}
    elif example == "foreman":
        if task["kind"] != "multi_label" or tuple(labels) != FOREMAN_LABELS:
            raise ContractError("Foreman requires its ten declared assessment labels")
        probabilities = vector({k: output[k] for k in labels if k in output}, labels)
        thresholds = policy.get("thresholds", {k: policy.get("threshold", 0.5) for k in labels})
        thresholds = vector(thresholds, labels)
        result["outcome"] = labels_out([k for k in labels if probabilities[k] >= thresholds[k]], labels)
        result["probabilities"] = {"kind": "label_marginals", "values": probabilities}
    elif example == "upwork":
        result["outcome"] = class_out(output.get("decision"), labels)
        result["observations"] = scalar_observations(output, ("fit", "lane_confidence"))
        for group in ("gates", "normalized"):
            values = output.get(group, {})
            if not isinstance(values, dict):
                raise ContractError(f"{group} must be a numeric observation map")
            for name, value in values.items():
                result["observations"][f"{group}_{name}"] = {
                    "kind": "scalar", "value": number(value)}
        # Final decisions must be captured from compose(); a fit-only replay would
        # lose gates, review flags, competition penalties, and contradiction logic.
    elif example == "tax":
        result["outcome"] = class_out(output.get("form"), labels)
        p = number(output.get("formConfidence"), probability=True)
        result["observations"] = {"minimum_stage_probability": {"kind": "scalar", "value": p}}
        for name, values in output.get("probabilities", {}).items():
            if not isinstance(values, dict) or not values:
                raise ContractError("tax stage distribution must be a nonempty map")
            result["observations"][f"stage_{name}"] = {
                "kind": "categorical", "values": {k: number(v, probability=True) for k, v in values.items()}}
        if "minimum" in policy and p < policy["minimum"]:
            result["outcome"] = abstain("below_minimum_stage_probability")["outcome"]
    elif example == "classifier-dev":
        scores = vector(output.get("scores"), labels)
        if task["kind"] == "multi_label":
            threshold = policy.get("threshold", 0.7)
            result["outcome"] = labels_out([k for k in labels if scores[k] >= threshold], labels)
            result["probabilities"] = {"kind": "label_marginals", "values": scores}
            # output.label is merely the top tag, NOT the selected set.
        else:
            result["outcome"] = class_out(output.get("label"), labels)
            categorical(result, scores, labels)
            if output.get("confidence") is not None:
                result["confidence"] = number(output["confidence"], probability=True)
    elif example == "filing":
        result["outcome"] = class_out(output.get("category"), labels)
        p = output.get("categoryConfidence")
        if p is not None:
            p = number(p, probability=True)
            result["observations"]["selected_category_probability"] = {"kind": "scalar", "value": p}
        destination = output.get("destinationCategory")
        if policy.get("disposition", False):
            if destination not in labels + ["Need review", "Suspected prompt injection", "Not processable"]:
                raise ContractError("unknown filing disposition")
            if destination == "Suspected prompt injection":
                result["outcome"] = abstain("security_quarantine")["outcome"]
            elif destination == "Not processable":
                result["outcome"] = abstain("not_processable")["outcome"]
            elif "minimum" in policy:
                # Missing probability preserves the upstream baseline behavior.
                if p is not None and p < policy["minimum"]:
                    result["outcome"] = abstain("category_needs_review")["outcome"]
            elif destination == "Need review":
                result["outcome"] = abstain("category_needs_review")["outcome"]
            elif destination not in labels:
                raise ContractError("unknown filing disposition")
    elif example == "compaction":
        if str(output.get("reason", "")).lower().startswith("pinned"):
            raise ContractError("pinned calls are not model-classified episodes")
        for field in ("keepCall", "keepResult"):
            result["observations"][field] = {"kind": "bernoulli", "value": number(output.get(field), probability=True)}
        action = output.get("action")
        if "threshold" in policy:
            t = policy["threshold"]
            action = ("keep" if output["keepResult"] >= t else
                      "drop_result" if output["keepCall"] >= t else "drop_call")
        result["outcome"] = class_out(action, labels)
    elif example == "router":
        result["outcome"] = class_out(output.get("tier"), labels)
        # This is final application policy output. Do not assign it the raw
        # exact-model Choice's confidence or infer cost/task success.
    elif example == "jev-align":
        if task["kind"] == "multi_label":
            probs = vector(output.get("label_probabilities"), labels)
            result["probabilities"] = {"kind": "label_marginals", "values": probs}
            result["outcome"] = labels_out([k for k in labels if probs[k] >= policy.get("threshold", 0.5)], labels)
        elif output.get("probability") is not None:
            result = binary(output["probability"], labels, policy.get("threshold", 0.5))
        else:
            result["outcome"] = class_out(output.get("choice"), labels)
            if output.get("probabilities") is not None:
                categorical(result, output["probabilities"], labels)
            if output.get("confidence") is not None:
                result["confidence"] = number(output["confidence"], probability=True)
    if task["kind"] == "single_label" and result["outcome"]["type"] == "labels":
        raise ContractError("set outcome in single-label task")
    if task["kind"] == "multi_label" and result["outcome"]["type"] == "class":
        raise ContractError("class outcome in multi-label task")
    return result


def harmonize(rows: list[dict]) -> list[str]:
    """Keep the complete population; demote partial scoring families to observations.

    Missing signals are not invented, and no rows are dropped. The returned list
    must be disclosed in preparation provenance. The core remains authoritative.
    """
    demoted = []
    for field in ("probabilities", "confidence"):
        present = sum(field in row for row in rows)
        if 0 < present < len(rows):
            demoted.append(field)
            for row in rows:
                if field not in row:
                    continue
                value = row.pop(field)
                observation = value if field == "probabilities" else {"kind": "reported_confidence", "value": value}
                row.setdefault("observations", {})[f"partial_{field}"] = observation
    return demoted
