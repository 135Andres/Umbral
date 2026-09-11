#!/usr/bin/env python3
"""
EXP-1 instrument: semantic projection over an arbitrary filesystem.

This is an EXPERIMENTAL INSTRUMENT, not FSP architecture. It uses stdlib only
(sqlite3 + FTS5), no embeddings, no LLM, no database beyond a scratch index.

Two measured questions:
  A. Can a useful semantic projection exist over arbitrary files with no
     user-supplied taxonomy?  (retrieval tests, baseline vs. enrichment)
  B. Does semantic identity stay coherent when the filesystem changes
     externally?  (mutation tests)

Modes compared:
  B0  lexical baseline: BM25 over path + basename + body (FTS5), OR-terms
  B1  B0 + link graph: relative markdown links and basename mentions, 1 hop
  B2  B1 + typed edges (supersedes / blocked by / related) + currentness
      demotion of superseded documents, 2 hops with decay

Usage: python3 harness.py [--json OUT] [--workdir TMP]
"""
import os, re, sys, json, shutil, hashlib, sqlite3, tempfile, subprocess, argparse

CORPUS = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "corpus-messy")
TEXT_EXT = {".md", ".txt", ".csv", ".json", ".yaml", ".yml", ".py", ".html", ".cfg", ".ini"}

# ---------------------------------------------------------------- corpus

def read_text(path):
    try:
        with open(path, "r", encoding="utf-8") as fh:
            return fh.read()
    except (UnicodeDecodeError, OSError):
        return None

def load_corpus(root):
    docs = {}
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d != ".git"]
        for f in fn:
            p = os.path.join(dp, f)
            rel = os.path.relpath(p, root)
            st = os.stat(p)
            raw = open(p, "rb").read()
            docs[rel] = {
                "rel": rel,
                "dir": os.path.dirname(rel),
                "basename": f,
                "stem": os.path.splitext(f)[0],
                "ext": os.path.splitext(f)[1].lower(),
                "size": st.st_size,
                "mtime": int(st.st_mtime),
                "sha256": hashlib.sha256(raw).hexdigest(),
                "text": read_text(p),
            }
    return docs

TOKEN = re.compile(r"[A-Za-z0-9_]+")

def tokens(text):
    return [t.lower() for t in TOKEN.findall(text or "") if len(t) > 1]

# ---------------------------------------------------------------- projection (deterministic)

LINK_RE = re.compile(r"\[[^\]]*\]\(([^)]+)\)")
BARE_RE = re.compile(r"(?<![\w/])((?:\.\./|[\w.-]+/)*[\w.-]+\.(?:md|txt|csv|json|py|yaml|yml|html))")
TYPED_RE = re.compile(r"^\s*(supersedes|superseded by|blocked by|blocks|related|see)\s*:?\s*(.+)$",
                      re.IGNORECASE | re.MULTILINE)
QUESTION_RE = re.compile(r"^\s*Q\d+\s*\((open|resolved)[^)]*\)\s*(.+)$", re.IGNORECASE | re.MULTILINE)
DATE_RE = re.compile(r"\b(20\d\d-\d\d-\d\d)\b")
AUTHOR_RE = re.compile(r"^\s*Author:\s*(.+)$", re.IGNORECASE | re.MULTILINE)
CHECKBOX_RE = re.compile(r"^\s*\[( |x|X)\]\s*(.+)$", re.MULTILINE)

