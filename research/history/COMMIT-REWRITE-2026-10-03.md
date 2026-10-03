# Commit history rewrite — 2026-10-03

Status: **FROZEN RECORD** (written once, not edited).

On 2026-10-03, at the owner's instruction (private, 2026-10-03), the author and committer name
`Andres` was replaced by `135Andres` — the owner's GitHub name — in every commit of this
repository. Nothing else changed: emails, dates, messages and trees are as they were, and the
41 commits authored as `Hermes` keep that identity. Because a commit's hash covers its author,
every commit from the first one authored as `Andres` onwards received a new hash.

Records written before this date cite some of the old hashes, and frozen records are not
edited. This table resolves them. Living documents were updated to the new hashes the same day.
The full map (40-character hashes) was produced by `git filter-repo` and is reproduced here
abbreviated to 12 characters, which is unique in this repository.

| Old | New | Subject |
|---|---|---|
| `ce514caf2ca9` | `0a1ab74675ee` | feat(umbral): add the v0.1 workspace observation instrument |
| `d8175e841cf3` | `30f6d34a5f13` | test(umbral): acceptance matrix, filesystem matrix and falsification harness |
| `3a1ac96f09dd` | `9de50a171e49` | docs(decisions): record UD-014 for the v0.1 stack |
| `3fa9373fb96e` | `f09e594dbec6` | docs(versions): record v0.1 with its evidence and known limitations |
| `f3f8e372928e` | `2aee5758e108` | docs: point the repository at the v0.1 instrument |
| `fdcbfb0f4cbb` | `4dc98f6351d0` | ci(umbral): verify the new crate with fmt, clippy and tests |
| `73984e5c42ec` | `cc993b1af3ce` | docs(versions): register the reader protocol result — A1 not satisfied |
| `b6551415ec9f` | `58a3e6b13c99` | fix(umbral): label computed values as derived, not observed |
| `6f5675627bdd` | `933c319454d9` | docs(versions): register the protocol transcript, the redaction, and protocol v2 |
| `ce779e065ff8` | `ed0a9529695b` | docs(experiments): register EVIDENCIA-B, the second reader protocol run |
| `cf5348016065` | `5ad4093034db` | fix(umbral): accept paths that are not valid UTF-8 |
| `2bd92603d902` | `b934b495f108` | docs(experiments): apply the EVIDENCIA-B dispositions |
| `d4afb90ee439` | `e341900e2391` | fix(tests): exercise inode reuse across observations |
| `0d6a7d17b261` | `96d23e6a0e61` | test(contract): require all four output labels |
| `24da9128914b` | `e4a3d5fe1c72` | test(harness): exercise operation generator coverage |
| `5e786a46469c` | `095191bc3f70` | docs(contributing): state that documentation moves with the code |
| `97755c823b5c` | `5379be5a7105` | ci: run both workflows on the version branch too |
| `3c43f4dde440` | `115b5818b28e` | docs(contributing): state where a version's work lives |
| `dca83765d19b` | `8a9f79bad730` | docs(research): cite the kernel's inode timestamp semantics |
| `c78815d1ef05` | `d6fd184d64ce` | docs(experiments): record the ctime probe and its limits |
| `a6655bb9218c` | `7fbb3e00f826` | docs(decisions): record the evidence classes and the v0.2 commitments |
| `a7f34e6154a8` | `fcb3f07b0dff` | docs(candidates): propose the v0.2 scope |
| `46b4389b3ea3` | `a62b9953af54` | docs(experiments): prepare the first delegated reading check |
| `3828bfc46474` | `2a099ae485aa` | docs: correct what research/sources/ holds |
| `7008dfcbc890` | `ff728cb12a8e` | docs(experiments): record first independent AI reading |
| `b489698e96a1` | `86300c155e62` | docs(experiments): preregister second AI reading run |
| `8a50eb2bf9ec` | `849036172dd6` | docs(experiments): close AI reading cross-check |
| `e7a3c7f56a14` | `c5557a8417f3` | docs(decisions): refine basis traceability for v0.2 |
| `816f4ad1df17` | `b413eda6c0fa` | docs(v0.2): close technical design |
| `37220d22e7b9` | `1eab144268b6` | docs(decisions): record v0.2 output grammar and contract versioning |
| `79a74907e8a1` | `9261ad12541d` | docs: improve identifier and technical-design discoverability |
| `3d9caaac7ec1` | `a736bc76fb36` | docs: make identifier registry canonical |
| `d975ca238818` | `a3c9985f0e18` | docs: route identifier discovery through canonical registry |
| `f02b8b941d9f` | `1154810aa70e` | docs(decisions): record historical guarantees |
| `793b77f36b9a` | `1b1e1f46b593` | docs(v0.2): activate version development line |
| `5e32b3a1a583` | `f73d48680d32` | docs(decisions): record D6/D7 semantic requirements |
| `e46a0c2a5417` | `1130aa8ab7fe` | docs(decisions): record D8/D9 semantic decisions |
| `f7f9ff11d4d5` | `683da4f1d641` | docs(decisions): record owner decision on D1/P5 |
| `9297d518ee17` | `6cbecf1e99e7` | chore(gitignore): keep local review reports out of the tree |
| `59455718eabe` | `a06ddcc5e226` | docs(readme): lead with the problem and a real session |
| `3ce85ccedba3` | `6325fb8e9cb6` | docs(history): record the 2026-10-02 audit and its remediation plan |
| `8d09fdf22574` | `074a2f7e7bc5` | docs(decisions): record the licence and the v0.1 defect corrections |
| `d915049b3490` | `75948f5f0610` | chore(license): relicense under GPL-3.0-or-later |
| `3f6846f54e03` | `2050419e34d2` | docs(research): define the identifiers the v0.2 records cite |
| `55ce0483fcb2` | `5618f512ea10` | docs(agents): state the public/private boundary and the quotation rule |
| `3ab81d9e073e` | `f04d2b53f98f` | docs: correct stale status statements |
| `e053a344e9ee` | `e5f4b86442c8` | ci(umbral): key the cache on the workspace lockfile |
| `908f14ed1285` | `cef9e45c7fee` | fix(umbral): classify each path once in a path swap |
| `ad2f573322df` | `a2ab64d2361b` | fix(umbral): refresh the content guard's baseline on retry |
| `cb1abbfcc58f` | `ca9e14110d86` | fix(umbral): report previous-side conflicting candidates |
| `c714ade98921` | `d693dad48763` | fix(umbral): state whether a created entry's reference was complete |
| `b9e42aa3f727` | `6df904f939dd` | fix(umbral): persist content-acquisition errors in the log |
| `14f64c398e30` | `4be313aa0e04` | fix(umbral): make every check verification falsifiable |
| `0af870a5ed14` | `85227b9fee07` | fix(umbral): list recorded workspaces exactly and as derived |
| `f38c5dcb9ef3` | `15050534bd8f` | docs: show a real session with the corrected output |
| `d4b0fb5d6d08` | `03117c9f2fdd` | docs(candidates): redraft the v0.2 acceptance criteria |
| `2a9b6a488168` | `ea670ed167ea` | docs(umbral): propose the umbral-output/1 contract |
| `8c9dcc6702e9` | `e2577bcce762` | docs(decisions): accept the v0.2 criteria and slice 1 |
| `31b74ce64b91` | `4391c776faf5` | feat(umbral): write and read output as umbral-output/1 |
| `a24372b351cf` | `77f4af6652a2` | docs: record v0.2 slice 1 |
| `1c7835ac2805` | `b408a80ec276` | docs(decisions): adopt the v0.2 vocabularies (UD-031) |
| `3533e8d5c40f` | `861e1caee293` | feat(umbral): unstable reading is unknown; kebab-case reason tokens |
| `a8eff5ddbfef` | `d407159a7794` | docs: register D-V01-13 to D-V01-16 and authorize their correction (UD-032) |
| `9f569cafc77d` | `1c640d032b02` | fix(umbral): correct D-V01-13 to D-V01-16 |
| `7228be2fb9c5` | `f12750ba76db` | docs(decisions): slice 2 decisions and acceptance of slice 2a (UD-033) |
| `d5a790d7558a` | `598511687de7` | feat(umbral): per-observation basis; show names its observation (v0.2 slice 2a) |
| `3e4111c52204` | `feeec90ba25f` | docs(candidates): draft the slice 2b criteria |
| `3395c99ac896` | `683b66bfabe2` | docs(decisions): slice 2b decisions and acceptance (UD-034) |
| `192d9d88707a` | `98845e574322` | feat(umbral): every changes verdict states its basis (v0.2 slice 2b) |
| `8bf90234d962` | `87a9c449df55` | experiments: specify the E-TD-2 and E-TD-3 runs before executing them |
| `a657ed0fcc10` | `81c246b99dbe` | experiments: E-TD-2/E-TD-3 probe and its ext4 CI job |
| `cdec92512727` | `f00613c19aa4` | experiments: record the E-TD-2 and E-TD-3 results [skip ci] |
| `90dee254cc9b` | `ef4cb15ec274` | docs(decisions): ctime enters the skip condition; D-V01-17 (UD-035) |
| `af94401d68c4` | `1b1b9c7e880b` | fix(umbral): consult two valid readings before the metadata (D-V01-17) |
| `1898c0f45228` | `226565bfc70f` | docs(candidates): draft the slice 3 criteria |
| `11753e6a9ef7` | `ea10c247ca37` | docs(decisions): slice 3 decisions and acceptance (UD-036) |
| `5340dd379a60` | `8d50874f8383` | feat(umbral): read only what changed — the O(changes) skip (v0.2 slice 3) |
| `c3ea228131a3` | `11636a5f7091` | docs(candidates): draft the slice 4 criteria |
| `283a34ce6def` | `d89a93a18e13` | docs(decisions): slice 4 decisions and acceptance (UD-037) |
| `a13b17e6ae72` | `1d83bdd92461` | feat(umbral): traversal facts and the rules of each run; A2-V8 and A2-V9 (v0.2 slice 4) |
| `c71e77b12525` | `779cd5859ff5` | test(umbral): E-TD-4's restoring writer and E-TD-8; docs: v0.2 evidence by criterion |
| `3cd6f78e27bf` | `a3b795bb5589` | experiments: specify the E-TD-6/E-TD-11 reading before generating its material |
| `68378925ed3d` | `163faebb158f` | experiments: freeze the E-TD-6/E-TD-11 material and its delivery |
| `67761ea18a2e` | `ec748509fee9` | experiments: record the E-TD-6/E-TD-11 reading |
| `9aa4ef749e7b` | `749b2a9b44f5` | release(umbral): declare v0.2 evidence complete (UD-038); crate 0.2.0 |
| `56a25268f3e3` | `86a6a485aaea` | Merge pull request #1 from 135Andres/v0.2 |
