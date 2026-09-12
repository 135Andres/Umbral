# ENVIRONMENT INTELLIGENCE — evidence base

Status: **RESEARCH ARTIFACT (T5), unratified.** Evidence base for
ENVIRONMENT-INTELLIGENCE.md (the hypothesis set) and the coexistence strategy document.
Sources are numbered EI-n and are external unless marked S-n (the ingested corpus, see
research/RESEARCH-INDEX.md).

Verification markers, same convention as research/COEXISTENCE-RESEARCH.md:
  VERIFIED      = the primary text was read in this session (search snippet + page fetch)
  SNIPPET-LEVEL = only a search-result description was available; treat as a lead
  KNOWN-THEORY  = long-established, textbook-level material not re-fetched this cycle

Scope limit: this file records what exists and what it already solves. It does not select
anything. Where a claim below says "solved", it means solved FOR PARTICIPANTS WHO COOPERATE,
which is the constraint that matters for Umbral (see EI-19, EI-20).

================================================================================
A. SELF-DESCRIPTION AND CAPABILITY DISCOVERY — PRIOR ART
================================================================================

EI-1  Kubernetes API discovery. VERIFIED. "Each Kubernetes cluster publishes the
      specification of the APIs that the cluster serves. There are two mechanisms that
      Kubernetes uses to publish these API specifications; both are useful to enable
      automatic interoperability."
      -> A cluster self-describes its own API surface to any client. This is the closest
      existing analogue to "an environment describes itself to an unknown participant".
      Difference: the description is of the API, not of the project content, and it
      requires a live server.
      URL: https://kubernetes.io/docs/concepts/overview/kubernetes-api/

EI-2  kubectl explain. VERIFIED. "Describe fields and structure of various resources.
      Fields are identified via a simple JSONPath identifier."
      -> Two-stage disclosure of a schema: resource -> field -> sub-field, on request. This
      is progressive disclosure implemented in a CLI, and it has existed for years.
      URL: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_explain/

EI-3  D-Bus Introspectable. VERIFIED (interface reference + specification). Every D-Bus
      object MAY expose org.freedesktop.DBus.Introspectable, whose Introspect() returns an
      XML description of the object's interfaces, methods, signals and arguments.
      -> Runtime introspection of a live interface, standardised, with a documented format.
      URL: https://dbus.freedesktop.org/doc/dbus-specification.html

EI-4  SNMP SMIv2 / MIB. VERIFIED (RFC 2578, RFC 3418). Management information is modelled
      as a collection of managed objects in a virtual information store, the MIB, defined
      in MIB modules with a formal structure.
      -> A machine-readable model of "what an agent can be asked about", with types and
      semantics, decoupled from any particular manager.
      URL: https://datatracker.ietf.org/doc/html/rfc2578.html

EI-5  JMX / MBeans. VERIFIED. "The Java VM includes a platform MBean server and platform
      MBeans for use by management applications that conform to the Java Management
      Extensions (JMX) specification." MBeans are registered, discoverable at runtime, and
      expose attributes, operations and notifications.
      -> Runtime registration + discovery + typed attributes + change notification. The
      Basic/Advanced/Diagnostic tiering is an implementation choice on top of this.
      URL: https://docs.oracle.com/en/java/javase/25/management/overview-java-se-monitoring-and-management.html

EI-6  /proc and sysfs. VERIFIED (kernel documentation, man 5 sysfs). "The proc file system
      acts as an interface to internal data structures in the kernel... used to obtain
      information about the system and to change certain kernel parameters at runtime."
      -> A canonical instance of the exact design Umbral contemplates: runtime truth exposed
      AS ORDINARY FILES, readable by anything, with no SDK and no cooperation required.
      This is the single most important precedent in this file (see EI-20).
      URL: https://kernel.org/doc/html/latest/filesystems/proc.html , man7.org/linux/man-pages/man5/sysfs.5.html

EI-7  MCP tools. VERIFIED (specification 2026-07-28). Servers expose tools; clients list
      them (tools/list) and invoke them (tools/call); tools carry descriptions and JSON
      Schema input schemas.
      -> Capability discovery exists and is standardised. What MCP does NOT carry in the
      tool listing: authority requirements, mutability class, egress implications, or any
      versioning of the state a tool returns (the versioning gap was established in
      research/COEXISTENCE-RESEARCH.md CO-3).
      URL: https://modelcontextprotocol.io/specification/2026-07-28/server/tools

