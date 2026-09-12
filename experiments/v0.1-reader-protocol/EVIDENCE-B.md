# EVIDENCIA-B — READER PROTOCOL v2, ISOLATED-CONTEXT AGENT

**Classification: EVIDENCIA-B — READER PROTOCOL / AGENT ISOLATED CONTEXT.**

Status: EVIDENCE RECORDED. Registered 2026-09-12.

This run **does not satisfy A1 by itself.** It is **not** an independent human reader, **not** an
independent human run, and **not** evidence equivalent to an independent third party. Its purpose
is to obtain additional evidence about the legibility of the surface — not to substitute for
independent human validation, and **not** to correct the methodological weakness D4 recorded for
the first run.

The first run's record is **not** deleted, superseded or modified. It stands as
[`README.md`](README.md) §5, and it remains the record of what v0.1's output looked like at the
time.

---

## 1. Why this run exists

The first run (§5) answered Q1–Q5 and could not answer Q6, Q7 or Q8: the subject's tree was clean,
so every category that could have carried a statement of a limit was present but at zero. That
produced F-V01-3, with two causes:

1. the protocol did not require a negative case in the subject's tree;
2. the tool never affirmatively states its own structural limits.

The owner's disposition was to **amend the protocol and not the surface** — repairing the
experiment rather than extending the product (§6). Protocol v2 requires the subject's tree to
contain negative cases.

A second run then had to be executed by someone who had not participated in the design or the
implementation, and who had not seen the contract, the reports or the expected reading. No such
person was reachable from this environment, and the owner authorised the fallback explicitly:
a subagent with genuinely isolated context, recorded under this classification.

## 2. Method of execution

- **Reader.** A subagent with a fresh context, dispatched for this purpose alone. It did not
  write the code, has no memory of the design, and was given the operational sheet as its only
  instruction about the task.
- **Isolation of the artifact.** The tool binary was copied out of the repository to
  `/tmp/umbral-reader-v2/bin/umbral` (sha256 `5d0a8a37a72186fa`, identical to the original at
  `target/debug/umbral`) so that the reader never needs to enter the repository at all.
- **Isolation of state.** `XDG_DATA_HOME=/tmp/umbral-reader-v2/data`, so the run could not touch
  the owner's Umbral state.
- **Fixture.** `/tmp/umbral-reader-v2/subject`, prepared by the reader itself as protocol v2
  requires, outside the repository.
- **Instructions.** [`protocol-v2-operational-sheet.md`](protocol-v2-operational-sheet.md) — the
  operational sheet, reproduced below in §4. It contains the fixture recipe and the seven steps,
  and it contains no interpretation of any output.
- **Prohibitions given to the reader.** Not to read, list, search or open anything under the
  repository root; not to read the source, the README, the documentation or the experiment
  records; not to inspect the binary; not to search the web; not to ask what the output means;
  not to modify or recompile the tool.
- **Product untouched.** No code, configuration or behaviour was changed for this run. No change
  was made to favour the reader. Nothing was corrected before the result was known.

## 3. Context available to the reader

**Not available:** the label contract; any report by the implementing agent; the first run's
results; F-V01-2; F-V01-3; the source code; `umbral/README.md`; anything under `docs/`; the v0.1
plan; any internal decision; any expected answer; any interpretation of any field.

**Disclosed by the reader, and recorded rather than glossed:** the reader's environment contained
project-context text — an `AGENTS.md` block naming this repository by its absolute path. The reader
states that it ignored that context, did not enter, list, read or search that path, and did not
read the tool's source, README or experiment records.

**The isolation was therefore not perfect.** A pointer to the project existed in the reader's
context. See §8, limitation L-1.

## 4. The operational sheet

Verbatim, as delivered. It is the only instruction the reader received about the task.

