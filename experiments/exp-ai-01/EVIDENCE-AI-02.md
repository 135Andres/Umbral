# EVIDENCIA-AI-02 — second delegated reading of a non-UTF-8 path (Run 02)

Status: **RESULT RECEIVED, ANALYSED INDEPENDENTLY** (2026-09-13). Analysis only. No
comparison with Run 01 has been performed. No implementation, no commit, no push.

```
evidence class   AI-INDEPENDENT / A1-AI   (UD-015)
experiment       EXP-AI-01
run              Run 02
participant      external AI (identity not provided; not recorded rather than guessed)
mode             SEMI-BLIND
material         exp-ai-01/material-v1  (sha256 8ad75576edad4d6c5da84d2c396a1b56d91c8902fe7992d7ba19e202155f2678)
prompt           prompt-to-participant.md  §B, unmodified  (sha256 766d1deafcc848dd171d9faac8c17e88d1ebe9b90720850e5ecce9c9e4696666)
raw response     response-02-external-ai.txt  (verbatim, unmodified)
registered       2026-09-13
```

**This does not satisfy A1.** A1 remains HUMAN-INDEPENDENT and NOT SATISFIED. This run is
AI-INDEPENDENT evidence, auxiliary, per `UD-015`.

## 0. SCOPE OF THIS DOCUMENT, AND ITS LIMITATIONS

This document analyses **Run 02 only**. It does not compare Run 02 with Run 01, does not
use Run 01 as a key, does not compute agreement, and makes no claim that anything is
reproduced across participants. The comparison is a separate step, to be performed only
after the owner reviews this analysis.

**Honest limitation on independence.** The author of this analysis designed the experiment,
wrote the questions, built the material, and has seen Run 01's analysis. That knowledge
cannot be set aside by declaration. What is done instead:

- every assessment below is made against `material.txt` and against Run 02's own text;
- no assessment cites Run 01's answers, and no answer of Run 02's is scored against them;
- where a claim of Run 02's has a home in an already-allocated finding, that is recorded as
  an identifier-allocation fact, not as a comparison (§7);
- the reader of this document should treat the analysis as **not blind**, and weight the
  comparative step accordingly.

---

## 1. EVIDENCE-AI-02 STATUS

Received and preserved. Classified AI-INDEPENDENT / A1-AI. Pending the owner's review, then
the formal comparison.

**Delivery anomaly, reported separately as required.** The owner's message contained the
participant's response **twice** — once as the labelled raw block, and once again after the
pipeline diagram. Both copies were transcribed and compared programmatically: **identical**,
5738 bytes each, byte for byte, including all whitespace and the final line. One copy is
preserved. The duplication has no effect on the evidence; it is recorded because an
unexplained duplication in a delivery channel is the kind of thing that should be visible
rather than silently normalised.

**Metadata not supplied, and therefore not recorded:** the participant's identity or model
version; the time of receipt; whether the participant had any prior context. What is recorded
is the registration date. Nothing has been invented to fill these.

---

## 2. RAW EVIDENCE PRESERVATION

`experiments/exp-ai-01/response-02-external-ai.txt`

- 5830 bytes, sha256 `bcefda679400cfe9cf1e8d13f869dc86e0358cbf39b08de6aa0ea11f3ad0221c`.
- One metadata header line, outside the response content, matching the convention used for
  Run 01's raw file: the header names the file as unmodified and points to this analysis.
- The participant's text is otherwise **untouched**: no correction of wording, formatting,
  inference, confidence level, `UNKNOWN`, or conclusion.
- Verified: Q1–Q8 all present; no markdown headings; no analysis markers; the payload is
  byte-identical to the received copy.
- Nothing from this analysis has been written into the raw file.

---

## 3. Q1–Q8 ANALYSIS

