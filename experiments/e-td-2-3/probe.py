#!/usr/bin/env python3
"""E-TD-2 and E-TD-3 probe. Specification: README.md in this directory (fixed before the run);
protocols: docs/candidates/V0.2-TECHNICAL-DESIGN.md §H.1 and §H.2.

Usage:
    python3 probe.py --umbral PATH/TO/umbral --label NAME DIR

Runs every arm against a temporary directory created under DIR and removed afterwards, and
writes one JSON document to standard output. Standard library only.
"""

import argparse
import hashlib
import itertools
import json
import os
import platform
import shutil
import subprocess
import sys
import tempfile
import time

REPETITIONS = 50          # README §2: per E-TD-2 variant
GRANULARITY_WAIT_S = 1.0  # README §2: §H.1 step 4
BUDGET = 1000             # README §2: E-TD-3 attempts
LENGTH = 16
VARIANTS = ["restore", "atomic", "touch", "chmod"]


# ---------------------------------------------------------------------------------------
# The skip condition of §C.1, as a pure function over two lstat results
# ---------------------------------------------------------------------------------------

def decide(prev, cur, with_ctime):
    """`prev` is the previous observation (None if absent): a dict with the lstat fields and
    `stable` (whether its content reading was stable). `cur` is the current lstat dict."""
    if prev is None:
        return "read"
    ok = (
        prev["is_file"] and cur["is_file"]          # 1
        and prev["stable"]                          # 2: a verified baseline
        and prev["dev"] == cur["dev"]               # 3
        and prev["ino"] == cur["ino"]
        and prev["size"] == cur["size"]             # 4
        and prev["mtime_ns"] == cur["mtime_ns"]     # 5
    )
    if with_ctime:
        ok = ok and prev["ctime_ns"] == cur["ctime_ns"]  # 6
    return "skip" if ok else "read"


def h2c_exhaustive():
    """H2-c: adding ctime never enlarges the skipped set. Every combination of equal/different
    for each field the condition reads, plus kind and stability."""
    keys = ["is_file", "stable", "dev", "ino", "size", "mtime_ns", "ctime_ns"]
    counterexamples = 0
    cases = 0
    for bits in itertools.product([True, False], repeat=len(keys)):
        eq = dict(zip(keys, bits))
        prev = {"is_file": True, "stable": eq["stable"], "dev": 1, "ino": 1, "size": 1,
                "mtime_ns": 1, "ctime_ns": 1}
        cur = {"is_file": eq["is_file"], "dev": 1 if eq["dev"] else 2,
               "ino": 1 if eq["ino"] else 2, "size": 1 if eq["size"] else 2,
               "mtime_ns": 1 if eq["mtime_ns"] else 2, "ctime_ns": 1 if eq["ctime_ns"] else 2}
        cases += 1
        if decide(prev, cur, True) == "skip" and decide(prev, cur, False) != "skip":
            counterexamples += 1
    return {"cases": cases, "counterexamples": counterexamples}


# ---------------------------------------------------------------------------------------
# Filesystem helpers
# ---------------------------------------------------------------------------------------

def stat(path):
    st = os.lstat(path)
    return {
        "is_file": os.path.isfile(path) and not os.path.islink(path),
        "dev": st.st_dev,
        "ino": st.st_ino,
        "size": st.st_size,
        "mtime_ns": st.st_mtime_ns,
        "ctime_ns": st.st_ctime_ns,
        "atime_ns": st.st_atime_ns,
    }


