#!/usr/bin/env python3
# EXPERIMENT MIN-1 (throwaway research fixture — NOT product code, NOT a schema decision)
#
# Question: in a minimal record model, which fields are load-bearing?
# Method: represent the information required by 12 project scenarios (see
# research/scratch-minimum-model-scenarios.md) as records; then ABLATE one field at a
# time and show which required query becomes unanswerable.
#
# This tests INFORMATION requirements only. It does not select storage, format, or
# product architecture. Results are EXPERIMENT-class evidence for the reduction table.

import json, itertools, sys

# Each scenario supplies (description, [queries that a future reader must answer]).
# A query is a function over records: r has fields
#   subj, pred, obj, src (who/what produced the record), kind (observed|asserted|proposed
#   |accepted|disputed|retracted), t_valid, t_record, scope
R = lambda **kw: kw

SCENARIOS = {
 "S2_windows": {
   "records": [R(subj="project", pred="supports", obj="windows", kind="retracted",
                 src="maintainer", t_valid="2024-03", t_record="2024-03", scope="project",
                 note="decided after closed discussion")],
   "queries": {
     "may_add_windows_ci": lambda rs: all(not (r["pred"]=="supports" and r["obj"]=="windows"
         and r["kind"]=="retracted") for r in rs) is False,
   }},
 "S3_superseded_guideline": {
   "records": [R(subj="perf-guideline-1", pred="superseded_by", obj="perf-guideline-2",
                 kind="accepted", src="lead", t_valid="2025-01", t_record="2025-01", scope="repo")],
   "queries": {
     "which_guideline_current": lambda rs: [r["obj"] for r in rs if r["pred"]=="superseded_by"
         and r["kind"]=="accepted"],
   }},
 "S8_agent_disagreement": {
   "records": [R(subj=("planA","planB"), pred="contradicts", obj=None, kind="disputed",
                 src="FSP-recorder", t_valid=None, t_record="2026-01", scope="repo")],
   "queries": {
     "is_there_unresolved_conflict": lambda rs: any(r["kind"]=="disputed" for r in rs),
   }},
 "S7_contract_deadcode": {
   "records": [R(subj="module_m", pred="must_behave_identically", obj="v2.1",
                 kind="accepted", src="external:customer-contract", t_valid="2023..",
                 t_record="2023-06", scope="module_m")],
   "queries": {
     "may_delete_deadcode": lambda rs: not any(r["pred"]=="must_behave_identically"
         and r["kind"]=="accepted" for r in rs),
   }},
 "S12_unknown_actor": {
   "records": [R(subj="notes.md", pred="modified", obj=None, kind="observed",
                 src="UNKNOWN", t_valid=None, t_record="2026-02-01T03:14", scope="file")],
   "queries": {
     "who_changed": lambda rs: [r["src"] for r in rs if r["pred"]=="modified"][-1],
   }},
 "S9_editor_decision": {
   "records": [R(subj="chapter3-subplot", pred="keep", obj=None, kind="retracted",
                 src="editor", t_valid="2025-11", t_record="2025-11", scope="book",
                 authority="editor>owner")],
   "queries": {
     "may_expand_subplot": lambda rs: not any(r["pred"]=="keep" and r["kind"]=="retracted"
         and r.get("authority","")=="editor>owner" for r in rs),
   }},
 "S11_rename": {
   # the assertion was recorded against the OLD name; a future reader must decide
   # whether it still refers after src/core -> src/kernel
   "records": [R(subj="src/core", pred="lacks_test_coverage", obj=None, kind="observed",
                 src="ci-audit", t_valid=None, t_record="2025-09", scope="repo",
                 identity="ent:kernel-module")],
   "queries": {
     "does_old_assertion_still_refer": lambda rs: [r["subj"] for r in rs
         if r["pred"]=="lacks_test_coverage"],
   }},
 "S3b_time": {
   # two guidelines, both accepted at different times; 'which is current' needs valid-time
   "records": [R(subj="g1", pred="governs", obj="perf", kind="accepted", src="lead",
                 t_valid="2024-01..2025-01", t_record="2024-01", scope="repo"),
               R(subj="g2", pred="governs", obj="perf", kind="accepted", src="lead",
                 t_valid="2025-01..", t_record="2025-01", scope="repo")],
   "queries": {
     "current_as_of_2026": lambda rs: [r["subj"] for r in rs if r["pred"]=="governs"
         and r["kind"]=="accepted" and "2026" in r["t_valid"]],
   }},
 "S13_retroactive_correction": {
   # in 2026 someone records that g1 stopped governing in 2025-01 (backdated correction).
   # 'what did we believe in 2025-05' needs assertion-time, not valid-time.
   "records": [R(subj="g1", pred="governs", obj="perf", kind="accepted", src="lead",
                 t_valid="2024-01..", t_record="2024-01", scope="repo"),
               R(subj="g1", pred="governs", obj="perf", kind="retracted", src="lead",
                 t_valid="2025-01", t_record="2026-02", scope="repo",
                 correction_of="record:1")],
   "queries": {
     "believed_in_2025_05": lambda rs: [ (r["kind"]) for r in rs if r["pred"]=="governs"
         and r["t_record"] <= "2025-05" ][-1],
   }},
 "S2b_scope": {
   # the windows decision binds the project but NOT the user's unrelated fork
   "records": [R(subj="project", pred="supports", obj="windows", kind="retracted",
                 src="maintainer", t_valid="2024-03", t_record="2024-03",
                 scope="project:fsp-lib", note="decided after closed discussion")],
   "queries": {
     "windows_ci_in_fork": lambda rs: not any(r["obj"]=="windows" and r["kind"]=="retracted"
         and r["scope"].startswith("project:fsp-lib") for r in rs),
   }},
}

FIELDS = ["subj","pred","obj","src","kind","t_valid","t_record","scope"]

def ablate(field, sc):
    """Return query names that become UNANSWERABLE (wrong answer or key-missing) when
    `field` is removed from every record."""
    broken = []
    for name, q in sc["queries"].items():
        try:
            # ablation = the field is absent; simulate by making access raise
            class Guard(dict):
                def __getitem__(self, k):
                    if k == field: raise KeyError(field)
                    return dict.__getitem__(self, k)
            rs = [Guard(r) for r in sc["records"]]
            # normalize: queries access via r[...] so KeyError propagates
            q(rs)
        except KeyError:
            broken.append(name)
        except Exception:
            broken.append(name)  # wrong-shape answer also counts as broken
    return broken

report = {}
for scname, sc in SCENARIOS.items():
    for f in FIELDS:
        br = ablate(f, sc)
        if br:
            report.setdefault(scname, {})[f] = br

print(json.dumps(report, indent=1))
load_bearing = sorted({f for sc in report.values() for f in sc})
never_broken = [f for f in FIELDS if f not in load_bearing]
print("\nLOAD-BEARING (ablation breaks a required query):", load_bearing)
print("NOT load-bearing in these fixtures (no query needs it):", never_bearing if False else never_broken)