def project(docs):
    """Deterministic semantic projection: no user metadata, no model."""
    names = {d["basename"]: d["rel"] for d in docs.values()}
    names.update({d["stem"]: d["rel"] for d in docs.values()})

    def resolve(doc_dir, ref):
        """Path-relative first, then basename. Returns None if unresolvable."""
        ref = ref.strip().strip("`").strip()
        cand = os.path.normpath(os.path.join(doc_dir, ref))
        if cand in docs:
            return cand
        base = os.path.basename(ref)
        return names.get(base) or names.get(os.path.splitext(base)[0])

    proj = {}
    for rel, d in docs.items():
        t = d["text"] or ""
        links, typed, questions, dates, authors, todos = set(), [], [], [], [], []
        refs = [m.group(1) for m in LINK_RE.finditer(t)]
        refs += [m.group(1) for m in BARE_RE.finditer(t)]
        for ref in refs:
            if ref.startswith(("http://", "https://", "#")):
                continue
            r = resolve(d["dir"], ref)
            if r and r != rel:
                links.add(r)
        for m in TYPED_RE.finditer(t):
            kind, rest = m.group(1).lower(), m.group(2).strip()
            for cand in BARE_RE.findall(rest):
                r = resolve(d["dir"], cand)
                if r and r != rel:
                    typed.append((kind, r))
        # basename mentions anywhere in the text (a weak, high-recall edge)
        mentioned = {names[s] for s in names if s != d["basename"] and s in t}
        for m in QUESTION_RE.finditer(t):
            questions.append({"state": m.group(1).lower(), "text": m.group(2).strip()})
        dates = DATE_RE.findall(t)
        authors = [a.strip() for a in AUTHOR_RE.findall(t)]
        todos = [m.group(2).strip() for m in CHECKBOX_RE.finditer(t)]
        proj[rel] = {"links": sorted(links), "typed": typed, "mentioned": sorted(mentioned),
                     "questions": questions, "dates": sorted(set(dates)), "authors": authors,
                     "todos": todos}
    # currentness: a doc is superseded if another doc's typed edge names it
    superseded_by = {}
    for rel, p in proj.items():
        for kind, tgt in p["typed"]:
            if kind.startswith("supersed") and tgt in proj:
                superseded_by.setdefault(tgt, []).append(rel)
    for rel in proj:
        proj[rel]["superseded_by"] = superseded_by.get(rel, [])
        proj[rel]["superseded"] = bool(superseded_by.get(rel))
    return proj

# ---------------------------------------------------------------- index + search

def build_index(docs, proj, path):
    if os.path.exists(path):
        os.remove(path)
    db = sqlite3.connect(path)
    db.execute("CREATE VIRTUAL TABLE docs USING fts5(rel, basename, body, tokenize='unicode61')")
    for rel, d in sorted(docs.items()):
        # the true path is indexed: unicode61 already splits on '/' '-' '_' '.',
        # so a path-word hit is a signal without a second synthetic column
        db.execute("INSERT INTO docs(rel, basename, body) VALUES (?,?,?)",
                   (rel, d["stem"], d["text"] or ""))
    db.commit()
    return db

def lexical(db, query, limit=10):
    terms = sorted(set(tokens(query)))
    if not terms:
        return []
    match = " OR ".join('"%s"' % t for t in terms)
    try:
        rows = db.execute(
            "SELECT rel, bm25(docs, 3.0, 3.0, 1.0) AS s FROM docs WHERE docs MATCH ? "
            "ORDER BY s LIMIT ?", (match, limit)).fetchall()
    except sqlite3.OperationalError:
        return []
    return [(-s, r) for r, s in rows]          # higher = better

def search(db, docs, proj, query, mode):
    base = lexical(db, query, limit=10)
    if mode == "B0":
        return [(r, s, "lexical") for s, r in base]
    scores = {r: s for s, r in base}
    why = {r: "lexical" for r, _ in base}
    def bump(target, amount, reason, hop):
        if target not in docs:
            return
        if scores.get(target, 0) < amount:
            scores[target] = amount
            why[target] = "%s(h%d)" % (reason, hop)
    # hop 1: expand along links/mentions from the top lexical hits
    for s, r in base[:4]:
        for tgt in proj[r]["links"] + proj[r]["mentioned"]:
            bump(tgt, s * 0.55, "linked", 1)
        for kind, tgt in proj[r]["typed"]:
            if mode == "B2":
                bump(tgt, s * 0.75, "typed:%s" % kind, 1)
    # hop 2 (B2 only): typed edges from hop-1 arrivals
    if mode == "B2":
        for r in list(scores):
            for kind, tgt in proj.get(r, {}).get("typed", []):
                bump(tgt, scores[r] * 0.4, "typed:%s" % kind, 2)
    # currentness: demote documents that another document supersedes
    if mode == "B2":
        for r in list(scores):
            if proj[r]["superseded"]:
                scores[r] *= 0.45
                why[r] = why.get(r, "lexical") + "+superseded"
    ranked = sorted(scores.items(), key=lambda kv: -kv[1])
    return [(r, s, why.get(r, "lexical")) for r, s in ranked[:10]]

