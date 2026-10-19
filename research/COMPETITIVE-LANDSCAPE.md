# COMPETITIVE LANDSCAPE — what already exists where Umbral intends to stand

Status: **RESEARCH ARTIFACT (T5), unratified.** Evidence base for one question only: what
already exists in the external sector that could occupy the position Umbral describes. It
selects nothing, changes no direction, and is not a decision.

Every source here is external and public. Sources are numbered CL-n.

Verification markers, same convention as `research/COEXISTENCE-RESEARCH.md` and
`research/ENVIRONMENT-INTELLIGENCE-RESEARCH.md`:

```
VERIFIED      = the primary text was read in this cycle (page fetched)
SNIPPET-LEVEL = only a search-result description was available; treat as a lead
KNOWN-THEORY  = long-established material not re-fetched this cycle
```

**Scope limit, stated first because it governs every claim below.** This is a search result,
not a census. "No competitor found" means *not found by this search cycle* — projects that are
unindexed, below a popularity threshold, private, or described in another language do not
appear. Product pages and READMEs are self-descriptions, not evaluations: no claim in this file
about another product's behaviour was independently tested. No market size, revenue or user
number for the segment "local-first tool for an individual" was found at all.

Cycle: 2026-10-04. Provider: one search backend, several query batches. Fetch of four primary
pages. No account, no trial, no code executed from any project named here.

================================================================================
0. THE QUESTION RESOLVES INTO FOUR SEPARABLE CLAIMS
================================================================================

"Does Umbral have competition?" has no single answer, because "the same as Umbral" is four
different claims, taken from `README.md`, `docs/canonical/VISION.md` and `umbral/README.md`:

```
(i)    your files stay where they are, in your structure, with no mandated taxonomy;
(ii)   a local record of what is known about them, stating the origin of every value
       (observed / derived / ambiguous / unknown);
(iii)  only what changed is re-read, and reuse is declared — never presented as verification;
(iv)   semantic capability (projects, decisions, evidence, AI sessions, context assembly)
       as a non-mandatory overlay, readable by humans and by several AIs from different vendors.
```

(i)–(iii) are what v0.2 actually does. (iv) is the vision, not built, and its premise is not
validated (`research/RESEARCH-INDEX.md` gap 4: no report tests the central assumption
empirically). The sector answer is different for each claim, so this file is organised by claim.

================================================================================
A. CLAIM (iv) — "AI over your own local files": SATURATED
================================================================================

