# ADR 007 - Revisiting real-time updates

Status: proposed
Supersedes: the 2024-08-14 decision in decisions-2024.md (only if accepted)

Context
Users on the Delta Corp pilot have started asking for instant updates on the
dashboard. The polling loop gives 30 second staleness.

Options
1. keep polling (status quo, zero work)
2. move to websockets (memory cost, better freshness)
3. long polling (middle ground, more server state)

Decision
Not taken yet. This record exists so the reasoning is not lost.
Blocked by: the rate limiting open question (see open-questions.md).
Related: bug-1421.md, latency-notes.md
