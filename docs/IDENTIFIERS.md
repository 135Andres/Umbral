# IDENTIFIERS — namespace registry

Status: **DERIVED INDEX (DT8) — navigational only. Not authority.**
It points at the documents that own each namespace; it never restates what an identifier means.
Last audited: 2026-09-13, against the repository at `37220d2`.

## 1. What this document is, and what it is not

**What it is.** A registry of the *namespaces* used in this repository — the prefixes that
identifiers carry — so that a reader, human or agent, can answer three questions quickly:

- which document defines this prefix;
- which numeric range belongs to it;
- whether the prefix is used by more than one document.

**What it is not.**

- It is **not authority**. The authority ladder lives in [`../README.md`](../README.md); the
  documents named in the OWNER column are the authority for their own identifiers.
- It is **not a dictionary of identifiers**. There is no row per `UD-017`, per `P34` or per `Q25`.
  Those definitions live in their owner documents. A registry of ~45 namespaces stays stable for
  months; a dictionary of ~200 identifiers would be stale within a session.
- It is **not a second source of truth**. Nothing here may be quoted as a definition. If this file
  and an owner document disagree, the owner document wins and this file is wrong.
- It is **not a decision**. Registering a collision is not resolving it. No namespace has been
  renamed, and none may be renamed on the strength of anything written here.

**Why ranges are the point.** Without the RANGE column, this registry could not answer anything a
text search could not already answer. With it, a reader can tell that `P34` belongs to the design
(P1–P42) and not to PRINCIPLES (P1–P9) without opening either document. That single column is what
justifies the file's existence.

## 2. How to read a row

| Column | Meaning |
|---|---|
| **PREFIX** | the token as it literally appears, before its separator (`M`, `UD`, `F-TD`) |
| **OWNER** | the document that defines the namespace. The canonical location. |
| **RANGE** | the values actually in use at the audit date, compressed (`M1-M5` means M1 through M5) |
| **KIND** | what class of information the identifiers denote |
| **SCOPE** | `project` · `version-local` (one development version) · `experiment-local` (one experiment only) · `historical` |
| **COLLISION** | `—` if the prefix has exactly one owner; otherwise the other owners, with their ranges |

A namespace marked **experiment-local** is not a project identifier: it is scoped and declared
inside one experiment's own files, and must not be cited from outside it.

## 3. The registry

