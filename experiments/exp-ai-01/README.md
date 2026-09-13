# EXP-AI-01 — independent reading of a non-UTF-8 path

Status: **PREPARED, NOT RUN.** No participant has received this. The owner delivers it.

- Mode: **SEMI-BLIND** (objective given, conclusion withheld)
- Material: [`material.txt`](material.txt) — `exp-ai-01/material-v1`, 2669 bytes,
  sha256 `8ad75576edad4d6c5da84d2c396a1b56d91c8902fe7992d7ba19e202155f2678`
- Generator: [`capture_material.py`](capture_material.py)
- Package for delivery: [`prompt-to-participant.md`](prompt-to-participant.md)
- Evidence class this will produce: **AI-INDEPENDENT** (`UD-015`). It cannot satisfy A1.

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

## 6. HOW THE RESULT WILL BE EVALUATED

Per question, against the material, not against consensus:

1. what the participant answered, literally;
2. what they quoted as evidence, and whether it is actually in the material;
3. whether anything in `INFERENCE` was presented as `EVIDENCE`;
4. whether the question itself may have induced the answer (checked first, against §5);
5. whether an `UNKNOWN` reflects insufficient material or insufficient effort — and note
   that `UNKNOWN` is not scored as failure;
6. what the answer establishes about legibility, and what it does not;
7. what remains open.

Divergence between participants, if more than one runs this, is analysed as its own object:
same material and incompatible readings means something is under-determined, and that is a
finding about the surface or about the prompt — not an average to be taken.

No majority rule. A single participant contradicting the expected reading is not thereby
wrong, and two agreeing is not thereby right.

## 7. WHAT A RESULT COULD AND COULD NOT SUPPORT

Could support: that the output is (or is not) readable by a reader with no prior knowledge,
for the specific things asked; discovery of ambiguities; an unblocking input for a technical
decision under `UD-015`'s A1-AI.

Could not support: any claim about human legibility; satisfaction of A1; validation of the
tool's correctness; anything about v0.2, which is not implemented.

## 8. STATE

- No participant has been contacted. Hermes does not contact, execute or select one.
- No response exists. Nothing has been analysed.
- The experiment is prepared and waiting for the owner to deliver it.
