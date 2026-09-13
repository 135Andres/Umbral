#!/usr/bin/env python3
"""ctime probe — does ctime move when a change is otherwise hidden?

Reproduces the measurements recorded in EXP-CTIME.md. Read-only with respect to the
repository: every probe runs in a throwaway temp directory under the filesystem being
tested, and the directory is removed afterwards.

Usage:
    python3 ctime_probe.py [dir ...]

With no arguments it probes /tmp and the current user's home directory, which on the
machine this was written for are tmpfs and btrfs respectively.
"""

import json
import os
import shutil
import subprocess
import sys
import tempfile
import time

WRITES = 300


def fields(path):
    st = os.lstat(path)
    return {
        "ino": st.st_ino,
        "size": st.st_size,
        "mtime_ns": st.st_mtime_ns,
        "ctime_ns": st.st_ctime_ns,
    }


def probe(parent, label):
    d = tempfile.mkdtemp(dir=parent, prefix="ctime-")
    out = {"fs": label, "dir": d}
    try:
        f = os.path.join(d, "a.txt")
        with open(f, "w") as fh:
            fh.write("hello world\n")  # 12 bytes
        s0 = fields(f)
        time.sleep(0.05)

        # 1. Same size, mtime restored exactly: the case v0.1 documents as its
        #    stat guard's blind spot.
        st = os.stat(f)
        with open(f, "w") as fh:
            fh.write("HELLO WORLD\n")  # same 12 bytes
        os.utime(f, ns=(st.st_atime_ns, st.st_mtime_ns))
        s1 = fields(f)
        out["same_size_mtime_restored"] = {
            "size_equal": s0["size"] == s1["size"],
            "mtime_equal": s0["mtime_ns"] == s1["mtime_ns"],
            "ino_equal": s0["ino"] == s1["ino"],
            "ctime_equal": s0["ctime_ns"] == s1["ctime_ns"],
            "ctime_delta_ns": s1["ctime_ns"] - s0["ctime_ns"],
        }

        # 2. chmod only — metadata change, content untouched.
        a = fields(f)
        os.chmod(f, 0o600)
        b = fields(f)
        out["chmod_only"] = {
            "mtime_equal": a["mtime_ns"] == b["mtime_ns"],
            "ctime_equal": a["ctime_ns"] == b["ctime_ns"],
        }

        # 3. Hard link creation touches the target's ctime.
        a = fields(f)
        os.link(f, os.path.join(d, "link.txt"))
        b = fields(f)
        out["hardlink_created"] = {
            "mtime_equal": a["mtime_ns"] == b["mtime_ns"],
            "ctime_equal": a["ctime_ns"] == b["ctime_ns"],
            "nlink": os.lstat(f).st_nlink,
        }

        # 4. Atomic replacement: new inode, mtime and size can match.
        a = fields(f)
        tmp = os.path.join(d, ".tmp")
        with open(tmp, "w") as fh:
            fh.write("HELLO WORLD\n")
        os.utime(tmp, ns=(st.st_atime_ns, st.st_mtime_ns))
        os.replace(tmp, f)
        b = fields(f)
        out["atomic_replace"] = {
            "ino_equal": a["ino"] == b["ino"],
            "size_equal": a["size"] == b["size"],
            "mtime_equal": a["mtime_ns"] == b["mtime_ns"],
            "ctime_equal": a["ctime_ns"] == b["ctime_ns"],
        }

        # 5. Granularity: consecutive writes, count ctime collisions.
        g = os.path.join(d, "g.txt")
        with open(g, "w") as fh:
            fh.write("0")
        prev = os.lstat(g).st_ctime_ns
        deltas = []
        collisions = 0
        for i in range(WRITES):
            with open(g, "w") as fh:
                fh.write(str(i))
            cur = os.lstat(g).st_ctime_ns
            if cur == prev:
                collisions += 1
            deltas.append(cur - prev)
            prev = cur
        out["granularity"] = {
            "writes": WRITES,
            "ctime_unchanged_after_write": collisions,
            "min_delta_ns": min(deltas),
            "max_delta_ns": max(deltas),
        }

        # 6. Is ctime settable from userland? utime sets atime/mtime only.
        a = fields(f)
        os.utime(f, ns=(st.st_atime_ns, st.st_mtime_ns))
        b = fields(f)
        out["utime_can_forge_ctime"] = not (a["ctime_ns"] != b["ctime_ns"])

        # 7. touch -d moves mtime into the past; ctime follows the clock.
        r = subprocess.run(
            ["stat", "-c", "mtime=%y ctime=%z", f], capture_output=True, text=True
        )
        subprocess.run(["touch", "-d", "2001-01-01 00:00:00", f], check=False)
        r2 = subprocess.run(
            ["stat", "-c", "mtime=%y ctime=%z", f], capture_output=True, text=True
        )
        out["touch_backdated"] = {"before": r.stdout.strip(), "after": r2.stdout.strip()}
    finally:
        shutil.rmtree(d, ignore_errors=True)
    return out


def main():
    dirs = sys.argv[1:] or ["/tmp", os.path.expanduser("~")]
    print(json.dumps([probe(d, d) for d in dirs], indent=2))


if __name__ == "__main__":
    main()
