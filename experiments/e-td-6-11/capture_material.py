#!/usr/bin/env python3
"""Generate the material for E-TD-6 / E-TD-11 (README.md in this directory, specified before
the material was generated).

Deterministic apart from timestamps. Builds the fixture under /tmp/umbral-check-02, runs the
umbral binary over it with an isolated state directory, captures every output verbatim, dumps
the durable record as text, applies the six transformations of V0.2-TECHNICAL-DESIGN.md §H.3,
and writes the three messages the owner delivers.

Usage:
    python3 capture_material.py <path-to-umbral-binary>
"""

import json
import os
import random
import shutil
import sqlite3
import subprocess
import sys
import textwrap
import time

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", ".."))
OUT = os.path.join(HERE, "material")
BASE = b"/tmp/umbral-check-02"
ROOT = BASE + b"/subject"
DATA = BASE + b"/data"

NAMES = [
    b"two  spaces.txt",
    b"line\nbreak.txt",
    b"back\\slash\nnl.txt",
    b"cr\rname.txt",
    b"tab\tname.txt",
    b"bytes-\xff.txt",
    b"sep  \\ back.txt",
]

REFERENCE_KEYS = ("observation", "reference", "compared", "counterpart", "content-source")
SUBJECT_KEYS = ("path", "observation")


# ---------------------------------------------------------------------------------------
# The contract's escaping (umbral/CONTRACT.md §4), for echoing command arguments only
# ---------------------------------------------------------------------------------------

DECEPTIVE = set(range(0x80, 0xA0)) | {0x061C, 0x200E, 0x200F, 0x2028, 0x2029, 0xFEFF} \
    | set(range(0x202A, 0x202F)) | set(range(0x2066, 0x206A)) | set(range(0x200B, 0x200E))


def escape(value: bytes) -> str:
    out = []
    text = value.decode("utf-8", errors="surrogateescape")
    chars = list(text)
    for i, ch in enumerate(chars):
        cp = ord(ch)
        if 0xDC80 <= cp <= 0xDCFF:  # a byte that is not valid UTF-8
            out.append("\\x%02X" % (cp - 0xDC00))
        elif ch == "\\":
            out.append("\\\\")
        elif cp < 0x20 or cp == 0x7F:
            out.append("\\x%02X" % cp)
        elif ch == " " and (i == 0 or i == len(chars) - 1 or chars[i - 1] == " "
                            or (i + 1 < len(chars) and chars[i + 1] == " ")):
            out.append("\\x20")
        elif cp in DECEPTIVE:
            out.append("".join("\\x%02X" % b for b in ch.encode()))
        else:
            out.append(ch)
    return "".join(out)


# ---------------------------------------------------------------------------------------
# Fixture and capture
# ---------------------------------------------------------------------------------------

def j(*parts: bytes) -> bytes:
    return b"/".join(parts)


def write(rel: bytes, body: bytes):
    with open(j(ROOT, rel), "wb") as fh:
        fh.write(body)


class Session:
    def __init__(self, binary):
        self.binary = binary
        self.env = dict(os.environ, XDG_DATA_HOME=DATA.decode())
        self.blocks = []  # (command line, output text, exit code)

    def run(self, *args: bytes):
        p = subprocess.run([self.binary.encode(), *args], env=self.env, capture_output=True)
        shown = " ".join(escape(a) for a in args)
        self.blocks.append((f"$ umbral {shown}", p.stdout.decode("utf-8"), p.returncode))

    def note(self, text):
        self.blocks.append((f"# {text}", None, None))

    def render(self, transform=None):
        parts = []
        for cmd, out, code in self.blocks:
            if out is None:
                parts.append(cmd + "\n")
                continue
            body = transform(out) if transform else out
            parts.append(f"{cmd}\n{body}exit={code}\n")
        return "\n".join(parts)


def build(binary):
    shutil.rmtree(BASE, ignore_errors=True)
    os.makedirs(j(ROOT, b"locked"))
    write(b"a.txt", b"alpha\n")
    write(b"b.txt", b"bravo\n")
    write(b"c.txt", b"charlie\n")
    write(b"locked/inside.txt", b"inside\n")
    write(b"link-one", b"linked\n")
    os.link(j(ROOT, b"link-one"), j(ROOT, b"link-two"))
    for i, n in enumerate(NAMES):
        write(n, b"name %d\n" % i)

    s = Session(binary)
    s.run(b"init", ROOT)
    s.run(b"observe", ROOT)
    time.sleep(1.2)
    s.note("a.txt edited; locked/ made unreadable (mode 000)")
    write(b"a.txt", b"alpha, edited\n")
    os.chmod(j(ROOT, b"locked"), 0o000)
    s.run(b"observe", ROOT)
    time.sleep(1.2)
    s.note("locked/ readable again; c.txt renamed to d.txt; link-two deleted")
    os.chmod(j(ROOT, b"locked"), 0o755)
    os.rename(j(ROOT, b"c.txt"), j(ROOT, b"d.txt"))
    os.unlink(j(ROOT, b"link-two"))
    s.run(b"observe", ROOT)
    s.run(b"status", ROOT)
    s.run(b"changes", ROOT)
    for rel in [b"a.txt", b"b.txt", b"d.txt", b"link-one", b"locked", NAMES[1], NAMES[5]]:
        s.run(b"show", ROOT, rel)
    return s