def first_hop(results, target):
    """How far from a lexical hit the target was: 0 = lexical, 1 or 2 = graph."""
    for r, s, w in results:
        if r == target:
            if "h2" in w:
                return 2
            if "h1" in w:
                return 1
            return 0
    return None

# ---------------------------------------------------------------- query set

QUERIES = [
    {"id": "Q1", "class": "exact-name", "q": "deploy checklist",
     "accept": ["deploy-checklist.md"], "partial": [], "forbidden": ["archive/old-spec-v1.md"]},
    {"id": "Q2", "class": "vocabulary-mismatch",
     "q": "how long do users wait for the screen to update",
     "accept": ["research/latency-notes.md"], "partial": ["metrics.json"],
     "forbidden": ["archive/old-spec-v1.md", "archive/meeting-2022.md"]},
    {"id": "Q3", "class": "conceptual", "q": "what would hosting cost us",
     "accept": ["cost-model.csv", "vendor-comparison-nimbus-stratus.md"],
     "partial": ["archive/pricing-old.csv"],
     "forbidden": ["archive/old-spec-v1.md"]},
    {"id": "Q4", "class": "relationship", "q": "which work is blocked by unresolved questions",
     "accept": ["deploy-checklist.md", "open-questions.md"], "partial": ["TODO.txt"],
     "forbidden": ["archive/old-spec-v1.md"]},
    {"id": "Q5", "class": "multi-hop", "q": "if we choose Nimbus what else is affected",
     "accept": ["research/vendor-security-review.md", "vendor-comparison-nimbus-stratus.md"],
     "partial": ["api-draft.md", "q3-planning.md"], "forbidden": ["archive/old-spec-v1.md"]},
    {"id": "Q6", "class": "irrelevant", "q": "chocolate cake recipe with butter",
     "accept": [], "partial": [], "forbidden": None},
    {"id": "Q7", "class": "missing-information", "q": "what is our GDPR data retention policy",
     "accept": ["open-questions.md"], "partial": [], "forbidden": None},
    {"id": "Q8", "class": "provenance", "q": "who decided the polling approach and when",
     "accept": ["decisions-2024.md"], "partial": ["ADR-007-realtime-updates.md"],
     "forbidden": ["archive/old-spec-v1.md"]},
]

def grade(q, results, top=3):
    """Returns a verdict per the mandate's vocabulary. No plausibility scoring."""
    got = [r for r, _, _ in results[:top]]
    allgot = [r for r, _, _ in results]
    verdict = {"query": q["id"], "class": q["class"], "top3": got,
               "detail": [(r, round(s, 3), w) for r, s, w in results[:top]],
               "result": "not-found"}
    if not q["accept"] and not q["partial"]:
        # abstention tests
        verdict["result"] = "correct-abstention" if not allgot else "spurious-results"
        return verdict
    hops = {}
    for a in q["accept"] + q["partial"]:
        h = first_hop(results, a)
        if h is not None:
            hops[a] = h
    verdict["hops"] = hops
    if any(a in got for a in q["accept"]):
        verdict["result"] = "retrieved-correctly"
    elif any(a in allgot for a in q["accept"]):
        verdict["result"] = "retrieved-below-top3"
    elif any(a in allgot for a in q["partial"]):
        verdict["result"] = "partially-retrieved"
    else:
        verdict["result"] = "not-found"
    if q["forbidden"] and any(f in got for f in q["forbidden"]):
        verdict["result"] += " +forbidden-in-top3"
    return verdict

# ---------------------------------------------------------------- mutations

