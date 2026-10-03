# E-TD-6 and E-TD-11 — an independent reading of the v0.2 output

Status: **SPECIFIED 2026-10-03, BEFORE THE MATERIAL IS DELIVERED.** Protocols:
[`V0.2-TECHNICAL-DESIGN.md`](../../docs/candidates/V0.2-TECHNICAL-DESIGN.md) §H.3 (E-TD-11, pre-registered)
and §H.4 (E-TD-6, listed). Requested by the owner before closing v0.2 (private instruction,
2026-10-03). Results go in §6 of this file, after the readings return.

- Evidence class: **AI-INDEPENDENT** (`UD-015`). It can find defects and unblock decisions. It
  **cannot satisfy A2-V13**, which requires a human reader, and it says nothing about A1.
- Mode: **SEMI-BLIND** — the participant gets the objective, never our conclusions, as in
  EXP-AI-01.
- Delivery: the owner delivers the three messages to one external AI, in order, each only after
  the previous reply, and saves every reply verbatim (`responses/`). The author does not
  contact, select or brief the participant.

## 1. The three messages

| Message | Gives | Asks | Experiment |
|---|---|---|---|
| `send-1.md` | the captured session only — **no contract** | what each line says; for each file, whether its bytes were read in the last run or an earlier reading was used; which runs were complete; how unusual names are written | **E-TD-6**, and the question A2-V13 asks (P2b) |
| `send-2.md` | the same session, the contract (`umbral/CONTRACT.md`) and the README sections that define the fields, and six transformed versions | for each result: its subject, and the observation(s) and field set of its evidence; whether any transformation changes or loses that | **E-TD-11 step 1** — can produce D1, D2, D3 |
| `send-3.md` | the durable record (the log, as text) | resolve every reference and state the values; do they match what the result says | **E-TD-11 step 2** — can produce D4 |

Message 1 comes first so that the contract cannot inform the legibility reading, and message 3
last so that the record cannot inform step 1 — the separation §H.3 requires.

## 2. Fixture — §H.3's, built by `capture_material.py`

Three runs over one directory:

- run 1: every entry new;
- run 2: `a.txt` edited; `locked/` unreadable (an incomplete run, `traversal=not-descended`);
- run 3: `c.txt` renamed to `d.txt`; one of two hard links (`link-two`) deleted; `locked/`
  readable again; `b.txt` never touched (its reading carried since run 1).

Names, each its own file: `two  spaces.txt`, `line⏎break.txt` (LF), `back\slash⏎nl.txt`
(backslash and LF), `cr␍name.txt` (CR), `tab⇥name.txt` (TAB), `bytes-\xFF.txt` (not UTF-8),
`sep  \ back.txt` (the delimiter and a backslash).

## 3. Deviations from the pre-registration, stated before delivery

- **Names changed since §H.3 was written.** It speaks of `basis=content` and `metadata-stable`;
  the implemented output says `content=fresh` and `content=reused` (`UD-031`, `UD-033`). The
  hypotheses are read with the new names; nothing else changes.
- **H11-d is vacuous by design.** It asks whether a carried reading after a rename is locatable.
  `UD-036` decided that the skip compares only the same path, so a renamed file is read, never
  carried. The fixture keeps the rename; the expected answer is a fresh reading at the new path.
  A carried reading is still present (`b.txt`, H11-g's case).
- **The record is given as text** (`record.txt`, every row of `run` and `observation`), not as a
  SQLite file, so a reader without tools can use it. The bytes are not changed; blobs are written
  in hexadecimal.
- **Transformations** (§H.3's six) are produced mechanically by the capture script:
  1. reordering — every output's lines permuted (fixed seed);
  2. subject removal — `path=` and `observation=` deleted from every line;
  3. re-flow — every line wrapped at 60 columns;
  4. delimiter split — every item on its own line;
  5. serialization round-trip — to JSON per the contract, and back;
  6. position change — every reference field moved to the end of its line.

## 4. What counts as a result

Each failure is reported under exactly one of §H.3's classes (D1 applicability, D2 provenance,
D3 grammar, D4 durable reconstruction), with D3 taking precedence over D2 and D1 over D2.
"No ambiguity found in this configuration" is a complete result.

## 5. Material

*(frozen when generated: file names, sizes and sha256 recorded here before delivery)*

## 6. Results

*(empty until the readings return)*