```
UMBRAL v0.1 — READER PROTOCOL v2 — OPERATIONAL INSTRUCTIONS

1. The tool        /tmp/umbral-reader-v2/bin/umbral   (already built)
2. Your directory  prepare a directory that looks like a working directory in use, containing:
                     - several ordinary files, at least two with byte-identical contents;
                     - at least one subdirectory;
                     - at least one symbolic link, and at least one pointing at a path that
                       does not exist;
                     - at least one file that cannot be read (permissions 000);
                     - at least one file whose name is not valid UTF-8, if the system permits.
                   The tool does not write anything inside it.
3. State           prefix every command with XDG_DATA_HOME=/tmp/umbral-reader-v2/data
4. Commands        run in this order, recording exit status and the complete output:
                     1. umbral init <dir>
                     2. umbral observe <dir>
                     3. make three or four changes of your own choosing: create, edit, rename, delete
                     4. umbral observe <dir>
                     5. umbral status <dir>
                     6. umbral changes <dir>
                     7. umbral show <dir> <a path inside <dir> that you choose>
5. Questions       answer from the output alone:
                     Q1 How many entries are known in that directory?
                     Q2 Which of them are verified by content, and which are not?
                     Q3 When was it last observed, and was that observation complete?
                     Q4 What changed since the previous observation, and how does the tool know?
                     Q5 Is anything ambiguous? Why?
                     Q6 Name one thing the tool does NOT know.
                     Q7 Name one thing the tool asserts and one thing it refuses to assert.
                     Q8 Is there anywhere in the output where you cannot tell whether something
                        was observed or computed?
6. Send back       the complete raw output of every command; your answers exactly as written,
                   including any you are unsure about; anything that confused you.
                   Do not summarise and do not tidy the output.
```

## 5. Transcript

**The raw transcript is the primary evidence and is registered verbatim at
[`transcript-run2-agent.txt`](transcript-run2-agent.txt)** — 16 KB, complete, unedited, including
the fixture-build commands, every command line, every exit status, and stdout and stderr
separately. Nothing was cleaned, reordered or summarised.

**No redaction was applied to this transcript.** The workspace identifier it contains
(`7cdfef079d3102e8`) is derived from `/tmp/umbral-reader-v2/subject`, which is a scratch test
fixture, not a private directory. The first run's redaction (§5.8) was for a different reason: the
identifier there fingerprinted a real private path.

Checked before publication: zero absolute personal paths, and zero occurrences of the project's
internal record names.

## 6. The reader's literal answers

Reproduced exactly as written. Nothing paraphrased, nothing corrected.

> **Q1 — How many entries are known in that directory?**
>
> 9 entries are known. `status` says: `derived   entries=9  files=6  dirs=1  symlinks=2  other=0`.
> The 6 files are alpha.txt, beta2.txt, newfile.txt, notes.md, secret.txt and the
> non-UTF-8-named file; the 1 dir is data/; the 2 symlinks are link-to-alpha and broken-link.
> (The first observation also reported entries=9, but over a different set: it still had beta.txt
> and data/inner.txt and did not yet have beta2.txt or newfile.txt.) The output reports a single
> count for the last run; it does not list the entries, so I name them by combining the counts
> with my own knowledge of the fixture and the `show` probes.

> **Q2 — Which of them are verified by content, and which are not?**
>
> `status` gives: content-verified=5; content-not-verified=1 reason=unstable-or-unreadable;
> content-verification-not-applicable=3 reason=not-a-regular-file. From the `show` probes I can
> name: verified by content (a hash is shown) = alpha.txt (hash dc5a4edb8240), beta2.txt (hash
> dc5a4edb8240), notes.md (run2 hash 076d5fc1d208), newfile.txt (hash 606c2525825b). That is 4
> names for 5 verified entries; by arithmetic (6 files, 1 of them not verified) the fifth verified
> file is the non-UTF-8-named file weird-\xFF\xFE.txt — but `show` panics on that path so I never
> saw its hash. NOT verified by content = secret.txt (hash=none, stability=none, reason
> unstable-or-unreadable). Neither verified nor not-verified, because content verification does not
> apply (not regular files) = data/ (dir), link-to-alpha, broken-link (symlinks).

> **Q3 — When was it last observed, and was that observation complete?**
>
> Last observed = run 2. `status`: `last-run=2  started=2026-09-12T23:43:53.968Z  entries=9` and
> `last-run=2  complete=true`. `observe` run 2: started=2026-09-12T23:43:53.968Z,
> finished=2026-09-12T23:43:53.969Z, complete=true. So the last observation started
> 2026-09-12T23:43:53.968Z and finished 23:43:53.969Z, and it was reported complete
> (complete=true).

> **Q4 — What changed since the previous observation, and how does the tool know?**
>
> `changes` (compared from-run=1 to-run=2, complete=true) reports: unchanged=6, modified=1,
> created=1, deleted=1, renamed-or-moved=1, recreated=0, unobserved=0, ambiguous=0; and the
> items: renamed-or-moved path=beta2.txt old-path=beta.txt; deleted path=data/inner.txt
> object-survives=false; created path=newfile.txt; modified path=notes.md content-changed=true.
> That matches the four edits I made (create newfile.txt, append to notes.md, mv beta.txt
> beta2.txt, rm data/inner.txt). HOW it knows: the output shows it derives changes by comparing
> the two stored observations (`compared from-run=1 to-run=2`), each change carrying
> scan-complete=true and the modified one content-changed=true. The output does NOT state the
> matching rule used (e.g. whether paths are matched by name, and renames inferred from a content
> hash). I can see beta2.txt's run=2 hash dc5a4edb8240 equals the hash alpha.txt/beta.txt had,
> which would allow rename detection by content, but the output never says that is the mechanism,
> so this is inference, not something the output states.