def reconcile(registry, docs, state):
    """Path+hash registry; hash match -> move/rename; basename+similarity -> edit.
    This measures the problem. It is not a proposed identity architecture.
    `state` carries cross-mutation memory so a pre-existing duplicate is not
    re-reported on every mutation."""
    events = []
    by_hash = {}
    for rel, d in docs.items():
        by_hash.setdefault(d["sha256"], []).append(rel)
    live = {rel: d["sha256"] for rel, d in docs.items()}
    # 1. duplicates: report only duplicates that are NEW since the previous state
    dups = {h: sorted(rels) for h, rels in by_hash.items() if len(rels) > 1}
    for h, rels in dups.items():
        if h not in state["dups"]:
            events.append({"event": "duplicate-content", "hash": h[:12], "paths": rels})
    state["dups"] = set(dups)
    # 2. renames/moves: a known hash whose recorded path is gone but hash is present elsewhere
    for ent in registry:
        was_absent = ent.get("present") is False
        if ent["path"] in live:
            if live[ent["path"]] != ent["sha256"]:
                events.append({"event": "content-changed-in-place", "path": ent["path"]})
                ent["sha256"] = live[ent["path"]]
                ent["text"] = docs[ent["path"]]["text"] or ""
            elif was_absent:
                events.append({"event": "restored", "path": ent["path"]})
            ent["present"] = True
            continue
        if ent["sha256"] in by_hash and ent["path"] not in live:
            new = [p for p in by_hash[ent["sha256"]] if p != ent["path"]]
            if new:
                events.append({"event": "rename-or-move", "from": ent["path"], "to": new[0]})
                ent["path"] = new[0]
                ent["present"] = True
                continue
        if ent["path"] not in live:
            # content changed: try basename match with a similarity score
            best, best_score = None, 0.0
            for rel, d in docs.items():
                if d["basename"] == os.path.basename(ent["path"]):
                    old_t = set(tokens(ent.get("text", "")))
                    new_t = set(tokens(d["text"] or ""))
                    if old_t or new_t:
                        j = len(old_t & new_t) / max(1, len(old_t | new_t))
                        if j > best_score:
                            best, best_score = rel, j
            if best and best_score >= 0.5:
                events.append({"event": "edit-in-place", "path": best,
                               "similarity": round(best_score, 3)})
                ent["path"], ent["sha256"] = best, docs[best]["sha256"]
                ent["text"] = docs[best]["text"] or ""
                continue
            events.append({"event": "deleted-or-unknown", "path": ent["path"]})
            ent["present"] = False
    # 3. new files not in the registry
    known = {e["sha256"] for e in registry if e.get("present", True)}
    for rel, d in docs.items():
        if d["sha256"] not in known and not any(e["path"] == rel for e in registry):
            events.append({"event": "new-file", "path": rel})
    for ent in registry:
        if ent["present"] is not False and ent["path"] in live:
            ent["sha256"] = live[ent["path"]]
            ent["text"] = docs[ent["path"]]["text"] or ""
            ent["present"] = True
    return events

def stale_refs(docs, proj, before_paths):
    """Links pointing at a path that no longer exists."""
    stale = []
    for rel, p in proj.items():
        for tgt in p["links"] + [t for _, t in p["typed"]]:
            if tgt not in docs and tgt in before_paths:
                stale.append({"in": rel, "target": tgt})
    return stale

