# EVIDENCIA-AI-01 — first delegated reading of a non-UTF-8 path

Status: **RESULT RECEIVED, ANALYSED** (2026-09-13). Analysis only. No implementation, no
commit, no push, no change to A1.

```
evidence class   AI-INDEPENDENT / A1-AI   (UD-015)
experiment       EXP-AI-01
participant      external AI (identity not provided; not recorded rather than guessed)
mode             SEMI-BLIND
material         exp-ai-01/material-v1  (sha256 8ad75576edad4d6c5da84d2c396a1b56d91c8902fe7992d7ba19e202155f2678)
binary           sha256 c481136ab0caee371204f6417fe2f6dc94554a68737ed4ed62f75765fe387089
raw response     response-01-external-ai.txt   (verbatim, unmodified)
```

**This does not satisfy A1.** A1 remains HUMAN-INDEPENDENT and NOT SATISFIED. This run is
AI-INDEPENDENT evidence, auxiliary, per `UD-015`.

**This is not combined with the author's analysis as two independent readers.** Hermes
designed this experiment, wrote its questions knowing the answers, and built its material.
Its role here is analyst, not a second participant.

---

## 1. WHAT THE PARTICIPANT WAS AND WAS NOT GIVEN

Given: the objective; the captured session; eight questions; the response template; the
prohibition on reading source or searching for the project; the statement that `UNKNOWN` is
valid and unpenalised.

Not given: the expected conclusions; the earlier defect in this area; any finding
identifier; which questions are diagnostic; the implementation of the escaping; any other
participant's answer.

Whether the participant had any prior context is **UNKNOWN** — it was not recorded and is
not inferable from the response. The response contains nothing that suggests outside
knowledge of the project, and the participant twice flagged its own reliance on external
convention, which is what a participant working only from the material does. That is
consistent with, not proof of, the intended condition.

---

## 2. VERIFICATION PERFORMED ON THE RESPONSE

Before assessing, every `EVIDENCE` quotation was checked against `material.txt`.

- All quotations are **exact**. No misquote was found.
- Q8's count is correct: the material contains exactly seven `exit=0` lines.
- Q1's counts match the material line exactly.

Two facts about the material were established while checking, and they matter for Q2 and Q8:

- `exit=` and the `$ ` command echo are **not tool output**. The tool never prints either
  (verified: no such string in `umbral/src/`). Both are produced by the capture harness,
  `capture_material.py`.
- Therefore the material presents tool output and harness scaffolding **in the same block
  with no visible boundary**. The participant had no way to tell them apart.

---

## 3. Q1–Q8 ANALYSIS

| Q | Participant answer | Directly supported? | Inference quality | External assumptions | Assessment |
|---|---|---|---|---|---|
| **Q1** | 6 entries: 5 files, 1 dir, 0 symlinks | **Yes** — quoted line is in the material, counts exact | None claimed; correctly reported as stated rather than derived | None | **Correct observation** |
| **Q2** | Three leading words: `observed`, `derived`, `exit=0`; semantics inferred from usage | **Partly** — `observed`/`derived` are tool output; `exit=0` is harness scaffolding | **Strong.** Explicitly marked the semantic split as inferred "not stated in the material" | None for the split; treated a harness line as tool output | **Correct observation + correct inference + material defect** (see P-AI01-2) |
| **Q3** | No, neither `show` output states its path | **Yes** — verified: neither output block contains a path token | Correct and checkable ("completeness can be checked by inspection") | None | **Correct observation.** Genuine legibility finding (F-AI01-1) |
| **Q4** | Not the same form; one plain, one `\x`-escaped with uppercase hex | **Yes** — both lines quoted exactly | **Exemplary precision**: claimed high confidence only for the character difference, and deferred "escaped" to the Q6 line rather than smuggling it in | None | **Correct observation** |
| **Q5** | `UNKNOWN` for exact bytes; named the missing specification | **Yes** — the two quoted lines are the only relevant ones | **The strongest result.** Refused to decode, named the missing spec, and flagged the external convention as an assumption rather than using it | Explicitly identified and set aside | **Appropriately UNKNOWN** — see §4 |
| **Q6** | Yes; the line is an explanation, with stated limits | **Yes** — line quoted exactly | Correct. The four things it leaves open are all genuinely open in the line | None | **Correct observation + correct inference** |
| **Q7** | The tool claims completeness; an independent reader cannot verify reading succeeded | **Yes** — all quoted lines present | **Strong and central.** Distinguished the tool's assertion from a reader's ability to establish it | None | **Correct inference.** See §5 |
| **Q8** | No failure; seven `exit=0` | **Yes** — seven occurrences, verified | **Exemplary separation**: "high that no non-zero status appears; medium that this means success, because that relies on external convention" | Identified explicitly (POSIX convention) and declared | **Correct observation + correct separation + material defect** (P-AI01-2) |

