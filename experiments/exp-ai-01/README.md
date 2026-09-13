# EXP-AI-01 — independent reading of a non-UTF-8 path

Status: **CLOSED** (2026-09-13) — `PASS WITH DOCUMENTED INSTRUMENT DEFECT`. Two independent
runs executed and compared; see §8 and [`COMPARISON-AI-01-AI-02.md`](COMPARISON-AI-01-AI-02.md).
`material-v1` is frozen as this experiment's historical record.

- Mode: **SEMI-BLIND** (objective given, conclusion withheld)
- Material: [`material.txt`](material.txt) — `exp-ai-01/material-v1`, 2669 bytes,
  sha256 `8ad75576edad4d6c5da84d2c396a1b56d91c8902fe7992d7ba19e202155f2678`
- Generator: [`capture_material.py`](capture_material.py)
- Package for delivery: [`prompt-to-participant.md`](prompt-to-participant.md)
- Run 02 plan (pre-registered): [`plan-run-02.md`](plan-run-02.md)
- Evidence class produced: **AI-INDEPENDENT** (`UD-015`). It cannot satisfy A1.

## 1. WHY THIS ONE FIRST

Chosen as the first delegated experiment because it is cheap, self-contained and its verdict
is checkable against raw output rather than against anyone's opinion. It also has a
precedent of being useful: the isolated-context run registered as EVIDENCIA-B found a real
defect in exactly this area, which neither the author nor the owner had found in their own
passes.

## 2. WHAT IT ASKS

Whether a reader with no prior knowledge of the tool, the contract or the implementation can
tell — from the captured output alone — what the tool reports for a directory containing an
entry whose name is not valid UTF-8.

Concretely, the questions probe: the entry accounting; the meaning of the leading words on
each line; whether a single-path command states which path it is reporting about; how a
non-literal path is written and what that does and does not reveal about the actual bytes;
whether the output explains the non-literal form; whether the output supports a claim that
everything was read; and whether anything ended abnormally.

The material contains a genuine case of each thing being asked about. Nothing in it is
hypothetical.

## 3. WHY SEMI-BLIND AND NOT BLIND

The task is not meaningful without knowing that it is about what the output reports, and the
material itself names the tool in every command line. A BLIND framing would have to strip the
commands, which would remove the very thing under examination (the escaped path appears in a
command echo and in the `changes` output). Semi-blind is the honest choice here, and it is
the default for this reason.

ADVERSARIAL was rejected: there is no asserted property to falsify. This measures legibility.

## 4. WHAT WAS WITHHELD, AND WHY

Not disclosed to the participant: the earlier defect, its finding identifiers, the expected
answers, which questions are the diagnostic ones, the implementation of path rendering, and
any previous participant's answer.

The reason is in the method, not in secrecy for its own sake: a participant who knows what
the defect was will look for that defect and will report having found or not found it, which
is a different measurement from "what does this output say".

## 5. PROVENANCE AND ITS LIMITS — READ THIS

- The **material** was produced by Hermes (AGENT-INTERNAL) using the v0.1 binary at
  sha256 `c481136ab0caee371204f6417fe2f6dc94554a68737ed4ed62f75765fe387089`. It is a
  verbatim capture, reproducible with `capture_material.py`; it was not edited by hand.
- The state directory was redirected via `XDG_DATA_HOME` so the capture contains no
  home-directory path, and the generator **fails loudly** if one appears. Verified: the
  material contains no personal path.
- The **questions** were written by Hermes, who knows the answers. This is the weakest point
  of the package. A designer who knows what the output contains can accidentally ask in a way
  that points at it. Mitigations, such as they are: the questions ask what the output shows
  rather than whether the tool behaved correctly; the two questions about the non-literal
  path are phrased so that "the output does not determine this" is a fully valid answer; and
  the objective is stated at a level that does not name the non-UTF-8 case.
  **A leading question that survives into the delivered prompt would be a defect of this
  experiment, not of the tool.** If a participant's answer suggests a question pushed them
  somewhere, that goes in the analysis.
- The material was produced *after* the v0.1 defect in this area was fixed. So this is not a
  reproduction of that defect: it is a check of whether the current output is legible. That
  distinction is recorded here so a later reader does not mistake a clean result for evidence
  about the old defect.

## 6. HOW RESULTS ARE EVALUATED

Per question, against the material, not against consensus:

