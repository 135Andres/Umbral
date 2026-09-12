# READER PROTOCOL — v0.1

Status: EXPERIMENT RECORD. Protocol designed before implementation; dry pass executed by the
author-agent. **The formal run, by a person who did not write the code, is PENDING.**

Classification of this record: experiment. It is evidence about whether v0.1's acceptance
criterion A1 holds. It is not a decision, and it changes no status.

---

## 1. WHY THIS EXISTS

v0.1's contract is not "the code runs". It is "a person who did not write this can find out
what is known, when it was known, and what changed". That is only established by having such a
person do it. The protocol was written before the tool, so that the tool could not be shaped
to answer questions it had been shown.

The subject must not be the author. Per the owner's decision D4, if the owner runs it, the
record must declare that methodological weakness. An agent that wrote the code is weaker still,
so the run below is labelled a **dry pass** and does not satisfy the criterion.

## 2. PROTOCOL

> **For any run after 2026-09-12, use §6 (protocol v2), which adds a fixture requirement.**
> This section is preserved as it was executed.

**Subject.** A person who did not write the code. Preferred: someone other than the project
owner. If neither is available, the owner, with the weakness declared in the record.

**What the subject receives.** The binary and a directory of their own whose contents they
know. Nothing else — not this repository, not the source, not an explanation of the design.

**Steps, executed by the subject.**

1. `umbral init <their directory>`
2. `umbral observe <their directory>`
3. Make three or four changes of their own choosing — create, edit, rename, delete. The
   protocol does not say which.
4. `umbral observe <their directory>` again.
5. `umbral status`, `umbral changes`, `umbral show <a path they choose>`

**Questions, answered from the output alone.**

| | Question |
|---|---|
| Q1 | How many entries are known in that directory? |
| Q2 | Which of them are verified by content, and which are not? |
| Q3 | When was it last observed, and was that observation complete? |
| Q4 | What changed since the previous observation, and how does the tool know? |
| Q5 | Is anything ambiguous? Why? |
| Q6 | Name one thing the tool does NOT know. |
| Q7 | Name one thing the tool asserts and one thing it refuses to assert. |
| Q8 | Is there anywhere in the output where you cannot tell whether something was observed or computed? |

**Recording.** The raw answers, including any that are wrong, and the transcript of the
output. Nothing is summarised.

**Pass condition.** Q1–Q8 answered correctly from the output alone. A failure is a finding,
and has two legitimate dispositions: fix it inside v0.1, or declare it outside v0.1's contract
and narrow the contract explicitly in this record. Narrowing it silently is not a disposition.

---

## 3. DRY PASS — EXECUTED 2026-09-12 BY THE AUTHOR-AGENT

> **Note added 2026-09-12:** the transcript below was produced **before** the F-V01-2
> correction, so it shows the earlier labelling — for example `observed  run=1  entries=7`,
> where `run` and `entries` are now `derived`. It is left as it was produced, because it is the
> record of what that run saw. Current labelling: §7.

**This does not satisfy A1 and does not count as the formal run.** It was run to validate that
the protocol's questions are answerable at all from the output, and to catch output defects
before a human spends time on it. The subject wrote the code, so any question it answers is
answered with knowledge the protocol is designed to withhold.

Subject's directory: a small project tree with five files (two of them with identical content)
in two subdirectories, on tmpfs. The subject then edited one file, created one, deleted one,
and renamed the created one into a subdirectory.

### Transcript

**`umbral init`**

```
observed  root=/tmp/umbral-rp-root/proyecto
observed  canonical=/tmp/umbral-rp-root/proyecto
observed  workspace-id=8e51a5b619e6d91f
observed  state-dir=/tmp/umbral-rp-data/umbral/ws-8e51a5b619e6d91f
derived   initialised=true
```

**`umbral observe` (first)**

```
observed  run=1  root=/tmp/umbral-rp-root/proyecto  canonical=/tmp/umbral-rp-root/proyecto
observed  run=1  entries=7
observed  run=1  content-verified=5
observed  run=1  content-not-verified=0  reason=unstable-or-unreadable
observed  run=1  started=2026-09-12T22:28:07.927Z  finished=2026-09-12T22:28:07.927Z
derived   run=1  complete=true
```

**`umbral observe` (after the changes)**

