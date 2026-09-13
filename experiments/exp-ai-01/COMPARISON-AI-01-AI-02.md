# COMPARISON — EXP-AI-01 Run 01 ↔ Run 02

Status: **COMPLETE — EXP-AI-01 CLOSED** (2026-09-13). `PASS WITH DOCUMENTED INSTRUMENT
DEFECT`. Evidence step only. No implementation, no push.

```
compared       EVIDENCIA-AI-01 (Run 01)  ↔  EVIDENCIA-AI-02 (Run 02)
class          AI-CROSS-CHECK / AGENT-ANALYZED COMPARISON  (NOT a blind comparison — §2)
outcome        7 of 8 questions REPRODUCED · 1 PARTIALLY REPRODUCED · 0 DIVERGENT
experiment     EXP-AI-01 CLOSED — PASS WITH DOCUMENTED INSTRUMENT DEFECT
A1-AI          AI-INDEPENDENT EVIDENCE = 2 RUNS  (independence of delivery not verifiable here)
material       exp-ai-01/material-v1  sha256 8ad75576edad4d6c5da84d2c396a1b56d91c8902fe7992d7ba19e202155f2678  (unchanged, frozen)
prompt         prompt-to-participant.md §B  sha256 766d1deafcc848dd171d9faac8c17e88d1ebe9b90720850e5ecce9c9e4696666  (unchanged)
sources        response-01-external-ai.txt · EVIDENCE-AI-01.md · response-02-external-ai.txt · EVIDENCE-AI-02.md
```

Per-run detail is not repeated here. See [`EVIDENCE-AI-01.md`](EVIDENCE-AI-01.md) and
[`EVIDENCE-AI-02.md`](EVIDENCE-AI-02.md).

---

## 1. SCOPE

Determine whether EXP-AI-01 produced **reproducible evidence between two independent
participants** — not whether two answers match, but whether two readers independently
extracted the same facts, respected the same epistemic limits, and diverged only where the
instrument permits divergence.

Out of scope and not addressed: whether the tool is correct; whether A1 is satisfied;
whether any product change is warranted. This document does not authorize implementation.

---

## 2. METHODOLOGICAL LIMITATION — COMPARISON NOT BLIND

**Recorded at the outset, not buried.**

The comparator designed EXP-AI-01, wrote its questions, built its material, analysed Run 01
before Run 02, and wrote both per-run analyses. It is **not a blind comparator**. The
declaration of non-blindness does not remove it.

What this costs, stated plainly:

- the comparator has an interest in the experiment having worked, and an interest in the
  instrument's defects being attributable to the instrument rather than to the design;
- it knows Run 01's answers while reading Run 02's, which is exactly the condition that
  produces spurious agreement;
- it cannot independently verify the delivery conditions of either run (§3).

What is done instead of pretending otherwise:

- every comparison below is made against `material.txt` and the two raw responses;
- factual claims about the material were **verified programmatically**, not recalled (§3);
- divergence is reported where it exists rather than smoothed, including the one that
  weakens the instrument (§6);
- classification of this document is **AI-CROSS-CHECK / AGENT-ANALYZED**, never a
  human-independent measurement.

A reader who weights this document should weight it accordingly. A human independent review
of this comparison would be a stronger artifact, and none has been performed.

---

## 3. EVIDENCE INTEGRITY

All seven artifacts verified present and unmodified before comparison.

| Artifact | Bytes | sha256 (first 16) |
|---|---|---|
| `response-01-external-ai.txt` | 6160 | `fd73119b419dbd42` |
| `EVIDENCE-AI-01.md` | 28991 | `146d04d494287f79` |
| `response-02-external-ai.txt` | 5830 | `bcefda679400cfe9` |
| `EVIDENCE-AI-02.md` | 24391 | `ec4c00c0cc274947` |
| `material.txt` | 2669 | `8ad75576edad4d6c` |
| `prompt-to-participant.md` | 10165 | `766d1deafcc848dd` |
| `plan-run-02.md` | 8967 | `dd7e37742d556531` |

Both raw responses verified verbatim: Q1–Q8 present, no markdown headings, no analysis
markers, metadata headers outside the response content. Neither was modified during this
comparison.

**Facts about the material re-verified for this document, not recalled:**