| Q | Participant answer | Directly supported? | Inference quality | External assumptions | Assessment |
|---|---|---|---|---|---|
| **Q1** | 6 entries: 5 files, 1 dir, 0 symlinks, 0 other | **Yes** — the quoted line is in the material and the numbers are exact | Sound. Cross-checks the counts against `entries=6` in three places, all of which are present in the material | None | **Correct observation.** The added cross-check is supported |
| **Q2** | Two distinct words: `observed`, `derived`; semantics inferred from usage | **Yes** for the list — both are tool output. Excludes `exit=0`, which has no "rest of the line" | **Strong.** Explicitly separates the objectively visible labels from the inferred semantics, and grades confidence accordingly (high for the list, medium for the description) | None for the split | **Correct observation + correct inference.** See §6 for what its exclusion of `exit=0` depends on |
| **Q3** | No for both: neither `show` output states its path | **Yes** — verified: neither output block contains a path token | Correct, and the participant states its method ("verified by absence of path substring") | None | **Correct observation.** See §5 |
| **Q4** | Two paths; not the same form; describes the escape character by character | **Yes** — both lines quoted exactly | **Precise.** Describes `\xFF` and `\xFE` as four-character sequences with leading backslash, letter `x`, two uppercase hex digits; locates them between `notes/weird-` and `.txt` | None | **Correct observation** |
| **Q5** | `UNKNOWN` for exact bytes; lists what the material fails to define | **Yes** — both cited lines are present and are the only relevant ones | **The strongest result in the run.** See §4 | Identifies a conventional reading (`\xFF` = byte 0xFF) and declines to adopt it, because no rule is shown | **Appropriately UNKNOWN** |
| **Q6** | Yes, the line is an explanation; leaves four things open | **Yes** — line quoted exactly | Correct. All four items it lists are genuinely open in that line | None | **Correct observation + correct inference** |
| **Q7** | `UNKNOWN` if verified independently; the tool asserts completeness but does not enumerate | **Yes** — all quoted lines present | **Strong, and it grounds the caution in a concrete, checkable gap:** only three file paths are ever named, so completeness cannot be checked from the material | None | **Correct inference.** See §5 |
| **Q8** | No failure shown; seven commands ended with status 0 | **Yes** — seven `exit=0` occurrences, verified; no other value | **Exemplary separation:** high confidence that no non-zero value appears; medium confidence that this means success, because the meaning rests on convention | Identified explicitly (CLI convention) and declared as such | **Correct observation + correct separation** |

### Observed / inferred / refused, in the participant's own terms

- **Literally observed:** the counts and their agreement across three lines (Q1); the two
  leading labels (Q2); the absence of a path token in the `show` output (Q3); the two
  rendering forms and their exact characters (Q4); the encoding line's existence and wording
  (Q6); the seven exit values (Q8).
- **Inferred:** the semantic split between `observed` and `derived`, and that it is not
  defined in the material (Q2); that a reader must associate `show` output to its subject by
  position (Q3); what the encoding line leaves open (Q6); that the tool's completeness is an
  assertion rather than an established fact (Q7).
- **Required external convention, and declared:** that `exit=0` means success (Q8); the
  conventional reading of `\xNN`, raised and then declined (Q5).
- **Correctly refused:** the byte sequence (Q5); any absolute claim that every entry was read
  (Q7); any claim about what the escape syntax means (Q5, Q6).

**One imprecision, immaterial to its conclusion.** In Q3 the participant refers to "those six
output lines". Each `show` block is three lines including its `exit=0`, so six is right for
the captured blocks — but two of those six are harness-generated, not tool output. The claim
(no path substring in any of them) holds regardless, so the conclusion is unaffected. It is
noted because the count treats the capture as one uniform surface, which is the subject of §6.

No answer was scored against a stored key. No answer was compared with any other run.

---

## 4. Q5 — THE PRIMARY EPISTEMIC SIGNAL

The value of Q5 is not that the answer is `UNKNOWN`. A participant can reach `UNKNOWN` by
guessing that the question is a trap. What matters is the reasoning, and this participant
named exactly the right missing items:

> "the material never defines the escaping: whether `\xFF` means byte 0xFF, whether case
> matters, whether backslash itself is escapable, which portion is escaped, or what the
> original length is."

Five specific things, all genuinely undefined by the material, and the last one — the
original length — is a point that is easy to miss and would matter to anyone trying to
recover the name.

It also handled the available temptation explicitly rather than silently:

> "The earlier command-line form `notes/weird-\xff\xfe.txt` with lowercase is input, not tool
> output, and does not define output decoding."

That is a correct and non-obvious attribution. The material contains two renderings of the
same name; the participant determined that the lowercase form belongs to the command line
rather than to the tool's output, and therefore carries no decoding authority. It declined to
read a rule off it.