def record():
    """Every row of the durable record, as text. Blobs in hexadecimal; paths also written in
    the contract's escaped form so that a reference can be matched by eye."""
    ws = [d for d in os.listdir(DATA + b"/umbral") if d.startswith(b"ws-")][0]
    db = sqlite3.connect((DATA + b"/umbral/" + ws + b"/observations.sqlite").decode())
    lines = []
    for table, order in [("schema_meta", "key"), ("run", "run_id"), ("observation", "run_id, path")]:
        cur = db.execute(f"SELECT * FROM {table} ORDER BY {order}")
        cols = [c[0] for c in cur.description]
        lines.append(f"TABLE {table}")
        for row in cur:
            fields = []
            for c, v in zip(cols, row):
                if isinstance(v, bytes):
                    if c in ("path", "root"):
                        fields.append(f"{c}={escape(v)}")
                    fields.append(f"{c}_hex={v.hex()}")
                else:
                    fields.append(f"{c}={'NULL' if v is None else v}")
            lines.append("  " + " | ".join(fields))
        lines.append("")
    return "\n".join(lines)


# ---------------------------------------------------------------------------------------
# The six transformations (§H.3), applied to each command's output
# ---------------------------------------------------------------------------------------

def parse_line(line):
    label, rest = line[:9].rstrip(), line[10:]
    return label, rest.split("  ")


def emit(label, items):
    return f"{label:<9} " + "  ".join(items)


def key(item):
    return item.split("=", 1)[0] if "=" in item else None


def lines_of(out):
    return out.rstrip("\n").split("\n") if out.strip() else []


def t_reorder(out):
    ls = lines_of(out)
    random.Random(2026).shuffle(ls)
    return "".join(l + "\n" for l in ls)


def without(items, keys):
    kept = []
    for it in items:
        k = key(it)
        if k in keys or (k and k.endswith("-encoding") and k[: -len("-encoding")] in keys):
            continue
        kept.append(it)
    return kept


def t_subject_removal(out):
    res = []
    for l in lines_of(out):
        label, items = parse_line(l)
        res.append(emit(label, without(items, SUBJECT_KEYS)))
    return "".join(l + "\n" for l in res)


def t_reflow(out):
    res = []
    for l in lines_of(out):
        res.extend(textwrap.wrap(l, 60, break_on_hyphens=False, drop_whitespace=False) or [""])
    return "".join(l + "\n" for l in res)


def t_split(out):
    res = []
    for l in lines_of(out):
        label, items = parse_line(l)
        res.append(label)
        res.extend("    " + it for it in items)
    return "".join(l + "\n" for l in res)


def to_json(out):
    return [{"label": parse_line(l)[0], "items": parse_line(l)[1]} for l in lines_of(out)]


def t_json(out):
    doc = to_json(out)
    back = "".join(emit(d["label"], d["items"]) + "\n" for d in doc)
    assert back == out, "the serialization round-trip changed the output"
    return json.dumps(doc, ensure_ascii=False, indent=1) + "\n"


def t_position(out):
    res = []
    for l in lines_of(out):
        label, items = parse_line(l)
        moved = [it for it in items if key(it) in REFERENCE_KEYS
                 or (key(it) or "").removesuffix("-encoding") in REFERENCE_KEYS]
        res.append(emit(label, [it for it in items if it not in moved] + moved))
    return "".join(l + "\n" for l in res)


TRANSFORMS = [
    ("1-reordering", "every output's lines permuted", t_reorder),
    ("2-subject-removal", "`path=` and `observation=` deleted from every line", t_subject_removal),
    ("3-reflow", "every line wrapped at 60 columns", t_reflow),
    ("4-delimiter-split", "every item on its own line, under its label", t_split),
    ("5-serialization", "converted to JSON per the contract (the round trip back to text was "
     "checked to be byte-identical)", t_json),
    ("6-position-change", "every reference field moved to the end of its line", t_position),
]