| Claim under test | Verified result |
|---|---|
| `exit=0` appears 7 times, no other value | **confirmed** |
| Neither `show` output contains a path token | **confirmed** (lines 33–34, 38–39) |
| `path-encoding` line appears once, at line 64, after both `modified` lines | **confirmed** |
| Distinct file entries named anywhere in the material | **3** (`notes/one.txt`, `notes/three.txt`, the non-UTF-8 entry) |
| Files claimed by `status` | **5** — so **2 entries are never named** |
| `exit=` and `$ umbral` appear in `umbral/src/` | **no** — both are harness-generated |

The fourth and fifth rows matter: they confirm the factual core of Run 02's Q7 reasoning
(§11).

---

## 4. Q1–Q8 COMPARISON

| Q | Run 01 | Run 02 | Coincidencia | Diferencia | Explicación | Reproducibilidad |
|---|---|---|---|---|---|---|
| **Q1** | 6 entries: 5 files, 1 dir, 0 sym, 0 other; same line cited | Same answer; same line cited | **Total** — identical counts, identical citation | Run 02 adds a cross-check (sums to 6; matches `entries=6` in three places) | Run 02 slightly more thorough; no substantive difference | **REPRODUCED** |
| **Q2** | **Three** words: `observed`, `derived`, `exit=0` | **Two** words: `observed`, `derived` | **Partial** — both identify `observed`/`derived` and both state the semantic split is inferred, not defined | Run 01 includes `exit=0`; Run 02 excludes it | **Material defect** (`P-AI01-2`): `exit=` is harness scaffolding, unmarked. Run 01 read the block as uniform; Run 02 matched the question's phrasing ("before the rest of the line"). Both defensible on the material | **PARTIALLY REPRODUCED** |
| **Q3** | No, neither `show` output states its path | No, neither does | **Total** — same conclusion, same reason, same method | Wording only | Both scoped to the tool's lines and said so | **REPRODUCED** |
| **Q4** | Not the same form; `\x` escapes with uppercase hex; defers "escaped" to Q6 | Not the same form; describes `\xFF`/`\xFE` as 4-char sequences with positions | **Total** — same conclusion | Run 02 more detailed; Run 01 more careful about not importing the encoding notion into Q4 | Complementary emphases, not a conflict | **REPRODUCED** |
| **Q5** | `UNKNOWN` for exact bytes; lists syntax, scope, literal chars, case, inversion; raises and sets aside the convention | `UNKNOWN` for exact bytes; lists the convention, case, backslash escapability, escaped portion, **original length**; attributes lowercase form to input | **Strong** — same limit, same reasoning class, both refuse to decode, both decline the convention | Run 02 adds **original length** and the **input-vs-output attribution** | Run 02 marginally more precise on two specifics; neither contradicts the other | **REPRODUCED (strong)** |
| **Q6** | Yes; leaves open subject, syntax, byte recovery, scope | Yes; leaves open meaning, decoding, subject, true bytes | **Total** — same conclusion, overlapping sets of what is left open | Run 01 frames the scope question (whole output vs final command); Run 02 frames the subject question | Different phrasing of the same gap | **REPRODUCED** |
| **Q7** | Tool asserts completeness; vocabulary undefined in-material, so only the assertion is supported | `UNKNOWN` if verified independently; **only 3 of 5 file paths named**, so completeness cannot be checked | **Total conclusion** — both refuse to equate assertion with verification | **Different supporting reasons**: Run 01 cites undefined vocabulary; Run 02 cites the missing enumeration | Complementary. Run 02's is concrete and **verified true** (§3) | **REPRODUCED** |
| **Q8** | No failure; 7 × `exit=0`; separates presence from meaning, declares convention | Same; adds that no signal or error text appears | **Total** — same separation, same confidence structure | Run 02 notes stderr absence | Same reasoning, Run 02 marginally broader | **REPRODUCED** |

### What "reproduced" means here

Not identical wording. A question is **REPRODUCED** when both participants:

1. report the same **fact** about the material, verified against it;
2. respect the same **epistemic limit** — neither claims more than the material supports;
3. differ, if at all, only in **precision or framing**, without contradiction.

Under that definition, a divergence in wording is not a divergence in result, and a
difference in detail is not a failure of reproduction.