1. what the participant answered, literally;
2. what they quoted as evidence, and whether it is actually in the material;
3. whether anything in `INFERENCE` was presented as `EVIDENCE`;
4. whether the question itself may have induced the answer (checked first, against §5);
5. whether an `UNKNOWN` reflects insufficient material or insufficient effort — and note
   that `UNKNOWN` is not scored as failure;
6. what the answer establishes about legibility, and what it does not;
7. what remains open.

Divergence between participants is analysed as its own object: same material and incompatible
readings means something is under-determined, and that is a finding about the surface or about
the prompt — not an average to be taken. Applied to the two runs, it produced exactly one
divergence, on Q2, which traced to this package's material rather than to the tool.

No majority rule. A single participant contradicting the expected reading is not thereby
wrong, and two agreeing is not thereby right.

## 7. WHAT A RESULT COULD AND COULD NOT SUPPORT

Supported: that the output is readable, by a reader with no prior knowledge, for the specific
things asked; the discovery of ambiguities; an input to a technical decision under `UD-015`'s
A1-AI.

Not supported: any claim about human legibility; satisfaction of A1; validation of the tool's
correctness; anything about v0.2, which is not implemented.

## 8. STATE — CLOSED

**EXP-AI-01 is closed: `PASS WITH DOCUMENTED INSTRUMENT DEFECT`.**

- No participant has been contacted by Hermes. Hermes does not contact, execute or select one.
  The owner delivered each run personally (**author-attested delivery condition**, §11 of the
  comparison — an attestation, not an independent technical verification).
- **Two responses received and analysed**, each with its raw text preserved verbatim:
  - Run 01 → [`EVIDENCE-AI-01.md`](EVIDENCE-AI-01.md) · [`response-01-external-ai.txt`](response-01-external-ai.txt)
  - Run 02 → [`EVIDENCE-AI-02.md`](EVIDENCE-AI-02.md) · [`response-02-external-ai.txt`](response-02-external-ai.txt)
- **Comparison:** [`COMPARISON-AI-01-AI-02.md`](COMPARISON-AI-01-AI-02.md) — 7 of 8 questions
  reproduced, 1 partially reproduced, 0 divergent. Classification `AI-CROSS-CHECK`; the
  comparator is **not blind** and says so.
- **Result:** two independent readers converged on the central epistemic signals — refusing to
  decode `\xFF\xFE` while naming the missing decoding specification (Q5), and separating the
  tool's assertion of completeness from what a reader can independently establish (Q7). The one
  divergence (Q2) traces to this package's own material, not to the participants and not to the
  tool: tool output and harness scaffolding were presented in the same captured block.
- **Findings, none acted on:** `F-AI01-1`, `F-AI01-2` (product legibility, already allocated in
  this namespace); `P-AI01-1`, `P-AI01-2`, `P-AI01-3` (defects of this package, the author's).
- **Classified AI-INDEPENDENT / A1-AI.** It does not satisfy A1 and is not presented as doing
  so. See `UD-015`.
- **`material-v1` is frozen** as the historical record of this experiment. The unspecified
  escape was part of the observed phenomenon and must not be corrected.
- **Nothing implemented, and no push.** Technical follow-ups are `CANDIDATE TECHNICAL
  FOLLOW-UP` (comparison §13), not authorized work.
- **Next step is a project decision**: which of these results should feed v0.2. Implementation
  requires separate, explicit authorization.

## 9. RESULTS OF RECORD

Summarised from the comparison (§10a, §14); the comparison holds the detail.

Established by **both** runs: the entry accounting; `show` does not name its subject; the two
paths are written in different forms; the bytes of `\xFF\xFE` cannot be reconstructed from the
material, because no decoding specification is given; `path-encoding=escaped` does not by
itself determine a reversible encoding; the completeness assertion is not independently
checkable from the material; seven `exit=0` values are present and their meaning rests on an
external convention; `observed`/`derived` are visible labels with inferred semantics.

Established by **one** run only: that the escape declaration is global and names no path; that
only 3 of 5 file entries are ever named; the original-length omission; the attribution of the
lowercase rendering to the command line.

Divergent: whether `exit=0` belongs to the output's leading vocabulary — caused by this
package's unmarked harness scaffolding.

**Not established:** A1; that the tool is correct; the independence of delivery; that the
escaping is defective in implementation; that `show` must change. `Q25` and `D-PEND-2` remain
open.