### What was observed vs inferred vs refused

- **Literally observed:** the counts (Q1); the set of leading tokens (Q2); the absence of a
  path token in the `show` output (Q3); the two rendering forms (Q4); the encoding line's
  existence and wording (Q6); the seven exit values (Q8).
- **Inferred:** the functional split between `observed` and `derived` (Q2); that one form is
  an encoding (Q4, explicitly deferred to Q6); what the encoding line leaves open (Q6); that
  the tool's completeness is an assertion rather than an established fact (Q7).
- **Required external convention, and the participant said so:** that `exit=0` means success
  (Q8); that `\xFF` might denote a byte (Q5, raised and then set aside).
- **Correctly refused:** the byte sequence of the escaped name (Q5); any absolute claim that
  reading succeeded (Q7); any claim about what the escape syntax means (Q5, Q6).

No answer was scored against a stored key. Each was assessed on whether the material
supports it and on whether the participant separated what it observed from what it inferred.

---

## 4. THE MOST IMPORTANT POSITIVE SIGNAL — Q5

The value of Q5 is not that the answer is `UNKNOWN`. A participant can reach `UNKNOWN` by
guessing that the question is a trap, or by finding the material confusing. Neither would
indicate anything.

What matters is **what the participant said was missing**, and it named exactly the right
things:

> "the material does not define the escape syntax, its scope, whether other characters are
> literal, case significance … or how to invert it. Therefore actual byte sequence cannot be
> determined from output alone."

It also handled the temptation explicitly rather than silently:

> "If external convention `\xFF` = byte 0xFF is assumed, it would suggest bytes 0xFF 0xFE at
> that position, but the material does not define …"

That is the correct move: an external convention was available, would have produced a
plausible byte sequence, and the participant surfaced it as an assumption instead of
reporting it as a finding.

**Positive signal recorded:** the experiment can detect epistemic overreach, and this
participant did not commit it. Concretely — it refused a byte-level claim, and it justified
the refusal by naming the missing specification rather than by expressing discomfort.

**What this is not.** It is not evidence that the escaping is wrong, ambiguous by accident,
or unrecoverable. It is evidence that the escaping **is not declared in the output**, so a
reader with no prior knowledge cannot recover it. Those are different claims, and only the
second is supported. The tool's escape is `\xNN` uppercase with `\\` for a literal
backslash, chosen so it is unambiguous, and it is documented in `docs/versions/v0.1.md` —
but that documentation is not in the material, and the material is all a reader has.

---

## 5. Q7 — THE DISTINCTION IS VALID

The participant wrote:

> "complete, verified, observable are tool-defined terms with no definition in the material,
> so they support only that the tool asserts success, not that reading actually succeeded."

This is a **valid epistemic distinction**, and it is the correct reading of the material.

It is three things at once, and they should be recorded separately:

1. **Valid epistemic distinction (primary).** "The tool reports `complete=true`" and "the
   files were read" are different statements. The first is observable in the output; the
   second is not established by the output. The participant is right, and this is exactly the
   distinction the experiment was built to test.
2. **Experiment/material limitation (secondary).** The material offers no independent
   cross-check — no directory listing from another tool, no second source. So the question
   "was everything really read?" is unanswerable *from this material* by construction. That
   is a limit of the experiment, not a defect of the tool.
3. **Output-contract observation (tertiary).** `complete`, `verified`, `observable` and
   `unstable-or-unreadable` are tool-defined terms with no in-output definition. This is the
   same class as `F-V01-9`, and it is not new.

**Explicitly not claimed:** that Umbral's content verification is incorrect, insufficient or
misleading. The participant did not claim that, and this analysis does not either. What is
established is only that the output's own assertions are not independently checkable from
the output alone — which is true of any tool reporting on its own work, and is the reason
the label contract exists in the first place.