EI-8  A2A AgentCard. VERIFIED (A2A project docs; schema reference). An Agent Card is a
      standardised JSON document at /.well-known/agent-card.json stating "who an agent is,
      where to reach it, what it can do, and how access is secured". Agent Skills describe
      specific capabilities.
      -> Self-description as a published artefact, including an access/security section.
      This is the closest thing in existence to a "self-description layer" — for AGENTS, not
      for environments, and it assumes both sides speak A2A.
      URL: https://github.com/google-a2a/A2A (tutorial: agent skills and card)

EI-9  Agent Skills and progressive disclosure. VERIFIED (Anthropic engineering post +
      platform docs). "This filesystem-based architecture enables progressive disclosure:
      Claude loads information in stages as needed, rather than consuming context upfront.
      Skills can contain three types of content, each loaded at a different [stage]."
      -> Progressive disclosure is the DESIGNED behaviour of a shipped, first-party agent
      platform, and it is filesystem-based. Umbral's progressive-disclosure hypothesis is
      therefore an ADAPTATION, not a discovery (see the novelty table).
      URL: https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills

EI-10 Tool Search Tool / Programmatic Tool Calling / Tool Use Examples. VERIFIED (Anthropic
      engineering post, 2025-11-24). "Tool Search Tool, which allows Claude to use search
      tools to access thousands of tools without consuming its context window";
      "Programmatic Tool Calling, which allows Claude to invoke tools in a code execution
      environment"; "Tool Use Examples, which provides a universal standard for
      demonstrating how to effectively use a given tool." Stated motivation: "tool results
      and definitions can sometimes consume 50,000+ tokens before an agent reads a request".
      -> Context-economy through deferred, searchable capability disclosure is now
      commodity practice on at least one major platform. It also confirms the economic
      premise: context spent on interfaces is context not spent on the work.
      URL: https://www.anthropic.com/engineering/advanced-tool-use

EI-11 Progressive disclosure as a UX principle. VERIFIED (Nielsen, 2006-12-03). "Progressive
      disclosure defers advanced or rarely used features to a secondary screen, making
      applications easier to learn and less error-prone. Initially, show users only a few of
      the most important options. Offer a larger set of specialized options upon request."
      -> The term and the principle are 20 years old and were formulated for HUMAN
      interfaces, not for agents. Umbral applying it to AI participants is an ADAPTATION of an
      established pattern. Nielsen also documents the failure mode: complexity deferred is
      still complexity, and dialogs that hide too much become unusable — directly relevant
      to objection OBJ-10.
      URL: https://www.nngroup.com/articles/progressive-disclosure/

EI-12 OpenAPI / JSON Schema / service discovery (UDDI, DNS-SD). SNIPPET-LEVEL for UDDI
      failure; VERIFIED for DNS-SD (RFC 6763).
      - DNS-SD (RFC 6763): "how DNS resource records are named and structured to facilitate
        service discovery." Works over ordinary DNS, no registry required.
      - UDDI: the three founding vendors discontinued the public UDDI Business Registry;
        academic evaluation found intrinsic problems including lack of explicit data typing
        and inability to carry time-varying information.
      -> The one historical lesson that matters most here: a CENTRAL registry that everyone
        must publish into and consult failed commercially, while a decentralised, no-
        registry-required mechanism (DNS-SD) survived. Directly supports EI-20 and objection
        OBJ-1's inverse.
      URL: https://datatracker.ietf.org/doc/html/rfc6763 ; https://www.infoworld.com/article/2215182/microsoft-ibm-sap-discontinue-uddi-registry-effort.html

================================================================================
B. ENVIRONMENT STATE, RECONCILIATION AND MAINTENANCE — PRIOR ART
================================================================================