---

## 5. STRONG CONVERGENCE

Three convergences are substantive, and they are the reason this experiment is worth
continuing.

**(a) Q5 — the epistemic limit on `\xFF\xFE`.** Both refused to decode; both named the
missing specification; both identified the available external convention and declined it
rather than adopting it silently. Neither invented semantics for `\xNN`. This is the
primary signal, and it converged.

**(b) Q7 — assertion versus verification.** Both maintained the separation between "the
tool reports `complete=true`" and "a reader can establish that reading occurred", and both
reached it independently by different routes. Neither converted the tool's self-report into
a verified fact, and neither turned the gap into a claim that the tool is wrong.

**(c) Q2 — labels visible, semantics inferred.** Both stated explicitly that `observed` and
`derived` are objectively present in the output while their **meaning** is inferred from
usage rather than defined by the material. This is a distinction the questions did not
supply; both made it unprompted.

---

## 6. DIVERGENCES

### Real divergence

**Q2 — two leading words versus three.** The only divergence in the comparison.

- **Is it contradictory?** No. Run 01 reports `exit=0` as a third leading token; Run 02
  reports only two. Both statements are true of the material: `exit=0` **is** a line
  beginning with a single word, and it **is not** tool output. The participants answered
  slightly different readings of "output lines".
- **Caused by the material?** Yes — decisively. `exit=` is written by the capture harness,
  and the material marks no boundary between harness scaffolding and tool output. This is
  the pre-registered defect `P-AI01-2`, and it is the divergence it was predicted to cause.
- **Caused by the question?** Partly. "Most output lines begin with a single word **before
  the rest of the line**" describes `observed`/`derived` lines precisely and describes
  `exit=0` poorly, since it has nothing after it. Run 02's exclusion follows that phrasing;
  Run 01's inclusion follows the material's uniform presentation. Both are reasonable.
- **Is either participant wrong?** **No.** Neither is called incorrect. The instrument is
  ambiguous, and the divergence is a measurement of that ambiguity.
- **Which is more useful?** Run 02's answer is closer to what the question asked about the
  tool. Run 01's answer is closer to what a reader sees. Both are legitimate; the
  divergence itself is the finding.

### Non-substantive / presentational divergences

| Divergence | Nature |
|---|---|
| Q4 — Run 02 describes the escape character-by-character; Run 01 says "backslash-x escapes with uppercase hex" | **Precision**, same conclusion. Run 01 explicitly refuses to call the second form "escaped" until Q6 licenses it — arguably the more disciplined ordering; Run 02's description is more informative |
| Q5 — Run 02 adds original length and the input-vs-output attribution | **Precision**, same limit. Both additions are correct and verifiable |
| Q7 — different supporting reasons for the same conclusion | **Complementary evidence**, not disagreement. Undefined vocabulary (Run 01) and missing enumeration (Run 02) are both true and both independently sufficient |
| Q1, Q8 — Run 02 adds a cross-check / stderr note | **Thoroughness**, no conflict |
| Confidence levels — Run 01 medium on Q2's description; Run 02 "high for the list, medium for the description" | **Presentation.** Run 02's split is more precise about what is certain |
| Q5 confidence wording — Run 01 "low for any byte claim, high for UNKNOWN"; Run 02 "high that bytes are undetermined" | **Presentation.** Same position |

**On "was one participant more precise?"** Run 02 is more precise on three specific points:
the character-level description of the escape (Q4), the original-length omission (Q5), and
the enumeration gap (Q7, verified true). Run 01 is more precise on one: it refuses to apply
the word "escaped" before the material licenses it (Q4), and it names the inversion question
explicitly (Q5).

**Neither is more correct.** Precision is not correctness, and a more detailed answer to a
question about an under-specified surface is not a better reading of it — it is a fuller
report of the same limit. The differences are recorded as observations about the two
answers, not as a ranking.

---

## 7. EXPERIMENT DEFECTS — IMPACT ON THIS COMPARISON