CL-1  Local-first PKM with AI. Obsidian (+Copilot, Smart Connections), Logseq, Anytype, Tana,
      Capacities, Heptabase. SNIPPET-LEVEL (comparison articles, not the vendors' own pages).
      -> This is a mature market with a free, plugin-rich incumbent (Obsidian) and an
      object-model rival that has raised capital: Anytype announced $13.4M led by Balderton
      (blog.anytype.io). A third-party profile lists Anytype at ~306 employees, Berlin.
      SNIPPET-LEVEL.
      -> These solve "your files, your AI, no cloud" for notes. They do not record provenance
      of what is known, and they are applications you adopt, not a record kept about a folder
      you already have.

CL-2  Self-hosted AI over personal documents. Khoj (~37k stars), AnythingLLM (~65k stars),
      PrivateGPT, Onyx/Danswer. SNIPPET-LEVEL (aggregator comparisons).
      -> Free, open source, installable, and widely recommended. Whatever Umbral's semantic
      layer eventually does with documents, this is the default answer a user finds first.

CL-3  "Second brain" over a local folder with MCP. GNO (VERIFIED: gno.sh — one SQLite index
      across folders, "evidence behind every answer", ten AI clients in one command), stoa,
      homekb, Okto Neuron, MemoryWiki, vaultex, fsbrain, second-brain (lopax76), MegaMemory,
      memento-mcp, global-agent-memory, project-mem-mcp, semilattice, org-semantic,
      git-semantic. SNIPPET-LEVEL except GNO.
      -> These occupy "make my folder legible to my AI" almost exactly, and they are cheap to
      produce. CL-4 below measures how cheap.

CL-4  The cost of entry is approximately zero. Two independent 2026 counts of the MCP
      ecosystem: 38,212 servers in the official registry (crawl dated 1 Oct 2026), and
      136,217 discoverable servers exposing 1,067,334 tools (Glimind census dated 2026-10-05).
      SNIPPET-LEVEL, and the two counts disagree because they measure different bars.
      -> The direction of the number is what matters: "an MCP server that indexes your folder"
      is not a defensible position. It is a weekend.

CL-5  MEMM / AI Context OS. VERIFIED (memm.dev/docs/architecture-and-operating-model).
      Self-described as "a filesystem-first memory layer for AI agents… files are the source
      of truth". Markdown files with strict frontmatter (`id`, `type`, `l0`) are canonical
      memories; other `.md` files are deliberately ignored; a `.cache/memory-usage.json` is
      declared non-canonical support infrastructure; L0/L1/L2 progressive loading; a router
      generating `claude.md`, `.cursorrules`, `.windsurfrules`, `.ai/index.yaml` and an MCP
      prelude.
      -> Closest neighbour to claim (i) *and* to the "files are truth, index is a projection"
      commitment. Two differences that matter: the workspace must still adopt MEMM's convention
      (frontmatter, `.ai/`, `inbox/`) for a file to count as knowledge, and the router writes
      into the workspace. Neither is a defect for its purpose; both are outside Umbral's stated
      constraints.

================================================================================
B. CLAIMS (i)+(iv) — "one workspace where humans and agents share files": OCCUPIED
================================================================================

CL-6  Agent-native filesystems and shared drives. origofs, arg.ai, Fastyr Drive, ClawFS,
      Nexus, Locality, CortexFS, Spacedrive. SNIPPET-LEVEL except origofs.
      -> origofs: VERIFIED (github.com/danplischke/origofs) — "content-addressed storage with
      a real metadata database (Postgres or SQLite), opt-in Git-style versioning, and per-actor
      attribution built in… you always know who changed what… an agent's whole session can be
      reverted in one call". This is claim (i)'s outcome (attribution) reached by a different
      route: interpose a new filesystem rather than observe an existing one.
      -> Spacedrive: a Rust cross-platform file explorer on a "Virtual Distributed File System",
      positioned in 2026 as "Your data. Your devices. Your AI employees." SNIPPET-LEVEL.

CL-7  "Agent-native workspace" products. The project's own
      `research/COEXISTENCE-RESEARCH.md` CO-18 already records this space as occupied at the
      orchestration/terminal level (even.dev, agentmux.ai, Vecbase), with the observation that
      none of them claims the user's own filesystem as the substrate. This cycle found no
      reason to revise CO-18. See §G for what that does and does not mean.

================================================================================
C. CLAIMS (ii)+(iii) — "a local record that states the origin of every value": NEAR-EMPTY
================================================================================

CL-8  Nolane Habitat. VERIFIED (glama.ai/mcp/servers/Nolane-x/Nolane-habitat/tree, v0.1.0-alpha.20,
      Python). The single closest neighbour found. Its own words: "a local project-intelligence
      substrate that gives coding agents a durable, governed, revision-aware environment around
      a software project — without replacing canonical source files as truth". It publishes an
      explicit authority ordering:

          SOURCE_EXACT > OBSERVED_EXACT > COMPILER_PRECISE > PARSER_DERIVED
                       > HEURISTIC_DERIVED > MODEL_INFERRED

      plus "memory does not magically upgrade a claim", "recalling weak evidence later does not
      make it stronger", epistemic items (facts, assumptions, unknowns, contradictions),
      verification receipts as durable state, checkpoint/resume, governed mutations with
      journal and rollback, and fail-honest capability claims ("can execute" separated from
      "is isolated").
      -> Same epistemic instinct as claim (ii), different object and different shape. Habitat is
      a substrate an agent works *inside* (it stages, journals and commits source changes,
      and keeps its own state), scoped to software projects, addressed to coding agents. It is
      not a read-only observer of a folder the user owns, it has no stated output contract, and
      nothing in its published description corresponds to Umbral's `ambiguous` verdict or to
      declaring `content=reused` with the observation that actually read the bytes.
      -> Consequence to record plainly: the *idea* of ranking evidence by origin is not
      unoccupied. What is unoccupied is the specific combination in §D.

CL-9  Code-context engines. Augment Code Context Engine, Sourcegraph, Greptile, CocoIndex,
      RepoMix, Semble, Graft, qmd, codegraph, Synapse MCP, MegaMemory. SNIPPET-LEVEL.
      -> A crowded, commercially contested category (one comparison matrix alone carries eight
      tools). They answer "what should the model see", not "what is known and how do we know".
      A separate category, listed here so it is not mistaken for an empty one.

================================================================================
D. THE PIECES OF (ii)+(iii) ALREADY BUILT BY OTHERS
================================================================================

No single product found does (i)+(ii)+(iii). The parts exist, separately:

CL-10 readproof. SNIPPET-LEVEL. "gives every document an AI agent reads a stable identity, a
      freshness policy, and a content-addressed snapshot — and records every run as a manifest
      you can diff, replay byte for byte without touching the live source, and hand over as
      evidence". Closest thing found to claim (iii) as a product — for agent inputs and RAG,
      not for a user's folder, and it is about reproducibility of a run, not about the
      acquisition state of every entry in a tree.

CL-11 Verifying what an agent claims it did. execution-proofs, agent-attest, attest, Causari,
      ChainProof, Provena (two distinct projects), trace-file-lineage, provenance-verification
      (a portable "trust state" standard). SNIPPET-LEVEL.
      -> Notably, these exist *because* the gap is real: Causari records the prompt, the model,
      the files read and the reasoning, "without asking the agent's permission"; ChainProof
      sells "a durable record of what an agent attempted, changed, produced, and learned";
      trace-file-lineage answers "where did this file come from… with honest uncertainty".
      -> Every one of them observes the *agent*. Umbral observes the *filesystem*. That is the
      one axis on which Umbral is not a late entrant.

CL-12 Staleness and drift detection for documentation. Staleguard, staledocs, Thally (publishes
      "machine-legible last_updated / last_verified dates" and a `drift` check), selfdoc,
      docs-guardian, autocontext (a `.context.yaml` with a `derived_fields` array separating
      machine-derived ground truth from LLM-generated narrative). SNIPPET-LEVEL.
      -> CL-12 is the closest existing thing to claim (ii) *in the small*: they mark what was
      verified and when, and flag content that drifted from its source. They do it per page,
      by convention (frontmatter, tags), not as a contract over a whole tree.

CL-13 Standards. C2PA / Content Credentials (a provenance standard, now with TikTok on its
      steering committee and camera manufacturers signing at capture); in-toto/SLSA for
      attestation of a build chain; OpenLineage (a Linux Foundation lineage specification) and
      DataHub. SNIPPET-LEVEL for C2PA and OpenLineage; KNOWN-THEORY for in-toto/SLSA.
      -> The sector already agrees that provenance is worth standardising, and the enterprise
      version of the distinction is documented: lineage is "how did this data get here",
      provenance is "where did this come from and can I trust it for this decision".
      -> Umbral's `observed/derived/ambiguous/unknown` is a local, per-line instance of a
      question the wider sector has already decided is legitimate. That is support for the
      vocabulary and competition for the claim that it is new.

================================================================================
E. PRIOR ART: CLAIM (iii)'S SKIP RULE IS NOT NOVEL
================================================================================

CL-14 Build systems already do exactly this, and already hit the same correctness edge.
      Bazel: the Skyframe incremental evaluation framework; `--cache_computed_file_digests`;
      the stat-before-hash optimisation (issue 21044); and PR 18115, titled **"Use ctime in
      file digest cache key"**, whose own rationale is that mtime plus inode are not sufficient
      because a file can be replaced with `mv` preserving the inode.
      VERIFIED for the titles and quoted rationale; KNOWN-THEORY for the general practice.
      Watchman, Ninja and ccache are the same family.
      -> `UD-035`/`UD-036` — re-read only when dev/ino/size/mtime/ctime no longer match — is
      established practice in incremental build systems, including the specific use of `ctime`
      against mtime restoration. The ext4 inode-reuse experiment recorded in `umbral/README.md`
      (1000 of 1000) reproduces a concern that Bazel's changelog already names.
      -> Two consequences, and the second is the uncomfortable one. First, the design is
      correct and independently corroborated: that is real support. Second, **it is not
      defensible by novelty**, and a reader familiar with build systems will say so. The
      defensible part of claim (iii) is not the heuristic; it is the *declaration* — that a
      reused reading is labelled `reused`, carries `content-source=<run>:<path>`, and is never
      counted as verification in that run. No build system found here publishes that to a user.

================================================================================
F. WHERE THE MONEY AND THE ATTENTION ARE (context, not competition)
================================================================================

CL-15 Agent-memory infrastructure is the funded battleground, and it is the opposite design.
      Mem0 ($24M, Series A led by Basis Set), Cognee ($7.5M seed), Zaro ($5.1M, Cherry
      Ventures), Engram ($98M from General Catalyst, Kleiner Perkins, Sequoia), plus Zep,
      Letta, Supermemory, OMEGA, Basic Memory. SNIPPET-LEVEL (press and vendor pages).
      -> A third-party comparison states the split directly: "The first camp — mem0, Letta,
      Zep, cognee — extracts your data into a derived store… That processed artifact is the
      memory." SNIPPET-LEVEL.
      -> Read this as the sector's verdict, not as a rival: the capital went to the derived
      store. Umbral's thesis is that the derived store is the wrong answer. A thesis that the
      market has already funded the opposite of is a real risk, and it belongs in
      `docs/candidates/OPEN-QUESTIONS.md`, not in this file.

CL-16 Enterprise AI context. Glean: ~$7.2B valuation, ~$300M ARR (2026), described as building
      "the layer beneath the interface" with a permissions-aware knowledge graph. Microsoft
      Recall (local snapshots of the screen, opt-in, Copilot+ PCs only) and Copilot bundling.
      SNIPPET-LEVEL.
      -> Not competition for Umbral's niche. Relevant as the ceiling: if this problem becomes
      large, the answer at scale is an enterprise index with a permissions model, not a CLI
      that refuses to write into a folder.

================================================================================
G. WHERE NO COMPETITOR WAS FOUND, AND WHY THAT IS NOT AUTOMATICALLY GOOD
================================================================================

Not found in this cycle, in any product:

```
1  a tool that observes a folder and never writes inside it, with no marker file;
2  a per-line contract stating how each value was obtained, including `ambiguous` with a
   named reason and `unknown` with a named reason;
3  a declared reuse of an earlier reading, naming the observation that read the bytes;
4  an incremental skip whose reuse is labelled as reuse rather than as verification.
```

That is the honest positive result of this cycle, and it must be read together with four
qualifications:

- It is a *search* result (§ scope limit). Four things not found is weak evidence of four
  things not existing.
- MEMM and Habitat each occupy a large part of it (CL-5, CL-8), by different means.
- Umbral's own audit trail already records that the human-independent test of legibility is
  pending (`A1`, `A2-V13`; `UD-038`). Zero competitors plus zero demonstrated users is a
  hypothesis, not an opportunity: it can equally mean nobody has found value there.
- The adjacent spaces are dense, and the *reason* they are dense is that their value is
  obvious to a user. Claim (iii)'s value is not obvious to anyone who has not been burned by
  a confident wrong answer about a file.

================================================================================
WHAT THIS EVIDENCE DOES NOT SETTLE
================================================================================

- Whether claim (iv) has any value at all. Nothing here tests it; `EXP-1` is the artifact that
  would, and this file does not substitute for it.
- Whether claim (iii)'s declaration is worth anything to a real reader. The eight first-reader
  difficulties in `experiments/e-td-6-11/` and the pending human protocol are the evidence
  that matters, and this cycle produced none of it.
- Whether the difference from CL-8 (Habitat) or CL-5 (MEMM) is material to any user, or a
  distinction that only its author can see.
- Whether a market exists for a local-first tool for an individual. No number was found for it
  in either direction.
- Whether any product named here has, in fact, the properties its own page claims. Nothing was
  installed, run or audited.
- Whether Umbral should compete at all in the spaces in §A and §B. That is the owner's
  decision, and this file records no recommendation about it.