---

## 6. Q3 AS A LEGIBILITY FINDING — BOTH, RECORDED SEPARATELY

The mandate's distinction applies cleanly, and **both** sides are real:

**(a) Product side — genuine.** `umbral show` emits `derived run=1 hash=… stability=…` and
`observed kind=… size=… mtime=…` and **never names the path it is reporting about**. This is
inconsistent with the tool's own sibling command: `changes` emits `path=notes/three.txt` on
every mutation line. So within one tool, one command identifies its subject and another does
not. That is a legibility observation about the product, not about the protocol, and it
matters whenever output is captured, piped, concatenated or read out of context — which is
precisely the situation this experiment creates.

**(b) Protocol side — also real, and separate.** Q3 asked "does that output state which path
it is reporting about?" with "the complete output of each command" as the frame. A reader
interpreting "output" as the tool's own stdout answers "no" correctly; a reader including the
invocation line answers "yes". The protocol did not settle which reading it wanted. That is a
protocol ambiguity, and it is recorded as one rather than folded into the product finding.

Note the participant did not confuse them: it scoped its answer to the tool output, then in
`UNEXPECTED_FINDINGS` restated the point as a property of the two `show` outputs. It got the
boundary right.

---

## 7. THE ESCAPE CONTRACT — WHAT IS AND IS NOT ESTABLISHED

`path-encoding=escaped  reason=path-is-not-valid-utf8` establishes, and only establishes:

- that **some** path in this command's output was rendered in an escaped form;
- that the reason is that a path is not valid UTF-8;
- that the output explicitly marks the rendering as non-literal.

It does **not** establish:

- **which** path it applies to — the line names no path;
- the escape syntax: what `\xNN` means, whether `NN` is hex or decimal, case significance;
- whether the escape is reversible, or how to invert it;
- whether non-escaped characters in the same string are literal;
- the scope: this command only, or the whole output.

The mandate's warnings are all confirmed by this: `\xNN` semantics are **not** self-evident
from the material; uppercase/lowercase equivalence is **not** established by any contract
visible to the reader; and reversibility is **not** implied by the notation's familiarity.
The tool's implementation happens to be unambiguous and reversible, but the output does not
say so, and a reader may not assume it.

Classification: **output-contract gap (primary)** and **product legibility debt** (same class
as `F-V01-8`/`F-V01-9`). It is **not** a repository documentation gap: the escape is recorded
in `docs/versions/v0.1.md`. The gap is between what the tool prints and what a reader of that
output can reconstruct.

---

## 8. THE CASE DIFFERENCE — RESOLVED, AND NOT A PRODUCT BUG

The participant observed `notes/weird-\xff\xfe.txt` (lowercase) in the command echo and
`notes/weird-\xFF\xFE.txt` (uppercase) in the `changes` output, and correctly reported only
that "the material does not explain case change" without calling it a bug.

**Origin established by inspection, not inference:**

| Where | Renderer | Case |
|---|---|---|
| `$ umbral show … notes/weird-\xff\xfe.txt` | the capture harness: Python `bytes.decode("utf-8","backslashreplace")` | **lowercase** |
| `derived modified path=notes/weird-\xFF\xFE.txt` | the tool: `format!("\\x{b:02X}")` in `report.rs` | **uppercase** |

So the difference is between **two different renderers appearing in one capture** — the
harness's echo of the command line, and the tool's own output. It is not a tool
inconsistency, not a normalisation difference inside the tool, and **not a bug**. The tool is
internally consistent: every escaped path it prints uses uppercase `\xNN`.

The participant could not have known this, because the material does not mark which lines are
the tool's and which are the harness's. That is a defect of the material (P-AI01-2), and it is
the same defect that shaped Q2 and Q8.

**Falsifiable follow-up if it is ever worth confirming:** run the tool twice over a
non-UTF-8 fixture, once through a harness that echoes the command and once without any echo,
and check that every tool-produced escaped path is uppercase in both. Expected result: no
difference attributable to the tool. This is a low-value confirmation; it is recorded as
available, not as recommended.

---

## 9. Q2 AGAINST THE EXISTING LABEL CONTRACT

The participant says the semantic distinction between `observed` and `derived` is "inferred
from pattern of contents, not stated in the material", with medium confidence for that reason.

**This confirms existing findings; it does not reveal a new one.**