| Defect | Produced divergence? | Detected by |
|---|---|---|
| **Tool output mixed with harness scaffolding** (`P-AI01-2`) | **YES** — it is the sole cause of the Q2 divergence, and it affects how Q3 and Q8 count "output lines" | **Neither**, as a defect. Run 02 identified the lowercase form as input rather than tool output in Q5, which is adjacent but not the same observation. Neither participant said "this line may not come from the tool" |
| **Two renderings of one name, unmarked** (`P-AI01-1`) | **No** | **Both** — both recorded the case difference as unexplained in `UNEXPECTED_FINDINGS`. Run 02 went further and determined which rendering belongs to the input |
| **Leading framing in Q3 and Q5** (`P-AI01-3`) | **No** | **Neither** (as expected — participants do not audit the questions). Both answered substantively with reasoning, which is the evidence that the framing did not determine the answer |
| **Escape not fully specified in the output** (`F-AI01-2`) | **No** — it is the object of study, not a fault in the instrument | **Both** — both named it as the reason for `UNKNOWN` |

**The distinction that matters:** a defect that exists but produces no divergence has a
different status from one that changes the result. Three of the four defects were inert for
this comparison. **One was not**, and it is the one that must be fixed before any further
run: an instrument that silently changes what a question measures will keep producing
divergence that looks like disagreement between participants.

The defects are the author's. Neither participant is responsible for any of them, and no
participant's answer is discounted because of one.

---

## 8. PRODUCT LEGIBILITY FINDINGS

**No new product finding is created by this comparison.** Both runs engaged claims that
already have homes, and two runs agreeing does not upgrade a claim's status — it is
`AI-CROSS-CHECK` evidence, not independent confirmation of a fact about the world.

| Observation | Home | Status after comparison |
|---|---|---|
| `show` reports observations without naming the subject path | allocated in this experiment's namespace | **Reproduced** across two runs. Both reached it independently by the same verifiable method. Classification unchanged: legibility debt |
| The escaped rendering is not self-declared (syntax, reversibility, scope, subject) | allocated in this experiment's namespace | **Reproduced.** Both named it as the reason for `UNKNOWN`. The comparison adds a sharper form: the `path-encoding` declaration is **global and has no subject** — it appears once, after both `modified` lines, and names no path, so a reader cannot tell which entry it describes |
| Only 3 of 5 file entries are ever named; no listing command exists | `F-V01-1` (v0.1, accepted) | **Engaged by Run 02, quantified and verified.** Run 01 did not reach this specific point |
| `observed`/`derived` are visible labels whose semantics are undefined in the output | `F-V01-9` (v0.1, outside v0.1) — **class only**, see below | **Reproduced** in substance by both |

`F-V01-2`, `F-V01-4`, `F-V01-5` are not engaged by either run and keep their dispositions.
**V0 is not reopened.**

### Finding-identifier verification

Every `F-V01-n` cited above was checked against `docs/versions/v0.1.md`, where the records
live, before this document was finalised. The check was performed because a reference to a
finding that does not exist, or that says something else, would be a fabrication of
traceability.

| Identifier | Exists | Its recorded claim | Does the citation match? |
|---|---|---|---|
| `F-V01-1` | **yes** — `docs/versions/v0.1.md`, table and limitation 8 | "coverage gap: `status` gives counts, not the identity of verified entries"; the prose adds that identifying them "requires one `show` per path; there is no listing command" | **yes.** The comparison's "a reader cannot obtain the identity of the entries behind a count" is the same claim |
| `F-V01-2` | **yes** | output-contract defect: values labelled `observed` that the tool computes. Corrected | **yes** — cited only as *not engaged*, which is accurate |
| `F-V01-4` | **yes** | `show` panicked on a non-UTF-8 path argument. Corrected | **yes** — cited only as *not engaged* |
| `F-V01-5` | **yes** | a test named non-UTF-8 CLI coverage and did not exercise it. Corrected | **yes** — cited only as *not engaged* |
| `F-V01-8` | **yes** | where an observed and a derived value coincide, the labels alone do not let the rule be reconstructed | **yes** — cited as adjacent to Q2 but not the same claim |
| `F-V01-9` | **yes** | the output's vocabulary is defined nowhere in the output — **enumerating** `stability`, `unstable-or-unreadable`, `scan-complete`, `object-survives`, `recreated`, `unobserved`, `other`, and whether `entries` includes the root | **partially.** See below |