**Positive signal recorded:** the participant independently recognised that `\xFF\xFE` cannot
be decoded into original filesystem bytes from the material alone, **identified the missing
encoding contract rather than merely asserting absence**, and avoided importing an unstated
convention.

**Honest counterweight, the same one that applies to any such result.** Modern assistants are
trained toward calibrated caution, and a well-argued `UNKNOWN` is partly what such a system
does by default. That weakens any inference of the form "this participant is exceptionally
rigorous". What the evidence supports is narrower and still useful: on this material the
participant did not overreach, and the reason it gave is the right one.

---

## 5. Q3 AND Q7 — LEGIBILITY OBSERVATIONS

### Q3 — `show` does not name its subject

The observation is verifiably correct: lines 33–34 and 38–39 of `material.txt` are the tool's
entire output for the two `show` commands, and neither contains a path token. The path appears
only in the invocation, which the participant correctly attributes to the command line.

**Classification: both, and they are separable.**

- **Product side.** The tool's own sibling command prints a subject on every relevant line —
  `changes` emits `path=notes/three.txt` and `path=notes/weird-\xFF\xFE.txt` on its mutation
  lines — while `show` prints none. That inconsistency is a property of the tool's output, not
  of the protocol. It matters whenever output is captured, piped, or read out of context.
- **Material side.** The material presents the invocation and the output in one block with no
  boundary, so a reader must decide where "the output" begins. The participant scoped its
  answer to the tool's lines and was explicit about the method, which is the correct handling
  of an under-determined frame.

### Q7 — the tool's assertion versus independent verification

The participant's answer is epistemically justified, and this is worth stating precisely
because it is the crux of the experiment:

> "Counts are internally consistent with 6 entries … suggesting success if trusted, but only
> three file paths are ever named … Without a full entry list a reader cannot check that every
> filesystem entry was covered."

The material reports `files=5`, and exactly three file entries are ever named anywhere in it
(`notes/one.txt`, `notes/three.txt`, and the non-UTF-8 entry). The remaining two are never
named. So the claim "every entry was read" cannot be checked from the material — not because
the tool is wrong, but because the material never enumerates what would have to be checked.

**This is three things at once, and they are recorded separately:**

1. **Valid epistemic distinction (primary).** "The tool reports `complete=true`" and "every
   entry was read" are different statements. The first is observable in the output; the second
   is not established by it.
2. **Material limitation (secondary).** The material offers no independent cross-check — no
   listing from another tool, no enumeration. The question is unanswerable *from this
   material* by construction.
3. **Relation to a recorded project gap (tertiary).** That a reader cannot obtain the identity
   of the entries behind a count is the gap already recorded as `F-V01-1` (see §8).

**Explicitly not claimed:** that Umbral's content verification is incorrect, insufficient or
misleading. The participant did not claim it and this analysis does not either. What is
established is that the output's own assertions are not independently checkable from the
output alone.

---

## 6. EXPERIMENT / PROTOCOL FINDINGS

These are properties of the package, verified against `material.txt` — not judgements about
the participant.

### The material does not distinguish tool output from harness scaffolding

`exit=` and the `$ umbral …` invocation lines are **not tool output**: neither string exists
anywhere in `umbral/src/`. Both are written by the capture harness, `capture_material.py`.
The material places them in the same block as the tool's lines with no boundary.

**How this run interacted with it:**

- **Q2** — the participant listed only `observed` and `derived`. That is a defensible reading:
  `exit=0` is a complete line with no "rest of the line", so it does not match the question's
  phrasing. But the exclusion was possible because of the line's **shape**, not because
  anything in the material declares `exit=` to be non-tool. A reader who took the block as one
  uniform surface would list three words, and would not be wrong on the material's own terms.
- **Q8** — the participant correctly identified that the meaning of `exit=0` rests on an
  external convention. The question would have been a cleaner test of the tool's own failure
  signalling had the boundary been visible.
- **Q3** — the "six output lines" count includes the two `exit=0` lines, again treating the
  block as one surface.

The defect therefore remains operative: two questions measure the capture format as well as
the tool.

### Two questions carry leading framing

- **Q3** — "Does that output state which path it is reporting about? Answer yes or no" is a
  yes/no question about an absence, which invites "no".
- **Q5** — "what can you **not** determine" presupposes that something cannot be determined.