def mutations(workdir, corpus):
    results = []
    shutil.rmtree(workdir, ignore_errors=True)
    shutil.copytree(corpus, workdir)

    def snap():
        d = load_corpus(workdir)
        return d, project(d)

    docs, proj = snap()
    registry = [{"path": r, "sha256": d["sha256"], "text": d["text"] or "", "present": True}
                for r, d in sorted(docs.items())]

    state = {"dups": set()}
    reconcile(registry, docs, state)      # baseline pass: learn pre-existing duplicates

    def run(name, fn, description, expect_lost=()):
        before_paths = set(load_corpus(workdir).keys())
        fn()
        docs2, proj2 = snap()
        events = reconcile(registry, docs2, state)
        stale = stale_refs(docs2, proj2, before_paths)
        lost = [e["path"] for e in events if e["event"] == "deleted-or-unknown"]
        unexpected = [p for p in lost if os.path.basename(p) not in expect_lost]
        collisions = [e for e in events if e["event"] == "duplicate-content"]
        results.append({
            "mutation": name, "what": description,
            "events": events,
            "stale_links": stale,
            "entities_tracked": len(registry),
            "entities_lost": lost,
            "unexpected_loss": unexpected,
            "collisions": [c["paths"] for c in collisions],
            "identity_preserved": not unexpected,
            "reconciliation_actions": len(events),
        })

    run("M1-rename", lambda: os.rename(os.path.join(workdir, "deploy-checklist.md"),
                                      os.path.join(workdir, "deploy.md")),
        "rename deploy-checklist.md -> deploy.md")
    run("M2-move", lambda: (os.makedirs(os.path.join(workdir, "research"), exist_ok=True),
                            os.replace(os.path.join(workdir, "misc/bug-1421.md"),
                                       os.path.join(workdir, "research/bug-1421.md"))),
        "move misc/bug-1421.md -> research/bug-1421.md")
    run("M3-dir-move", lambda: os.rename(os.path.join(workdir, "archive"),
                                         os.path.join(workdir, "old")),
        "move directory archive/ -> old/")
    run("M4-duplicate", lambda: shutil.copy2(os.path.join(workdir, "decisions-2024.md"),
                                             os.path.join(workdir, "decisions-2024 (copy).md")),
        "duplicate decisions-2024.md (identical content, two live paths)")
    run("M5-delete", lambda: os.remove(os.path.join(workdir, "notes.txt")),
        "delete notes.txt", expect_lost=("notes.txt",))
    run("M6-restore", lambda: shutil.copy2(os.path.join(corpus, "notes.txt"),
                                           os.path.join(workdir, "notes.txt")),
        "restore notes.txt from outside the workspace")
    def edit():
        p = os.path.join(workdir, "docs/api.md")
        with open(p, "a") as fh:
            fh.write("\nTODO: define rate limits before the pilot extension.\n")
    run("M7-external-edit", edit, "append a line to docs/api.md (content edit)")
    def atomic():
        p = os.path.join(workdir, "docs/faq.md")
        tmp = p + ".tmp"
        shutil.copy2(p, tmp)
        with open(tmp, "a") as fh:
            fh.write("\nQ: Is there a mobile app?\nA: Not yet.\n")
        os.replace(tmp, p)
    run("M8-atomic-save", atomic, "write tmp + rename over docs/faq.md")
    def git(d, *a):
        env = dict(os.environ, GIT_AUTHOR_NAME="x", GIT_AUTHOR_EMAIL="x@x",
                   GIT_COMMITTER_NAME="x", GIT_COMMITTER_EMAIL="x@x")
        return subprocess.run(["git", "-C", d, *a], env=env, capture_output=True)

    git(workdir, "init", "-q"); git(workdir, "add", "-A"); git(workdir, "commit", "-qm", "base")
    head = git(workdir, "rev-parse", "--abbrev-ref", "HEAD").stdout.decode().strip()

    def branch_out():
        git(workdir, "checkout", "-q", "-b", "rework")
        os.rename(os.path.join(workdir, "app"), os.path.join(workdir, "service"))
        with open(os.path.join(workdir, "README.md"), "a") as fh:
            fh.write("\n(branch: service/ instead of app/)\n")
        git(workdir, "add", "-A"); git(workdir, "commit", "-qm", "rename app -> service")

    run("M9-vcs-branch-switch", branch_out,
        "git checkout of a branch that renamed a directory (bulk replacement, in effect)")
    run("M10-vcs-return", lambda: git(workdir, "checkout", "-q", head),
        "git checkout back to the original branch (bulk reversal)")
    return results

# ---------------------------------------------------------------- main