**The one imprecision found, and it is corrected rather than smoothed.** `F-V01-9`'s
recorded claim is a **class** — the output's vocabulary is defined nowhere in it — but its
**enumeration** does not list `observed` or `derived` among the undefined terms. So:

- the **class** covers this observation: `observed`/`derived` are output vocabulary;
- the **specific instance** is an addition to the enumeration, not a restatement of it.

Recorded here so that a reader does not infer that `F-V01-9` already named the labels. The
observation stands on its own evidence (both runs report inferring the split from usage); it
is attributed to `F-V01-9` as the same class, and extending that finding's enumeration is a
decision for the owner, not something this comparison does silently.

---

## 9. REPRODUCIBILITY MATRIX

| Q | Classification | Basis |
|---|---|---|
| Q1 | **REPRODUCED** | Identical fact, identical citation |
| Q2 | **PARTIALLY REPRODUCED** | Same labels and same epistemic claim about the split; divergence on the word count, caused by a known instrument defect |
| Q3 | **REPRODUCED** | Identical conclusion, identical reason |
| Q4 | **REPRODUCED** | Identical conclusion; difference is detail level |
| Q5 | **REPRODUCED** | Same limit reached independently, by compatible reasoning |
| Q6 | **REPRODUCED** | Identical conclusion, overlapping sets of what is left open |
| Q7 | **REPRODUCED** | Same conclusion reached by two different, both-correct reasons |
| Q8 | **REPRODUCED** | Identical separation of observation from convention |

**7 of 8 REPRODUCED. 1 PARTIALLY REPRODUCED. 0 DIVERGENT. 0 NOT ASSESSABLE.**

No question is `NOT ASSESSABLE`: every question was answered by both, and every factual
claim either made was checkable against the material.

---

## 10. GLOBAL ASSESSMENT OF EXP-AI-01

**The question:** could two independent participants consistently extract the facts and the
limits that the material permits them to sustain?

**The answer: yes**, on the substantive objective.

```
EXP-AI-01 = PASS WITH DOCUMENTED INSTRUMENT DEFECT
```

This classification is deliberately **not** a bare "the experiment passed". The instrument
was not clean, and the formulation has to carry that:

- **The substantive objective was reproduced.** Two participants independently extracted the
  same facts and respected the same limits.
- **Q5 and Q7 are the central epistemic signals**, and both converged. Q5: both refused to
  decode `\xFF\xFE` and named the missing decoding specification. Q7: both separated the
  tool's assertion of completeness from what a reader can independently establish.
- **Q2 produced a divergence**, and it is the one question that did not reproduce.
- **The Q2 divergence is attributed to the material/harness design**, not to the
  participants: tool output and harness scaffolding were presented in the same captured
  block, with no boundary.
- **That divergence is not a factual disagreement between participants.** Both readings are
  reasonable given what they were shown; neither is incorrect.
- **The instrument was therefore not completely clean**, and a future experiment of this kind
  must separate tool output from runner output (§7, §12).

**Alternative classification, recorded because it is defensible.** If `PASS` requires every
question to reproduce, this is **PARTIAL**. What worked is the substantive content — 7 of 8
questions, including both questions designed to test epistemic discipline. What did not is a
question whose answer was ambiguous because the instrument was ambiguous. The decision of
record is `PASS WITH DOCUMENTED INSTRUMENT DEFECT`; `PARTIAL` remains an available reading and
the distinction changes none of the findings.

**What the PASS does not mean:** that the answers are correct; that the tool is correct; that
the surface is legible to humans; that A1 is any closer to satisfaction. It means the
experiment, as an instrument, behaved reproducibly on the question it was designed to ask —
and falsified one part of its own design while doing so.

---

## 10a. WHAT EXP-AI-01 ESTABLISHED

The results of record. Each is evidence about what the material makes recoverable by a
reader, and none is a claim about the correctness of the implementation.

**A. Two independent participants reached `UNKNOWN` on the exact bytes of `\xFF\xFE`, and
justified it by the absence of a decoding specification.** Neither decoded; neither invented
semantics for `\xNN`; both identified the conventional reading and declined it.

**B. Two independent participants distinguished the tool's self-report of
completeness/verification from what could be independently established from the material.**
Both kept `complete=true` / `content-verified=5` as an assertion rather than a verified fact.

