<!-- Reply to send-3.md, pasted by the owner on 2026-10-03, saved verbatim. Model: not stated by the owner. -->

Final step — the stored log checked against the session's references
I read the record as the persisted form of the same three observations. Two columns are the key to this task:
- hash_read_run — the run whose observation actually read and hashed the bytes. NULL for a directory.
- hash_hex — the full digest, of which the session's output prints only the first 12 hex digits.
The record contains no duplicate (run_id, path) row, so every well-formed reference names at most one row. schema_meta says schema_version=umbral-v0.2.1 and ctime_recorded_from_run=1 (so ctime is present for all three runs).
1. The observation index the references resolve against
Decoded path names (from path_hex), because five of them never appear in the session output:
stored/escaped    actual bytes
back\\slash\x0Anl.txt    back\slash + 0x0A + nl.txt
cr\x0Dname.txt    cr + 0x0D + name.txt
sep\x20\x20\\ back.txt    sep + 0x20 0x20 + 0x5C + 0x20 + back.txt
tab\x09name.txt    tab + 0x09 + name.txt
two\x20\x20spaces.txt    two + 0x20 0x20 + spaces.txt
Run 1 (14 rows; every file hash_read_run=1):
path    dev    ino    kind    size    mtime (s.ns)    ctime (s.ns)    hash_hex (12)
a.txt    40    154802    file    6    1791012319.332360814    …332360814    ac678d92b3d7
b.txt    40    154803    file    6    …332384033    …332384033    2001794aa22d
back\slash␊nl.txt    40    154809    file    7    …332481888    …332481888    d4b2775a7094
bytes-ÿ.txt    40    154812    file    7    …332521859    …332521859    bac25e1466ab
c.txt    40    154804    file    8    …332399471    …332399471    6fefa7c34afd
cr␍name.txt    40    154810    file    7    …332495297    …332495297    02abd08cc2e6
line␊break.txt    40    154808    file    7    …332467985    …332467985    0a61c6a450a2
link-one    40    154806    file    7    …332426636    …332426636    7fde9f117e69
link-two    40    154806    file    7    …332426636    …332426636    7fde9f117e69
locked    40    154801    dir    60    …332399471    …332399471    (NULL)
locked/inside.txt    40    154805    file    7    …332413286    …332413286    5e8dd55c207a
sep␠␠\␠back.txt    40    154813    file    7    …332534974    …332534974    87ee89c134c8
tab␉name.txt    40    154811    file    7    …332508786    …332508786    522b4b038576
two␠␠spaces.txt    40    154807    file    7    …332452648    …332452648    6b338193c78d
Run 2 (13 rows; no locked/inside.txt):
path    dev    ino    kind    size    mtime    ctime    hash_hex(12)    hash_read_run
a.txt    40    154802    file    14    1791012320.535922078    …535922078    29a077efa2d8    2
b.txt    40    154803    file    6    …332384033    …332384033    2001794aa22d    1
back\slash␊nl.txt    40    154809    file    7    …332481888    …332481888    d4b2775a7094    1
bytes-ÿ.txt    40    154812    file    7    …332521859    …332521859    bac25e1466ab    1
c.txt    40    154804    file    8    …332399471    …332399471    6fefa7c34afd    1
cr␍name.txt    40    154810    file    7    …332495297    …332495297    02abd08cc2e6    1
line␊break.txt    40    154808    file    7    …332467985    …332467985    0a61c6a450a2    1
link-one    40    154806    file    7    …332426636    …332426636    7fde9f117e69    1
link-two    40    154806    file    7    …332426636    …332426636    7fde9f117e69    1
locked    40    154801    dir    60    …332399471    1791012320.535922078    (NULL)    NULL
sep␠␠\␠back.txt    40    154813    file    7    …332534974    …332534974    87ee89c134c8    1
tab␉name.txt    40    154811    file    7    …332508786    …332508786    522b4b038576    1
two␠␠spaces.txt    40    154807    file    7    …332452648    …332452648    6b338193c78d    1
(locked run 2 also carries obs_error=… Permission denied (os error 13) and traversal=not-descended.)
Run 3 (13 rows; c.txt gone, d.txt added, locked/inside.txt back):
path    dev    ino    kind    size    mtime    ctime    hash_hex(12)    hash_read_run
a.txt    40    154802    file    14    1791012320.535922078    …535922078    29a077efa2d8    2
b.txt    40    154803    file    6    …332384033    …332384033    2001794aa22d    1
back\slash␊nl.txt    40    154809    file    7    …332481888    …332481888    d4b2775a7094    1
bytes-ÿ.txt    40    154812    file    7    …332521859    …332521859    bac25e1466ab    1
cr␍name.txt    40    154810    file    7    …332495297    …332495297    02abd08cc2e6    1
d.txt    40    154804    file    8    …332399471    1791012321.737049936    6fefa7c34afd    3
line␊break.txt    40    154808    file    7    …332467985    …332467985    0a61c6a450a2    1
link-one    40    154806    file    7    …332426636    1791012321.737049936    7fde9f117e69    3
locked    40    154801    dir    60    …332399471    1791012321.737049936    (NULL)    NULL
locked/inside.txt    40    154805    file    7    …332413286    …332413286    5e8dd55c207a    3
sep␠␠\␠back.txt    40    154813    file    7    …332534974    …332534974    87ee89c134c8    1
tab␉name.txt    40    154811    file    7    …332508786    …332508786    522b4b038576    1
two␠␠spaces.txt    40    154807    file    7    …332452648    …332452648    6b338193c78d    1
Counts fall out correctly: run 1 → 13 files with hash_read_run=1; run 2 → 1 file with hash_read_run=2 (a.txt), 11 with hash_read_run=1; run 3 → 3 files with hash_read_run=3 (d.txt, link-one, locked/inside.txt), 9 with earlier runs. Total rows 14+13+13 = 40 = log-observations.
2. Reference-by-reference audit
2.1 changes (reference-run=2, compared-run=3)
Reference    Row found    Listed fields in the record    Consistent?
reference=2:c.txt (reference-fields=dev,ino)    (2, c.txt)    dev=40, ino=154804    ✅
compared=3:d.txt (compared-fields=dev,ino)    (3, d.txt)    dev=40, ino=154804    ✅ — equal dev,ino to the reference, so renamed-or-moved is supported
reference=2:link-two (reference-fields=dev,ino)    (2, link-two)    dev=40, ino=154806    ✅
counterpart=3:link-one (counterpart-fields=dev,ino)    (3, link-one)    dev=40, ino=154806    ✅ — same inode as the deleted name, so object-survives=true is supported
compared-absent=3 (for link-two)    no row    —    ✅ correct: run 3 has no link-two row
reference-absent=2 (for locked/inside.txt)    no row    —    ✅ correct: run 2 has no locked/inside.txt row
compared=3:locked/inside.txt (compared-fields=dev,ino)    (3, locked/inside.txt)    dev=40, ino=154805    ✅
All changes references resolve to exactly one row. The two absence markers correctly name no row. The counts (unchanged=11, modified=0, created=1, deleted=1, renamed-or-moved=1) are consistent: the 11 common paths are a, b, back\slash, bytes, cr, line, link-one, locked, sep, tab, two.
2.2 show a.txt
Reference    Row    Listed fields    Consistent?
observation=1:a.txt    (1,a.txt)    kind=file size=6 mtime=…19.332360814 ctime=…19.332360814; hash=ac678d92b3d7…    ✅ (output prints exactly these; content=fresh, hash_read_run=1)
observation=2:a.txt    (2,a.txt)    kind=file size=14 mtime=…20.535922078 ctime=…20.535922078; hash=29a077efa2d8…    ✅ (content=fresh, hash_read_run=2)
observation=3:a.txt    (3,a.txt)    kind=file size=14 mtime=…20.535922078 ctime=…20.535922078; hash=29a077efa2d8…    ✅
content-source=2:a.txt    (2,a.txt)    hash=29a077efa2d8…, hash_read_run=2    ✅ — run 3's bytes really were read in run 2
reference=1:a.txt (fields dev,ino,kind,size,mtime,hash)    (1,a.txt)    dev=40 ino=154802 file 6 …332360814 ac678d92b3d7    ✅
compared=2:a.txt (same fields)    (2,a.txt)    dev=40 ino=154802 file 14 …535922078 29a077efa2d8    ✅ — size/mtime/hash all differ → modified, content-changed=true
reference=2:a.txt    (2,a.txt)    as above    ✅
compared=3:a.txt    (3,a.txt)    dev=40 ino=154802 file 14 …535922078 29a077efa2d8    ✅ — all listed fields equal → unchanged, content-changed=false
2.3 show b.txt
Reference    Row    Listed fields    Consistent?
observation=1:b.txt    (1,b.txt)    file 6 …332384033; hash=2001794aa22d…    ✅ fresh
observation=2:b.txt    (2,b.txt)    file 6 …332384033; hash=2001794aa22d…    ✅
observation=3:b.txt    (3,b.txt)    file 6 …332384033; hash=2001794aa22d…    ✅
content-source=1:b.txt (on obs 2 and obs 3)    (1,b.txt)    hash_read_run=1    ✅ — both reused readings trace to run 1
reference=1:b.txt / compared=2:b.txt (dev,ino,kind,size,mtime,hash)    (1,b.txt),(2,b.txt)    dev=40 ino=154803 file 6 …332384033 2001794aa22d — identical    ✅ unchanged
reference=2:b.txt / compared=3:b.txt    (2,b.txt),(3,b.txt)    identical    ✅ unchanged
2.4 show d.txt
Reference    Row    Listed fields    Consistent?
observation=3:d.txt    (3,d.txt)    kind=file size=8 mtime=…19.332399471 ctime=…21.737049936; hash=6fefa7c34afd…, hash_read_run=3    ✅ content=fresh
No comparison line, hence no reference=/compared= in this output.
2.5 show link-one
Reference    Row    Listed fields    Consistent?
observation=1:link-one    (1,link-one)    file 7 …332426636; 7fde9f117e69    ✅ fresh
observation=2:link-one    (2,link-one)    file 7 …332426636; 7fde9f117e69, hash_read_run=1    ✅
observation=3:link-one    (3,link-one)    file 7 mtime …332426636 ctime …737049936; 7fde9f117e69, hash_read_run=3    ✅ content=fresh (ctime moved)
content-source=1:link-one    (1,link-one)    hash_read_run=1    ✅
reference=1:link-one / compared=2:link-one    (1),(2)    dev=40 ino=154806 file 7 …332426636 7fde9f117e69 — identical    ✅ unchanged
reference=2:link-one / compared=3:link-one    (2),(3)    identical on the six listed fields (ctime differs but is not listed)    ✅ unchanged
2.6 show locked
Reference    Row    Listed fields    Consistent?
observation=1:locked    (1,locked)    kind=dir size=60 mtime=…332399471 ctime=…332399471; hash NULL    ✅
observation=2:locked    (2,locked)    kind=dir size=60 mtime=…332399471 ctime=…20.535922078; obs_error=… Permission denied, traversal=not-descended    ✅ matches the unknown line's error
observation=3:locked    (3,locked)    kind=dir size=60 mtime=…332399471 ctime=…21.737049936    ✅
reference=1:locked / compared=2:locked (dev,ino,kind)    (1),(2)    dev=40 ino=154801 dir — identical    ✅ unchanged
reference=2:locked / compared=3:locked (dev,ino, or dev,ino,kind)    (2),(3)    dev=40 ino=154801 (kind=dir) — identical    ✅ unchanged either way
2.7 show line\x0Abreak.txt
Reference    Row    Listed fields    Consistent?
observation=1:line\x0Abreak.txt    (1)    file 7 …332467985; 0a61c6a450a2    ✅ fresh
observation=2:…    (2)    file 7 …332467985; 0a61c6a450a2, hash_read_run=1    ✅
observation=3:…    (3)    file 7 …332467985; 0a61c6a450a2, hash_read_run=1    ✅
content-source=1:line\x0Abreak.txt (obs 2, obs 3)    (1)    hash_read_run=1    ✅
reference=1: / compared=2: and reference=2: / compared=3:    (1),(2),(3)    dev=40 ino=154808 file 7 …332467985 0a61c6a450a2 — all equal    ✅ both unchanged
2.8 show bytes-\xFF.txt
Reference    Row    Listed fields    Consistent?
observation=1:bytes-\xFF.txt    (1)    file 7 …332521859; bac25e1466ab    ✅ fresh
observation=2:…    (2)    file 7 …332521859; bac25e1466ab, hash_read_run=1    ✅
observation=3:…    (3)    file 7 …332521859; bac25e1466ab, hash_read_run=1    ✅
content-source=1:bytes-\xFF.txt (obs 2, obs 3)    (1)    hash_read_run=1    ✅
reference=1: / compared=2: and reference=2: / compared=3:    (1),(2),(3)    dev=40 ino=154812 file 7 …332521859 bac25e1466ab — all equal    ✅ both unchanged
2.9 Summary of the audit
- References naming no row: only reference-absent=2 (for locked/inside.txt) and compared-absent=3 (for link-two). Both are absence markers, and both are correct.
- References naming more than one row: none — every (run_id, path) is unique, and no path contains :.
- Value mismatches: none. Every listed field (dev,ino,kind,size,mtime,hash) matches the record, and every printed hash prefix matches the first 12 hex digits of the full hash_hex.
3. Can the record say where bytes were actually read? Yes — hash_read_run
hash_read_run names the run whose observation produced the hash, so for any reused reading the record states the true source:
Observation    File    content= in output    hash_read_run    Bytes actually read in
2:a.txt    a.txt    fresh    2    run 2
3:a.txt    a.txt    reused    2    run 2
2:b.txt    b.txt    reused    1    run 1
3:b.txt    b.txt    reused    1    run 1
2:link-one    link-one    reused    1    run 1
3:link-one    link-one    fresh    3    run 3
3:d.txt    d.txt    fresh    3    run 3
3:locked/inside.txt    locked/inside.txt    (not shown by show)    3    run 3
2 / 3:line\x0Abreak.txt    line…    reused    1    run 1
2 / 3:bytes-\xFF.txt    bytes…    reused    1    run 1
(all run-1 files)    —    fresh    1    run 1
This resolves the one thing the session output alone could not: the third fresh read of run 3 is locked/inside.txt (hash_read_run=3, 7 bytes). With d.txt (8) and link-one (7) that is exactly the content-read-bytes=22 and content-read-entries=3 reported by run 3. It was re-read because it was a path new to run 3 (run 2 never descended locked/), even though its metadata matches run 1.
The record also supplies the names the output never printed — back\slash␊nl.txt, cr␍name.txt, sep␠␠\␠back.txt, tab␉name.txt, two␠␠spaces.txt — so the five files that were only ever counted are now identified.
4. Points that remain inconsistent, ambiguous, or need a caveat
1. The show locked 2→3 field list is still unresolved. The session capture reads reference-fields=dev,ino / compared-fields=dev,ino; all six transformed copies read dev,ino,kind. The record supports both (dev=40, ino=154801, kind=dir are equal on both sides), so it cannot settle which field list the writer actually emitted. This is a discrepancy between the session and its transformed copies, not between output and record.
2. unchanged for a.txt 2→3 rests on a carried hash. The comparison lists hash among its fields and finds it equal, but hash_read_run=2 shows that value was not read in run 3. The result is consistent, but its content-changed=false is a statement about a reused value, exactly as the spec labels it.
3. ctime is recorded for every row but never appears in any *-fields= list. All comparisons over files and the directory are settled without it, yet the record's ctime is precisely what explains the fresh reads of link-one (ctime moved when link-two was unlinked) and d.txt/locked/inside.txt (new paths). The field lists shown in the results do not reveal the skip decision's real inputs.
4. schema_version=umbral-v0.2.1 versus the output's tool-version=0.1.0. By the spec (§7) these are different numbers — log schema versus crate version — so this is not a mismatch, but it is worth stating that the log was written by a newer slice than the tool-version the output echoes.
5. ctime_recorded_from_run=1. No row falls back to fields=ctime reason=not-recorded, and every show prints ctime for all three runs — consistent with the record, and consistent with the spec's note that only older runs would lack it.
Bottom line: every reference in the session's changes and show output resolves to exactly one log row; all listed field values match; the only no-row references are the two intentional absence markers, and both are correct. The record additionally answers the provenance question outright through hash_read_run, and identifies the previously unnamed files and the previously unidentified third fresh read of run 3.
