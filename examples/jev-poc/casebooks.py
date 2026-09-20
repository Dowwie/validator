"""Readable synthetic reference proposals, never human-approved golden truth.

Counts reproduce the plan's construction targets. Family members are deliberately
related and remain in the same partition. These templates test an integration;
they are not diverse enough to estimate production accuracy.
"""
from __future__ import annotations

import json
from pathlib import Path
from adapters import FOREMAN_LABELS

SOURCES = {
    "sift": ("kbhuw/jev-sift", "966de12e2bb5f94d47886ee51f30a07ec8ef1607"),
    "foreman": ("thruwire/foreman", "a7d21d18d306a0cb9f3e15acefbdb5663521405c"),
    "upwork": ("ekkyarmandi/jev-upwork-job-classification", "bb072c82b390e74f66587e698f50006cb761c614"),
    "tax": ("kyotofin/tax-doc-classifier", "3e95a77f763c6becb78472f8b2ce2f54237f9214"),
    "classifier-dev": ("mrmps/classifier-dev", "33ca63816f2bc7e93c3f2d0715f7896500370739"),
    "filing": ("Charlyhno-eng/jev-document-classification", "17cdc20acb407e7cfb623ef47d32e1b8568a5c86"),
    "compaction": ("tamaratran/fast-jev-compaction", "e3f262a7f4d42bd8dd32ced30d26176f7cb545b0"),
    "router": ("gargpratyush/jev-router", "38da6b84ea01241bfc41fbddc0928d0f40a703f0"),
    "jev-align": ("sutro-sh/jev-align", "49753df924d30c0d3642b58e0b9b1e89921dc102"),
}


def case(slug, family, index, model_input, label, rationale, held_out=False):
    expected = ({"type": "labels", "labels": list(label)} if isinstance(label, (list, tuple))
                else {"type": "class", "label": label})
    return {"case_id": f"{slug}-{family:02d}-{index:02d}",
            "family_id": f"{slug}-{family:02d}",
            "partition": "held_out" if held_out else "development",
            "model_input": model_input, "proposed_expected": expected,
            "rationale": rationale,
            "review": {"state": "pending", "reviewer": None, "method": None}}


def book(slug, labels, rubric, cases, multi=False, **configuration):
    return {"schema_version": 1, "example": slug,
            "reference_authority": "proposed_synthetic", "creator": "assistant",
            "source": dict(zip(("repository", "commit"), SOURCES[slug])),
            "task": {"kind": "multi_label" if multi else "single_label", "labels": list(labels)},
            "rubric": rubric, "configuration": configuration, "cases": cases}