- **`F-V01-9` (vocabulary undefined in the output) — strengthened.** That finding lists
  `stability`, `unstable-or-unreadable`, `scan-complete`, `object-survives`, `recreated`,
  `unobserved`, `other`. A second, independent reader of a different evidence class now
  reports that it had to infer `observed`/`derived` themselves from usage. The class of the
  gap is confirmed and slightly widened, but the finding already covers it.
- **`F-V01-8` (labels alone do not let the rule be reconstructed) — strengthened.**
  `F-V01-8` says that where an observed and a derived value coincide, the labels do not let
  the rule be reconstructed. The participant reconstructed the rule from *usage pattern*,
  which is the same limitation seen from the reader's side.
- **`F-V01-2` — not reopened.** That finding was about values *mislabeled* `observed`; it was
  corrected and is closed. The participant's observation is about the labels not being
  *defined*, which is `F-V01-8`/`F-V01-9`, a different and already-recorded matter. Nothing in
  this response contradicts the `F-V01-2` disposition.

V0 is not reopened. Nothing here bears on V0's frozen record.

---

## 10. Q8 AND THE `exit=` ARTIFACT

The separation the participant made is exactly right, and it is worth stating what each half
rests on:

- **"Seven `exit=0` values are present"** — directly observable, verified, and the count is
  right.
- **"`exit=0` means successful termination"** — requires either an explicit contract or the
  POSIX convention. The material supplies no contract, so this rests on external convention,
  and the participant said so, downgrading its confidence accordingly.

**The experiment did not intend this distinction, and it should not have been necessary.**
`exit=` is not the tool's output; it is the harness's. By printing it in the same block as
the tool's lines, the material invited the reader to treat harness scaffolding as tool
behaviour. The participant's handling was appropriate given what it was given — it neither
overclaimed nor refused a question it could partly answer.

Had the material marked the boundary, Q8 would have been a cleaner test of whether a reader
can find a failure signal in the tool's own output. As it stands, Q8 partly measures the
harness. Recorded as P-AI01-2.

---

## 11. LEADING-QUESTION AND LEAKAGE REVIEW

The prompt and material were re-read against the response.

| Q | Leading risk | Assessment |
|---|---|---|
| Q1 | None | Neutral. Asks for numbers and their source. |
| Q2 | Low | "describe what kind of statement the lines … appear to make" invites inference but does not point at an answer. |
| **Q3** | **Mild** | "Does that output state which path it is reporting about? Answer yes or no" is a yes/no question about an absence, which invites "no". A neutral form: "What does that output state about which path it concerns?" |
| Q4 | Low | Presupposes a difference exists, but Q4 is where the difference becomes observable, so the presupposition is licensed by the material. |
| **Q5** | **Mild-to-moderate** | "what can you **not** determine" presupposes that something cannot be determined, which invites `UNKNOWN`. A neutral form would ask what can be determined and leave the negative open. |
| Q6 | Low-to-mild | "a name not being written literally" presupposes the escaped form, already established by Q4. |
| Q7 | **None** | Well constructed: "quote whatever supports **or fails to support** that conclusion" explicitly licenses both directions. |
| Q8 | None | Factual, and the participant's uncertainty came from the material, not the question. |

**Recorded as an experiment defect (P-AI01-3), with two qualifications:**

1. **The participant is not blamed for following a leading frame.** Where a question is
   leading, the defect is the question's.
2. **The leading framing did not hollow out the answers.** Q5's `UNKNOWN` is not a bare
   refusal: the participant named the missing specification and set aside the external
   convention explicitly. A merely-led participant would have answered `UNKNOWN` and stopped.
   So the leading risk is real but did not manufacture the positive signal in §4.

No evidence of leakage was found. Nothing in the prompt or material disclosed the earlier
defect, the finding identifiers, the expected answers, or the implementation. The objective
was stated at a level that does not name the non-UTF-8 case; the case is discoverable only by
reading the material, which is the intent of a semi-blind design.

---

## 12. NEW FINDINGS PROPOSED

Namespace: **`F-AI01-n`** for findings about the product surfaced by EXP-AI-01, **`P-AI01-n`**
for defects of the experiment itself. A new namespace is used rather than extending
`F-V01-n`/`P-V01-n`, because those are tied to the v0.1 reader-protocol record, which is
frozen; new numbers there would misattribute the source. Where a finding only strengthens an
existing one, no new identifier is created (§9).

