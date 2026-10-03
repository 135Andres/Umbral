# EXP-AI-01 — plan for run 2

Status: **PLAN. NOT DELIVERED, NOT RUN.** Prepared 2026-09-13.

The owner delivers this. Hermes does not contact, execute or select a participant.

## 0. WHAT THIS RUN IS FOR

An **independent replication on unchanged material**. The question is not whether a second
participant reaches the same answers — it is whether an independent reader reproduces the
same *observation/inference behaviour*, above all whether it also refuses to decode the
escaped name and names the missing specification.

A replication on modified material would answer a different question. That is why
`material-v1` is frozen for this run and `material-v2` is deferred (§6).

## 1. FROZEN INPUTS — VERIFY BEFORE DELIVERY

| Artifact | Bytes | sha256 |
|---|---|---|
| `material.txt` (`material-v1`) | 2669 | `8ad75576edad4d6c5da84d2c396a1b56d91c8902fe7992d7ba19e202155f2678` |
| `prompt-to-participant.md` (the artifact to send) | 10165 | `766d1deafcc848dd171d9faac8c17e88d1ebe9b90720850e5ecce9c9e4696666` |

**Verified:** the material block embedded in §B of `prompt-to-participant.md` is
**byte-identical** to `material.txt` — 2669 bytes, same sha256. So the existing prompt artifact
*is* the delivery for run 2, with nothing to regenerate and nothing to adjust.

**Before delivering, re-check both hashes.** If either differs, stop: `material-v1` has
changed and the replication would not be a replication.

The historical capture is frozen as of run 1. `material.txt` must not be edited, regenerated,
reformatted or "improved" before run 2 — including the parts identified as defective.

## 2. WHAT THE PARTICIPANT RECEIVES

Exactly §B of `prompt-to-participant.md`, unchanged: everything between the two rules, and
nothing else.

- the objective, stated at the level that does not name the non-UTF-8 case;
- the captured session, `material-v1`, verbatim;
- Q1–Q8, as written for run 1;
- the response template;
- the prohibitions (no source inspection, no web search, no assuming unshown behaviour);
- the statement that `UNKNOWN` is valid and unpenalised;
- the anti-leading clause.

**No preamble, no explanation of why the question is being asked, no hint.** If the
participant asks what the tool is or what the output should say, the answer is that it must
come from the material.

## 3. WHAT THE PARTICIPANT MUST NOT RECEIVE

This is the isolation that makes the run independent:

- run 1's raw response (`response-01-external-ai.txt`);
- the analysis (`EVIDENCE-AI-01.md`);
- any finding identifier — `F-AI01-1`, `F-AI01-2`, `P-AI01-1`, `P-AI01-2`, `P-AI01-3`, or any
  `F-V01-*`;
- which questions are the diagnostic ones;
- what we consider the correct answers;
- that a defect was ever found in this area, or that this follows a reader protocol;
- how the tool implements the escaping, or that a `path-encoding` line exists by design;
- any statement about what run 1 concluded.

The prompt was written before run 1 and contains none of this. **Do not annotate it, do not
add a note explaining the experiment, and do not mention that this is a second run.** A
participant told it is a replication will behave differently.

## 4. KNOWN DEFECTS THAT WILL RECUR — ACCEPTED, NOT FIXED

`P-AI01-1`, `P-AI01-2` and `P-AI01-3` are defects of this package, identified in run 1.
**They are deliberately not fixed for run 2**, because fixing them would change the material
and void the replication. The cost is accepted and stated in advance:

- the participant will spend effort on the `exit=` scaffolding that is not tool output
  (`P-AI01-2`), and may report it as an `UNEXPECTED_FINDING`;
- it will see two renderings of the same name with no boundary between them (`P-AI01-1`) and
  may report the case difference;
- Q3 and Q5 still carry their leading framing (`P-AI01-3`).

**Recording the expectation in advance matters.** If run 2 also trips on these, the defect is
reliably detectable and that is itself a result. If it does not, run 1's participant detected
more than average — also a result. Either outcome is informative precisely because the defects
were left in place on purpose.

When run 2's response is analysed, the same distinction applies as in run 1: these are the
experiment's defects, not the participant's, and not the product's.