| PREFIX | OWNER | RANGE | KIND | SCOPE | COLLISION |
|---|---|---|---|---|---|
| `MC §n` | `research/sources/PROJECT-MASTER-CONTEXT.md` (**private, untracked**) | §1–§52+ | charter / user intent | project | — |
| `UD` | `docs/decisions/DECISIONS.md` | UD-001–UD-022 | decision record | project | — |
| `A` | `docs/canonical/INVARIANTS.md` | A1–A11 | invariant | project | **yes** — see §4.8 |
| `P` | `docs/canonical/PRINCIPLES.md` | P1–P9 | principle | project | **yes** — see §4.2 |
| `C` | `docs/canonical/CONSTRAINTS.md` | C1–C12 | constraint | project | — |
| `R` | `docs/canonical/REQUIREMENTS.md` | R1–R13 | requirement | project | **yes** — see §4.4 |
| `J` | `docs/canonical/REQUIREMENTS.md` | J1–J8 | possibility (roadmap) | project | **yes** — see §4.6 |
| `H` | `docs/candidates/ARCHITECTURE-HYPOTHESES.md` | H1–H25 | hypothesis | project | **yes** — see §4.12 |
| `Q` | `docs/candidates/OPEN-QUESTIONS.md` | Q1–Q25 | open question | project | **yes** — see §4.12 |
| `T` | `docs/candidates/OPEN-QUESTIONS.md` + `docs/candidates/ARCHITECTURE-HYPOTHESES.md` | T1–T7 | tension | project | — (two documents, one namespace, by design) |
| `S` | `research/RESEARCH-INDEX.md` | S0–S12 | external source | project | **yes** — see §4.7 |
| `K` | `docs/candidates/RESEARCH-AGENDA.md` §7.6 | K1–K9 | comparison criterion | project | — (cited bare elsewhere) |
| `G` | `docs/candidates/RESEARCH-AGENDA.md` | G1–G7 | research gap | project | **yes** — see §4.5 |
| `E-CO` | `docs/candidates/COEXISTENCE-STRATEGIES.md` | E-CO-1–E-CO-9, E-CO-6a | experiment (proposed) | project | — |
| `OBJ` | `docs/candidates/ENVIRONMENT-INTELLIGENCE.md` | OBJ-1–OBJ-13 | objection | project | — |
| `RM` | `research/PROJECT-REALITY-MINIMUM-MODEL.md` | RM-1–RM-18 | research evidence | project | — |
| `FAL` | `research/PROJECT-REALITY-MINIMUM-MODEL.md` | FAL-1–FAL-3 | falsification | project | — |
| `PASS` | `research/PROJECT-REALITY-MINIMUM-MODEL.md` | PASS-1–PASS-4 | research pass | project | — |
| `E-MIN` | `research/PROJECT-REALITY-MINIMUM-MODEL.md` | E-MIN-1 | experiment (proposed) | project | — |
| `CO` | `research/COEXISTENCE-RESEARCH.md` | CO-1–CO-18 | research source | project | — |
| `EI` | `research/ENVIRONMENT-INTELLIGENCE-RESEARCH.md` | EI-1–EI-27 | research source | project | — |
| `DP` | `docs/DOCUMENTATION-ARCHITECTURE.md` | DP-1–DP-10 | documentation principle | project | — |
| `DT` | `docs/DOCUMENTATION-ARCHITECTURE.md` | DT1–DT11 | document type | project | — |
| `DR` | `research/DOC-ARCHITECTURE-RESEARCH.md` | DR-1–DR-11 | research source | project | — |
| `DQ` | `docs/DOCUMENTATION-ARCHITECTURE.md` | DQ-1–DQ-5 | documentation open question | project | **yes** — see §4.12 |
| `MR` | `docs/DOCUMENTATION-ARCHITECTURE.md` | MR-1–MR-5 | maintenance risk | project | — |
| `DM` | `docs/DOCUMENTATION-ARCHITECTURE.md` | DM0–DM5 | migration step | historical | — |
| `EXP` | `experiments/<exp-id>/EXP-<id>.md` | EXP-1–EXP-4, EXP-AI-01, EXP-CTIME, EXP-DOC-1 | experiment | project | **yes** — two naming forms, see §4.12 |
| `F` | `experiments/v0-harness/FALSIFICATION-REPORT.md` | F-1–F-6 | falsification finding | V0 | — (family of three, see §4.12) |
| `SC` | `docs/v0/V0-IMPLEMENTATION-PLAN.md` §9 | SC-1–SC-5 | completion criterion | V0 | — |
| `INV` | `docs/v0/V0-IMPLEMENTATION-PLAN.md` §12 | INV-1–INV-8 | invariant | V0 | — |
| `F-V01` | `experiments/v0.1-reader-protocol/README.md` | F-V01-1–F-V01-9 | reader-protocol finding | version-local (v0.1) | — |
| `P-V01` | `experiments/v0.1-reader-protocol/EVIDENCE-B.md` | P-V01-1–P-V01-2 | protocol defect | version-local (v0.1) | — |
| `D-V01` | `docs/versions/v0.1.md` | D-V01-1–D-V01-4 | implementation defect | version-local (v0.1) | **yes** — written bare as `D4`, see §4.10 |
| `N-V01` | `docs/versions/v0.1.md` | N-V01-1 | presentation note | version-local (v0.1) | — |
| `A1-AI` | `experiments/exp-ai-01/` | A1-AI | evidence class | project | — |
| `DOC-FRICTION` | `experiments/exp-doc1/DOC-FRICTION-LOG.md` | 001–018 | friction finding | experiment-local (EXP-DOC-1) | — |
| `M` | four owners | M0–M10 | see §4.1 | mixed | **yes** — see §4.1 |
| `XQ` | `experiments/exp1/EXP-1.md` | XQ-1–XQ-5 | open question | experiment-local (EXP-1) | — |
| `O` | `experiments/exp1/EXP-1.md` | O1–O7 | observation | experiment-local (EXP-1) | — |
| `L` | `experiments/exp1/EXP-1.md` | L1–L5 | limitation | experiment-local (EXP-1) | **yes** — see §4.9 |
| `MIN` | `experiments/min1/MIN-1.md` | MIN-1 | experiment | project | — |
| `H-DOC` | `experiments/exp-doc1/EXP-DOC-1.md` | H-DOC-1–H-DOC-2 | hypothesis | experiment-local (EXP-DOC-1) | — |
| `CTF` | `experiments/exp-ai-01/COMPARISON-AI-01-AI-02.md` | CTF-1–CTF-3 | candidate-test finding | experiment-local (EXP-AI-01) | — |
| `F-TD` | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | F-TD-1–F-TD-11 | design finding | version-local (v0.2) | — |
| `E-TD` | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | E-TD-1–E-TD-11 | design experiment | version-local (v0.2) | — |
| `P-TD` | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | P-TD-A–P-TD-J | design property (pre-grammar form) | version-local (v0.2) | — |
| `P` (design) | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | P1–P42 | falsifiable property | version-local (v0.2) | **yes** — see §4.2 |
| `PC` | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | PC-1–PC-2 | contract property | version-local (v0.2) | — |
| `G` (design) | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | G-1–G-5 | grammar family | version-local (v0.2) | **yes** — see §4.5 |
| `V` (design) | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | V1–V4 | contract-versioning family | version-local (v0.2) | **yes** — see §4.3 |
| `R-A…R-D` | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | R-A, R-B, R-C, R-D | observation-reference alternative | version-local (v0.2) | **yes** — see §4.4 |
| `D` (design) | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | D1–D4 | defect class | version-local (v0.2) | **yes** — see §4.10 |
| `L` (design) | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | L1–L5 | known limitation | version-local (v0.2) | **yes** — see §4.9 |
| `P-D2` | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | P-D2 | property (ctime) | version-local (v0.2) | — |
| `H11` | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | H11-a–H11-g | hypothesis (E-TD-11) | version-local (v0.2) | — |
| `M` (design) | `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | M1–M5 | evidence-representation model | version-local (v0.2) | **yes** — see §4.1 |
| `R` (scope) | `docs/candidates/V0.2-SCOPE-PROPOSAL.md` §7 | R1–R5 | risk | version-local (v0.2) | **yes** — see §4.4 |
| `D-PEND` | `docs/decisions/DECISIONS.md` + `docs/candidates/V0.2-TECHNICAL-DESIGN.md` | D-PEND-1, D-PEND-2 | pending decision | project | — (two documents, one namespace, by design) |
| `J.1` | `docs/decisions/DECISIONS.md` + `docs/candidates/V0.2-TECHNICAL-DESIGN.md` §E.5 | J.1 | owner decision (traceability) | project | **yes** — see §4.6 |
| `§n` | every document | — | section number | per document | **yes** — see §5 |

## 4. Collisions

**Registering a collision does not resolve it.** No namespace here has been renamed, and none is
proposed to be renamed by this file. The purpose of this section is that a reader who sees a bare
identifier knows it is ambiguous *before* drawing a conclusion from it.

| # | Prefix | Owners | Overlap |
|---|---|---|---|
| 4.1 | `M` | 4 | total — M1–M5 is inside three other ranges |
| 4.2 | `P` | 3 | partial — P1–P9 and P1–P2 inside P1–P42 |
| 4.3 | `V` | 2 | total — V1–V4 vs the product versions V0/V1 |
| 4.4 | `R` | 3 | R1–R5 in two owners; R-A…R-D in a third |
| 4.5 | `G` | 2 | G1–G7 vs G-1…G-5 — distinguishable only by the hyphen |
| 4.6 | `J` | 2 | J1–J8 vs J.1 — distinguishable only by the point |
| 4.7 | `S` | 2 | S1–S12 vs S1–S13 |
| 4.8 | `A` | 2 | A1 is both |
| 4.9 | `L` | 2 | L1–L5 in two owners, different subjects |
| 4.10 | `D` | 2 | D1–D4 vs `D-V01-4` written bare |
| 4.11 | `K` | 1 | **not a collision** — recorded for completeness |
| 4.12 | `DQ`, `EXP`, `H`, `Q` | 2+ | four further collisions found by the audit |

**4.1 — `M`.** Four owners. `docs/candidates/COEXISTENCE-STRATEGIES.md` (M1–M8) and
`docs/candidates/ENVIRONMENT-INTELLIGENCE.md` (M9–M10) share one proposed-mechanism namespace;
`experiments/exp1/EXP-1.md` (M1–M10) uses it for mutation classes; `experiments/exp-doc1/DOC-FRICTION-LOG.md`
(M0–M10) for reader findings; and `docs/candidates/V0.2-TECHNICAL-DESIGN.md` (M1–M5) for the
evidence-representation models. The last is the one in active use, and `docs/decisions/DECISIONS.md`
cites `M1`/`M2` in that sense. `docs/DOCUMENTATION-ARCHITECTURE.md` §12 records the first four and
declares them scoped; the fifth postdates that note.
**Until renamed or qualified, write the context: "M1 (design)", "M1 (coexistence)".**

**4.2 — `P`.** Three owners: `docs/canonical/PRINCIPLES.md` (P1–P9),
`research/PROJECT-REALITY-MINIMUM-MODEL.md` (P1–P2, model primitives), and
`docs/candidates/V0.2-TECHNICAL-DESIGN.md` (P1–P42, falsifiable properties). The ranges nest, so a
number alone does not resolve. **Qualify: "P34 (design property)", "P6 (principle)".**

**4.3 — `V`.** Two owners, and the ambiguity reaches into a decision record.
`docs/candidates/V0.2-TECHNICAL-DESIGN.md` uses V1–V4 for contract-versioning families (V1 = version
on every line, V2 = version in a header, V3 = external contract, V4 = self-describing values);
thirteen other files use `V0` for the frozen prototype and `V1` for the version reserved for
external audit. `UD-022` records `V2` in the design sense. **Qualify: "V2 (contract versioning)".**

**4.4 — `R`.** Three owners: `docs/canonical/REQUIREMENTS.md` (R1–R13),
`docs/candidates/V0.2-SCOPE-PROPOSAL.md` §7 (R1–R5, risks), and
`docs/candidates/V0.2-TECHNICAL-DESIGN.md` (R-A…R-D, observation-reference alternatives). R1–R5
exists in two owners with different meanings; the design's own cross-reference to `R2` lands inside
that overlap. The lettered forms R-A…R-D are unambiguous. **Qualify the numeric ones.**

**4.5 — `G`.** `docs/candidates/RESEARCH-AGENDA.md` defines G1–G7 (research gaps);
`docs/candidates/V0.2-TECHNICAL-DESIGN.md` defines G-1–G-5 (grammar families). Both are written
bare in `docs/decisions/DECISIONS.md`, in the same file. The hyphen is the only difference.
**Write the hyphen deliberately: "G3 (gap)", "G-3 (grammar family)".**

**4.6 — `J`.** `docs/canonical/REQUIREMENTS.md` defines J1–J8 (possibilities);
`docs/decisions/DECISIONS.md` and the design's §E.5 use `J.1` for the owner's traceability decision.
In plain text `J1` and `J.1` are nearly indistinguishable. **Spell it out: "the J.1 decision".**

**4.7 — `S`.** `research/RESEARCH-INDEX.md` defines S0–S12 (external sources);
`research/scratch-minimum-model-scenarios.md` uses S1–S13 for scenarios. The second file is working
scratch, but `research/PROJECT-REALITY-MINIMUM-MODEL.md` cites both kinds, so `S11` carries two
meanings inside one research artifact. **Say which: "S11 (source)", "S11 (scenario)".**

**4.8 — `A`.** `docs/canonical/INVARIANTS.md` defines A1–A11;
`docs/versions/v0.1.md` uses `A1` for v0.1's acceptance criterion, whose current state is
**NOT SATISFIED**. A reader who meets `A1` cannot tell which is meant, and the two have opposite
states. **Write "A1 (invariant)" or "A1 (v0.1 criterion)".**

**4.9 — `L`.** `experiments/exp1/EXP-1.md` defines L1–L5 as that experiment's limitations, and
`docs/canonical/PROJECT-DIRECTION.md` cites them as `L1/L3`.
`docs/candidates/V0.2-TECHNICAL-DESIGN.md` defines L1–L5 as the design's known limitations.
Same numbers, different subjects. **Qualify by document.**

**4.10 — `D`.** `docs/candidates/V0.2-TECHNICAL-DESIGN.md` defines D1–D4 as the E-TD-11 defect
classes. `docs/versions/v0.1.md` defines the namespace `D-V01-n` but writes one of its own defects
bare as "the first run's D4 weakness". **Always write `D-V01-4` in full.**

**4.11 — `K`.** Single owner (`docs/candidates/RESEARCH-AGENDA.md` §7.6, K1–K9, comparison
criteria). Recorded here because `docs/canonical/PROJECT-DIRECTION.md` and
`docs/decisions/DECISIONS.md` cite `K1-K9` without saying where they are defined. **Not a
collision** — a bare-use case, which the OWNER column above resolves.

**4.12 — four further collisions.** Each was found by the audit and none is recorded anywhere else:
`DQ` (DQ-1–DQ-5 documentation open questions in `docs/DOCUMENTATION-ARCHITECTURE.md`; DQ-1–DQ-8
experiment criteria in `experiments/exp-doc1/EXP-DOC-1.md`; and `DQ-6` used as a *defect class*
name in `research/RESEARCH-INDEX.md` and `research/history/STAGE-2026-09-11b-v0-transition.md`).
`EXP` (numeric form `EXP-1`…`EXP-4`; alphabetic form `EXP-AI-01`, `EXP-CTIME`, `EXP-DOC-1`).
`H` (H1–H25 hypotheses; `H-DOC-1`/`H-DOC-2` experiment hypotheses; `H11-a`…`H11-g` design
hypotheses). `Q` (Q1–Q25 project questions; `XQ-1`–`XQ-5` experiment questions — the `XQ` prefix
disambiguates, but `Q29` is cited in `DOC-FRICTION-018` as a *phantom* identifier that never
existed).

## 5. Tokens that look like identifiers but are not

| Token | What it is | Where |
|---|---|---|
| `§n`, `§E.32b`, `§7.6` | a section number, scoped to one document | every document |
| `MC §n` | a charter section — the only `§` form that is a project identifier | `docs/canonical/*`, `docs/decisions/*` |
| `UD-nnn`, `EXP-n`, `P-n` in prose | **templates**, not values — a document using them is describing the shape of an identifier, not citing one | the design, `CONTRIBUTING.md` |
| `L` | also the letter of the classification `L — TEMPORARY EXPLORATION`, which is vocabulary, not an identifier | the classification system |
| `M0`–`M3` | historical names for what are now `DM0`–`DM3`; they survive only inside frozen records, which are not to be corrected | `research/history/AUDIT-2026-09-10.md` |
| `V0`, `v0.1`, `v0.2` | version names, not identifiers of the `V` namespace | `README.md`, `docs/versions/` |
| `INV-1`…`INV-8` vs `A1`…`A11` | two separate invariant systems: `INV-n` belongs to the V0 prototype's plan, `A-n` is a project invariant | `docs/v0/` vs `docs/canonical/` |

## 6. Inventory corrections carried by this file

This registry was built by reading the repository, not by copying an earlier audit. Where an earlier
audit was incomplete, the difference is recorded here rather than silently fixed:

- The prefix list in `docs/DOCUMENTATION-ARCHITECTURE.md` §3 names 10 prefixes. The repository uses
  the ~45 rows above. §3 remains the authority for the *principle*; this file is the inventory.
- The ranges of `F-V01` (1–9, not 1–2), `DQ` (two owners), and `S` (two owners) were corrected
  against the files.
- `INV`, `SC`, `EI`, `CO`, `PASS`, `FAL`, `E-MIN`, `MIN`, `XQ`, `O`, `H-DOC`, `PC`, `A1-AI` and the
  `D-PEND` namespace were missing from that earlier audit entirely.
- `Q29` and `H14` are cited once as *phantom* identifiers in `DOC-FRICTION-018`; neither was ever
  defined. They are recorded as phantoms, not as ranges.

## 7. Where the authority lives

| Question | Go to |
|---|---|
| what an identifier means | its OWNER document, named above |
| what the project has committed to | [`decisions/DECISIONS.md`](decisions/DECISIONS.md) |
| the authority ladder | [`../README.md`](../README.md) |
| how the repository is organized, and where new information goes | [`DOCUMENTATION-ARCHITECTURE.md`](DOCUMENTATION-ARCHITECTURE.md) |
| where each document lives | [`README.md`](README.md) |

**Maintenance rule.** This file changes when a *namespace* is created, renamed or retired — a few
times a year — and never when an individual identifier is added. If it is being edited to keep up
with individual identifiers, it has become the second source of truth it exists to avoid.
