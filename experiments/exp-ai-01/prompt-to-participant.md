# EXP-AI-01 — delegation package

**Copy-paste artifact.** Andrés delivers this personally. Hermes does not contact, execute
or select a participant.

- Experiment id: `EXP-AI-01`
- Mode: **SEMI-BLIND** — the participant receives the objective, never our conclusion
- Material version: `exp-ai-01/material-v1`
  - `material.txt`, 2669 bytes, sha256 `8ad75576edad4d6c5da84d2c396a1b56d91c8902fe7992d7ba19e202155f2678`
  - produced by `capture_material.py` from binary sha256 `c481136ab0caee371204f6417fe2f6dc94554a68737ed4ed62f75765fe387089`

---

## A. WHAT ANDRÉS DOES (do not send this section)

1. Send the participant **everything between the two rules in §B**, unchanged. Nothing
   else. Do not add a preamble, a hint, or an explanation of why you are asking.
2. If the participant has no way to run programs, that is fine — the material is a captured
   transcript and the task is to read it. Do not offer to run anything for them.
3. Do not answer questions about what the tool is, what it is supposed to do, or what the
   output "should" say. If asked, say the answer must come from the material.
4. Do not send any other participant's answers, and do not show this package to anyone else
   before they have answered.
5. When the reply arrives, **save it verbatim** — see §C. Do not clean, summarise, translate
   or reformat it. If the participant's formatting is broken, that is part of the data.
6. Hand the raw reply back. It will be analysed against the material, not by majority vote.

---

## B. THE PROMPT TO SEND

────────────────────────────────── SEND FROM HERE ──────────────────────────────────

I would like an independent reading of a captured command-line session. You will be given
the complete output of a session; the task is to say what that output shows.

Please work **only from the material below**. It is self-contained: everything you need is
in it.

**OBJECTIVE**

Determine, from the captured output alone, what this tool reports about the directory it was
pointed at: which entries it accounts for, what it says about each, and whether its output
lets a reader identify and follow every entry it mentions.

**MATERIAL — a complete captured session, verbatim**

```
$ umbral init /tmp/umbral-check-01/subject
observed  canonical=/tmp/umbral-check-01/subject
derived   root=/tmp/umbral-check-01/subject
derived   workspace-id=708961ca8621766f
derived   state-dir=/tmp/umbral-check-01/data/umbral/ws-708961ca8621766f
derived   initialised=true
exit=0

$ umbral observe /tmp/umbral-check-01/subject
observed  canonical=/tmp/umbral-check-01/subject
derived   run=1  root=/tmp/umbral-check-01/subject
derived   run=1  entries=6
derived   run=1  content-verified=5
derived   run=1  content-not-verified=0  reason=unstable-or-unreadable
derived   run=1  started=2026-09-13T02:33:26.064Z  finished=2026-09-13T02:33:26.064Z
derived   run=1  complete=true
exit=0

$ umbral status /tmp/umbral-check-01/subject
observed  canonical=/tmp/umbral-check-01/subject
derived   root=/tmp/umbral-check-01/subject
derived   workspace-id=708961ca8621766f
derived   last-run=1  started=2026-09-13T02:33:26.064Z  entries=6
derived   last-run=1  complete=true
derived   entries=6  files=5  dirs=1  symlinks=0  other=0
derived   content-verified=5
derived   content-not-verified=0  reason=unstable-or-unreadable
derived   content-verification-not-applicable=1  reason=not-a-regular-file
derived   unobservable-paths=0  reason=not-observed-at-observation-time
derived   log-runs=1  log-observations=6
exit=0

$ umbral show /tmp/umbral-check-01/subject notes/one.txt
derived   run=1  hash=b6748fa64b14  stability=stable
observed  kind=file  size=10  mtime=2026-09-13T02:33:26.061Z
exit=0

$ umbral show /tmp/umbral-check-01/subject notes/weird-\xff\xfe.txt
derived   run=1  hash=ba73f69e9b28  stability=stable
observed  kind=file  size=6  mtime=2026-09-13T02:33:26.061Z
exit=0

$ umbral observe /tmp/umbral-check-01/subject
observed  canonical=/tmp/umbral-check-01/subject
derived   run=2  root=/tmp/umbral-check-01/subject
derived   run=2  entries=6
derived   run=2  content-verified=5
derived   run=2  content-not-verified=0  reason=unstable-or-unreadable
derived   run=2  started=2026-09-13T02:33:26.093Z  finished=2026-09-13T02:33:26.093Z
derived   run=2  complete=true
exit=0

$ umbral changes /tmp/umbral-check-01/subject
derived   compared  from-run=1  to-run=2  complete=true
derived   count  unchanged=4
derived   count  modified=2
derived   count  created=0
derived   count  deleted=0
derived   count  unobserved=0
derived   count  renamed-or-moved=0
derived   count  recreated=0
derived   count  ambiguous=0
derived   modified  path=notes/three.txt  content-changed=true  scan-complete=true
derived   modified  path=notes/weird-\xFF\xFE.txt  content-changed=true  scan-complete=true
derived   path-encoding=escaped  reason=path-is-not-valid-utf8
exit=0
```

