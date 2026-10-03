#!/usr/bin/env python3
"""Generate the material for EXP-AI-01 — the delegated non-UTF-8 reading check.

Deterministic. Produces a fixture under a temp root, runs the tool over it with an
isolated state directory, and writes the captured output verbatim to material.txt.

Nothing personal is written into the capture: the tool's state directory is redirected
with XDG_DATA_HOME so no home-directory path appears in the output, and the check at the
end fails loudly if one ever does.

Usage:
    python3 capture_material.py <path-to-umbral-binary> [output-dir]

Requires the binary to be built (see the experiment record). Paths are passed to the
tool as raw bytes, so the non-UTF-8 name reaches it exactly as the filesystem holds it.
"""

import hashlib
import os
import shutil
import subprocess
import sys
import time

BASE = "/tmp/umbral-check-01"
ROOT = BASE + "/subject"
DATA = BASE + "/data"

# 0xFF 0xFE is not valid UTF-8 in any position; the name is deliberately unreachable
# through a UTF-8 string literal.
NON_UTF8_REL = b"notes/weird-\xff\xfe.txt"


def build_fixture():
    shutil.rmtree(BASE, ignore_errors=True)
    os.makedirs(ROOT + "/notes", exist_ok=True)
    open(ROOT + "/readme.txt", "wb").write(b"alpha line\n")
    open(ROOT + "/notes/one.txt", "wb").write(b"beta line\n")
    open(ROOT + "/notes/two.txt", "wb").write(b"beta line\n")
    open(ROOT + "/notes/three.txt", "wb").write(b"gamma\n")
    open(os.fsdecode(ROOT.encode() + b"/" + NON_UTF8_REL), "wb").write(b"delta\n")


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    binary = os.path.abspath(sys.argv[1])
    outdir = os.path.abspath(sys.argv[2]) if len(sys.argv) > 2 else os.path.dirname(binary)
    if not os.path.exists(binary):
        sys.exit("binary not found: %s" % binary)

    build_fixture()
    env = dict(os.environ, XDG_DATA_HOME=DATA)
    lines = []

    def run(args):
        argv = [binary.encode()] + [a if isinstance(a, bytes) else a.encode() for a in args]
        p = subprocess.run(argv, capture_output=True, env=env)
        lines.append("$ umbral " + " ".join(a.decode("utf-8", "backslashreplace") for a in argv[1:]))
        lines.append(p.stdout.decode("utf-8", "backslashreplace").rstrip("\n"))
        if p.stderr:
            lines.append("--- stderr ---")
            lines.append(p.stderr.decode("utf-8", "backslashreplace").rstrip("\n"))
        lines.append("exit=%d" % p.returncode)
        lines.append("")

    # --- first observation ---
    run(["init", ROOT])
    run(["observe", ROOT])
    run(["status", ROOT])
    run(["show", ROOT, "notes/one.txt"])
    run(["show", ROOT, NON_UTF8_REL])

    # --- one mutation, then a second observation ---
    open(os.fsdecode(ROOT.encode() + b"/" + NON_UTF8_REL), "wb").write(b"delta changed\n")
    time.sleep(0.02)
    open(ROOT + "/notes/three.txt", "wb").write(b"gamma changed\n")

    run(["observe", ROOT])
    run(["changes", ROOT])

    text = "\n".join(lines).rstrip("\n") + "\n"
    os.makedirs(outdir, exist_ok=True)
    target = os.path.join(outdir, "material.txt")
    with open(target, "w") as fh:
        fh.write(text)

    digest = hashlib.sha256(open(binary, "rb").read()).hexdigest()
    print("material written to %s (%d bytes)" % (target, len(text)))
    print("umbral binary sha256: %s" % digest)
    if "/home/" in text or os.path.expanduser("~") in text:
        sys.exit("PRIVACY CHECK FAILED: a personal path appears in the material")
    print("privacy check: no personal path in the material")


if __name__ == "__main__":
    main()
