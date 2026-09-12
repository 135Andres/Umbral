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

Disposition: **PENDING owner decision.** The two legitimate options are to correct the labels
inside v0.1 (a repair of the contract the version already committed to, not new capability) or
to declare the distinction narrower than the version claimed and record that.

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

Disposition: **PENDING owner decision.** Fixing (1) is a protocol amendment and costs no product
surface. Fixing (2) would add an affirmative statement to the output, which changes the surface
after the acceptance criteria were frozen — a decision for the owner, not for this record.

**N-V01-1 — protocol delivery.** The instructions handed to the subject used `<dir>` as a
placeholder, and the shell read it as a redirection until the subject corrected it. Defect of
how the protocol was delivered, not of the tool. Disposition: use a literal path in the
instructions, or quote the placeholder, in any future run.

### 5.6 What is still missing

The **raw transcript of the six commands**, required by §2's recording rule. It has not been
received. Until it arrives this section is not closed, Q1 cannot be independently verified, and
the finding classes above rest on the subject's summary rather than on the output itself.