**C. Two independent participants detected that `show` output does not include the queried
path.** Both, independently, by the same verifiable method.

**D. Two independent participants found the `path-encoding=escaped` line insufficient on its
own to determine a complete reversible encoding.** Both named it as the reason for `UNKNOWN`.

**E. Q2 revealed that the material mixed real tool output with harness scaffolding, producing
a legitimate divergence.** One participant included `exit=0` among the visible vocabulary;
the other excluded it. Both were reading the same block correctly.

**F. Therefore the experiment was useful not only for evaluating participants, but for
falsifying part of its own design.** That is a methodological result, not a product one.

**Not converted into claims about the product.** A, B, C and D are statements about what the
material makes legible to a reader. They are not findings that the tool behaves incorrectly,
and no participant claimed they were.

## 10b. WHAT EXP-AI-01 DID NOT ESTABLISH

Recorded explicitly, so the boundary of this evidence is as visible as its content:

- **A1 was not demonstrated.** No human reader has run any protocol. Two AI participants do
  not satisfy A1, and nothing here advances it.
- **That Umbral is correct**, merely because two AIs read the material similarly. Agreement
  between two models is `AI-CROSS-CHECK` evidence; it is not verification of the tool.
- **Absolute independence of the participants.** Independence is procedurally supported and
  author-attested; it was not technically verified by Hermes (§11).
- **That the escaping is incorrect in implementation.** Only that it is not declared in the
  output.
- **That `show` must change.** Only that its output does not identify its subject.
- **The best design for `material-v2`.** Requirements are recorded (§12); no design is
  settled.
- **`D-PEND-2`** — whether `ctime` enters the v0.2 skip condition. Untouched by this
  experiment.
- **`Q25`** — persistence model. Untouched.

---

## 11. A1 / A1-AI STATUS

```
A1     HUMAN-INDEPENDENT   NOT SATISFIED. Unchanged.
A1-AI  AI-INDEPENDENT EVIDENCE = 2 RUNS
       Independence of delivery was not independently verifiable by Hermes.
```

**On the claim of independence**, stated at the precision the evidence supports.

**Evidence of procedural independence in the record:**

- the same frozen material and the same prompt section for both runs;
- the prompt contains no reference to a prior run;
- run 02's plan was committed **before** run 02 was executed;
- the second participant did not receive run 01's response, the analyses, any finding
  identifier, or any expected answer;
- Q2 diverged, and the two runs differ in precision — neither pattern is what a copied or
  shared answer produces.

**What is NOT claimed: `independence proven`.** Hermes did not control the delivery channel
and cannot confirm what was actually transmitted. The classification therefore rests on
procedural evidence plus the owner's attestation, not on technical verification.

**AUTHOR-ATTESTED DELIVERY CONDITION.** The project owner states that they delivered each
run personally. This is recorded as an attestation by the author of the record — it is part
of the chain of custody known to them, and it is **not** an independent technical
verification. A reader should treat it as a declared condition of the experiment rather than
as a verified fact.

**A1 is not satisfied by any number of AI runs, and this comparison does not change that.**
Nothing here is promoted to `HUMAN-INDEPENDENT`. A1 requires a person other than the author
executing a protocol without interpretive help, and none has done so.

---

## 12. MATERIAL-V2 ASSESSMENT

**Not built, and not to be built now.** Each defect classified by what it requires.

| # | Defect | Classification | Reasoning |
|---|---|---|---|
| 1 | **Tool output not separated from harness scaffolding** | **Must be fixed before any further run** | The only defect that produced divergence. It silently changes what Q2 and Q8 measure. Fixing it makes a next run measure the tool rather than the capture format |
| 2 | **Escape not specified in the output** | **Must NOT be fixed** | It is the object of study. Declaring the escape in the material would destroy the question and the finding |
| 3 | **`show` does not name its subject** | **Must NOT be fixed** | A product property under observation, not an instrument fault |
| 4 | **Leading framing in Q3/Q5** | **Should be fixed before a further run** | Produced no divergence, but a leading question cannot be ruled out as a partial cause of a future answer. Lower priority than #1 |
| 5 | **No entry enumeration, so Q7 cannot be fully checked** | **Requires a different experiment** | Supplying an enumeration means changing the tool's output (a listing command), which is `F-V01-1` and outside v0.2's decided scope. It is not a material fix |
| 6 | **Two renderings of one name, unmarked** (`P-AI01-1`) | **Should be fixed with #1** | Both are consequences of the same decision to place the invocation echo in the same block as the tool's output. Neither participant was misled, which is why it ranks below #1 |