def make_books():
    books = {}
    products = ["bed scheduling", "pharmacy inventory", "surgical scheduling", "radiology workflow",
                "staff rostering", "patient check-in", "infection tracking", "hospital billing",
                "laboratory workflow", "clinical messaging", "discharge planning", "meal ordering",
                "equipment tracking", "ward capacity", "appointment booking", "emergency triage",
                "clinical document storage", "hospital procurement", "blood-bank inventory", "bedside alerts"]
    query = "Does this text provide evidence that the company currently sells software to hospitals?"
    cases = []
    for i, product in enumerate(products):
        texts = [
            (f"We license our {product} software to hospitals. Three hospital customers renewed this year.", "relevant", "An existing hospital-software offering and sales relationship are explicit."),
            (f"Our hospital licenses {product} software from an outside vendor. We do not sell any software.", "irrelevant", "The described organization is a buyer and explicitly not a seller."),
            (f"Our {product} platform is available on an annual subscription to hospital operators, not just individual consumers.", "relevant", "An actual subscription offering to hospital operators is described."),
            (f"We planned a hospital {product} product, but canceled it before launch. Today we sell restaurant accounting software only.", "irrelevant", "The hospital offering never launched; current sales are unrelated."),
        ]
        for j, (text, label, rationale) in enumerate(texts):
            cases.append(case("sift", i, j, {"query": query, "text": text}, label, rationale, i >= 15))
    books["sift"] = book("sift", ["irrelevant", "relevant"],
        "Positive requires explicit evidence of a current software offering sold/licensed to hospitals. Buyers, discontinued plans and keyword mentions alone are negative; unresolved cases are withheld.", cases)

    states = [
        ("A required external credential is absent; operator approval is needed. No work has begun.", ["needs_human"], "blocked", ""),
        ("The latest worker implemented one required endpoint. Two explicitly required endpoints remain. Tests have not been written.", ["meaningful_progress"], "running", "+ one required endpoint implemented"),
        ("All required endpoints are implemented. Only a smoke test exists; edge-case tests and independent verification remain.", ["implementation_complete", "meaningful_progress", "needs_verification"], "running", "+ all requested endpoints implemented"),
        ("An independent reviewer verified every requirement and required edge case. All relevant tests pass. No open work or approvals remain.", ["implementation_complete", "tests_sufficient", "requirements_satisfied", "ready_to_finish", "meaningful_progress"], "running", "+ requested implementation and passing edge-case tests"),
        ("The active worker repeated exactly the same failing command six times with the same error and no changes. There is no new evidence of progress.", ["worker_stuck"], "running", ""),
        ("The job requests retry handling. Instead the worker only changed marketing-page colors and describes no work on retries.", ["work_off_track"], "running", "+ marketing colors changed"),
        ("The repository instructions prohibit editing generated/. The only new changes are edits under generated/. The requested API change remains undone.", ["agents_md_drift", "work_off_track"], "running", "+ generated/client.py edited"),
        ("All requested code appears present, but a required independent verification pass has not run and relevant tests are failing.", ["implementation_complete", "needs_verification"], "running", "+ requested code present; required tests fail"),
    ]
    cases = []
    for job in range(20):
        for j in range(3):
            text, labels, status, diff = states[(job * 3 + j) % len(states)]
            observation = {
                "original_job": f"Implement three retry-enabled API endpoints for fictional service {job}; cover timeout, cancellation and retry exhaustion, and obtain independent verification.",
                "run_id": f"fictional-job-{job}", "factory_status": status, "iteration": j,
                "active_workers": [], "worker_history": [], "latest_worker_output": text,
                "worker_exit_status": {}, "worker_elapsed_seconds": {},
                "git_status": "modified" if diff else "clean", "git_diff": diff,
                "changed_files": ["generated/client.py"] if "generated/" in diff else (["service.py"] if diff else []),
                "test_results": [{"summary": text}], "verification_results": ([{"summary": text}] if "independent reviewer verified" in text else []), "recent_events": [],
                "previous_assessment": None, "previous_intervention": None,
                "attempts": 1, "failures": [], "elapsed_factory_seconds": 60,
                "agents_md_path": "AGENTS.md", "agents_md_instructions": "Do not edit generated/. Independent verification is required.",
            }
            cases.append(case("foreman", job, j, observation, labels,
                              "Proposed dimensions follow the explicit snapshot evidence, not the worker's unsupported completion claim. All ten negatives also require human review.", job >= 15))
    books["foreman"] = book("foreman", FOREMAN_LABELS,
        "Review all ten dimensions separately under the pinned questions. Code completion is distinct from requirements satisfied and sufficient verification. Mere worker self-report is not evidence. Withhold a whole case if any label is unresolved.", cases, multi=True)

    profile = {"core_stack": ["Python", "FastAPI", "PostgreSQL"], "secondary_stack": ["TypeScript"],
               "weak_areas": ["mobile apps", "native desktop"],
               "verified_evidence": ["Delivered paid production API integrations"],
               "positioning": "Solo senior API contractor", "availability": "Remote only; 20 hours/week",
               "rate_floor_usd_per_hour": 100, "target_rate_usd_per_hour": 130, "package_floor_usd": 2000}
    cases = []
    for i in range(30):
        descriptions = [
            (f"Paid remote solo engagement: build a FastAPI/PostgreSQL integration for inventory service {i}. Existing API docs and acceptance tests are provided. Budget $5,000; no unpaid sample or onsite work.", "apply", "Explicit paid, scoped remote API work matches the frozen profile."),
            (f"Build a complete custom FastAPI prototype for service {i} without payment before we decide whom to hire. The unpaid sample is mandatory.", "skip", "The stated mandatory unpaid custom sample violates an explicit exclusion."),
            (f"We may need an API integration for service {i}, but the scope, budget, solo-versus-team arrangement and onsite requirement have not been decided. Please discuss these first.", "review", "Material unresolved engagement conditions require human review."),
        ]
        for j, (text, label, why) in enumerate(descriptions):
            cases.append(case("upwork", i, j, {"job": {"title": f"Service {i} integration", "description": text},
                "client": {"payment_verified": True}, "freelancer": profile}, label, why, i >= 20))
    books["upwork"] = book("upwork", ["apply", "review", "skip"],
        "Use the fixed fictional freelancer preferences. Mandatory unpaid custom samples are skip; clearly scoped paid remote stack-matching work may be apply; unresolved material requirements are review. Final labels are preferences, not expected earnings.", cases)

    forms = [("form-1040", "Form 1040", "U.S. Individual Income Tax Return"),
             ("form-1040-schedule-a", "Schedule A (Form 1040)", "Itemized Deductions"),
             ("form-w-2", "Form W-2", "Wage and Tax Statement"),
             ("form-1099-int", "Form 1099-INT", "Interest Income"),
             ("form-5471", "Form 5471", "Information Return of U.S. Persons With Respect to Certain Foreign Corporations"),
             ("form-5471-schedule-j", "Schedule J (Form 5471)", "Accumulated Earnings and Profits")]
    cases = []
    for f, (label, heading, title) in enumerate(forms):
        for variant in range(4):
            family = f * 4 + variant
            for j in range(2):
                lines = [heading, title, "FICTIONAL TEXT FIXTURE — NOT A FILED RETURN", "Tax year 2025",
                         f"Example section {variant + 1}; fields intentionally left blank.",
                         "An informational note mentions other tax forms; this document's printed identity is above."]
                if j:
                    lines = ["Continuation of the document identified in the footer.",
                             "Fictional blank table; no taxpayer data.", title, heading]
                cases.append(case("tax", family, j, {"lines": lines}, label,
                                  "The printed form identity establishes the proposed document class; no tax computation is being validated.", family % 3 == 2))
    for family in range(24, 30):
        for j in range(2):
            lines = [f"Fictional municipal transport permit TP-{family}", "Local transit application, not a federal tax form.", f"Blank example page {j+1}"]
            cases.append(case("tax", family, j, {"lines": lines}, "not_in_this_list",
                              "The explicitly non-tax document has no identity in the restricted fixture vocabulary.", family % 3 == 2))
    books["tax"] = book("tax", [x[0] for x in forms] + ["not_in_this_list", "blank"],
        "Classify the printed form identity, not a form mentioned in passing. Unidentifiable continuation pages are withheld. This draft uses a restricted vocabulary; freeze the full upstream registry before unrestricted application capture.", cases,
        vocabulary_scope="restricted_fixture_only; full-registry capture requires a reviewed vocabulary update")

    labels = ["billing", "technical", "account_access", "feature_request"]
    cases, multicases = [], []
    for i in range(30):
        texts = [f"My subscription for workspace {i} was charged twice. Please refund the duplicate charge; everything else works.",
                 f"The documented CSV export in workspace {i} crashes with an exception. I can sign in and billing is fine; please fix the existing export.",
                 f"I cannot sign in to workspace {i}. The password-reset link is expired. Please restore account access.",
                 f"Workspace {i} works as documented. Please add an offline mode that is not currently offered."]
        for j, (text, label) in enumerate(zip(texts, labels)):
            cases.append(case("classifier-dev", i, j, {"text": text}, label,
                              "The explicit primary requested action defines the class; incidental keywords do not override it.", i >= 20))
    books["classifier-dev"] = book("classifier-dev", labels,
        "Classify the primary requested action. Duplicate charges/refunds are billing; broken documented features are technical; login recovery is account access; new capabilities are feature requests. Withhold equally primary mixed requests.", cases)
    multilabels = ["refund_requested", "login_problem", "bug_report", "feature_request"]
    for i in range(24):
        variants = [(f"Thanks, workspace {i} works well. No request today.", []),
                    (f"Please refund the duplicate workspace {i} charge. Login and features work.", ["refund_requested"]),
                    (f"Workspace {i}'s reset page crashes, and I cannot sign in. Please fix the crash and restore access.", ["login_problem", "bug_report"]),
                    (f"Please add an offline mode for workspace {i}, and refund the duplicate payment. There are no broken features or login problems.", ["refund_requested", "feature_request"])]
        for j, (text, selected) in enumerate(variants):
            multicases.append(case("classifier-dev-multi", i, j, {"text": text}, selected,
                                  "Every positive is stated explicitly and every unselected label is proposed absent, pending full review.", i >= 16))
    books["classifier-dev-multi"] = book("classifier-dev", multilabels,
        "Label all explicit requests/problems, including an empty set. Absence is a reviewed negative, not missing annotation. Native top label is not the full set.", multicases, multi=True)

    cases = []
    categories = ["Invoices", "Contracts", "Reports", "Correspondence"]
    for i in range(20):
        texts = [f"Invoice for fictional project {i}. Services: API integration. Amount due: 2000 sample units. Payment requested within 30 days. No real transaction.",
                 f"Agreement for fictional project {i}. Supplier shall deliver the integration; purchaser shall pay the fee. The parties accept these obligations upon signature.",
                 f"Status report for fictional project {i}. Completed: specification. In progress: implementation. Risks: test coverage. This is a project update, not a demand for payment.",
                 f"Hello team, please see the invoice attached separately for project {i}. This message only forwards that document; the attachment is not part of this text. Best regards."]
        for j, (text, label) in enumerate(zip(texts, categories)):
            cases.append(case("filing", i, j, {"fileName": f"document-{i}-{j}.txt", "text": text, "categories": categories}, label,
                              "Main document purpose governs; an email discussing a separate invoice remains correspondence.", i >= 15))
    books["filing"] = book("filing", categories,
        "Use main purpose: request for payment=Invoices; accepted obligations=Contracts; status findings=Reports; communications about separately attached documents=Correspondence. Review/quarantine are dispositions, not semantic categories.", cases)

    cases = []
    for i in range(20):
        goal = f"Diagnose the unresolved deployment failure for fictional service {i}. Retain the only unrecoverable error trace and explicit safety constraints; do not repeat irreversible calls. Earlier unrelated tasks are complete."
        items = [
            ("Read", {"file_path": f"deleted-trace-{i}.log"}, "The only copy of the incident trace: E_DEPLOY_SIGNATURE_MISMATCH. Source log has since been deleted.", "keep"),
            ("Read", {"file_path": "constraints.txt"}, "Deployment safety constraints: " + "context " * 60 + "Never rotate the signing key; an audit hold prohibits it. This file is no longer accessible.", "keep"),
            ("Bash", {"command": "git status --short"}, "Current status was checked successfully; no file changes. Remaining lines repeat formatting. " * 8, "drop_result"),
            ("Read", {"file_path": "deployment-checklist.txt"}, "Checklist was consulted. Its contents are still available unchanged, and the relevant current goal is already preserved. " * 8, "drop_result"),
            ("Read", {"file_path": "completed-marketing-task.txt"}, "Earlier marketing typo was fixed and independently verified. It has no dependency on this deployment.", "drop_call"),
            ("Bash", {"command": "ls old-scratch-directory"}, "Irrelevant scratch files from a completed task: a.tmp b.tmp", "drop_call"),
        ]
        messages = [{"role": "user", "text": goal, "toolUses": []}]
        for j, (tool, args, text, action) in enumerate(items):
            uid = f"call-{i}-{j}"
            messages.extend([{"role": "assistant", "text": "", "toolUses": [{"tool_use_id": uid, "tool": tool, "input": args}]},
                             {"role": "user", "text": "", "toolUses": [], "toolResults": [{"tool_use_id": uid, "text": text}]}])
        messages.extend({"role": "assistant" if j % 2 else "user", "text": "Current deployment investigation remains open.", "toolUses": []} for j in range(6))
        for j, (_, _, _, action) in enumerate(items):
            cases.append(case("compaction", i, j, {"messages": messages, "goal": goal, "tool_use_id": f"call-{i}-{j}"}, action,
                              "Proposed least-destructive action under the explicit current goal; human review must settle whether exact content or call history remains necessary.", i >= 15))
    books["compaction"] = book("compaction", ["keep", "drop_result", "drop_call"],
        "Keep unique unrecoverable needed content; drop_result only when call history and retained head suffice; drop_call only for no remaining dependency. Pinned calls are excluded. These are provisional retention preferences, not proven downstream success.", cases)

    cases = []
    for i in range(20):
        prompts = [(f"Print the value of 2 + 2 for example {i}; no repository edits.", "haiku"),
                   (f"Add a unit-tested timeout argument to the existing small HTTP helper in module {i}; its contract is fully specified.", "sonnet"),
                   (f"Diagnose a cross-service intermittent race in system {i}; requirements conflict, several ownership boundaries are unclear, and the root cause is unknown.", "opus")]
        for j, (prompt, tier) in enumerate(prompts):
            cases.append(case("router", i, j, {"prompt": prompt, "current": "sonnet", "available": ["haiku", "sonnet", "opus"], "contextTokens": 1000}, tier,
                              "A proposed rubric preference for task complexity, not an observed cheapest-successful-model result.", i >= 13))
    books["router"] = book("router", ["haiku", "sonnet", "opus"],
        "Mechanical tasks prefer fast, specified bounded engineering prefers balanced, unresolved cross-system reasoning prefers strong. Human preference labels do not establish costs or success. Family-preserving split is 39/21, not the plan's incompatible 40/20 target.", cases)

    cases = []
    for i in range(20):
        stories = [(f"Airport {i} reopens a runway after maintenance; airlines resume scheduled flights.", "aviation"),
                   (f"Workshop {i} services commercial aircraft engines for passenger airlines.", "aviation"),
                   (f"Company {i}'s profits take flight, a metaphor for improved retail sales. It operates no aircraft.", "not_aviation"),
                   (f"City {i} expands its bus network. No airport or aircraft operations are discussed.", "not_aviation")]
        for j, (text, label) in enumerate(stories):
            cases.append(case("jev-align", i, j, {"title": f"Local update {i}", "text": text}, label,
                              "Aircraft/airport operations are directly relevant; metaphors and bus-only transport are not.", i >= 15))
    books["jev-align"] = book("jev-align", ["not_aviation", "aviation"],
        "Positive means factual aircraft or airport operations. Metaphorical flight and other transport alone are negative. The external test families stay outside optimization and label acquisition.", cases)
    return books


def write_books(directory: Path) -> dict:
    directory.mkdir(parents=True, exist_ok=False)
    summary = {}
    for slug, value in make_books().items():
        path = directory / f"{slug}.json"
        path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        summary[slug] = {partition: sum(c["partition"] == partition for c in value["cases"])
                         for partition in ("development", "held_out")}
        summary[slug]["review_state"] = "pending"
    return summary
