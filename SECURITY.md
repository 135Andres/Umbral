# Security Policy

Umbral is an early research project. It has **no release, no supported version and no
deployed service**. What exists is documentation and one frozen prototype
(`fsp-check/`), which runs locally and has no network surface.

That said, security claims are exactly the kind of claim this project refuses to make
loosely, so this policy is written plainly.

---

## What counts as a vulnerability here

In scope:

- **In the prototype (`fsp-check/`)** — anything that makes it read or modify files it was
  not asked to touch; a crash or corrupted state that could be mistaken for valid
  recorded evidence; a hash recorded as valid for bytes that were not the bytes read;
  a path-traversal or symlink-following behaviour that escapes the scan root; resource
  exhaustion triggered by a hostile file tree.
- **In this repository** — committed credentials, tokens or private keys; private
  information about a person published by mistake; a document that instructs a
  contributor or an agent to do something unsafe while looking authoritative.

Out of scope (by design, not by oversight — see
[`experiments/v0-harness/V0-CLOSEOUT.md`](experiments/v0-harness/V0-CLOSEOUT.md)):

- absence of power-loss durability;
- absence of multi-writer concurrency control;
- absence of authentication, authorization or encryption — the prototype has no such
  surface;
- performance characteristics of an experimental prototype;
- anything about a product that does not exist yet.

## How to report

**Do not open a public issue, discussion or pull request for a vulnerability.**

> **TODO / MAINTAINER DECISION REQUIRED:** the private reporting channel (a security
> contact address, or GitHub private vulnerability reporting) has not been established
> yet. This section must name a real channel before this repository is made public. Until
> then, report nothing publicly and note the finding for the maintainer.

When a channel exists, a report should include:

- what the issue is, and where (file, function, commit);
- how to reproduce it, as concretely as possible — a command, a fixture, a file tree;
- what an attacker could achieve, and what they could not;
- the version or commit you tested;
- whether you have already disclosed it anywhere else.

Do not include real personal data, credentials or private file contents in a report. If a
reproduction needs a file tree, generate a synthetic one.

## What to expect

No response-time commitment is made. This project has no full-time maintainer and no
funding; promising a service level would be exactly the kind of claim the project avoids.
Reports are read, and genuine issues are fixed or recorded as known limitations.

## Known limitations that are not vulnerabilities

The prototype is deliberately partial. Its limitations are documented rather than
patched over: scan-based change detection (no event stream), process-kill crash
consistency only, one logical writer, no reader-facing surface, no full-text index. See
the closeout record for the complete list.