> **Q5 — Is anything ambiguous? Why?**
>
> Yes, several things are ambiguous. (a) alpha.txt and beta.txt were byte-identical (same content,
> both hash dc5a4edb8240). When I renamed beta.txt to beta2.txt, content alone cannot say which of
> the two identical files moved; the tool nonetheless reports renamed-or-moved old-path=beta.txt
> and ambiguous=0, with no visible basis for choosing beta.txt over alpha.txt. From the output
> alone that choice looks arbitrary and I cannot verify it. (b) secret.txt is reported `unchanged`
> across the two runs even though its content was never read (hash=none, stability=none); the
> output does not show what that 'unchanged' was based on. (c) the directory data/ is reported
> `unchanged` even though the `observed` lines directly above show its size changed 60 -> 40 and
> its mtime changed (…40.948Z -> …51.515Z); the word 'unchanged' conflicts with the observations
> printed with it. (d) broken-link is printed as kind=symlink with nothing indicating its target
> does not exist, so it is indistinguishable from link-to-alpha except by size (14 vs 9).
> (e) `unobservable-paths=0 reason=not-observed-at-observation-time` attaches a reason to a zero
> count, and I cannot tell whether the reason explains the zero or would describe nonzero cases.

> **Q6 — Name one thing the tool does NOT know.**
>
> The tool does not know the content of secret.txt: `hash=none`, `stability=none`, and
> `content-not-verified=1 reason=unstable-or-unreadable`. Because it has no content for that file
> it cannot tell whether secret.txt's contents changed. (Other things it does not know: it never
> reports that broken-link's target is missing; and it cannot address the non-UTF-8-named file at
> all — `show` panics with exit 101.)

> **Q7 — Name one thing the tool asserts and one thing it refuses to assert.**
>
> Asserts: `derived modified path=notes.md content-changed=true scan-complete=true` — it asserts
> positively that notes.md's content changed between run 1 and run 2 (hash 0208ded62d4b ->
> 076d5fc1d208). Refuses: it refuses to verify content of anything that is not a regular file —
> `content-verification-not-applicable=3 reason=not-a-regular-file` (data/, link-to-alpha,
> broken-link); and it refuses to assert any content for secret.txt (`hash=none`,
> `stability=none`).

> **Q8 — Is there anywhere in the output where you cannot tell whether something was observed or
> computed?**
>
> Every output line carries an explicit `observed` or `derived` prefix, so most of the time the
> answer is stated. But there are places the label does not settle it. (1) In `show`, the run=1
> block (`derived run=1 hash=… stability=…`) is tagged derived, but the output never shows whether
> that hash was computed during run 1 and stored, or recomputed now from stored data. (2)
> `derived run=1 -> 2 unchanged path=secret.txt` is tagged derived yet rests on no observed
> content at all (hash=none), so I cannot tell what it was computed from. (3) In `status`, counts
> such as `entries=9 files=6 dirs=1 symlinks=2 other=0` are tagged derived although they look like
> a direct tally of observed entries; the line between counting observations and computing is not
> visible. (4) `init` tags `canonical=…` as observed and `root=…` as derived even though both are
> the same path string, so I cannot tell what distinguished observing it from deriving it.

## 7. Evaluation of Q1–Q8

Evaluated against the output alone, and against the contract the output claims to implement.

| | Answered | Verdict |
|---|---|---|
| Q1 | yes | **Correct.** `entries=9`; consistent with the counts and with the fixture. The reader notes it could not name the entries from `status` alone. |
| Q2 | yes | **Correct and complete.** 5 verified, 1 not verified, 3 not applicable — the right partition, and it names them. It could name only 4 of the 5 verified files and said so, because `show` panics on the fifth. |
| Q3 | yes | **Correct.** Run 2, the right timestamp, `complete=true`. |
| Q4 | yes | **Correct**, and it correctly reports the limit of its own answer: the four changes match, and it states plainly that the matching rule is *not* shown, offering its hash comparison only as inference. |
| Q5 | yes | **Correct, and stronger than the question.** It reports the tool's own `ambiguous=0` and then lists five genuine ambiguities. |
| Q6 | yes | **Correct.** `hash=none` for secret.txt is exactly the limit, and it does not invent a different one. **This is the question the first run could not answer.** |
| Q7 | yes | **Correct.** A positive assertion (notes.md content changed) and a refusal (content verification does not apply; no content asserted for secret.txt). **The first run could answer only half of this.** |
| Q8 | yes | **Correct, and the answer is a finding, not a clean bill of health.** All four places it names are real. **The first run could not answer this at all.** |