### Requirements carried forward for any future experiment of this kind

Recorded as requirements, not as a design. No design is settled and nothing is to be built
yet:

1. **Separate tool output from harness scaffolding**, unambiguously, so a reader never has to
   judge which lines the tool produced.
2. **Make the subject/path explicit in output where the question requires it** — or state
   deliberately that it is not, when the absence is the object of study.
3. **Specify the escape, or deliberately leave it unspecified according to the experiment's
   objective.** For this experiment, leaving it unspecified was correct.
4. **Review Q3 and Q5 for unnecessary framing**, so that a leading question cannot be a
   partial cause of an answer.
5. **Design a different experiment if completeness/coverage is to be tested.** It cannot be
   tested with a material that lacks an enumeration.

**material-v1 remains frozen as the historical record of EXP-AI-01.** The unspecified escape
was precisely part of the observed phenomenon in Q5/Q6; correcting it would destroy the
finding and void the comparison. No retrospective change, under any framing.

---

## 13. TECHNICAL FOLLOW-UPS — CANDIDATE TECHNICAL FOLLOW-UP, NOT AUTHORIZED

All three are classified **CANDIDATE TECHNICAL FOLLOW-UP**. None is a confirmed bug. None is
authorized for implementation. No code was changed.

**CTF-1 — the escape declaration has no subject.** `path-encoding=escaped` names no path, so
a reader cannot tell which entry it describes. **Candidate**, not a bug: it is a legibility
observation about what the output makes recoverable. Bearing: `UD-017` requires `basis` to be
traceable to the evidence that produced it, and the same failure mode — a value that does not
identify what it applies to — is available in that design. Worth considering there.

**CTF-2 — `show` output carries no explicit subject/path, while `changes` does.** **Candidate**,
not a bug. The inconsistency is internal to the tool. Belongs with the deferred legibility
question that already holds `F-V01-8`/`F-V01-9`; **not** in v0.2's decided scope.

**CTF-3 — a reader cannot obtain the identity of the entries behind a count.** **Candidate**,
and already recorded as `F-V01-1` (verified present, §8), accepted for v0.1 and handed to a
later version. This comparison does not change that disposition and does not propose to.

Every `F-V01-n` referenced anywhere in this document was verified against
`docs/versions/v0.1.md` before finalisation; see §8. No identifier was created or invented to
support a reference.

---

## 14. WHAT CAN BE SAID NOW — EPISTEMIC CLASSIFICATION

Nothing here is elevated from `AI-CROSS-CHECK` to `HUMAN-INDEPENDENT`.

**ESTABLISHED BY BOTH RUNS** (independently reached, verified against the material, same
epistemic limits respected):

- the entry accounting: 6 entries, 5 files, 1 directory, 0 symlinks, 0 other;
- neither `show` output states the path it reports about;
- the two reported paths are written in different forms, and the second uses `\xNN` escapes;
- the exact bytes of `\xFF\xFE` cannot be reconstructed from the material, and the reason is
  the absence of a decoding specification;
- `path-encoding=escaped` does not, by itself, determine a complete reversible encoding;
- the tool's assertion of completeness is not independently checkable from the material;
- seven `exit=0` values are present, and their *meaning* rests on an external convention;
- `observed`/`derived` are visible labels whose semantics are inferred from usage.

**ESTABLISHED BY ONLY ONE RUN:**

- the escape declaration is **global and names no path**, so its subject is undeterminable
  (Run 02's Q6 framing; the comparison verified the underlying fact, but Run 01 did not raise
  the subject question in this form);
- **only 3 of 5 file entries are ever named** (Run 02, Q7 — verified true by the comparison);
- **original length** is among the missing pieces for reconstructing the name (Run 02, Q5);
- the lowercase rendering belongs to the **command line**, not the tool's output (Run 02, Q5);
- the absence of any error text or signal on stderr (Run 02, Q8);
- the counts' internal consistency across three lines (Run 02, Q1).