def digest(path):
    with open(path, "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


def write(path, data):
    with open(path, "wb") as fh:
        fh.write(data)


def filesystem(path):
    try:
        out = subprocess.run(["findmnt", "-n", "-T", path, "-o", "FSTYPE,OPTIONS"],
                             capture_output=True, text=True, check=True).stdout.split()
        return {"type": out[0], "options": out[1] if len(out) > 1 else ""}
    except (OSError, subprocess.CalledProcessError, IndexError):
        return {"type": "UNKNOWN", "options": "UNKNOWN"}


# ---------------------------------------------------------------------------------------
# The real verdict: the umbral binary (v0.1's implementation, which reads every file)
# ---------------------------------------------------------------------------------------

class Umbral:
    def __init__(self, binary, root, state):
        self.binary, self.root = binary, root
        self.env = dict(os.environ, XDG_DATA_HOME=state)
        self.run("init")

    def run(self, cmd):
        return subprocess.run([self.binary, cmd, self.root], env=self.env,
                              capture_output=True, text=True).stdout

    def verdict(self, name):
        """The verdict `changes` gives for `name` between the last two runs. An unchanged
        entry has no line of its own; it is read from the `unchanged` count."""
        out = self.run("changes")
        for line in out.splitlines():
            items = line[10:].split("  ")
            if f"path={name}" in items:
                return items[0]
        if "count  unchanged=1" in out:
            return "unchanged"
        return "NOT-FOUND"


def derived_v02(decision, v01):
    """The verdict under a skip, derived (README §1): a skip carries the previous hash, and a
    same-path pair with equal identity, size and mtime is classified without consulting the
    hash. Under a read, the observation holds what v0.1's does."""
    return "unchanged" if decision == "skip" else v01


# ---------------------------------------------------------------------------------------
# E-TD-2 — the metadata-restoring writer (§H.1)
# ---------------------------------------------------------------------------------------

def e_td_2_once(parent, umbral_bin, variant, rep):
    d = tempfile.mkdtemp(dir=parent, prefix="etd2-")
    state = tempfile.mkdtemp(prefix="etd2-state-")
    try:
        f = os.path.join(d, "f")
        b1 = (b"A" if rep % 2 == 0 else b"C") * LENGTH
        b2 = (b"B" if rep % 2 == 0 else b"D") * LENGTH
        write(f, b1)
        s1 = stat(f)                                      # step 2
        h1 = digest(f)                                    # step 3: first observation reads
        u = Umbral(umbral_bin, d, state)
        u.run("observe")
        time.sleep(GRANULARITY_WAIT_S)                    # step 4
        if variant == "restore":                          # step 5
            write(f, b2)
            os.utime(f, ns=(s1["atime_ns"], s1["mtime_ns"]))
        elif variant == "atomic":
            tmp = os.path.join(d, ".f.tmp")
            write(tmp, b2)
            os.utime(tmp, ns=(s1["atime_ns"], s1["mtime_ns"]))
            os.rename(tmp, f)
        elif variant == "touch":
            os.utime(f)
        elif variant == "chmod":
            os.chmod(f, 0o600 if (os.lstat(f).st_mode & 0o777) != 0o600 else 0o644)
        s2 = stat(f)                                      # step 6
        h2 = digest(f)
        same = {k: s1[k] == s2[k] for k in ("size", "mtime_ns", "dev", "ino", "ctime_ns")}
        valid = variant != "restore" or all(same[k] for k in ("size", "mtime_ns", "dev", "ino"))
        prev = dict(s1, stable=True)
        decisions = {c: decide(prev, s2, c == "ctime") for c in ("metadata", "ctime")}  # step 7
        u.run("observe")
        v01 = u.verdict("f")
        return {
            "variant": variant,
            "valid": valid,
            "bytes_differ": h1 != h2,
            "equal": same,
            "ctime_delta_ns": s2["ctime_ns"] - s1["ctime_ns"],
            "decision": decisions,
            # step 8: a read compares the new hash with the first; a skip compares nothing
            "change_detected": {c: decisions[c] == "read" and h1 != h2 for c in decisions},
            "verdict_v01": v01,
            "verdict_v02_derived": {c: derived_v02(decisions[c], v01) for c in decisions},
        }
    finally:
        shutil.rmtree(d, ignore_errors=True)
        shutil.rmtree(state, ignore_errors=True)


def summarise_e_td_2(reps):
    out = {}
    for v in VARIANTS:
        rs = [r for r in reps if r["variant"] == v]
        valid = [r for r in rs if r["valid"]]
        out[v] = {
            "repetitions": len(rs),
            "invalid": len(rs) - len(valid),
            "bytes_differ": sum(r["bytes_differ"] for r in valid),
            "ctime_equal": sum(r["equal"]["ctime_ns"] for r in valid),
            "skip_metadata": sum(r["decision"]["metadata"] == "skip" for r in valid),
            "skip_ctime": sum(r["decision"]["ctime"] == "skip" for r in valid),
            "detected_metadata": sum(r["change_detected"]["metadata"] for r in valid),
            "detected_ctime": sum(r["change_detected"]["ctime"] for r in valid),
            "verdicts_v01": count(r["verdict_v01"] for r in valid),
            "verdicts_v02_derived_metadata": count(r["verdict_v02_derived"]["metadata"] for r in valid),
            "verdicts_v02_derived_ctime": count(r["verdict_v02_derived"]["ctime"] for r in valid),
            "ctime_delta_ns_min": min((r["ctime_delta_ns"] for r in valid), default=None),
        }
    # H2-b's falsifier: bytes differ, size/mtime/dev/ino equal, ctime equal.
    restore = [r for r in reps if r["variant"] == "restore" and r["valid"]]
    out["h2b_counterexamples"] = sum(r["bytes_differ"] and r["equal"]["ctime_ns"] for r in restore)
    # H2-a: without ctime, the case is not visible at the observation surface.
    out["h2a_visible_without_ctime"] = sum(r["decision"]["metadata"] == "read" for r in restore)
    # H2-d: the derived v0.2 verdict against v0.1's, per condition.
    out["h2d_disagreements"] = {
        c: sum(r["verdict_v02_derived"][c] != r["verdict_v01"] for r in reps if r["valid"])
        for c in ("metadata", "ctime")
    }
    return out


# ---------------------------------------------------------------------------------------
# E-TD-3 — inode reuse with matching metadata (§H.2)
# ---------------------------------------------------------------------------------------

def e_td_3(parent, umbral_bin):
    d = tempfile.mkdtemp(dir=parent, prefix="etd3-")
    state = tempfile.mkdtemp(prefix="etd3-state-")
    try:
        p = os.path.join(d, "p")
        contents = [b"X" * LENGTH, b"Y" * LENGTH]
        write(p, contents[0])                             # step 1
        u = Umbral(umbral_bin, d, state)
        reused = []
        for attempt in range(1, BUDGET + 1):
            s1 = stat(p)
            u.run("observe")
            os.unlink(p)                                  # step 2
            write(p, contents[attempt % 2])               # step 3: different bytes, same length
            os.utime(p, ns=(s1["atime_ns"], s1["mtime_ns"]))
            s2 = stat(p)                                  # step 4
            if (s1["dev"], s1["ino"]) != (s2["dev"], s2["ino"]):
                continue
            prev = dict(s1, stable=True)
            decisions = {c: decide(prev, s2, c == "ctime") for c in ("metadata", "ctime")}
            u.run("observe")                              # step 8
            v01 = u.verdict("p")
            reused.append({
                "attempt": attempt,
                "size_equal": s1["size"] == s2["size"],
                "mtime_equal": s1["mtime_ns"] == s2["mtime_ns"],
                "ctime_equal": s1["ctime_ns"] == s2["ctime_ns"],
                "decision": decisions,                    # step 7
                "verdict_v01": v01,
                "verdict_v02_derived": {c: derived_v02(decisions[c], v01) for c in decisions},
            })
        return {
            "budget": BUDGET,
            "reuse_observed": len(reused),
            "first_reuse_attempt": reused[0]["attempt"] if reused else None,
            "skip_metadata": sum(r["decision"]["metadata"] == "skip" for r in reused),
            "skip_ctime": sum(r["decision"]["ctime"] == "skip" for r in reused),
            "ctime_equal": sum(r["ctime_equal"] for r in reused),
            "verdicts_v01": count(r["verdict_v01"] for r in reused),
            "verdicts_v02_derived_metadata": count(r["verdict_v02_derived"]["metadata"] for r in reused),
            "verdicts_v02_derived_ctime": count(r["verdict_v02_derived"]["ctime"] for r in reused),
            "outcome": ("reuse observed" if reused
                        else f"not provoked in {BUDGET} attempts"),
            "first_cases": reused[:5],
        }
    finally:
        shutil.rmtree(d, ignore_errors=True)
        shutil.rmtree(state, ignore_errors=True)


def count(values):
    out = {}
    for v in values:
        out[v] = out.get(v, 0) + 1
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--umbral", required=True)
    ap.add_argument("--label", required=True)
    ap.add_argument("dir")
    a = ap.parse_args()
    umbral_bin = os.path.abspath(a.umbral)
    started = time.time()
    reps = [e_td_2_once(a.dir, umbral_bin, v, i) for v in VARIANTS for i in range(REPETITIONS)]
    result = {
        "label": a.label,
        "kernel": platform.release(),
        "filesystem": filesystem(a.dir),
        "parameters": {"repetitions": REPETITIONS, "granularity_wait_s": GRANULARITY_WAIT_S,
                       "budget": BUDGET, "length": LENGTH},
        "h2c": h2c_exhaustive(),
        "e_td_2": summarise_e_td_2(reps),
        "e_td_3": e_td_3(a.dir, umbral_bin),
        "seconds": round(time.time() - started, 1),
    }
    json.dump(result, sys.stdout, indent=1, sort_keys=True)
    print()


if __name__ == "__main__":
    main()
