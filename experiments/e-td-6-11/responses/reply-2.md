<!-- Reply to send-2.md, pasted by the owner on 2026-10-03, saved verbatim. Model: not stated by the owner. -->

New step — reading the results against the umbral-output/1 specification
I keep the earlier answers as they stand. Below, A is the per-result analysis (subject + evidence), B is the effect of each transformation on A, C is the round-trip question.
Two preliminaries used throughout:
- Not result lines. In changes and show, derived contract=umbral-output/1 is the contract header (grammar/version, whole output), and exit=N is outside the contract (§1: exit codes not covered). Neither is a result.
- Not evidence. observation=, reference-encoding=/compared-encoding=/path-encoding=/observation-encoding=/content-source-encoding= are the identification field (§6a) and the encoding annotations (§5). The spec says both "state nothing about the filesystem" and the annotation "does not count as a field of an observed line". So I treat them as grammar, not evidence.
Observation fields I can cite: physical identity dev,ino; observed metadata kind,size,mtime,ctime; derived content hash,stability,metadata,content and the pointer content-source. (Note: ctime is shown on observed lines and recorded with every observation, yet never appears in a *-fields= list — see C.)
A. Each result line: subject and evidence
A.1 changes (header names reference-run=2, compared-run=3)
Result line    Subject (entry/object)    Evidence (observation + fields)    Basis that is not an observation field
compared reference-run=2 compared-run=3 reference-complete=false compared-complete=true    the whole comparison (runs 2 and 3)    none    the two runs chosen, and each run's completeness (a fact about a whole run)
count unchanged=11    the whole comparison    none    tally over all pair verdicts
count modified=0    the whole comparison    none    tally
count created=1    the whole comparison    none    tally
count deleted=1    the whole comparison    none    tally
count unobserved=0    the whole comparison    none    tally
count renamed-or-moved=1    the whole comparison    none    tally
count recreated=0    the whole comparison    none    tally
count ambiguous=0    the whole comparison    none    tally
renamed-or-moved path=d.txt …    the object named d.txt in run 3 (named c.txt in run 2)    obs(2,c.txt) fields dev,ino; obs(3,d.txt) fields dev,ino    equality of dev+ino (identity) is the rule; run completeness (reference-complete=false, compared-complete=true) is a run fact
deleted path=link-two …    link-two as observed in run 2    obs(2,link-two) fields dev,ino; counterpart obs(3,link-one) fields dev,ino    the absence of link-two in run 3 (compared-absent=3); run completeness; object-survives=true is a derived conclusion
created path=locked/inside.txt …    locked/inside.txt as observed in run 3    obs(3,locked/inside.txt) fields dev,ino    the absence in run 2 (reference-absent=2); run completeness. The absence is caused by run 2's traversal-not-descended=1 on locked/, which is why reference-complete=false
A.2 show a.txt, show b.txt, show link-one, show line\x0Abreak.txt, show bytes-\xFF.txt
These four/six share the same shape: one observation pair per run (a derived line, then an observed line), then two comparison lines.
Per-observation pair for run R, path P
Result line    Subject    Evidence
derived observation=R:P hash=… stability=… metadata=… content=… [content-source=…]    obs(R,P)    its own derived fields: hash, stability, metadata, content; when content=reused, content-source=R':P says the bytes were read at obs(R',P), so the hash shown is obs(R',P)'s, not a new reading
observed observation=R:P kind=… size=… mtime=… ctime=…    obs(R,P)    its own observed fields: kind,size,mtime,ctime
Concretely:
- a.txt — obs(1,a.txt): hash ac678d92b3d7, size 6, fresh. obs(2,a.txt): hash 29a077efa2d8, size 14, fresh. obs(3,a.txt): hash 29a077efa2d8, content reused, content-source=2:a.txt → run 3's content is obs(2,a.txt)'s, not read in run 3.
- b.txt — obs(1,b.txt) fresh; obs(2,b.txt) reused, content-source=1:b.txt; obs(3,b.txt) reused, content-source=1:b.txt. Runs 2 and 3 carry run 1's hash.
- link-one — obs(1) fresh; obs(2) reused, content-source=1:link-one; obs(3) fresh (re-read). All three same hash 7fde9f117e69; obs(3)'s ctime moved to …21.737Z.
- line\x0Abreak.txt — obs(1) fresh; obs(2),(3) reused, content-source=1:line\x0Abreak.txt. Each line carries observation-encoding=escaped:control-character (grammar).
- bytes-\xFF.txt — obs(1) fresh; obs(2),(3) reused, content-source=1:bytes-\xFF.txt. Each line carries observation-encoding=escaped:not-valid-utf8 (grammar).
Comparison lines (all six files, a.txt with a modified as well)
Result line    Subject    Evidence    Non-observation basis
modified path=a.txt reference=1:a.txt reference-fields=dev,ino,kind,size,mtime,hash compared=2:a.txt compared-fields=… content-changed=true …    a.txt    obs(1,a.txt) fields dev,ino,kind,size,mtime,hash; obs(2,a.txt) same fields    run completeness (reference-complete=true, compared-complete=false). content-changed=true rests on the two hash fields differing
unchanged path=X reference=1:X … compared=2:X … content-changed=false …    X (b.txt, link-one, line…, bytes…)    obs(1,X) and obs(2,X) over dev,ino,kind,size,mtime,hash    run completeness
unchanged path=X reference=2:X … compared=3:X … content-changed=false …    X    obs(2,X) and obs(3,X) over dev,ino,kind,size,mtime,hash    run completeness
For b.txt, line…, bytes…, the run-3 hash is the reused run-1 hash, so content-changed=false for the 2→3 comparison rests on carried values (the spec labels this: content=reused, and "a reused reading is never presented as verified in that run").
A.3 show d.txt
Result line    Subject    Evidence
derived observation=3:d.txt hash=6fefa7c34afd … content=fresh    obs(3,d.txt)    own derived fields
observed observation=3:d.txt kind=file size=8 mtime=… ctime=…21.737Z    obs(3,d.txt)    own observed fields
There is no comparison line here. The c.txt→d.txt continuity is not represented in this output at all; it appears only in changes.
A.4 show locked (a directory — no content component)
Result line    Subject    Evidence
derived observation=R:locked hash=none stability=none metadata=fresh (R=1,2,3)    obs(R,locked)    own derived fields; no content= because a directory has no content component
observed observation=R:locked kind=dir size=60 mtime=… ctime=… (R=1,2,3)    obs(R,locked)    own observed fields
unknown observation=2:locked traversal=not-descended observation-error=…Permission denied (os error 13)    obs(2,locked)    the traversal failure and its error text — neither is a field of an observation; it is the reason the run is incomplete
unchanged path=locked reference=1:locked reference-fields=dev,ino,kind compared=2:locked compared-fields=dev,ino,kind …    locked    obs(1,locked) fields dev,ino,kind; obs(2,locked) fields dev,ino,kind
unchanged path=locked reference=2:locked reference-fields=dev,ino compared=3:locked compared-fields=dev,ino …    locked    obs(2,locked) fields dev,ino; obs(3,locked) fields dev,ino
B. Effect of each transformation on A
T1 — reordering (line permutation)
- changes: no result changes. Its header is still line 1, so the output is readable; every result line self-identifies (path=, reference=, compared=), and line order is not used to associate items. Subjects and evidence are identical.
- show locked: no result changes (header still line 1; permutation preserves items and their annotations).
- show a.txt, show b.txt, show d.txt, show link-one, show line\x0Abreak.txt, show bytes-\xFF.txt: every result becomes impossible to determine. In each, the header line was permuted away from position 1 (the first line is a verdict or an observed line). §6: "a first line that is not a header → refuse: missing contract header", and there is no recovery. So the specified reader rejects the whole output, and no result can be read — even though a human can still see the content. (The same applies to init.)
No subject/evidence value is altered anywhere in T1.
T2 — subject removal (path= and observation= deleted)
- changes: the answer to A does not change in content, but the subject is no longer stated. The subject of each verdict is still recoverable from the observation references: renamed-or-moved from compared=3:d.txt, created from compared=3:locked/inside.txt, deleted from reference=2:link-two (the counterpart link-one is distinguished by the field roles). The evidence (reference=/compared=/reference-fields=/compared-fields=) is intact. So: same subject, same evidence; only the explicit path= (the declared subject, UD-034) is gone, so the subject is inferred rather than declared.
- show: the observation lines become impossible to determine. With observation= gone, the derived hash=… and observed kind=… lines name neither run nor path; the run cannot be recovered without using line order, which §6a forbids ("no line depends on the one above it"). The same applies to the unknown line in show locked (the run it reports is no longer named). The comparison lines keep reference=/compared=, so their subjects (the path) and their evidence remain determinable. content-source= still names a source observation, but it names a different observation (where bytes were read), not the subject of the line it sits on.
T3 — reflow (hard wrap at 60 columns)
- §2: "a line feed is always a result boundary and nothing else." The wrap inserted real line feeds, so each continuation is a separate line. Continuation lines have no valid label (e.g. other=0 …, content-failed=0 …), and many first halves end with a separator (e.g. symlinks=0   then LF). §6 refuses an undefined label and "a separator at the end of a line", with no recovery.
- Result: every result in every output (all changes results and all show results) becomes impossible to determine — the reader refuses the output.
- §8 P38 / §2 also say that a display wrap does not change a line. If this transformation were read as a display-only wrap (no bytes inserted), nothing would change. But the material contains actual LF bytes, which §2 makes boundaries, so under the spec it is corruption, not wrapping.
T4 — delimiter-split (each item on its own line under its label)
- The grammar is <label padded to 9> <item>(  <item>)* on one line. Here the label stands alone and is not padded to the nine-character column, and the item lines begin with spaces (an undefined label). §6 refuses an undefined label and an empty item; no recovery.
- Result: every result in changes and all show becomes impossible to determine.
- Values are not corrupted: if a lenient reader regroups items by the label line above them, the subject and evidence are unchanged. But the specified reader has no recovery.
T5 — JSON serialization
- JSON is not umbral-output/1. The reader expects the text grammar; the first line [ is not the header → "refuse: missing contract header", and there is no recovery.
- Result: every result in changes and all show becomes impossible to determine under the specification.
- The JSON itself is faithful: each item is stored as an exact string, and the claimed byte-identical round trip means subject and evidence are preserved if read by a JSON reader. That reader, however, is not the specified one. (Note the JSON stores line\\x0Abreak.txt; only a JSON parser turns \\ into the single backslash that the umbral escape then decodes to 0x0A. Reading the raw JSON text under §4 would give the wrong byte — see C.)
T6 — position change (reference fields moved to the end)
- The lines remain well-formed: labels padded, two-space separators, header first, and every encoding annotation still directly follows its field (observation=…  observation-encoding=…; reference=…  reference-encoding=…; compared=…  compared-encoding=…).
- Associations are by key, and there are no repeated keys in these outputs, so the rule "keep every occurrence, in order" has nothing to reorder. No result's subject or evidence changes — a valid "changes nothing" finding.
- Two caveats: (a) the spec's wording "reference=… each followed by reference-fields=" is no longer literally true (the -fields now precede their reference); a key-based reader is unaffected, a positional one would not be. (b) The spec says item order "is part of the result", so T6 does change the textual form of the result even though it does not change A.
Summary table
Transformation    Effect on A
T1 reordering    changes and show locked: unchanged. Other six shows: impossible (header no longer first)
T2 subject removal    changes: subject still recoverable, evidence intact (subject no longer declared). show observation/unknown lines: impossible (run not named). show comparison lines: unchanged
T3 reflow    all results impossible (malformed lines / refusal); values split
T4 delimiter-split    all results impossible (grammar violated); values intact
T5 JSON    all results impossible under the spec (not the contract format); values intact to a JSON reader
T6 position change    nothing changes to A (keyed fields; annotations stay adjacent)
C. Does any value — in particular any name — fail to come back exactly?
In the captured session: no. The two unusual names decode exactly:
- line\x0Abreak.txt → line + byte 0x0A + break.txt, annotated escaped:control-character;
- bytes-\xFF.txt → bytes- + byte 0xFF + .txt, annotated escaped:not-valid-utf8.
Uppercase hex, the correct reason tokens, and each annotation immediately after its field. No value contains an unescaped ambiguous space, control byte or deceptive character. All plain values (canonical, root, workspace-id, state-dir, scope, the error text) round-trip.
Across the transformed set: yes, in two places.
- T3 — the state-dir value is cut by a line feed: …/umbral/ws-242c | 37d665addf11. Since a line feed is a result boundary (§2) and there is no recovery, the workspace identifier no longer comes back (state-dir decodes truncated; 37d665addf11 is a line with an undefined label). Likewise the observation-error message is split (…operation on  | /tmp/…/locked: Permission denied (os  | error 13)). These are present values that cannot be reconstructed.
- T2 — the values of path= and observation= do not come back because they were deleted. That is an omission, not a misreading; the unusual names still round-trip inside reference=, compared= and content-source=.
T1, T4, T5 and T6 do not corrupt any value. (In T5 the JSON stores line\\x0Abreak.txt; decoded by a JSON parser it is exactly line\x0Abreak.txt, so it round-trips — but read as raw text under §4 the doubled backslash would give a literal backslash, which is not the name on disk. That failure belongs to treating JSON as the contract format, not to the contract.)
One possible inconsistency worth flagging. In the captured session I read the show locked 2→3 comparison as consulting dev,ino (no kind), while all six transformed versions show dev,ino,kind there. A pure line permutation (T1) cannot change a field value, so if that reading is right, the "mechanically transformed" versions are not faithful to the session in this one field. I cannot resolve which of the two is the true original from the material alone; I record it rather than choose.