**DIVERGENT:**

- **whether `exit=0` is part of the output's leading vocabulary.** Run 01 includes it; Run 02
  excludes it. Both readings are defensible on the material; the divergence traces to the
  unmarked presence of harness scaffolding.

**NOT DETERMINABLE** from this experiment:

- whether the escape, if declared, would be sufficient for a reader;
- whether the `show` omission matters in interactive use;
- whether two AI readers agreeing predicts anything about a human reader;
- whether the Q2 divergence would reproduce with a third participant;
- what the participants had access to beyond the delivered prompt (the delivery condition is
  author-attested, not verified).

**NOT TESTED:**

- human legibility in any form (A1);
- the correctness of any tool verdict;
- `ctime` behaviour on ext4;
- any v0.2 mechanism, which is not implemented.

---

## 15. OPEN QUESTIONS

Carried forward, none answered here:

1. Would a reader **given** the escape contract find it sufficient? Two runs establish only
   that the contract is absent from the output, not what a present contract would need.
2. Would the `show` omission matter in interactive use, where the user typed the path?
3. Does the conventional reading of `\xNN` mislead readers in a way another rendering would
   not? Both declined the convention; that is two data points, not an answer.
4. Do two AI readers agreeing predict anything about a human reader? **Untested, and not
   answerable from this experiment.** The convergence is evidence about AI readers and about
   the material's determinacy, not about human legibility.
5. Would a third participant reproduce the Q2 divergence, or resolve it differently? A third
   run on fixed material would test whether the defect reliably produces the divergence.

---

## 16. DECISION RECOMMENDATION

1. **Accept this comparison** as `AI-CROSS-CHECK`, with its non-blind limitation recorded.
2. **Record A1-AI as `AI-INDEPENDENT EVIDENCE = 2 RUNS`**, with the delivery-independence
   qualification in §11 carried alongside it, never as `A1`.
3. **Close EXP-AI-01 as `PASS WITH DOCUMENTED INSTRUMENT DEFECT`** (§10), with `PARTIAL`
   recorded as the defensible stricter reading.
4. **Fix the tool/harness separation before any third run.** Nothing else needs fixing first.
5. **Do not revise `material-v1`.** It is the historical capture both runs were measured
   against.
6. **Do not implement anything.** The technical follow-ups are `CANDIDATE TECHNICAL FOLLOW-UP`,
   not authorized work.
7. **A1 remains unsatisfied** and is not advanced by any of this.
8. **The next step is a project decision**, not implementation: which of these results should
   feed v0.2. Implementation requires separate, explicit authorization.

---

## 17. DELIVERY ANOMALY — RUN 02

Recorded separately, and **not** as a discrepancy between participants.

The owner's message delivering Run 02's response contained it **twice**: once as the
labelled raw block, and once again after the pipeline diagram. Both copies were transcribed
and compared programmatically before preservation: **byte-identical**, 5738 bytes each,
including all whitespace and the final line.

- Classification: **DELIVERY ANOMALY**.
- Effect on the evidence: **none.** One copy is preserved; the preserved file's payload is
  byte-identical to both.
- Effect on the comparison: **none.** No comparison in this document depends on the
  duplication.
- Cause: **not speculated.** The available evidence establishes only that the two copies
  were identical. Whether the participant produced two outputs, whether the paste was
  duplicated, or whether something else occurred is `UNKNOWN` and is recorded as such.
- Why it is recorded at all: an unexplained duplication in an evidence-delivery channel is
  the kind of thing that should be visible in the record rather than silently normalised. If
  the same channel is used for a third run, the anomaly is worth watching for.

---

## 18. DECISION SUMMARY

```
A1              NOT SATISFIED
A1-AI           EVIDENCE RECEIVED — 2 INDEPENDENT AI RUNS
EXP-AI-01       COMPARISON COMPLETE  (PASS with a documented instrument defect)
IMPLEMENTATION  NOT AUTHORIZED
PUSH            NOT AUTHORIZED
```

Nothing was implemented; no production code or test was touched; `material-v1`, the prompt
and the plan are unchanged; `material-v2` was not built; nothing was committed or pushed.
V0 remains frozen; v0.1 remains not declared complete; `Q25` and `D-PEND-2` remain open.