# ---------------------------------------------------------------------------------------
# The contract given at step 1: CONTRACT.md and the README sections that define the fields
# ---------------------------------------------------------------------------------------

def readme_sections():
    text = open(os.path.join(REPO, "umbral", "README.md"), encoding="utf-8").read()
    wanted = ["## What this is", "### The output format", "### How each value was obtained",
              "### Reading only what changed", "### What the traversal saw",
              "### What a verdict rests on"]
    out, take = [], False
    for line in text.split("\n"):
        if line.startswith("#"):
            take = any(line.startswith(w) for w in wanted)
        if take:
            out.append(line)
    return "\n".join(out)


SEND_1 = """I would like an independent reading of a captured command-line session. Please work
**only from the material below**: everything you need is in it. Do not look anything up.

**OBJECTIVE**

Say, from the captured output alone, what this tool reports about the directory it was pointed
at, over the three times it observed it.

**QUESTIONS** — answer each separately, and say "cannot be determined from the output" wherever
that is the honest answer.

1. For every line of the output, what is it about (which entry, which run, or the whole run), and
   what does it state?
2. For each file mentioned in the last observation (`run=3`): were its contents actually read
   during that observation, or was an earlier reading used? If an earlier one, which? What in the
   output tells you?
3. For each line printed by `changes`: which entries it concerns, what it concludes, and what
   that conclusion rests on.
4. Which observations were complete? For any that was not, what was not seen, and why?
5. Several names contain unusual characters. How are they written? Can you state the exact name of
   each? Is there any name you cannot reconstruct?
6. Is there anything in the output that is contradictory, ambiguous, or that you could only
   understand by guessing?

**MATERIAL — a complete captured session, verbatim**

```
{session}```
"""

SEND_2 = """Thank you. Here is the specification the output follows, and six mechanically
transformed versions of the same session. Please keep your earlier answers as they are; answer
these as a new step.

**TASK**

A. Using the session from before and the specification below: for **each result line** of
   `changes` and of every `show`, state (i) which entry or object it applies to, and (ii) which
   observation(s) — run and path — and which fields of each its evidence consists of. If a result
   rests on something that is not a field of an observation (an absence, a fact about a whole
   run), say so.
B. For each transformed version below: does any result's answer to A change, become ambiguous, or
   become impossible to determine? Name the result and the transformation. A transformation that
   changes nothing is a valid finding.
C. Does any value — in particular any name — fail to come back exactly when you read the output
   under the specification?

**SPECIFICATION**

{contract}

---

{readme}

**TRANSFORMED VERSIONS**

{transforms}
"""

SEND_3 = """Thank you. Last step. Below is the tool's stored record (its log), as text: every row
of every table. Binary values are in hexadecimal; paths are also shown in the output's escaped
form.

**TASK**

For every reference in the session's `changes` and `show` output (`observation=`, `reference=`,
`compared=`, `counterpart=`, `content-source=`): find the row it names, state the values of the
fields the result lists, and say whether they are consistent with what the result asserts. Report
any reference that names no row, or more than one, and any value that does not match.

Also: for the files whose contents were not read in a given observation, can you tell from the
record in which observation their bytes were actually read?

**RECORD**

```
{record}```
"""


def write_messages(files):
    contract = open(os.path.join(REPO, "umbral", "CONTRACT.md"), encoding="utf-8").read()
    transforms = []
    for name, what, _ in TRANSFORMS:
        transforms.append(f"### Transformation {name}: {what}\n\n```\n{files['transform-' + name + '.txt']}```\n")
    msgs = {
        "send-1.md": SEND_1.format(session=files["session.txt"]),
        "send-2.md": SEND_2.format(contract=contract, readme=readme_sections(),
                                   transforms="\n".join(transforms)),
        "send-3.md": SEND_3.format(record=files["record.txt"]),
    }
    for name, body in msgs.items():
        with open(os.path.join(HERE, name), "w", encoding="utf-8") as fh:
            fh.write(body)


def main():
    binary = os.path.abspath(sys.argv[1])
    s = build(binary)
    os.makedirs(OUT, exist_ok=True)
    files = {"session.txt": s.render(), "record.txt": record()}
    for name, _, fn in TRANSFORMS:
        files[f"transform-{name}.txt"] = s.render(fn)
    for name, body in files.items():
        with open(os.path.join(OUT, name), "w", encoding="utf-8") as fh:
            fh.write(body)
    write_messages(files)
    shutil.rmtree(BASE, ignore_errors=True)
    for name in sorted(os.listdir(OUT)):
        print(name)


if __name__ == "__main__":
    main()