```
observed  run=2  root=/tmp/umbral-rp-root/proyecto  canonical=/tmp/umbral-rp-root/proyecto
observed  run=2  entries=7
observed  run=2  content-verified=5
observed  run=2  content-not-verified=0  reason=unstable-or-unreadable
observed  run=2  started=2026-09-12T22:28:07.935Z  finished=2026-09-12T22:28:07.935Z
derived   run=2  complete=true
```

**`umbral status`**

```
observed  root=/tmp/umbral-rp-root/proyecto  canonical=/tmp/umbral-rp-root/proyecto
observed  workspace-id=8e51a5b619e6d91f
observed  last-run=2  started=2026-09-12T22:28:07.935Z  entries=7
derived   last-run=2  complete=true
observed  entries=7  files=5  dirs=2  symlinks=0  other=0
observed  content-verified=5
observed  content-not-verified=0  reason=unstable-or-unreadable
derived   content-verification-not-applicable=2  reason=not-a-regular-file
unknown   unobservable-paths=0  reason=not-observed-at-observation-time
derived   log-runs=2  log-observations=14
```

**`umbral changes`**

```
derived   compared  from-run=1  to-run=2  complete=true
derived   count  unchanged=5
derived   count  modified=1
derived   count  created=1
derived   count  deleted=1
derived   count  unobserved=0
derived   count  renamed-or-moved=0
derived   count  recreated=0
derived   count  ambiguous=0
derived   deleted  path=build.log  object-survives=false  scan-complete=true
derived   modified  path=notas/borrador.md  content-changed=true  scan-complete=true
derived   created  path=notas/temporal.txt  scan-complete=true
```

**`umbral show notas/borrador.md`**

```
observed  run=1  kind=file  size=25  mtime=2026-09-12T22:28:07.923Z  hash=95ab8be11cc0  stability=stable
observed  run=2  kind=file  size=35  mtime=2026-09-12T22:28:07.930Z  hash=6ca62478c352  stability=stable
derived   run=1 -> 2  modified  path=notas/borrador.md  content-changed=true  scan-complete=true
```

**`umbral check`**

```
observed  state=/tmp/umbral-rp-data/umbral/ws-8e51a5b619e6d91f/observations.sqlite
observed  log-runs=2  log-observations=14
derived   referential-integrity=true  orphan-observations=0
derived   run-counts-agree=true  mismatched-runs=0
derived   duplicate-entries=0
derived   derived-state-recomputed=true  agrees-with-stored=true
derived   stored-tables=observation,run,schema_meta  derived-state-persisted=false
derived   consistent=true
```

**`umbral show no-existe.txt`** (a path never observed)

```
unknown   path=no-existe.txt  reason=not-observed-in-any-run
```

### Answers, from the output alone

| | Answer | Verdict |
|---|---|---|
| Q1 | `entries=7  files=5  dirs=2  symlinks=0  other=0` | answered |
| Q2 | `content-verified=5`, `content-not-verified=0`, and 2 entries where verification does not apply (`reason=not-a-regular-file`). All five files are verified. | answered at the level of counts — see F-V01-1 |
| Q3 | `last-run=2  started=2026-09-12T22:28:07.935Z`, and `derived  last-run=2  complete=true` | answered |
| Q4 | `count modified=1`, `deleted=1`, `created=1`, and one line per path carrying the evidence: `content-changed=true` for the edit, `scan-complete=true` for the deletion | answered |
| Q5 | `count ambiguous=0` — nothing was ambiguous in this run | answered |
| Q6 | `unknown  path=no-existe.txt  reason=not-observed-in-any-run`, and `unknown  unobservable-paths=0` | answered |
| Q7 | Asserts: `content-verified=5`, each with a hash in `show`. Refuses to assert: `show` on a directory reports `hash=none  stability=none` rather than a content claim, and the deleted file is reported as `deleted` only because the scan was complete | answered |
| Q8 | No. Every line begins with one of the four labels, and the label distinguishes what was read from the filesystem from what was computed | answered |

### Findings from the dry pass

**F-V01-1 — `status` reports content verification as counts, not identities.** Q2 can be
answered for this tree because the count covers every file. On a tree where some files are
verified and some are not, the reader would learn *how many* are unverified but not *which*.
Identifying them requires one `show` per path. The question is answerable, but not efficiently.

Disposition: **accepted for v0.1, and narrowed explicitly.** Adding a listing command after the
acceptance criteria were fixed would be scope growth, and identifying entries by criterion is
the territory of a later version (deterministic inspection). The limitation is recorded in
`docs/versions/v0.1.md` (limitation 8) and in the next-version handoff.