### F-AI01-1 — `show` does not identify its subject

- **Claim:** `umbral show` reports observations without naming the path they belong to, while
  `umbral changes` names `path=` on every mutation line. A reader of captured `show` output
  cannot tell which entry it describes.
- **Evidence:** `material.txt` lines 33–34 and 38–39 contain no path token; lines 111–112
  contain `path=`. Verified by inspection of the material and of `report.rs::show`.
- **Classification:** product legibility debt (same class as `F-V01-8`/`F-V01-9`).
- **Severity:** **significant** for machine-read or captured output; minor in an interactive
  session where the user typed the path.
- **Requires implementation:** no — not authorized, and outside v0.2's decided scope.
- **Independently testable:** yes. Any reader given only a captured `show` block, with the
  invocation line removed, cannot name the subject. That is a reproducible test.

### F-AI01-2 — the escaped-path rendering is not declared in the output

- **Claim:** the output states *that* a path was escaped and *why*, but not the escape
  syntax, its reversibility, its scope, or which path it applies to; a reader with no prior
  knowledge cannot recover the bytes.
- **Evidence:** `path-encoding=escaped  reason=path-is-not-valid-utf8` names no path and
  defines no syntax; the participant's Q5 and Q6, with the missing specification named.
- **Classification:** output-contract gap, primary; product legibility debt, secondary. **Not**
  a repository documentation gap — `docs/versions/v0.1.md` records the escape.
- **Severity:** **significant**, because v0.2 will add new field values and a new `basis`
  field, and `UD-017` requires `basis` to be traceable to the evidence that produced it. The
  same question — is the value self-describing to a reader who has only the output? — applies
  directly.
- **Requires implementation:** no. **But it bears on the v0.2 contract design** (§14).
- **Independently testable:** yes, and this run is an instance of it.

### P-AI01-1 — the material contains two different renderers with no visible boundary

- **Claim:** the capture echoes the command line using the harness's own escaping (lowercase
  `\xff\xfe`) while the tool prints uppercase `\xNN`, so one capture contains two renderings
  of the same name and the difference is unmarked.
- **Evidence:** harness echo line in `capture_material.py` uses
  `bytes.decode("utf-8","backslashreplace")`; `report.rs:476` uses `format!("\\x{b:02X}")`.
  Confirmed by inspection.
- **Classification:** experiment/material defect. **Not** a product bug — the tool is
  internally consistent.
- **Severity:** minor for the product; significant for the experiment, because it produced a
  spurious `UNEXPECTED_FINDING` and consumed participant attention.
- **Requires implementation:** no.
- **Independently testable:** yes — regenerate the material and confirm the difference is
  attributable to the harness, not the tool.

### P-AI01-2 — harness scaffolding is not distinguished from tool output

- **Claim:** the `$ ` command echo and `exit=N` lines are produced by the capture harness,
  not by the tool, and the material presents them in the same block as tool output with no
  boundary. Two questions (Q2, Q8) therefore partly measure the harness.
- **Evidence:** `exit=` and `$ umbral` appear nowhere in `umbral/src/`; both are written by
  `capture_material.py`. The participant counted `exit=0` among the tool's "leading words".
- **Classification:** experiment/material defect. This is the **most consequential** defect
  found, because it silently changed what two questions were measuring.
- **Severity:** **significant** for the experiment.
- **Requires implementation:** no.
- **Independently testable:** yes — the defect is visible in the material on inspection.

### P-AI01-3 — two questions carry leading framing