**QUESTIONS**

Q1. How many entries does the tool account for, and how many are files, directories and
symlinks? Cite the line or lines that give each number.

Q2. Most output lines begin with a single word before the rest of the line. List the
distinct such words that appear, and for each one describe what kind of statement the lines
it introduces appear to make. Base this only on the material.

Q3. Two commands ask the tool about one specific path each. Quote the complete output of
each of those two commands. Does that output state which path it is reporting about? Answer
yes or no for each, and quote what supports your answer.

Q4. In the final command, quote every path the output reports, exactly as written, character
for character. Is each one written in the same form as the others? If not, describe the
difference precisely.

Q5. For any path in Q4 that is not written in the same form as the others: from the output
alone, what can you determine about the actual bytes of that entry's name, and what can you
not determine? Be specific about the boundary.

Q6. Does the output anywhere offer an explanation for a name not being written literally?
Quote the line if so, and say what the explanation tells you and what it leaves open.

Q7. From the output alone, can you determine whether every entry in the directory was
successfully read? Quote whatever supports or fails to support that conclusion.

Q8. Did any command fail or end abnormally? Quote the relevant exit statuses and say what
they indicate about how each command ended.

**WHAT YOU MAY DO**

- Read the material above as many times as you like.
- Reason about it, and about command-line tools in general, from your own knowledge.

**WHAT YOU MAY NOT DO**

- Do not inspect the source code of this tool, and do not ask for it.
- Do not search for this tool or its project on the internet.
- Do not assume behaviour the material does not show. If you believe something is true of
  the tool but the material does not demonstrate it, say that it is not demonstrated.
- Do not run anything, and do not ask to run anything. The material is complete as given.

**HOW TO ANSWER**

For each question, give four separate fields:

```
Qn
ANSWER:      your answer, as briefly as it can be stated accurately
EVIDENCE:    the exact fragment(s) of the material that support it, quoted
INFERENCE:   anything you concluded that goes beyond what the material literally shows
CONFIDENCE:  high / medium / low, and the reason for that level
```

Keep `EVIDENCE` and `INFERENCE` genuinely separate. If your answer rests on reasoning rather
than on a quoted fragment, it belongs in `INFERENCE`, and that is a legitimate answer — it
is simply a different kind of statement, and the distinction matters more than the answer.

`UNKNOWN` is a valid and welcome answer wherever the material does not determine something.
There is no penalty for it. If you write an answer you are not confident in merely to
complete the questionnaire, that is worse than `UNKNOWN` — please do not.

**IMPORTANT**

Do not try to work out what result the sender is hoping for, and do not try to guess what
answer would be considered correct. There is no expected answer to aim at. Answer from the
material; where the material is insufficient, say so and say what is missing.

**AT THE END, ADD**

```
LIMITATIONS:          anything that stopped you answering better
UNANSWERED:           questions you could not answer, and why
UNEXPECTED_FINDINGS:  anything relevant you noticed that you were not asked about
```

─────────────────────────────────── SEND TO HERE ───────────────────────────────────

---

## C. PRESERVING THE RAW RESPONSE

When the reply arrives:

1. Save it **exactly as received** — no tidying, no reformatting, no translation, no
   deletion of anything that looks like a mistake. Save it as a plain text file:
   `experiments/exp-ai-01/response-<participant>-<date>.txt`.
2. Do not correct the participant's `EVIDENCE` quotations, even if they misquote the
   material. A misquote is itself a finding.
3. Record alongside it: the participant's model/version identifier, the date, and
   confirmation that `MATERIAL VERSION` was `exp-ai-01/material-v1`.
4. Do not send it to any other participant.
5. Hand it back raw. It will be analysed question by question against the material.

---

## D. WHAT WAS WITHHELD (recorded, for our side only)

The participant was not told, and must not be told:

- that this experiment follows a defect found earlier in a reader protocol;
- that the tool once failed on a name of this kind, or how it failed;
- any finding identifier (`F-V01-4`, `F-V01-5`, or any other);
- what the correct answers are, or which questions are the interesting ones;
- how the tool implements the path rendering, or that a `path-encoding` line exists by
  design;
- what any previous participant answered.

The objective was stated, because the task is not meaningful without it. Nothing about the
expected result was.
