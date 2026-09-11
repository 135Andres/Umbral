# VISION — FSP

Status: CURRENT (living) — canonical knowledge (T2).

Category B. Status rule for this file: USER INTENT where the Master Context (MC,
archived at research/sources/PROJECT-MASTER-CONTEXT.md) states the item; STRONG
HYPOTHESIS where it is a convergent research position. MC is the user's own founding
document — its statements carry user authority, but its architecture-level items remain
hypotheses by its own declaration (MC §16, §42, §52).

-------------------------------------------------------------------------------
PRODUCT THESIS
-------------------------------------------------------------------------------
Give individuals and small teams a workspace whose human interface is their own files and
folders on their own disk, while semantic capability — projects, tasks, decisions,
evidence, knowledge, AI sessions, context assembly — operates as a non-mandatory overlay
derived from those files rather than as a structure the user must maintain.

Source: S1 §1, S3 §Exec, S9 §Exec, S11 §2.
Status: USER INTENT (premise asserted across the corpus; ratification pending).

-------------------------------------------------------------------------------
CORE USER MENTAL MODEL
-------------------------------------------------------------------------------
The user believes they are working with ordinary files in ordinary folders, using
ordinary tools. Semantic entities are lenses they may adopt, never structures they must
create. The application is, from the user's side, a better filesystem manager — not a
database they must feed.

Source: S0 §9 index, S8 §2/§17, S11 §16 ("dominio de soberanía física").
Status: USER INTENT.

-------------------------------------------------------------------------------
NORTH STAR
-------------------------------------------------------------------------------
The user's own north-star statement (MC §51, verbatim):

> I open my project, immediately understand its current state, navigate it visually,
> ask any connected AI to help, give that AI exactly the context it needs, let it work
> safely, continue its work later with another AI if necessary, see what changed,
> understand why it changed, and retain complete control over my files and project.

And it should work locally, privately, without mandatory cloud services, without
advertisements, without requiring Git, without requiring one specific AI, and without
forcing one project methodology — while remaining extensible toward desktop, web, CLI,
API, local/remote server, AI agents, Telegram, Git and future integrations.
Source: MC §51. Status: USER INTENT (verbatim user statement).

The three earlier candidate formulations (N1 manager-with-AI-layer, N2 projection
engine, N3 files-with-memory) are retained below as Hermes interpretations of the same
thesis — no longer competing, since the user's own statement supersedes them:
  N1 "The filesystem manager you always wanted, with an AI and semantic layer you never
      had to configure." Source: S8 §17.
  N2 "A local projection engine that renders an arbitrary file tree legible to both a
      human and a model." Source: S11 §17 (single differentiator).
  N3 "Files, with memory: the corpus knows its own projects, decisions, evidence and
      history, without the user filing anything." Source: S4 §1, S1 §1 synthesis.

Target user (MC §1, §37, verbatim intent): useful to ordinary users while powerful
enough for serious technical projects and AI-assisted development; audience ranges from
individual, to small team, to technical team, to large user base — without prematurely
optimizing for massive scale, and without making choices that unnecessarily prevent
scaling later. Status: USER INTENT. Resolves Q3 at the intent level; sequencing
(which user to serve first) remains open.

UX priorities (MC §38, user-stated order):
1. Understanding how things connect.
2. Absolute control.
3. Productivity.
4. AI assistance.
5. Knowing where everything is.
Frustrations to avoid, in the user's priority order: excessive maintenance complexity,
excessive configuration, unauthorized system behavior, visually boring UX, difficulty
finding information. The product should be easy to teach to friends.

-------------------------------------------------------------------------------
ANTI-GOALS
-------------------------------------------------------------------------------
The product must not become:
  - a database with a filesystem export (the user's files would be a projection);
  - a system that only works if the user adopts a taxonomy (PARA, Projects/Tasks/...);
  - a chat box as the primary interface to the user's files;
  - an autonomous background writer that mutates user files unsolicited;
  - a service whose unavailability disables the user's own material;
  - a graph-visualisation product.

Most of these now carry direct user authority via MC §1 (the product is NOT list:
note-taking app, Obsidian clone, project manager, AI chat app, filesystem browser, Git
replacement, AI agent) and MC §39 (what the system must not assume by default).
Status: USER INTENT where MC states it; the "graph-visualisation product" clause is a
Hermes inference constrained by MC §7 (the user explicitly wants graph/timeline/kanban/
dashboard views — so the anti-goal is "graph as the product", not "graph as a view").

CORRECTION OF RESEARCH-REPORT VERDICTS BY USER AUTHORITY (recorded 2026-09-10):
S11's adversarial verdicts REJECT dashboards, graph views, proactive intelligence and
native versioning conflict with the user's explicit statements:
  - MC §7/§8: user wants dashboards, kanban, graph, timeline views, and a customizable
    command-center home screen. (S11's hairball/maintenance evidence stands as a design
    constraint — ego-scoped graphs, declarative low-maintenance views — not as a
    product rejection.)
  - MC §18: the user "wants this functionality strongly" (proactive detection of
    blocking relationships etc.), with disable/reduce controls and no auto-action.
  - MC §26: "Git without Git" — the user wants history/snapshots/diffs/rollback without
    Git knowledge, with dedicated research, and explicitly not reinventing Git.
User authority overrides research recommendations. S11's evidence is retained as
constraint material, never as product direction.