EI-13 Kubernetes controllers / reconciliation. VERIFIED. "controllers are control loops
      that watch the state of your cluster, then make or request changes where needed. Each
      controller tries to move the current cluster state closer to the desired state."
      -> The canonical architecture for "a process that continuously maintains environment
      state". CRITICAL STRUCTURAL CONDITION: reconciliation presupposes a DECLARED DESIRED
      STATE (the .spec) that the controller is allowed to enforce. This is the sharpest
      constraint on Umbral's Maintenance hypothesis: a user's filesystem has no desired state,
      so a maintenance role has no objective function unless one is supplied. See
      ENVIRONMENT-INTELLIGENCE.md H23 and tension T7 (ARCHITECTURE-HYPOTHESES.md).
      URL: https://kubernetes.io/docs/concepts/architecture/controller/

EI-14 Autonomic computing / MAPE-K. VERIFIED (Kephart & Chess, IEEE Computer 2003; IBM
      architectural approach, 2004). Computing systems that manage themselves given
      high-level objectives from administrators; the reference model is Monitor-Analyse-
      Plan-Execute over a shared Knowledge base.
      -> The Maintenance role is not a new idea; it is an autonomic manager. Twenty years of
      work exists on it, including its failure modes (goal specification, unpredictable
      interactions between autonomic elements, loss of operator understanding).
      URL: https://doi.org/10.1109/mc.2003.1160055

EI-15 Health probes and management endpoints. VERIFIED. Kubernetes liveness/readiness/
      startup probes: "Many applications running for long periods of time eventually
      transition to broken states, and cannot recover except by being restarted"; probes are
      "a diagnostic performed periodically by the kubelet on a container". Spring Boot
      Actuator exposes health, info, metrics, and separate liveness/readiness groups.
      -> The Basic tier of introspection is an established, standardised pattern with an
      established distinction: "is it alive" (liveness) vs "is it able to serve" (readiness)
      — a distinction Umbral's three-tier model does not yet make, and probably needs.
      URL: https://kubernetes.io/docs/concepts/workloads/pods/probes/

EI-16 Self-diagnosis commands. VERIFIED. `git fsck` ("Verifies the connectivity and validity
      of the objects in the database"); `npm doctor` ("runs a set of checks to ensure that
      your npm installation has what it needs"; checks connection, registry, versions,
      environment, permissions, cache).
      -> "Diagnostic" as a user-invoked, read-only, whole-environment check is ordinary
      practice in developer tooling. The Diagnostic tier is EXISTING PATTERN.
      URL: https://git-scm.com/docs/git-fsck ; https://docs.npmjs.com/cli/v12/commands/npm-doctor/

================================================================================
C. EPISTEMIC STATE, DISAGREEMENT AND CONFIDENCE — PRIOR ART
================================================================================

EI-17 Truth maintenance systems. KNOWN-THEORY (Doyle, "A Truth Maintenance System", MIT AI
      Lab TR 419 / Artificial Intelligence 1979; de Kleer, "An assumption-based TMS",
      Artificial Intelligence 1986). Doyle's stated motivation: "reasoning programs must be
      able to make assumptions and subsequently revise their beliefs when discoveries
      contradict [them]". A TMS records justifications linking beliefs to their dependencies
      so that retracting a premise retracts everything that depended on it, and so that
      contradictions are detected rather than silently overwritten.
      -> The exact machinery Umbral's epistemic-state and disagreement hypotheses describe has
      existed since 1979, including the dependency-tracking that makes "what would this
      invalidate?" answerable.
      URL: https://dspace.mit.edu/handle/1721.1/5733

EI-18 Belief revision and argumentation. KNOWN-THEORY. AGM postulates (Alchourrón,
      Gärdenfors, Makinson 1985) formalise expansion/contraction/Revision of a belief set.
      Dung's abstract argumentation frameworks (1995) formalise sets of arguments with an
      attack relation and semantics (grounded, preferred, stable) for deciding which
      arguments can stand together.
      -> A rigorous theory of "two claims that attack each other, with no decision yet" is
      available. Umbral does not need to invent it; it needs to decide whether it wants it.
      NOTE: not re-fetched this cycle (KNOWN-THEORY); flag before citing in a decision.