def scale_test(corpus, factors=(1, 10, 50)):
    """Cold index build, query latency and naive re-index cost as the corpus grows.
    Measurements are from THIS machine and this corpus only."""
    import time
    rows = []
    base = tempfile.mkdtemp(prefix="exp1-scale-")
    for f in factors:
        root = os.path.join(base, "x%d" % f)
        shutil.rmtree(root, ignore_errors=True)
        for i in range(f):
            shutil.copytree(corpus, os.path.join(root, "copy%03d" % i))
        t0 = time.perf_counter()
        docs = load_corpus(root)
        t_scan = time.perf_counter() - t0
        proj = project(docs)
        db = build_index(docs, proj, os.path.join(base, "i%d.db" % f))
        t_index = time.perf_counter() - t0
        t0 = time.perf_counter()
        for _ in range(20):
            lexical(db, "deploy checklist", limit=5)
        t_query = (time.perf_counter() - t0) / 20
        # naive re-index after a single-file edit: full re-hash of the tree
        victim = os.path.join(root, "copy000", "docs", "api.md")
        with open(victim, "a") as fh:
            fh.write("\nedit\n")
        t0 = time.perf_counter()
        docs2 = load_corpus(root)
        t_reindex_naive = time.perf_counter() - t0
        db.close()
        rows.append({"copies": f, "files": len(docs2),
                     "scan_s": round(t_scan, 4), "cold_index_s": round(t_index, 4),
                     "query_ms": round(t_query * 1000, 3),
                     "naive_reindex_after_1_edit_s": round(t_reindex_naive, 4),
                     "bytes": sum(d["size"] for d in docs2.values())})
    shutil.rmtree(base, ignore_errors=True)
    return rows

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", default=os.path.join(os.path.dirname(os.path.abspath(__file__)),
                                                   "results.json"))
    ap.add_argument("--workdir", default=None)
    args = ap.parse_args()

    corpus = os.path.abspath(CORPUS)
    docs = load_corpus(corpus)
    proj = project(docs)

    tmpdb = tempfile.mktemp(suffix=".db")
    db = build_index(docs, proj, tmpdb)

    out = {"corpus": {"root": corpus, "files": len(docs),
                      "text_files": sum(1 for d in docs.values() if d["text"] is not None),
                      "binary_files": sum(1 for d in docs.values() if d["text"] is None),
                      "bytes": sum(d["size"] for d in docs.values()),
                      "dirs": sorted({d["dir"] for d in docs.values()})},
           "projection": {"docs_with_links": sum(1 for p in proj.values() if p["links"]),
                          "typed_edges": sum(len(p["typed"]) for p in proj.values()),
                          "superseded_docs": sorted(r for r, p in proj.items() if p["superseded"]),
                          "questions_found": sum(len(p["questions"]) for p in proj.values()),
                          "dates_found": sum(len(p["dates"]) for p in proj.values()),
                          "authors_found": sum(len(p["authors"]) for p in proj.values())},
           "retrieval": {}, "mutations": []}

    for mode in ("B0", "B1", "B2"):
        out["retrieval"][mode] = [grade(q, search(db, docs, proj, q["q"], mode)) for q in QUERIES]

    wd = args.workdir or tempfile.mkdtemp(prefix="exp1-")
    out["mutations"] = mutations(wd, corpus)
    out["scale"] = scale_test(corpus)
    out["workdir"] = wd

    with open(args.json, "w") as fh:
        json.dump(out, fh, indent=2)

    # ---- console summary
    print("CORPUS", out["corpus"]["files"], "files,", out["corpus"]["bytes"], "bytes,",
          len(out["corpus"]["dirs"]), "directories")
    print("PROJECTION", out["projection"])
    print()
    print("%-4s %-22s %-9s %-9s %-9s" % ("id", "class", "B0", "B1", "B2"))
    for i, q in enumerate(QUERIES):
        row = [out["retrieval"][m][i]["result"] for m in ("B0", "B1", "B2")]
        print("%-4s %-22s %-9s %-9s %-9s" % (q["id"], q["class"], *[r[:9] for r in row]))
    print()
    for i, q in enumerate(QUERIES):
        print(q["id"], "|", q["q"])
        for m in ("B0", "B1", "B2"):
            d = out["retrieval"][m][i]
            print("   %s %-22s %s" % (m, d["result"], d["detail"][:3]))
    print()
    for m in out["mutations"]:
        print("%-16s identity_preserved=%-5s actions=%d stale_links=%d events=%s" % (
            m["mutation"], m["identity_preserved"], m["reconciliation_actions"],
            len(m["stale_links"]), [e["event"] for e in m["events"]][:4]))
    print()
    print("SCALE (this machine, this corpus, stdlib only)")
    print("%-7s %-8s %-9s %-10s %-9s %-12s" % ("copies", "files", "scan_s", "cold_idx_s",
                                               "query_ms", "reindex1_s"))
    for r in out["scale"]:
        print("%-7s %-8s %-9s %-10s %-9s %-12s" % (r["copies"], r["files"], r["scan_s"],
                                                   r["cold_index_s"], r["query_ms"],
                                                   r["naive_reindex_after_1_edit_s"]))
    print("\nwrote", args.json)

if __name__ == "__main__":
    main()
