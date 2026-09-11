# SCRATCH — scenarios for the minimum-model cycle (working notes, not canonical)

Status: WORKING NOTES (temporary exploration, L-class). Kept because the report cites the
scenarios by ID (S1-S13). Not canonical; not part of the map; do not cite as authority.

Method per scenario: what exists in files? in Git? what cannot be reconstructed?
what would have changed the future action? which candidate primitive carries it?

S1 personal script. Files: the script. Git: none (never committed).
   Missing later: WHY the retry limit is 3 (API throttle observed once, never written).
   Carried by: assertion + attribution + time (a recorded observation by the user).
   Test: smart AI reads code; limit looks arbitrary; rationale unrecoverable. IRREVERSIBLY LOST today.

S2 small OSS library. Files+Git complete. Missing: maintainer privately decided "no Windows
   support" after a fight in a closed Discord. Git shows CI skips Windows; reason absent.
   Future AI proposes adding Windows CI. Correct action depends on the standing decision.
   Carried by: decision (assertion + authority + ratification) — attribution qualifier alone
   does not encode that this was a DECISION with scope "project", not a comment.

S3 web app, two branches. Git identical except merge order. Missing: which of two conflicting
   performance guidelines is current. Both docs in repo; one superseded, pointer never updated.
   Carried by: supersession (relation or temporal assertion — test both).

S4 data pipeline. Files+Git fine. Missing: column "user_id" is PII under GDPR per legal
   review that lived in email. Nothing in repo. Constraint is external.
   Carried by: assertion (negative constraint) + provenance (source: legal review, external).

S5 multi-service project. Services A and B both "own" the shared schema dir. Missing: the
   ownership boundary agreed verbally. Future refactor of schema by an AI acting on A breaks B.
   Carried by: assertion + relation (applicability/scope) + authority (who agreed).

S6 regulated system (medical device). Files+Git fine. Missing: which of three validation
   documents is the governing one and which regulatory clause each test traces to.
   Carried by: relation (traceability) + validity interval + authority.

S7 long-lived enterprise code. Original team gone. Missing: a module is kept despite dead code
   because a customer contract (external, not in repo) requires its behavior byte-for-byte.
   Smart AI sees dead code, deletes it. Correct action blocked by external constraint.
   Carried by: assertion + provenance (external contract) + scope.

S8 project with multiple AI agents. Two agents produced contradictory refactor plans; both
   recorded. User has not decided. Files contain both; no record of the disagreement's status.
   Future AI reads one (latest), implements it. Correct behavior: surface the conflict.
   Carried by: assertion status (proposed/disputed, NOT accepted) + attribution.
   Collapse check: "conflicting records -> epistemic status + topology" — status alone per
   record is insufficient; the DISAGREEMENT itself (B says not-A) is a record about a record.

S9 book project (non-technical). Chapter 3 written, then an editor said "kill the subplot" —
   said verbally. Files still contain the subplot. Six months later a different AI "helpfully"
   expands it. Standing decision missing. Model must work identically: entity=chapter,
   decision, authority=editor (not the file owner!). Authority can outrank possession.

S10 research project. A hypothesis was tested and DISPROVEN; the negative result is in a
   paper, the repo contains only the (abandoned) analysis code. Future AI re-proposes it.
   Carried by: negative assertion + provenance (citation).

S11 renamed entity. Directory src/core renamed src/kernel across 40 files; Git tracks it.
   But an assertion recorded by FSP says "core/ has no test coverage" — does it still refer?
   Time+identity: the assertion is about the entity, which survived the rename. Entity as
   subject-position survives; file-path identity does not. Tests entity vs file identity.

S12 unknown actor. A file changed overnight; sync tool or user's other machine? No AI admits it.
   Required: record the change with attribution=UNKNOWN (never silently "the user").
   This is an OBSERVATION (tool-recorded), not an assertion by an actor — tests whether the
   model needs an actor at all for observations, or whether attribution is nullable.

Cross-cutting observations so far:
- Every scenario reduces to records ABOUT something (subject), WITH a source, WITH a time
  dimension (assertion-time; often also valid-time), and WITH a status in a small set.
- "Relation" never appears as carrying unique information except as a record whose subject
  is a pair — i.e., relations look like assertions whose subject is another record/pair.
- "Entity" never carries information by itself in any scenario; it exists as the anchor
  that assertions refer to, and its identity problem (S11) is real but resolvable by
  assertions, not by the entity record itself.
- Status/standing (proposed vs accepted vs disputed vs retracted) appears in S2,S3,S6,S8,S9,S10
  and is NOT reducible to attribution: two records can have identical attribution and
  different standing.