**8 of 8 answered.** The three questions that failed in the first run were answered here.

**The improvement is attributable to the protocol amendment, not to the product.** Nothing in
`umbral/` changed between the two runs. The first run failed Q6–Q8 because its tree contained no
negative case; protocol v2's fixture supplied one, and the same unchanged output then supported
all eight answers. That is F-V01-3's disposition working as intended.

**This is not A1 PASS.** A1 requires a reader who did not write the code; this reader did not
write it, but it is an agent of the same system that did, and the owner's own classification
governs. A1 remains **NOT SATISFIED** pending an independent human reader.

## 8. Findings

Separated by kind, as required. **No finding below has been acted on**: no code, no behaviour and
no documentation of the contract was changed after the result was known.

### Surface defects — the output is the problem

**F-V01-4 — `show` panics on a path argument that is not valid UTF-8.** Confirmed independently,
twice, outside this run. The tool **observes, counts and hashes** a non-UTF-8-named file, but
cannot be asked about it: `umbral show <root> <non-utf8>` exits **101** with an empty stdout and a
Rust panic on stderr.

```
thread 'main' panicked at .../library/std/src/env.rs:878:51:
called `Result::unwrap()` on an `Err` value: "weird-\xFF\xFE.txt"
```

Cause: `umbral/src/main.rs:53` — `let args: Vec<String> = std::env::args().collect();` unwraps
invalid Unicode. A crash is never an acceptable answer to a reader's question, and this one blocks
a whole class of paths that the plan's test matrix explicitly claimed. **The reader found this by
its own initiative, on a probe the protocol did not require.**

**F-V01-5 — the test that claims to cover F-V01-4 does not cover it.** This is the same family as
F-5 and F-6: a test whose name asserts coverage it does not have.

```rust
// umbral/tests/fs_matrix.rs:180
let out = s.run(&["show", r.as_str(), name.to_str().unwrap_or_default()]);
```