EI-19 Uncertain knowledge graphs. VERIFIED (AAAI 2019 "Embedding Uncertain Knowledge Graphs";
      arXiv 2510.24754). Uncertain KG embedding "learn[s] vector representations that capture
      both structural and uncertainty information"; a 2025 line of work observes existing
      methods "produce only point" estimates and adds statistical guarantees.
      -> Numeric confidence attached to relationships is an active research area whose
      current state is: point estimates are easy, CALIBRATED uncertainty is hard and recent.
      This is the evidence base for the confidence warning in ENVIRONMENT-INTELLIGENCE.md
      (H17): a displayed confidence number is an assertion of calibration the system may not
      possess.
      URL: https://doi.org/10.1609/aaai.v33i01.33013363 ; https://arxiv.org/html/2510.24754v1

EI-20 Dispute handling in a public knowledge corpus. VERIFIED (Wikipedia NPOV policy;
      disputed-statement template). Content must represent "fairly, proportionately, and, as
      far as possible, without editorial bias, all the significant views that have been
      published by reliable sources"; disputed statements are marked in place and the
      warning must not be removed without addressing the dispute.
      -> A working, 20-year-old social mechanism for exactly Umbral's requirement: hold
      competing claims without collapsing them, and mark contested state visibly at the
      point of the claim. It is a MODEL TO COPY, and its known failure (maintenance debt;
      dispute templates that persist for years) is directly relevant to OBJ-12.
      URL: https://en.wikipedia.org/wiki/Wikipedia:Neutral_point_of_view

================================================================================
D. PROVENANCE, ATTRIBUTION AND AUTHORITY — PRIOR ART
================================================================================

EI-21 W3C PROV and in-toto. KNOWN-THEORY (already recorded in the coexistence stage as CO-11,
      CO-12). PROV-DM/PROV-O model entities, activities and agents; in-toto attestations
      make verifiable claims about how an artefact was produced. Established constraint from
      that earlier work: an attestation proves ORIGIN, not TRUTH.
      -> Attribution vocabulary exists. It assumes the actor IDENTIFIES itself, which is the
      assumption Umbral's UNKNOWN-actor hypothesis (H18) explicitly refuses to make.

EI-22 Agent identity and delegated authorization. VERIFIED (Google Cloud Agent Identity
      overview; IETF draft KAIF; Ping Identity). Agents get strongly attested cryptographic
      identity based on SPIFFE; delegated authorization uses OAuth 2.0 token exchange
      (RFC 8693) to mint per-boundary tokens.
      -> "Which agent is this, and what may it do at this boundary" is solved for cooperating
      agents inside an identity infrastructure. It does not answer "what does this claim
      mean", and it presumes the agent has an identity infrastructure to begin with.
      URL: https://docs.cloud.google.com/iam/docs/agent-identity-overview

