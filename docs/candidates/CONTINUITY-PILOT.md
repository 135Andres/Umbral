# Continuity pilot

Status: **PRODUCT INTENT ACCEPTED; DETAILED DESIGN UNDER REVIEW**, 2026-10-16.
Authority for intent and sequence: [UD-039](../decisions/DECISIONS.md).
This page is a proposed pilot scope, not an implementation or a release claim.

## The job

Resume a project with another agent, or after a break, without reconstructing the entire
conversation or silently treating outdated evidence as current. Umbral's own development
is the first intended use case. A Git repository is the initial supported setting to test.

## Intended experience

Connect the tool once. Work normally. Ask the agent to continue.

The proposed integration retrieves the task's goal, accepted constraints, source references,
checks and next action. It compares the recorded sources with the selected working copy
and marks what needs review. Agents record their progress through the integration.
Automatic capture and automatic invocation are not available today and must be verified
in a real client; MCP access alone does not provide them.

## Build order

| Stage | Proposed deliverable | What must be demonstrated |
|---|---|---|
| Local program | Save and resume one task, with versioned source references | A fresh process retrieves the task; changed or missing sources are explicit |
| MCP access | Expose the same operations through a local process | Tool calls preserve the core result; protocol output stays separate from diagnostics |
| First integration | Retrieve on resumption and save progress through one real agent client | The ordinary request to continue reaches the tools and returns useful context |
| Pilot | Use it on an actual Umbral task and compare against Git plus a good handoff file | Report correctness, total effort, context size and unnecessary interruptions |

## Boundaries

- A saved agent statement is attributed, not independently verified. A source fingerprint
  does not prove the agent understood the source.
- A changed source requires review; it does not automatically falsify a conclusion.
- Matching recorded sources only supports a statement about those sources at the check time.
  It does not establish that all dependencies were recorded or prevent subsequent changes.
- Git supplies revisions and diffs. This pilot does not create another version-control system.
- Existing v0.2 observation commands and their output contract are preserved. Its pending
  human-reader criteria remain pending; they are not the pilot's effectiveness metric.
- No Scion integration, autonomous merges, GUI, cloud service or multi-agent scheduler is
  part of the first pilot.

## Repository map

| Path | Role |
|---|---|
| `umbral/` | Existing Rust observation instrument; the local-program design starts from this codebase |
| `docs/canonical/` | Current direction and constraints |
| `docs/decisions/` | Owner commitments, including UD-039 |
| `docs/candidates/` | Proposed mechanisms and this pilot scope |
| `docs/versions/` | Existing version evidence and limitations |
| `experiments/` and `research/` | Preserved measurements, research and history |
| `fsp-check/` | Frozen V0 experiment, excluded from the Cargo workspace |

The detailed working design and deliberations remain in the owner's private record.
Dependencies, persistence layout and tool schemas are proposals until that design is reviewed.
