<!-- Reply to send-1.md, pasted by the owner on 2026-10-03, saved verbatim. Model: not stated by the owner. -->

Independent reading of the captured umbral session
I worked only from the material in the prompt. No lookups, no external tools.
0. What the tool reports, in brief
umbral (tool-version 0.1.0, output contract umbral-output/1) keeps a per-workspace state directory and, on observe, walks the subject directory recording per entry a metadata set (dev, ino, kind, size, mtime, ctime) and a content record (hash, size, bytes). When metadata is unchanged it reuses a prior content reading instead of re-reading.
Three observations were made of /tmp/umbral-check-02/subject:
- Run 1 — complete. 14 entries = 13 files + 1 directory; all 13 files read fresh (90 bytes).
- Run 2 — incomplete (exit 3). 13 entries = 12 files + 1 dir. The subdirectory locked/ could not be descended (mode 000), so its one file was invisible; only the edited a.txt was re-read (1 entry, 14 bytes).
- Run 3 — complete (exit 0). 13 entries = 12 files + 1 dir; 3 files read fresh (22 bytes), 9 reused.
Between runs 2 and 3, c.txt was renamed to d.txt (same dev/ino), link-two was deleted (its inode survives through the hard-link name link-one), and locked/inside.txt became visible again. changes compares runs 2→3 and reports 11 unchanged, 1 renamed, 1 deleted, 1 "created" — the last being an artifact of run 2's incomplete traversal, which the tool itself flags (reference-complete=false).
Throughout, each line is class␣␣key=value…, with class observed (raw filesystem fact), derived (computed by the tool), or unknown (not determinable). That three-way labelling is an inference from how the lines are used, not something the output states.
1. Every line, by command (Q1)
1.1 umbral init …/subject
Line    About    States
derived contract=umbral-output/1    whole run    output schema version
observed canonical=/tmp/umbral-check-02/subject    whole run    canonicalised subject path (observed directly)
derived root=/tmp/umbral-check-02/subject    whole run    traversal root
derived workspace-id=242c37d665addf11    whole run    64-bit workspace id
derived state-dir=…/data/umbral/ws-242c37d665addf11    whole run    where state is stored, keyed by the workspace id
derived initialised=true    whole run    initialisation succeeded
exit=0    process    success
1.2 umbral observe (runs 1–3)
The observe block has a fixed schema. For every run the lines are:
Line (schema)    About    States
derived contract=…    whole run    schema version
observed canonical=…    whole run    canonical path
derived run=N root=…    run N    root
derived run=N entries=E files=F dirs=D symlinks=S other=O kind-unknown=K    run N    entry census
derived run=N metadata-fresh=… metadata-failed=…    run N    stat outcomes
derived run=N content-fresh=… content-reused=… content-failed=… content-not-attempted=… content-not-recorded=…    run N    content-read outcomes
derived run=N content-failed-diagnostics unstable-observation=… not-found=… permission-denied=… not-a-regular-file=… read-error=…    run N    breakdown of content-read failures (a labelled line; all counts zero in all three runs)
derived run=N traversal-complete=… traversal-not-descended=… traversal-metadata-failed=… traversal-not-recorded=… root-not-descended=…    run N    traversal outcomes
derived run=N tool-version=0.1.0 scope=recursive,symlinks-not-followed,no-exclusions    run N    tool version and walk scope
derived run=N content-read-entries=… content-read-bytes=…    run N    how much content was actually read this run
derived run=N started=… finished=…    run N    timestamps
derived run=N complete=…    run N    completeness verdict
exit=N    process    0 success, 3 incomplete observation (inferred)
Per-run values:
Field    run 1    run 2    run 3
entries / files / dirs    14 / 13 / 1    13 / 12 / 1    13 / 12 / 1
symlinks / other / kind-unknown    0 / 0 / 0    0 / 0 / 0    0 / 0 / 0
metadata-fresh / failed    14 / 0    13 / 0    13 / 0
content-fresh / reused / failed    13 / 0 / 0    1 / 11 / 0    3 / 9 / 0
content-not-attempted / not-recorded    0 / 0    0 / 0    0 / 0
content-failed-diagnostics (all five)    0    0    0
traversal-complete / not-descended    true / 0    false / 1    true / 0
traversal-metadata-failed / not-recorded    0 / 0    0 / 0    0 / 0
root-not-descended    false    false    false
content-read-entries / bytes    13 / 90    1 / 14    3 / 22
started = finished    07:25:19.335Z    07:25:20.537Z    07:25:21.739Z
complete    true    false    true
exit    0    3    0
Note: entries appears to exclude the root itself (13 files + 1 dir = 14, and dirs=1 although both the root and locked/ are directories). That the root is excluded is an inference; the output does not say so.
1.3 umbral status
Line    About    States
derived contract=…    whole run    schema version
observed canonical=…    whole run    canonical path
derived root=…    whole run    root
derived workspace-id=242c37d665addf11    whole run    workspace id
derived last-run=3 started=2026-10-03T07:25:21.739Z    whole workspace    index and start time of the most recent run
derived last-run=3 complete=true    whole workspace    most recent run is complete
derived run=3 entries=13 …    run 3    repeat of run-3 census
derived run=3 metadata-fresh=13 …    run 3    repeat
derived run=3 content-fresh=3 content-reused=9 …    run 3    repeat
derived run=3 content-failed-diagnostics …    run 3    repeat
derived run=3 traversal-complete=true …    run 3    repeat
derived run=3 tool-version=0.1.0 scope=…    run 3    repeat
derived log-runs=3 log-observations=40    whole workspace    3 runs, 40 stored observation records
exit=0    process    success
log-observations=40 is exactly 14 + 13 + 13 — one record per entry per run. status repeats only a subset of the observe block (no content-read-entries/bytes, no finished).
1.4 umbral changes
Line    About    States
derived contract=…    whole run    schema version
derived compared reference-run=2 compared-run=3 reference-complete=false compared-complete=true    whole comparison    which two runs are compared and their completeness
derived count unchanged=11    comparison    11 entries identical in both
derived count modified=0    comparison    none
derived count created=1    comparison    one entry present only in run 3
derived count deleted=1    comparison    one entry present only in run 2
derived count unobserved=0    comparison    none (meaning not fully derivable — see §6)
derived count renamed-or-moved=1    comparison    one identity-preserving rename
derived count recreated=0    comparison    none
derived count ambiguous=0    comparison    none
derived renamed-or-moved path=d.txt …    c.txt(2) vs d.txt(3)    rename concluded (details §3)
derived deleted path=link-two …    link-two(2) vs link-one(3)    deletion concluded, object survives
derived created path=locked/inside.txt …    locked/inside.txt(3)    creation concluded, reference incomplete
exit=0    process    success
1.5 umbral show <path> (seven invocations)
Each show prints, for the path: contract, then per available run an observation pair —
- derived observation=R:path hash=… stability=… metadata=… content=… [content-source=…]
- observed observation=R:path kind=… size=… mtime=… ctime=…
— followed by cross-run comparison lines and exit=0.
a.txt
- obs 1: hash=ac678d92b3d7, size=6, mtime=ctime=19.332Z, content=fresh.
- obs 2: hash=29a077efa2d8, size=14, mtime=ctime=20.535Z, content=fresh.
- obs 3: hash=29a077efa2d8, size=14, mtime=ctime=20.535Z, content=reused content-source=2:a.txt.
- modified 1→2, content-changed=true, reference-complete=true compared-complete=false.
- unchanged 2→3, content-changed=false, reference-complete=false compared-complete=true.
b.txt
- obs 1: hash=2001794aa22d, size=6, mtime=ctime=19.332Z, fresh.
- obs 2: same hash, content=reused content-source=1:b.txt.
- obs 3: same hash, content=reused content-source=1:b.txt.
- unchanged 1→2 and unchanged 2→3 (content-changed=false).
d.txt
- obs 3 only: hash=6fefa7c34afd, size=8, mtime=19.332Z, ctime=21.737Z, content=fresh. No comparison lines (no earlier observation under this name).
link-one
- obs 1: hash=7fde9f117e69, size=7, mtime=ctime=19.332Z, fresh.
- obs 2: same, reused source=1:link-one.
- obs 3: same hash, mtime=19.332Z, ctime=21.737Z, content=fresh.
- unchanged 1→2 and unchanged 2→3.
locked (a directory)
- obs 1: kind=dir size=60 mtime=ctime=19.332Z, hash=none stability=none.
- obs 2: kind=dir size=60 mtime=19.332Z ctime=20.535Z, hash=none stability=none.
- unknown observation=2:locked traversal=not-descended observation-error=IO error for operation on /tmp/umbral-check-02/subject/locked: Permission denied (os error 13)
- obs 3: kind=dir size=60 mtime=19.332Z ctime=21.737Z, hash=none stability=none.
- unchanged 1→2 (fields dev,ino,kind); unchanged 2→3 (fields printed as dev,ino in the capture — see §6).
line\x0Abreak.txt
- All three observations carry observation-encoding=escaped:control-character.
- obs 1: hash=0a61c6a450a2 size=7, fresh.
- obs 2: reused content-source=1:line\x0Abreak.txt content-source-encoding=escaped:control-character.
- obs 3: same, reused source=1.
- unchanged 1→2, unchanged 2→3, each with path-encoding / reference-encoding / compared-encoding fields.
bytes-\xFF.txt
- All three observations carry observation-encoding=escaped:not-valid-utf8.
- obs 1: hash=bac25e1466ab size=7, fresh; obs 2 and 3 reused source=1.
- unchanged 1→2, unchanged 2→3, with the encoding fields.
2. Provenance of each run-3 file (Q2)
Run-3 path    Read in run 3, or earlier?    Source    Evidence
a.txt    earlier reading used    run 2 (2:a.txt)    observation=3:a.txt … content=reused content-source=2:a.txt
b.txt    earlier reading used    run 1 (1:b.txt)    … content=reused content-source=1:b.txt
d.txt    read fresh in run 3    —    observation=3:d.txt … content=fresh
link-one    read fresh in run 3    —    observation=3:link-one … content=fresh
line\x0Abreak.txt    earlier reading used    run 1    content=reused content-source=1:line\x0Abreak.txt
bytes-\xFF.txt    earlier reading used    run 1    content=reused content-source=1:bytes-\xFF.txt
locked    n/a (directory, no content)    —    hash=none, kind=dir
locked/inside.txt    cannot be determined from the output    —    only changes mentions it (created, §3); no show was run for it
The aggregate confirms the picture: run 3 has content-fresh=3 content-reused=9 and content-read-entries=3 content-read-bytes=22. The two fresh reads that are named — d.txt (8 bytes) and link-one (7 bytes) — account for 15 of the 22 bytes, leaving one fresh 7-byte read. There are 12 files in run 3 (7 named + 5 never named); the four named reused files plus the five unnamed files exactly fill content-reused=9. That leaves six candidates for the one remaining fresh read (locked/inside.txt and the five unnamed files), of which five must be reused. The output does not identify which one was fresh. locked/inside.txt is the most plausible candidate (it is the entry whose observability changed between runs), but this is inference, not something the output states.
One important reading detail: when a file is reused, the hash printed for the new observation is the earlier reading, not a new measurement (a.txt run 3 shows run 2's hash; b.txt run 3 shows run 1's hash). So run-3 "hash" values for reused files are carried over, not recomputed.
3. The changes lines (Q3)
renamed-or-moved path=d.txt reference=2:c.txt reference-fields=dev,ino compared=3:d.txt compared-fields=dev,ino …
- Concerns: c.txt (run 2) and d.txt (run 3).
- Concludes: the entry was renamed/moved.
- Rests on: identical dev + ino (the same inode) under two different names. It does not rest on content or name similarity, and it does not assert the content is unchanged (only dev,ino were compared).
deleted path=link-two reference=2:link-two reference-fields=dev,ino compared-absent=3 counterpart=3:link-one counterpart-fields=dev,ino object-survives=true …
- Concerns: link-two (run 2) and link-one (run 3).
- Concludes: link-two was deleted.
- Rests on: link-two present in run 2 and absent in run 3; and a counterpart link-one that shares the same dev,ino, hence the same inode. object-survives=true means the underlying object still has a name (link-one) — i.e. a hard-link name was removed, not the data.
created path=locked/inside.txt reference-absent=2 compared=3:locked/inside.txt compared-fields=dev,ino …
- Concerns: locked/inside.txt.
- Concludes: the entry was created.
- Rests on: absence from the run-2 reference and presence in run 3. Crucially, reference-complete=false: run 2 never descended locked/, so the absence is explained by the failed traversal, not by the file's non-existence. The "created" label is therefore relative to an incomplete reference, and the tool flags that.
Count lines (unchanged=11, modified=0, created=1, deleted=1, unobserved=0, renamed-or-moved=1, recreated=0, ambiguous=0): whole-comparison tallies. They are consistent with the detail lines and with the entry counts: run 2's 13 entries = 11 unchanged + c.txt (renamed) + link-two (deleted); run 3's 13 = 11 unchanged + d.txt (renamed) + locked/inside.txt (created). modified=0 is consistent because a.txt's edit happened between runs 1 and 2, and changes compares 2→3.
4. Which observations were complete (Q4)
- Run 1 — complete (complete=true, traversal-complete=true, traversal-not-descended=0, exit=0). Everything was seen.
- Run 2 — incomplete (complete=false, traversal-complete=false, traversal-not-descended=1, exit=3). What was not seen: the contents of locked/, specifically locked/inside.txt. The reason is stated explicitly in the unknown line under show locked: IO error … Permission denied (os error 13) — the directory had been made mode 000. The file-count arithmetic confirms exactly one file was missed (run 1 files=13 → run 2 files=12, with no other file added or removed between those runs), and changes later names that file locked/inside.txt.
- Run 3 — complete (complete=true, exit=0).
Every cross-run comparison also carries completeness flags, and they line up: any comparison involving run 2 shows reference-complete=false (when run 2 is the reference) or compared-complete=false (when run 2 is the compared side). Run 1 and run 3 are always complete=true.
Note the separate axis: run 2's metadata was fresh for all 13 entries and its content failures were zero, yet the run is incomplete because traversal failed. Incompleteness here is about not seeing entries, not about failing to read ones it saw.
5. The unusual names (Q5)
Two names contain bytes that cannot be written literally in the output; both are rendered with backslash-hex escapes and tagged with an observation-encoding field.
- line\x0Abreak.txt — the \x0A is byte 0x0A (a newline). The actual name is the 14 bytes l i n e 0x0A b r e a k . t x t, i.e. line␊break.txt. The tag observation-encoding=escaped:control-character confirms the escape represents a real control byte. Reconstructable exactly.
- bytes-\xFF.txt — the \xFF is the single byte 0xFF, which is not valid UTF-8. The actual name is bytes- + 0xFF + .txt. The tag observation-encoding=escaped:not-valid-utf8 confirms it. Reconstructable exactly.
For both, show also prints path-encoding, reference-encoding, compared-encoding, and content-source-encoding, so the escaping is self-documenting at every occurrence.
Names I cannot reconstruct: the five files that are only ever counted, never listed (each run has 13/12/12 files of which only 7–8 are ever named). Their names do not appear anywhere in the material. Also, the exact escaped rendering is the tool's convention; the output documents it via the *-encoding fields, so the two shown names are recoverable, but any name containing a literal backslash would in principle be ambiguous — no such case appears.
6. Contradictory, ambiguous, or guess-dependent points (Q6)
1. created locked/inside.txt versus reality. The clearest case. The file was not created between runs 2 and 3; it was merely not seen in run 2 because locked/ was unreadable. The output is transparent about this (reference-complete=false), but the label "created" cannot be read as a statement about the filesystem. It is a statement relative to an incomplete reference, and only understanding that makes the line correct.
2. permission-denied=0 despite a Permission denied error. Run 2's content-failed-diagnostics line reports permission-denied=0, yet the same run produced Permission denied (os error 13). This is not actually contradictory — the diagnostics line counts content-read failures, whereas the error is a traversal failure counted under traversal-not-descended=1 and shown as an unknown line under show locked — but the naming invites misreading. The output does not explicitly state that the two diagnostics are disjoint.
3. deleted link-two with object-survives=true. Reads as a contradiction ("deleted" yet "survives"); it means the name was deleted while the inode still has another name (link-one). That both names are hard links is itself inferred from the shared dev,ino, not stated.
4. Reuse versus "unchanged". link-one was re-read fresh in run 3 (content=fresh) yet is reported unchanged in the 2→3 comparison, because the comparison fields are dev,ino,kind,size,mtime,hash and none of those differ. The re-read evidently happened because its ctime changed (deleting link-two bumped the inode's ctime), but ctime is not among the comparison fields. So the rule that drives reuse cannot be fully derived from the printed comparison fields; it must be guessed from the correlation (ctime changed → fresh; ctime unchanged → reused).
5. Circularity in content-changed=false. For every reused file the run-3 hash is the earlier hash (carried over). The unchanged … content-changed=false verdict therefore does not rest on a fresh content comparison — it is entailed by the reuse decision, which rests on metadata equality. The output labels this (content=reused, content-source=…), so it is honest, but the hash should not be read as new evidence.
6. unobserved=0 despite an entry missing from run 2. locked/inside.txt is absent from run 2, yet unobserved=0. So "unobserved" does not mean "absent from one of the compared runs"; its exact meaning is not derivable from the output. (It appears to mean something like "present but not fully comparable", but that is a guess.)
 7. The differing field list on the locked comparisons. The 1→2 comparison prints reference-fields=dev,ino,kind and compared-fields=dev,ino,kind; the 2→3 comparison prints the shorter reference-fields=dev,ino and compared-fields=dev,ino (as I read the capture). If that is not a transcription artifact, the output gives no reason for the difference — kind (dir) was observed in both runs. This is unexplained.
 8. The third fresh read in run 3 is unidentified. The aggregate (content-fresh=3, content-read-bytes=22) and the named fresh files (d.txt 8 bytes, link-one 7 bytes) require one more fresh 7-byte read, but the output never names it. Candidates are locked/inside.txt or one of the five unnamed files; the counts cannot distinguish them. Answering "which file" requires guessing.
 9. content-source semantics. b.txt in run 3 points to 1:b.txt, not 2:b.txt, even though run 2 also observed it. So content-source names the run where the content was actually read, not the immediately preceding observation; that reuse can skip runs is inferred from b.txt, line…, and bytes…, not stated.
10. Root counted or not? entries=14 files=13 dirs=1 in run 1 forces the reading that the root is excluded from the census (otherwise dirs would be 2, since locked/ is also a directory). The output never says the root is excluded.
11. Output-shape differences. init, observe, and status print observed canonical=…; changes and show do not. status omits content-read-entries/bytes and finished that observe prints. show d.txt prints only run 3 and no comparison lines, so the c.txt→d.txt continuity is invisible there and must be learned from changes. These are not errors, but they mean no single command shows the whole picture.
12. link-one/link-two are named "link" but reported kind=file with symlinks=0. The names suggest symlinks; the tool says regular files (hard links, per changes). Understanding this requires the cross-reference, not the names.
13. Instantaneous-looking timestamps. Each run's started equals its finished to the millisecond, and the run times are ~2 ms after the corresponding file mutations. Not contradictory, but it means the runs were extremely short and the output gives no finer timing.
14. The content-failed-diagnostics line's leading token has no =. Its format (a bare category label followed by counts) can only be understood by guessing that the first token is a heading rather than a key/value pair. Every other line is uniformly key=value.