**Assessment for this run:** the framing did not hollow out the answers. Q5's `UNKNOWN` is not
a bare refusal — it enumerates five specific undefined items — and Q3's "no" is independently
verifiable against the material. A merely-led participant would have answered `UNKNOWN` and
stopped. The framing risk is real and should be corrected before any further run, but it does
not account for the substance of these answers.

### Two renderings of one name, with no boundary

The material shows `notes/weird-\xff\xfe.txt` (lowercase, from the harness's echo of the
command line) and `notes/weird-\xFF\xFE.txt` (uppercase, from the tool: `report.rs:476` uses
`format!("\\x{b:02X}")`). One capture, two renderers, no marker.

This run resolved it correctly: it identified the lowercase form as input rather than tool
output, and declined to read a decoding rule from it. It also recorded the case difference in
`UNEXPECTED_FINDINGS` as unexplained, which is accurate — nothing in the material explains it.

### No context leakage found

Nothing in the response indicates knowledge it could not have derived from the material. It
names no finding, no earlier run, no implementation detail, and no expected answer. Its two
references to non-derivable-looking things — that the lowercase form is input, and that
"three file paths are ever named" — are both deductions from the material's structure,
checkable by inspection and both correct.

### The material is answerable

All eight questions were answerable: six fully, and two partially with `UNKNOWN` where the
material genuinely does not determine the answer. No question was unanswerable for a reason
attributable to the prompt's wording alone.

---

## 7. PRODUCT FINDINGS

**This run produced no genuinely new product claim.** Every product-level observation it makes
maps onto something already recorded, either in this experiment's finding namespace or in the
v0.1 record. Recording that honestly is more useful than allocating new identifiers for
restatements.

| Observation | Home | New identifier |
|---|---|---|
| `show` does not name its subject | already allocated in this experiment's namespace | **none** — re-allocating would duplicate the claim |
| The escaped rendering is not self-declared (syntax, reversibility, scope, subject) | already allocated in this experiment's namespace | **none** — same reason |
| Only three of five file entries are ever named; no listing exists | `F-V01-1` (v0.1, accepted) | **none** — already recorded |
| `observed`/`derived` are visible labels whose semantics are undefined in the output | `F-V01-9` (v0.1, outside v0.1) | **none** — already recorded |
| The `path-encoding` line is global and names no path | facet of the escape-declaration gap | **none** — a facet, not a separate claim |

**On identifier allocation, stated plainly so the comparison step is not prejudiced.**
Allocating `F-AI02-n` numbers for claims that already have homes would create two identifiers
for one fact, which this repository forbids. Whether Run 02 constitutes an independent
*reproduction* of those claims is exactly the question the comparison step will answer, and
**this analysis does not answer it**. If the owner prefers run-scoped identifiers as a
convention for the comparison, that is a naming decision for them, and the mapping above gives
them what they would need.

**One refinement worth recording**, which is a sharper statement than "the escape is not
declared": the `path-encoding=escaped` line appears **once**, on line 64, *after* both
`modified` lines (62–63), and names no path. So a reader cannot tell which of the two entries
it applies to — the declaration has no subject. That is a facet of the same gap, not a new
claim, but it is the most precise form of it recorded so far.

---

## 8. RELATION TO EXISTING V0/v0.1 FINDINGS

Cited as project findings. **V0 is not reopened; nothing here contradicts any frozen record.**

- **`F-V01-1` (coverage gap — `status` gives counts, not identities; no listing command).**
  Directly engaged by Q7. The participant's inability to check completeness is the same gap
  seen from a reader's side, and it quantifies it: three of five file entries named. `F-V01-1`
  is recorded as accepted for v0.1 and handed to a later version; this run does not change that
  disposition.
