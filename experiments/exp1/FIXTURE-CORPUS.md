# FIXTURE CORPUS — synthetic test data (not project memory)

`fixture-corpus-synthetic/` is the EXP-1 fixture corpus. It is **not** an FSP record and
states nothing about FSP.

What it is: 36 deliberately messy files for a **fictional** project called "Kestrel",
written for EXP-1 (see EXP-1.md). Its files are named like real project documents —
`decisions-2024.md`, `ADR-007-realtime-updates.md`, `open-questions.md`, `README.md` —
because realistic messiness is what the experiment required.

Rules for a reader:
- Do not cite anything in it as a project decision or requirement.
- Do not treat its `README.md` as this corpus's readme; this file is.
- The experiment's results are in `results.json`; its conclusions are in `EXP-1.md`.

Rationale for the explicit marker: three independent EXP-DOC-1 readers reported that a
tree listing could mistake this corpus for project memory (DOC-FRICTION-010/011). The
directory was renamed from `corpus-messy` and this marker added, rather than editing the
fixtures themselves, which would have changed the experiment's inputs and invalidated its
results.