**No output defect was found.** Nothing in the transcript was ambiguous about its own status,
and no line required knowledge from outside the output to interpret.

---

## 4. WHAT REMAINS TO BE DONE

> **UPDATE 2026-09-12:** the formal run has since been executed by the project owner. See §5.
> The text below is preserved as written, because it states what was still outstanding at that
> point.

The formal run, by a person who did not write the code. Until it happens, v0.1's criterion A1
is unverified and the version cannot be declared complete.

The protocol above is the thing to run. The questions are the ones to ask. The result belongs
in a new section of this record, appended — not substituted for the dry pass, which is part of
the evidence about how this version was checked.

---

## 5. FORMAL RUN — EXECUTED 2026-09-12 BY THE PROJECT OWNER

**Result: A1 NOT SATISFIED.** Q1–Q5 answered correctly; Q6, Q7 and Q8 not answered.

This section is registered, not closed: §2's recording rule requires the raw transcript of the
output, and it has not been received. The owner's answers are registered in full below.

### 5.1 Subject, and the methodological weakness declared

Subject: **the project owner** — the fallback §2 allows, with the weakness declared, as the
owner's decision D4 requires.

Three parts to the weakness, and the third is the serious one:

1. **Not a disinterested party.** The subject commissioned the work and approved the
   acceptance criteria being tested.
2. **The preferred subject was not used.** §2 prefers someone other than the owner.
3. **The subject was not a cold reader.** The subject had been shown the plan, the acceptance
   criteria, and reports describing the output contract and its four labels before running the
   protocol. Q1–Q5 being answered cleanly is therefore partly explained by prior knowledge the
   protocol is designed to withhold, and this run is weaker evidence than §2 intends.

The literal requirement *is* met: the code was written by an AI agent, so the owner did not
write it. That is a technicality, and it does not repair point 3.

It is still evidence, and not empty evidence: three of eight questions failed, and one of those
failures exposed a real defect in the output contract — found by a reader who *did* have prior
knowledge, which makes the defect more likely, not less, to be real.

### 5.2 Environment

| | |
|---|---|
| Binary | `target/debug/umbral`, HEAD `fdcbfb0`, working tree clean |
| State location | the default (`$XDG_DATA_HOME/umbral/`, falling back to `~/.local/share/umbral/`) |
| Observed tree | the owner's own directory; by the owner's answer, two files and no other entries |
| Protocol delivery note | **N-V01-1** — the instructions written for the run used the placeholder `<dir>`, which the shell read as a redirection. The owner hit this and corrected it. Recorded as a defect of the protocol's *delivery*, not of the tool. |

### 5.3 The owner's answers, verbatim

Reproduced as written. One redaction, marked: the workspace identifier in Q8 is a fingerprint
derived from the owner's private directory path, so the value is replaced by `<workspace-id>`
rather than published. Nothing else is altered, summarised or reordered.

> **Q1 — ¿Cuántas entradas se conocen en ese directorio?**
> 2 entradas. Son 2 archivos.
>
> **Q2 — ¿Cuáles están verificadas por contenido y cuáles no?**
> Las 2 entradas están verificadas por contenido. La salida indica content-verified=2 y content-not-verified=0.
>
> **Q3 — ¿Cuándo se observó por última vez, y esa observación fue completa?**
> La última observación fue el run 2, iniciado a 2026-09-12T22:55:37.975Z y terminado a 22:55:37.976Z. Fue completa: complete=true.
>
> **Q4 — ¿Qué cambió desde la observación anterior, y cómo lo sabe la herramienta?**
> a.txt fue modificado. La herramienta lo determina comparando las observaciones: modified=1 y específicamente content-changed=true. Además, a.txt pasó de hash 68d9bd28f591 a 6b6353db25d5.
>
> **Q5 — ¿Hay algo ambiguo? ¿Por qué?**
> No. La salida indica ambiguous=0.
>
> **Q6 — Nombra una cosa que la herramienta NO sabe.**
> La salida indica unknown unobservable-paths=0, así que no proporciona un ejemplo concreto de algo que no sepa sobre este directorio. No sería correcto inventar uno a partir de conocimiento externo.
>
> **Q7 — Nombra una cosa que la herramienta afirma y otra que se niega a afirmar.**
> Afirma que a.txt fue modificado entre run 1 y run 2.
> No encuentro en esta salida un ejemplo explícito de algo que la herramienta se niegue a afirmar. Hay categorías como unknown, ambiguous y content-not-verified, pero todas están en cero en esta prueba.
>
> **Q8 — ¿Hay algún punto en la salida donde no puedas distinguir si algo fue observado o calculado?**
> Sí. La distinción parece bastante clara en general mediante los prefijos observed, derived y unknown, pero no puedo determinar desde la salida solamente si ciertos valores dentro de una línea observed son directamente observados o si alguno fue previamente calculado por la implementación. Por ejemplo, workspace-id=<workspace-id> aparece como observed, aunque por su naturaleza podría ser un valor derivado. Sin documentación adicional no puedo establecerlo con certeza.
>
> No hubo errores de ejecución después de corregir el uso del marcador `<dir>`.