## 5. HANDLING THE RESPONSE

Identical to run 1, and in the same order:

1. Save it **verbatim**, unmodified — `experiments/exp-ai-01/response-02-<participant>-<date>.txt`.
   No tidying, no reformatting, no translation, no correcting misquotations.
2. Metadata goes **outside** the raw text, in the analysis document, never edited into it.
3. Record: experiment id, evidence class, participant identity **only if provided**, date,
   material version and hash, mode, and any uncertainty about execution conditions. Do not
   invent missing metadata — run 1's participant identity was not recorded rather than guessed,
   and the same rule applies.
4. Check every `EVIDENCE` quotation against `material.txt` before assessing anything.
5. **Do not send run 2's response to anyone, and do not send run 1's response to run 2's
   participant.**

## 6. COMPARISON — ONLY AFTER THE RAW RESPONSE ARRIVES

The comparison does not begin until run 2's raw response is saved. Not before, and not
partially.

Then, per question, against the material:

1. what each participant said, literally;
2. what evidence each cited, and whether it is in the material;
3. whether inference was presented as evidence;
4. whether the question induced the answer — checked first, since the leading framing is known;
5. whether the material was insufficient, or the participant under-tried;
6. what each answer establishes, and what it does not;
7. what remains `UNKNOWN`.

**Divergence is analysed, not averaged.** If the two participants read the same material
incompatibly, the first thing to establish is whether the material or the prompt is
under-determined — because both are known to be, in specific ways (§4). A divergence that
traces to a known package defect is a fact about the package. A divergence that does not is a
fact about the surface, and is the more interesting result.

**No majority rule.** Two participants agreeing does not establish truth; one contradicting
the other is not thereby wrong. With two participants there is no majority to take, and the
point still holds: agreement here is a data point about legibility, not a verdict.

**Do not merge run 1 and run 2 into one dataset with EVIDENCIA-B.** That run used different
material, different questions and a different protocol.

## 7. WHAT WOULD COUNT AS A USEFUL RESULT

Recorded in advance so the result is not fitted to a preferred outcome:

- **Q5 replication** — the participant refuses to decode `\xFF\xFE` **and** names the missing
  specification. This is the primary signal.
- **Q5 partial** — refuses but gives no reason, or decodes while flagging the assumption.
  A different behaviour, and worth recording as such.
- **Q5 overreach** — reports the byte sequence as established from the material. This would be
  the most informative negative result: it would mean the material invites a claim it does not
  support, and would make the package defect more serious than currently assessed.
- **Q3 replication** — also finds that `show` does not name its path (`F-AI01-1`).
- **Q7** — whether the tool's assertion is again distinguished from what a reader can
  establish.
- **New findings** — anything neither run 1 nor this analysis anticipated.

## 8. DEFERRED — DO NOT DO NOW

**`material-v2`** is not to be built yet. When it is, it should:

- separate tool output from harness scaffolding from execution metadata, so a reader can see
  which lines the tool produced (`P-AI01-2`);
- regenerate the command echo using the tool's own escaping, or drop the echo, so one capture
  does not contain two renderers of the same name (`P-AI01-1`);
- reword Q3 neutrally — ask what the output states about which path it concerns, rather than a
  yes/no about an absence;
- reword Q5 neutrally — ask what can be determined, leaving the negative open
  (`P-AI01-3`).

Changing the material before run 2 would contaminate the comparison. **That is the reason this
is deferred, not a lack of time.**

## 9. CHECKS BEFORE DELIVERY

- [ ] `material.txt` sha256 still `8ad75576…` — unchanged since run 1.
- [ ] `prompt-to-participant.md` sha256 still `766d1dea…` — unchanged since run 1.
- [ ] The embedded material still matches `material.txt` byte for byte.
- [ ] Sending §B only, with no preamble and no annotation.
- [ ] Nothing from §3 of this plan has reached the participant.
- [ ] The response will be saved verbatim, with metadata kept outside it.

## 10. STATE

- `material-v1` frozen and unchanged. No participant contacted for run 2.
- `material-v2` not built.
- Run 1 registered in commit `7008dfc`; analysed in `EVIDENCE-AI-01.md`.
- Comparison pending run 2's raw response.