EI-23 Capability-based security and least authority. VERIFIED (object-capability model;
      capability-based security; Miller's "Paradigm Lost: Abstraction Mechanisms for Access
      Control"). A capability is "a transferable right to perform one (or more) operations
      on a given object"; access-control systems are evaluated in part on how well they
      support the Principle of Least Authority.
      -> The separation Umbral requires — capability ≠ authorization — is the founding
      distinction of this entire field, not a new idea. Macaroons (Google) add attenuable,
      contextually-caveated credentials, which is a candidate mechanism for H22's
      "delegation" question.
      URL: https://en.wikipedia.org/wiki/Object-capability_model ; https://srl.cs.jhu.edu/pubs/SRL2003-03.pdf

EI-24 Consent and confirmation in shipped products. KNOWN-THEORY / SNIPPET-LEVEL. OAuth 2.0
      is an authorization framework (not authentication) and its scopes are the standard
      mechanism for narrowing delegated access; Linux capabilities(7) split root privilege
      into discrete named capabilities so a process holds only what it needs.
      -> Confirmation-required operations and per-capability grants are EXISTING PATTERN.
      Umbral's contribution can only be in what the confirmation SHOWS (the preview), not in
      the existence of confirmation.

================================================================================
E. WHETHER SELF-DESCRIPTION ACTUALLY WORKS — THE MEASURED EVIDENCE
================================================================================

EI-25 Tool-interface quality is a measured bottleneck. VERIFIED (arXiv:2602.20426, "Learning
      to Rewrite Tool Descriptions for Reliable LLM-Agent Tool Use", v2 2026-04-29). Abstract,
      quoted: "agent performance increasingly plateaus due to the quality of the tool
      interfaces these agents consume. Tool descriptions are often written for human
      developers and tolerate ambiguity that agents cannot resolve, particularly as the
      number of candidate tools grows." Reported result: "reducing accuracy degradation by
      29.23% and improving average query-level success by 60.89% on StableToolBench" as
      catalogs scale to 150+ candidates.
      -> THE CENTRAL PIECE OF EVIDENCE FOR THE SELF-DESCRIPTION HYPOTHESIS. It says the
      interface description is not a detail; it is a first-order determinant of whether an
      agent succeeds, and its importance GROWS with the number of candidates. It also says
      descriptions written for humans are systematically inadequate for agents — which is
      precisely the failure mode of documenting Umbral for humans and expecting AIs to cope.
      URL: https://arxiv.org/abs/2602.20426

EI-26 Tool selection fails predictably and is diagnosable. VERIFIED (arXiv:2607.04686
      ToolFailBench: "A model that never calls a needed tool and a model that calls the tool
      but ignores the result can l[ook identical in aggregate scores]"; arXiv:2608.04719
      canary tools: "Agent evaluations tell us that a model picked the wrong tool, but rarely
      why"); SNIPPET-LEVEL (EMNLP 2025, "Tool Preferences in Agentic LLMs are Unreliable":
      "LLMs rely entirely on the text des[criptions]").
      -> There is a measurement methodology for exactly the failure Umbral's self-description
      layer is meant to prevent (wrong capability chosen, right capability ignored). E-CO-7
      can be built on it rather than invented.
      URL: https://doi.org/10.48550/arxiv.2607.04686 ; https://arxiv.org/html/2608.04719v1

EI-27 Explanation and trust calibration. SNIPPET-LEVEL (C-XAI conceptual framework, UCL;
      "Exploring Trust Calibration in XAI — The Impact of Exposing Model Limitations to Lay
      Users"; Computers in Human Behavior study on XAI effects on trust). Trust calibration
      is "aligning user trust judgment with model capability"; exposing limitations changes
      lay users' trust.
      -> Relevant to the stance hypothesis (H17) and to objection OBJ-11: an explanation is
      not neutral. What Umbral discloses about its own confidence changes how much it is
      believed, independently of whether the disclosure is accurate.
      URL: https://discovery.ucl.ac.uk/id/eprint/10189099/1/1-s2.0-S2666659624000027-main.pdf

================================================================================
F. WHAT THE COEXISTENCE STAGE ALREADY ESTABLISHED (carried, not re-derived)
================================================================================
From research/COEXISTENCE-RESEARCH.md (CO-n), unchanged and not weakened by this cycle:
  CO-3   MCP has no versioning of the state a resource or tool returns; the 2026-07-28
         revision deprecated Roots, Sampling and Logging.
  CO-7   Stale belief in agents is a named, measured failure ("fresh memory, stale plans",
         arXiv:2609.03340).
  CO-9   Concurrent-writer harm is documented for DEVELOPERS running concurrent agents.
  CO-11  Provenance vocabularies exist; attestation proves origin, not truth.
  CO-12  Agent identity standards exist; they answer "which agent", not "what does this mean".
  CO-18  Products already claim "agent-native workspace" (even.dev, AgentMux, Vecbase).

================================================================================
G. THE 13-QUESTION PRIOR-ART MATRIX (mandate §19)
================================================================================
Columns: 1 self-description | 2 capabilities | 3 authority | 4 current state | 5 validity
         6 provenance | 7 disagreement | 8 survives provider change | 9 survives software
         disappearance | 10 works over the user's real filesystem | 11 requires vendor
         cooperation | 12 requires a live process | 13 what remains unsolved
Legend: Y yes | P partial | N no | - not applicable to this system.

System              | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10| 11| 12| 13 unsolved
--------------------|---|---|---|---|---|---|---|---|---|---|---|---|--------------
MCP server (EI-7)   | Y | Y | N | P | N | N | N | P | N | P | Y | Y | authority class,
                    |   |   |   |   |   |   |   |   |   |   |   |   | mutability, egress,
                    |   |   |   |   |   |   |   |   |   |   |   |   | versioning of returns
A2A agent card (EI-8)| Y| Y | P | P | N | N | N | P | N | N | Y | Y | same, plus both sides
                    |   |   |   |   |   |   |   |   |   |   |   |   | must speak A2A
AGENTS.md / Skills  | P | Y | N | N | N | N | N | Y | Y | Y | N | N | no authority, no
   (EI-9)           |   |   |   |   |   |   |   |   |   |   |   |   | state, no currency
OpenAPI / JSON Schema| Y| Y | N | N | N | N | N | Y | Y | N | N | N | no runtime state
Kubernetes API      | Y | Y | P | Y | N | P | N | N | N | N | Y | Y | project semantics;
   discovery (EI-1,2)|   |   |   |   |   |   |   |   |   |   |   |   | not the user's files
Kubernetes control- | P | Y | P | Y | P | P | N | N | N | N | Y | Y | no desired state to
   lers (EI-13)     |   |   |   |   |   |   |   |   |   |   |   |   | reconcile toward
D-Bus introspection | Y | Y | N | Y | N | N | N | Y | N | N | N | Y | local process only
   (EI-3)           |   |   |   |   |   |   |   |   |   |   |   |   |
SNMP / MIB (EI-4)   | Y | Y | P | Y | N | P | N | Y | Y | N | Y | Y | network devices; no
                    |   |   |   |   |   |   |   |   |   |   |   |   | project/knowledge model
JMX (EI-5)          | Y | Y | P | Y | N | P | N | Y | N | N | N | Y | in-process only
/proc, sysfs (EI-6) | P | P | N | Y | N | N | N | Y | Y | Y | N | N | exposes reality as
                    |   |   |   |   |   |   |   |   |   |   |   |   | files; no semantics,
                    |   |   |   |   |   |   |   |   |   |   |   |   | no authority
W3C PROV (EI-21)    | N | N | P | N | N | Y | P | Y | Y | Y | N | N | says nothing about
                    |   |   |   |   |   |   |   |   |   |   |   |   | truth, validity, or
                    |   |   |   |   |   |   |   |   |   |   |   |   | who may act
in-toto (EI-21)     | N | N | P | N | N | Y | N | Y | Y | Y | Y | N | same; needs signing
                    |   |   |   |   |   |   |   |   |   |   |   |   | infrastructure
Agent identity      | P | P | Y | N | N | P | N | Y | N | N | Y | Y | identity != meaning;
   (EI-22)          |   |   |   |   |   |   |   |   |   |   |   |   | needs infrastructure
Capability security | P | P | Y | N | N | N | N | Y | Y | Y | N | N | theory, not a product;
   (EI-23, EI-24)   |   |   |   |   |   |   |   |   |   |   |   |   | nothing to discover
Git                 | N | N | P | P | P | Y | P | Y | Y | Y | N | N | author != authority;
                    |   |   |   |   |   |   |   |   |   |   |   |   | no interpretation,
                    |   |   |   |   |   |   |   |   |   |   |   |   | no validity of BELIEF
Agent memory        | P | P | N | P | P | P | N | N | N | N | Y | Y | per-vendor, not
   (CO-4..CO-6)     |   |   |   |   |   |   |   |   |   |   |   |   | portable, not the
                    |   |   |   |   |   |   |   |   |   |   |   |   | user's
TMS / belief rev.   | N | N | N | N | Y | Y | Y | Y | Y | Y | N | N | theory with no
   (EI-17,18)       |   |   |   |   |   |   |   |   |   |   |   |   | filesystem embodiment
Wikipedia dispute   | N | N | P | P | P | P | Y | Y | Y | Y | N | N | social process; needs
   model (EI-20)    |   |   |   |   |   |   |   |   |   |   |   |   | continuous human labour

READING OF THE MATRIX (this is the load-bearing conclusion of this file):
  - No system in the matrix scores Y on columns 3, 5 and 9 simultaneously.
  - Column 9 (survives the software's disappearance) is satisfied only by systems that are
    TEXT IN THE USER'S FILES: /proc-style exposure, Git, AGENTS.md/Skills, PROV, in-toto,
    Wikipedia. Every system that satisfies column 3 (authority) or 5 (validity) at a useful
    strength requires a live process, a vendor, or an identity infrastructure.
  - The conjunction (authority + validity + no software, no vendor, on the user's actual
    filesystem) is empty in this matrix. That empty cell is the only defensible claim of
    differentiation this cycle has, and it is a claim about a COMBINATION, not an invention.
