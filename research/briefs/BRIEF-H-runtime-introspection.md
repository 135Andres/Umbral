# BRIEF-H — Runtime Introspection: what a system should say about itself

For an external reasoning model. Narrow problem, deep reasoning. Your output is external
research, not a project decision.

--------------------------------------------------------------------------------
1. PROBLEM
--------------------------------------------------------------------------------
A tool maintains derived state about a person's files — an index, detected relationships,
cached summaries, an activity record. When something looks wrong, or when a participant needs
to decide whether to trust what the tool says, the tool must explain ITSELF: what it is
actually doing, why it produced this result, what it is unsure about, and what it cannot
account for.

Documentation describes intended behaviour. This is about ACTUAL behaviour, now, on this
machine, for this project. Existing systems do this for operators. This one must do it for
AI participants as well, and for a human who is not an engineer.

QUESTION: How should a system expose what it is actually doing, why, and with what
confidence — to both a human and an unfamiliar AI — without asserting more certainty than it
has?

--------------------------------------------------------------------------------
2. CONSTRAINTS
--------------------------------------------------------------------------------
- Read-only by default; the deepest level is user-invoked.
- Must be cheap at the shallow level and precise at the deep level.
- Must survive the software: if the tool is gone, what it recorded must still be readable.
- Must not expose secrets, credentials, or content the user did not intend to share.
- Must not become the authority it reports on: explaining a decision is not re-deciding it.
- The human must be able to understand the shallow level without training.

--------------------------------------------------------------------------------
3. KNOWN EVIDENCE (established; do not re-derive)
--------------------------------------------------------------------------------
- Tiering is standard practice: Kubernetes separates liveness from readiness from startup
  probes; JMX exposes registered MBeans with attributes, operations and notifications;
  `git fsck` and `npm doctor` provide user-invoked, read-only whole-environment checks.
- The documentation/actuality gap is what configuration management calls DRIFT, and drift
  detection is a solved problem for configuration.
- Uncertain-knowledge-graph research shows point confidence estimates are easy and CALIBRATED
  uncertainty is recent and hard; most published methods produce point estimates only.
- Explanations change trust: exposing a system's limitations measurably shifts how much lay
  users trust it, independently of accuracy.
- Agents fail at tool use in distinguishable ways, but aggregate scores hide the difference
  between "never called the right tool" and "called it and ignored the result".

--------------------------------------------------------------------------------
4. UNKNOWNS
--------------------------------------------------------------------------------
- Whether a numeric confidence score helps participants or manufactures false precision.
  Our working position is that a number asserts a calibration the system has not measured —
  argue this either way.
- Whether an AI participant will ever request the deep tier, or will act on the shallow tier
  regardless of what the deep tier would have shown.
- Whether users can distinguish "the tool is unsure" from "the tool is broken".
- How much diagnostic detail is safe to expose to an AI that may be untrusted.

--------------------------------------------------------------------------------
5. WHAT WE WANT FROM YOU
--------------------------------------------------------------------------------
1. Propose the smallest useful structure for a self-report. If a three-level model
   (orientation / explanation / diagnosis) is wrong, propose the right one and say why.
2. Resolve the confidence question: how should certainty be represented so that it is
   informative, honest about not being calibrated, and not abusable by a reader who treats a
   number as a fact? Be concrete.
3. Specify what a system should report when it CANNOT account for something — a change it did
   not make, a result it cannot explain, an actor it cannot identify. What is the honest
   output, and how does a reader act on it?
4. Name what must never appear in a self-report, and why.
5. Give the experiment: a seeded fault, and a measurement that distinguishes a useful
   self-report from a decorative one.

--------------------------------------------------------------------------------
6. OUTPUT FORMAT
--------------------------------------------------------------------------------
- Plain-language summary first (5 sentences maximum).
- Then: the proposed structure; the confidence representation (concrete); the
  cannot-account-for output; the never-report list; the experiment design.
- Label every claim ESTABLISHED / REASONED / SPECULATIVE.

--------------------------------------------------------------------------------
7. PLAIN-LANGUAGE NOTE FOR THE USER
--------------------------------------------------------------------------------
In one sentence: *when the tool that organizes your files says something, how should it tell
you how sure it is and what it cannot account for — without sounding more certain than it
is?* This brief asks a model to design that self-report and how to test it.