### 5.4 Verdict, question by question

| | Answered | Assessment |
|---|---|---|
| Q1 | yes | "2 entries, both files" is consistent with the answer to Q2. **Not independently verifiable** until the transcript arrives. |
| Q2 | yes | The tree was small enough that the count fully determines the answer. This does **not** exercise F-V01-1 (counts rather than identities): with two files, "how many" and "which" coincide. |
| Q3 | yes | Correct, including the completeness statement. |
| Q4 | yes | Correct, and it cites the evidence the tool gives: the count, `content-changed=true`, and the hash transition read from `show`. |
| Q5 | yes | Correct. |
| Q6 | **no** | The subject could not name one thing the tool does not know, and explicitly refused to invent one from outside the output. That refusal is the correct behaviour under the protocol; the question still went unanswered. → **F-V01-3** |
| Q7 | **no** (one half) | An assertion was named correctly. No refusal could be named: every category that expresses a refusal was at zero. → **F-V01-3** |
| Q8 | **no** | The subject identified a genuine ambiguity and named a specific line. The doubt is correct: `workspace-id` is computed by the tool and labelled `observed`. → **F-V01-2**, a real defect |

**A1 is not satisfied.** The pass condition is Q1–Q8 answered correctly from the output alone.

### 5.5 Findings from the formal run

**F-V01-2 — lines labelled `observed` that are computed. Class: output-contract defect. Real.**
Found by Q8. The contract in `umbral/src/report.rs` defines `observed` as *a fact read from the
filesystem during a named run* and `derived` as *a result computed from observations*. Two
classes of line violate that:

- **Computed identifiers:** `workspace-id` (BLAKE3 over the canonical path) and `state-dir`
  (composed from the data home plus that id), in `init`, `status` and `workspaces`.
- **Counts and aggregates:** `entries`, `files`, `dirs`, `symlinks`, `other`,
  `content-verified`, `content-not-verified`, `unobservable-paths`, `log-runs`,
  `log-observations` — all computed from stored observations.

The subject found one instance of the first class. The defect is the class, not the instance:
the reader's doubt is legitimate for every one of those lines, and it will recur for any
reader. This is exactly the class of question Q8 exists to surface, and it surfaced it.

Disposition: **CORRECTED IN v0.1** (owner decision, 2026-09-12). The labels were repaired — a
repair of the contract the version had already committed to, not new capability — and the
distinction is now enforced by a test rather than by review. See §7.

**F-V01-3 — limits are expressed only reactively, so a fully-observed tree exhibits none.
Class: output-contract completeness, entangled with a protocol setup gap.** Found by Q6 and Q7.

On a tree where nothing is unobservable, nothing is unverified and nothing is ambiguous, the
output contains no affirmative statement of what the tool does not know or refuses to assert.
Every category that could carry such a statement is present but at zero. The reader is left
unable to answer, and correctly refuses to supply the answer from outside.

Two distinct causes, and they must not be merged:

1. **Protocol setup gap.** §2 does not require the subject's tree to contain a negative case —
   something unobservable, unverified or ambiguous. The questions presuppose one. A tree with a
   permission-denied file or a symlink would have exercised all three categories.
2. **Tool behaviour.** The tool has structural limits that are permanent and knowable — it
   records a fingerprint rather than the content, and it knows nothing about changes between
   observations — and it never states any of them. A reader cannot discover them from the output.

Disposition: **THE PROTOCOL IS AMENDED; THE SURFACE IS NOT** (owner decision, 2026-09-12).
Cause (1) is fixed by requiring the subject's tree to contain negative cases. Cause (2) is
deliberately NOT fixed: an affirmative statement of Umbral's limits is outside v0.1's contract
and may be evaluated later. The purpose of the amendment is to repair the experiment, not to
extend the product. See §6.