- **Claim:** Q3 ("does that output state which path…? Answer yes or no") and Q5 ("what can you
  **not** determine") presuppose the negative answer they invite.
- **Evidence:** the question texts, against the response's use of `UNKNOWN` on Q5.
- **Classification:** experiment/protocol defect. Mild. The participant's answer was
  independently justified and is not discounted.
- **Severity:** minor, but it must be corrected before the next round so a later result is
  not partly an artifact of the question.
- **Requires implementation:** no — a revision of the delegation package, if the round
  continues.
- **Independently testable:** no, by construction; it is a design judgement.

---

## 13. A1 / A1-AI STATUS

```
A1     HUMAN-INDEPENDENT   NOT SATISFIED. Unchanged. No human reader has run any protocol.
A1-AI  AI-INDEPENDENT      EVIDENCE RECEIVED. One run, EXP-AI-01, one participant.
```

Per `UD-015`: this run can surface ambiguities, find defects and unblock technical
decisions. It cannot satisfy A1, and it is not presented as doing so. EVIDENCIA-B remains a
separate AI-INDEPENDENT run from the v0.1 reader protocol; the two are **not** merged into a
body of "two independent AI readers" — they used different material, different questions and
different protocols, and combining them would manufacture agreement that was never tested.

---

## 14. WHAT THIS UNLOCKS, AND WHAT IT DOES NOT

**Establishes:**
- The path escaping is not self-describing to a reader with only the output.
- `show` does not identify its subject, inconsistently with `changes`.
- The label vocabulary is reconstructed from usage, not read from a definition.
- The output's self-reported completeness is not independently checkable from the output.
- The experiment can detect epistemic overreach, and this participant did not commit it.

**Does not establish:**
- Anything about human legibility, or A1.
- That the tool's content verification is wrong or that any verdict is incorrect.
- That the escaping is ambiguous, irreversible or defective — only that it is undeclared.
- That any v0.2 mechanism is needed, correct, or sufficient.
- Anything about ext4 or any unprobed filesystem.

**Strengthens:** `F-V01-9`, `F-V01-8` (independently reproduced by a different evidence
class). Does **not** reopen `F-V01-2`, `F-V01-4` or `F-V01-5`, all of which are closed with
dispositions the response does not contradict.

**Suggests:** `F-AI01-1`, `F-AI01-2`; and for the experiment, `P-AI01-1`, `P-AI01-2`,
`P-AI01-3`.

**Remains UNKNOWN:** whether a reader given the escape contract would find it sufficient;
whether the `show` omission matters to a human in interactive use; whether the participant
had any prior context; whether a second participant reproduces any of this.

**Technical work unlocked without claiming A1 — yes, and narrowly:**
- The `basis` contract design under `UD-017` now has a concrete precedent to satisfy: a new
  field value must be readable by someone who has only the output. `F-AI01-2` is the shape of
  the failure to avoid. This is contract and documentation investigation, not implementation.
- `F-AI01-1` is a candidate for the deferred legibility question (the same question that
  holds `F-V01-8`/`F-V01-9`), not for v0.2's decided scope.
- **No implementation is authorized by this analysis**, and none is required by it.

---

## 15. RECOMMENDATION

**Continue EXP-AI-01 with a second independent participant, on `material-v1` unchanged.**

The reasoning, stated with its cost:

- A replication is worth more than a revision right now, because the single most useful
  question is whether an independent reader reproduces the same *observation/inference
  behaviour* — specifically whether it also refuses to decode `\xFF\xFE` and names the
  missing specification. A revised material would answer a different question.
- The known defects are already identified and are visible on inspection, so a second run on
  the same material lets us check whether a different participant *also* trips on them. If it
  does, the defect is reliably detectable and that is itself a result. If it does not, the
  first participant's detection was sharper than average — also a result.
- **The cost is honest and must be accepted:** a second participant will spend effort on the
  `exit=` artifact. That is a real cost and the reason to revise eventually.

**After that run, revise the package to `material-v2`** with the boundary between tool output
and harness scaffolding marked, the command echo regenerated with the tool's own escaping, and
Q3 and Q5 reworded neutrally. Then, and only then, compare across material versions — which
will require treating the v1 and v2 results as separate comparisons, not one dataset.

**Do not** merge this result with any other, do not average across participants, and do not
treat a second agreement as confirmation.

---

## 16. DECISION SUMMARY

```
A1              NOT SATISFIED
A1-AI           EVIDENCE RECEIVED  (1 run, EXP-AI-01, AI-INDEPENDENT)
EXP-AI-01       CONTINUE           (second participant, material-v1 unchanged)
IMPLEMENTATION  NOT AUTHORIZED
PUSH            NOT AUTHORIZED
```

Nothing was implemented, no production code or test was touched, nothing was committed and
nothing was pushed. V0 remains frozen; v0.1 remains not declared complete; `Q25` remains
open; `D-PEND-2` (whether `ctime` enters the skip condition) remains open.

**Recorded, not acted on:** `F-AI01-1`, `F-AI01-2`, `P-AI01-1`, `P-AI01-2`, `P-AI01-3`.