- **`F-V01-9` (the output's vocabulary is defined nowhere in the output).** Engaged by Q2. The
  participant reports inferring the `observed`/`derived` distinction from usage. `F-V01-9`
  already lists this class of gap; no new identifier is warranted.
- **`F-V01-8` (labels alone do not let the rule be reconstructed where values coincide).**
  Adjacent to Q2 but not the same claim; this run did not address the coincident-value case.
- **`F-V01-2` — not engaged, not reopened.** That finding concerned values *mislabeled*
  `observed`; it was corrected and closed. Nothing in this response bears on it.
- **`F-V01-4` / `F-V01-5` — not engaged.** This run produced no crash and no test claim.
- **`F-V01-3`, `F-V01-6`, `F-V01-7`** — not addressed by this run.

---

## 9. A1 / A1-AI STATUS

```
A1     HUMAN-INDEPENDENT   NOT SATISFIED. Unchanged. No human reader has run any protocol.
A1-AI  AI-INDEPENDENT      EVIDENCE RECEIVED — 2 RUNS.
```

Per `UD-015`, this run can surface ambiguities, find defects and unblock technical decisions.
It cannot satisfy A1, and it is not presented as doing so. A1 is not closed, weakened, or
reinterpreted by any number of AI participants.

---

## 10. FOLLOW-UP QUESTIONS

Raised by this run, none answered here:

1. Would the `show`-omission be a problem in interactive use, where the user typed the path?
   This run establishes only that captured `show` output is not self-identifying.
2. Would a reader given the escape contract find it sufficient — and would it need to state
   the subject of the escape, given that the declaration is global?
3. Is there any presentation in which the tool's output and the capture's scaffolding are
   distinguishable without the reader having to judge line shapes?
4. The participant asked, in effect, for an enumeration of all six entries. Would providing
   one exceed v0.2's decided scope? (`F-V01-1` is the recorded home of that question.)
5. Does the conventional reading of `\xNN` mislead readers in a way that a different rendering
   would not? This run declined the convention rather than being misled by it — one data
   point, not an answer.

---

## 11. WHAT THIS RUN ESTABLISHES

- On this material, an independent reader **refuses to decode** `\xFF\xFE` into bytes, and
  justifies the refusal by naming five specific items the material does not define, including
  the original length.
- The participant correctly attributes the lowercase rendering to the command line rather than
  to the tool, and declines to read a decoding rule from it.
- The tool's self-reported completeness is distinguished from what a reader can independently
  establish, and the distinction is grounded in a concrete, checkable gap: three of five file
  entries are ever named.
- `exit=0`'s presence is separated from its meaning, with the external convention declared.
- The `observed`/`derived` labels are visible while their semantics are inferred from usage.
- `show` output does not name its subject.
- The material contains harness scaffolding indistinguishable from tool output, and two
  renderings of one name with no boundary between them.
- The eight questions are answerable from the material, six fully and two partially with
  `UNKNOWN` where the material does not determine the answer.

## 12. WHAT THIS RUN DOES NOT ESTABLISH

- Anything about human legibility, or A1.
- Anything about whether the tool's verdicts are correct, or whether content verification
  works. The participant made no such claim and neither does this analysis.
- That the escaping is ambiguous, irreversible or defective — only that it is not declared in
  the output. The tool's own escape is `\xNN` uppercase with `\\` for a literal backslash,
  unambiguous by construction, and documented in `docs/versions/v0.1.md`; none of that is in
  the material, and the material is all a reader has.
- That any of these observations are reproduced across participants. **No comparison has been
  performed.**
- Anything about ext4, or any filesystem not probed.
- Anything about v0.2, which is not implemented.

---

## 13. RECOMMENDATION

1. **Accept this analysis as the independent Run 02 record**, subject to the owner's review.
2. **Then perform the formal comparison**, as a separate step with its own document. The
   comparison should treat divergence as an object of analysis rather than an average, and
   should check first whether any divergence traces to a known package defect — the leading
   framing, the scaffolding, and the two renderings are all known and all capable of producing
   apparent disagreement that is really about the package.
3. **Do not allocate new identifiers** for the product observations until the comparison
   establishes whether they are reproductions or restatements.
4. **Do not revise the material** until the comparison is complete.
5. **No implementation** is authorized or required by this analysis.

---

## 14. DECISION SUMMARY

```
A1              NOT SATISFIED
A1-AI           EVIDENCE RECEIVED — 2 RUNS
EXP-AI-01       CONTINUE TO COMPARISON
IMPLEMENTATION  NOT AUTHORIZED
PUSH            NOT AUTHORIZED
```

Nothing was implemented; no production code or test was touched; `material-v1` and the prompt
artifact are unchanged; `material-v2` was not built; nothing was committed or pushed. V0
remains frozen; v0.1 remains not declared complete; `Q25` and `D-PEND-2` remain open.