**N-V01-1 — protocol delivery.** The instructions handed to the subject used `<dir>` as a
placeholder, and the shell read it as a redirection until the subject corrected it. Defect of
how the protocol was delivered, not of the tool. Disposition: use a literal path in the
instructions, or quote the placeholder, in any future run.

### 5.6 The transcript, as received

The primary evidence for the first attempt is the output quoted inside the subject's answers.
It was received in that form — as fragments quoted while answering — not as a separate
command-by-command transcript. Nothing below is reconstructed: these are the values the subject
quoted, reproduced exactly as they appeared.

```
content-verified=2
content-not-verified=0
complete=true
run 2, started 2026-09-12T22:55:37.975Z, finished 2026-09-12T22:55:37.976Z
modified=1
content-changed=true
hash 68d9bd28f591 -> 6b6353db25d5   (the two hashes of a.txt, run 1 and run 2)
ambiguous=0
unknown unobservable-paths=0
workspace-id=<workspace-id>
```

**Not received:** the complete output of the six commands. Its absence is recorded rather than
papered over: Q1's answer ("2 entries, both files") is therefore consistent with the quoted
counts but not independently verifiable, and the two finding classes in §5.5 rest on the
subject's summary plus these fragments rather than on the full output.

### 5.7 The first attempt, and N-V01-1

The instructions handed to the subject used `<dir>` as a placeholder. In the subject's shell the
first command produced, verbatim:

```
zsh: parse error near `\n'
```

The subject recognised the cause and replaced the placeholder with a literal path; the protocol
then ran without further incident. **N-V01-1 is a defect of how the protocol was presented, not
of the tool.** Disposition: instructions for a future run must contain a literal path or a
quoted placeholder.

### 5.8 Redaction of the workspace identifier

Recorded so the record does not misrepresent what it contains:

- the value **did appear** in the output the subject quoted, and the subject quoted it in full;
- it is **redacted in this record**, replaced by `<workspace-id>`;
- the value is derived deterministically from the workspace's canonical path (BLAKE3 over its
  raw bytes, first 8 bytes as hex), so publishing it would publish a fingerprint from which a
  guessable private path could be confirmed;
- the redaction was applied to protect a private path, not to remove an inconvenient fact, and
  the fact that the output contained the value is preserved here rather than deleted.

The identifier is not a secret and not a credential. It is withheld only because the repository
is public and the path it fingerprints is not.

---

## 6. PROTOCOL v2 — AMENDED 2026-09-12

**Why.** In the first run the subject's tree was small and clean: nothing unobservable, nothing
unverified, nothing ambiguous. Every category that could have carried a statement of a limit was
present but at zero, so Q6 and Q7 had no material and could not be answered. The protocol now
requires the subject's tree to contain negative cases.

The tool's surface is deliberately **not** changed. What changes is the fixture, and the purpose
is to repair the experiment rather than to extend the product.

### 6.1 The fixture requirement

The subject prepares a directory that looks like a working directory in use, containing at
minimum:

- several regular files, at least two of which have byte-identical contents;
- at least one subdirectory;
- at least one symbolic link, including at least one that points to a path that does not exist;
- at least one regular file whose permissions deny read access to the subject's own user (mode
  `000`, for example);
- at least one file whose name is not valid UTF-8, where the platform permits such a name;
- at least one file the subject will modify between the first and the second observation.

### 6.2 What this recipe is, and what it is not

It describes the **contents of the directory** — filesystem facts. It says nothing about what the
tool will report for any of them, and it must not be accompanied by any such statement.

Binding on whoever prepares and administers the run:

- do not tell the subject which category of output any of these will produce;
- do not tell the subject which of these the tool handles well or badly;
- do not give examples of correct answers;
- do not explain how to read the output before the answers are written;
- do not let preparing the fixture become a clue about the result;
- do not let the subject read this record, the repository or the source before answering.

### 6.3 The cost of the amendment, recorded

The subject now knows the fixture contains awkward cases. That is a small loss of cold reading:
"there is something here the tool may not describe" is nearer to the answer than a clean tree
would be.

The cost is accepted deliberately, because the measured alternative is worse. With a clean tree,
three of the eight questions could not be answered, and two of them produced nothing beyond "I
cannot tell from this output".

What the amendment does not do is say what the tool will report, or which of the awkward cases is
the interesting one. Q6 and Q7 still have to be answered by reading the output.

### 6.4 Unchanged from §2

The questions, the pass condition, the recording rule, the prohibition on summarising, and the
requirement that the subject did not write the code. The preference for a subject other than the
owner stands; the first run's fallback — the owner, with the weakness declared — is recorded in
§5.1.

### 6.5 Status of the first run

The first run is not retracted. It is evidence about the version as it stood, and it produced
F-V01-2, which was real and has since been corrected. A second run under this protocol tests the
corrected output and is a **new** experiment record rather than a revision of this one.

**Second run executed 2026-09-12**, by a subagent with isolated context, under the classification
**EVIDENCIA-B — READER PROTOCOL / AGENT ISOLATED CONTEXT**. It answered **8 of 8** questions, and
its dispositions are in [`EVIDENCE-B.md`](EVIDENCE-B.md); the raw transcript is
[`transcript-run2-agent.txt`](transcript-run2-agent.txt) and the sheet it received is
[`protocol-v2-operational-sheet.md`](protocol-v2-operational-sheet.md). It found nine issues,
including a crash (`show` panics on a non-UTF-8 path) and a test that claims coverage of exactly
that case without exercising it. **It does not satisfy A1**: it is an agent, not an independent
human reader, and it does not repair the first run's D4 weakness.

---

## 7. THE CORRECTION (F-V01-2), 2026-09-12

Applied in `umbral/`, with no change to what the tool observes. Only the label each value carries
changed, plus a test that makes the distinction enforceable.

### 7.1 The rule now written into the code

> **`observed`** — the filesystem reported this value for an entry during this run: the entry's
> path, kind, size and mtime, and the canonical form of the observed root, which is the
> filesystem's own answer to "where is this really".
>
> **`derived`** — the tool produced it. Everything computed, counted, aggregated, compared,
> assigned or composed: content fingerprints, stability verdicts, run identifiers, the run's own
> timestamps, workspace identifiers, composed paths, and configuration echoed back.

The line is drawn at *who produced the value*, not at *whether the tool knows it*. That is what a
reader needs in order to tell, from the output alone, whether a value came from their filesystem
or from the tool.

### 7.2 What moved

| Was labelled `observed` | Now | Because |
|---|---|---|
| `workspace-id` | `derived` | BLAKE3 over the canonical path — the value the reader questioned |
| `state-dir` | `derived` | composed from the data home plus that identifier |
| `state` (in `check`) | `derived` | the same composed path |
| `root` | `derived` | the caller's argument echoed back, not read from the filesystem |
| `run`, `last-run`, `from-run`, `to-run` | `derived` | identifiers the store assigns |
| `started`, `finished` | `derived` | the tool's own clock readings |
| `created`, `tool-version` | `derived` | written into the record by the tool |
| `entries`, `files`, `dirs`, `symlinks`, `other` | `derived` | counts |
| `content-verified`, `content-not-verified` | `derived` | counts |
| `content-verification-not-applicable` | `derived` | already correct |
| `unobservable-paths` | `derived` | a count, even though what it counts are things the tool does not know; the unknowns themselves are reported per path by `show` |
| `log-runs`, `log-observations` | `derived` | counts over the log |
| `hash` | `derived` | a function of the bytes the tool read |
| `stability` | `derived` | the outcome of comparing two readings |

Two lines that previously mixed categories were split, so that no line carries values of more
than one kind:

```
show:  derived   run=1  hash=ac678d92b3d7  stability=stable
       observed  kind=file  size=6  mtime=2026-09-12T23:10:41.214Z
```

Only four fields may now appear on an `observed` line: `canonical`, `kind`, `size`, `mtime`.

### 7.3 Enforcement, so the distinction cannot decay

The contract is a constant in `umbral/src/report.rs`, not a convention:

- `OBSERVED_FIELDS` — the complete set of fields an `observed` line may carry. An allowlist, not
  a denylist, so that adding one is a deliberate act.
- `DERIVED_ONLY_FIELDS` — the computed fields that may never appear on an `observed` line.
- `label_contract_violations` — the check, run against the rendered output of every command by
  `tests/output_contract.rs::no_command_labels_a_computed_value_as_observed`.

The test is shown to fail: `the_label_check_rejects_doctored_lines` feeds it every field the
reader's report named, in the exact form the reader found the defect, and requires the check to
reject each one.

### 7.4 What did not change

The tool observes exactly what it observed before. No new capability, no new command, no new
output content — one line was split in two and the labels moved. `fsp-check/` is untouched, and
the frozen V0 records are untouched.