`name` is the non-UTF-8 name; `to_str()` returns `None` for it, so `unwrap_or_default()` passes an
**empty string**. The test then asserts only that whatever came back is labelled. Verified: `show
<root> ""` returns `unknown path= reason=not-observed-in-any-run`, exit 0 — so the test passes
while the behaviour in its name is broken. The comment above it ("either it is found or it is
honestly reported as not found") is what made it vacuous: it accepts every outcome.

**Third occurrence of this family in this project.** F-5 was a cached build hiding a generator that
never fired; F-6 was an assertion that assumed a filesystem behaviour; F-V01-5 is a test that
silently substitutes an empty string for the input it names.

**F-V01-6 — `unchanged` states no basis and no scope.** The output prints two `observed` lines for
`data/` showing `size=60` then `size=40` and a changed mtime, and then says
`derived run=1 -> 2 unchanged path=data`. The same verdict is applied to `secret.txt`, whose
content was never read.

**`reconcile` is not at fault.** Its code comment on `same_observable` states the rule explicitly:
for regular files the observable state is size + mtime; for every other kind the metadata that
changes with content is not meaningful, so identity plus kind is the whole statement. A directory's
size and mtime change when a child is added or removed, which is not the directory being modified.
The behaviour is deliberate and defensible.

**The defect is that the output never says so.** A reader cannot distinguish a verdict the tool can
support from one it cannot, and here the tool printed the evidence that appears to contradict its
own conclusion. Compounding it, the doc comment at the top of `reconcile.rs` states the rule
unscoped ("`Modified` is decided by size + nanosecond mtime"), which is where the ambiguity starts.

**F-V01-7 — the basis for a rename claim is never shown.** The reader renamed `beta.txt` to
`beta2.txt` while `alpha.txt` held byte-identical content. The tool reported
`renamed-or-moved path=beta2.txt old-path=beta.txt` with `ambiguous=0`.

**`reconcile` is not at fault.** Matching is by physical identity, and exactly one candidate
carried the freed `dev`+`ino`, so the choice was justified and not arbitrary.

**The defect is that the justification is invisible.** The output shows no physical-identity
evidence anywhere, so from the output alone the reader cannot tell a justified match from a lucky
guess — and with two byte-identical files present, that is precisely the case where a reader will
suspect a guess. The reader said so: "from the output alone that choice looks arbitrary and I
cannot verify it."

**F-V01-8 — residual label legibility after the F-V01-2 correction.** The correction moved the
right values to `derived`, but where an observed and a derived value coincide, the labels alone do
not let a reader infer the rule:

- `init` labels `canonical=/tmp/...` as `observed` and `root=/tmp/...` as `derived`, and both are
  the same string. The rule is *who produced the value* (the filesystem's answer versus the
  caller's argument echoed back), which is correct — but a reader who sees one string under two
  labels cannot reconstruct it.
- counts are labelled `derived` "although they look like a direct tally of observed entries".

This is a real cost of the correction, found by a reader that had never seen the rule. It is
narrower than F-V01-2: the labels are right, the legibility is incomplete.

**F-V01-9 — the output's vocabulary is defined nowhere in the output.** `stability=stable`,
`stability=none`, `unstable-or-unreadable`, `scan-complete`, `object-survives=false`, `recreated`,
`unobserved`, `other=0`, and whether `entries` includes the root. The reader answered everything
despite this, by inference — so this is a legibility weakness rather than a blocker, and it is
recorded as such rather than inflated.

### Protocol defects

**P-V01-1 — the protocol's own command list cannot answer its own question.** Step 7 asks for
**one** `show`. Q2 asks *which* entries are verified by content. `status` gives counts only, so the
reader ran **nine extra `show` probes** to answer Q2 — and still could not name the fifth verified
file, because `show` panics on it (F-V01-4). The protocol as written is insufficient for Q2.

This confirms, from a second direction, the limitation already recorded as F-V01-1 in the v0.1
record: there is no listing command, so "which ones" requires one probe per path. The protocol
should either require the probes or the surface should answer the question.

**P-V01-2 — inconsistency in the instructions I administered.** The sheet forbade the reader from
reading anything under `/tmp/umbral-reader-v2` except the tool binary and its own directory, and
then required it to write the transcript into `/tmp/umbral-reader-v2`. The reader resolved this
correctly. Recorded because the instruction was mine and it was contradictory.

### Interpretation defects

**None.** No answer was wrong because the reader misunderstood the output. Every incorrect or
incomplete statement the reader made was traced to a surface defect (F-V01-4, F-V01-6, F-V01-7)
or to a gap in the protocol (P-V01-1). The reader's inference in Q4 was labelled by the reader
itself as inference rather than asserted as fact.

### Methodological limitations

**L-1 — the isolation was not complete.** The reader's environment carried project-context text: an
`AGENTS.md` block naming this repository by its absolute path. The reader reports it ignored that
context and did not enter the repository, and the transcript is consistent with that — it contains
no repository path and no project-internal term. But a pointer existed, and "the agent says it
ignored it" is the agent's own report, not something I can verify.

**L-2 — EVIDENCIA-B is an agent, not a person.** It is an agent of the same system family as the
one that wrote the code. That cuts both ways: a model may infer conventions from terse output more
readily than a person, and may also be more willing to reason about a private vocabulary. Neither
direction is measured here.

**L-3 — the reader ran more commands than the protocol required.** Its success on Q2 and Q8 rests
partly on nine extra `show` probes that the protocol did not ask for. A reader who ran exactly the
seven steps would have answered less.

**L-4 — the reader prepared its own fixture.** Inherent to protocol v2, and it means the reader
knew the fixture's contents — including that negative cases were present. This is the cost of the
amendment, recorded in §6.3 of the protocol.

**L-5 — one reader, one run.** No repeat, and no second reader to compare against.

**L-6 — the answers are self-reported.** Mitigated, not removed: I read the registered transcript
in full rather than the reader's summary, and I reproduced the one crash independently.

**This run does not correct D4.** The first run's methodological weakness — the subject was the
owner, who had seen the design — is not repaired by this run. D4's remedy is an independent human
reader, which remains outstanding.

## 9. What this run does not do

- It does not satisfy A1. A1 remains **NOT SATISFIED**.
- It does not make v0.1 complete. v0.1 remains **not declared complete**.
- It does not replace the first run's record, which stands unmodified at §5.
- It does not correct the D4 weakness.
- It does not close SC-5, Q25, Q2, Q5, Q1, Q15, 0.7, 0.8, or V1.

## 10. Findings that need a disposition

All nine findings are registered and **none has been acted on**. The owner decides whether each is
corrected inside v0.1, deferred, or declared outside v0.1's contract and the contract narrowed
explicitly — never narrowed silently.

The one that is not a matter of judgement: **F-V01-4 is a crash.** A tool that panics on a path it
can observe is a defect regardless of how the contract is drawn.
