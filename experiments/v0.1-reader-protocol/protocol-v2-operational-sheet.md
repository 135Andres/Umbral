UMBRAL v0.1 — READER PROTOCOL v2.1 — OPERATIONAL INSTRUCTIONS
=============================================================

Revision 2.1, 2026-09-12. Changes from 2.0 are recorded at the end.

What this is
------------

You will use a small command-line tool on a directory of your own, and then answer eight
questions about what you saw. It is a test of the tool's output, not of you.

Before you start
----------------

Do not read the project's source code, its documentation, or its experiment records. Use only
the tool and the output it produces.

If you have already seen any of those, say so. That matters for how the result is recorded,
and saying so does not invalidate anything.

If something is unclear, ask. The exchange will be recorded. Questions about what the output
*means* will not be answered while the test is running.

Your workspace
--------------

Everything you do happens in this directory, which is yours:

    /tmp/umbral-reader-v2/reader

Create it if it does not exist. Put the directory you will observe inside it, and write your
transcript inside it. Do not read anything under /tmp/umbral-reader-v2 outside your own
workspace and the tool binary named below.

1. The tool
-----------

    /tmp/umbral-reader-v2/bin/umbral

It is already built. You do not need to compile anything.

2. Your directory
-----------------

Prepare a directory that looks like a working directory in use. It must contain:

  - several ordinary files, at least two of which have byte-identical contents;
  - at least one subdirectory;
  - at least one symbolic link, and at least one symbolic link pointing to a path that does
    not exist;
  - at least one file you cannot read (permissions 000);
  - at least one file whose name is not valid UTF-8, if your system permits one.

Note the path of that directory. It is written `<dir>` below.

The tool does not write anything inside it.

3. Where the tool keeps its records
-----------------------------------

Outside your directory, under ~/.local/share/umbral/.

To keep that isolated, prefix every command with:

    XDG_DATA_HOME=/tmp/umbral-reader-v2/data

4. Commands
-----------

Run them in this order. Record the exit status of each one, and the complete output -- stdout
and stderr -- copied exactly as it appeared.

   1.  <binary> init <dir>

   2.  <binary> observe <dir>

   3.  Make three or four changes of your own choosing inside <dir>: create, edit, rename,
       delete. Nobody will tell you which.

   4.  <binary> observe <dir>

   5.  <binary> status <dir>

   6.  <binary> changes <dir>

   7.  <binary> show <dir> <path>   -- run this once for EVERY entry you can name inside
                                      <dir>, using that entry's path, including entries
                                      inside subdirectories and entries that are not regular
                                      files. Do not skip any, and do not choose a subset.

5. Questions
------------

Answer them from the output alone.

  Q1  How many entries are known in that directory?

  Q2  Which of them are verified by content, and which are not?

  Q3  When was it last observed, and was that observation complete?

  Q4  What changed since the previous observation, and how does the tool know?

  Q5  Is anything ambiguous? Why?

  Q6  Name one thing the tool does NOT know.

  Q7  Name one thing the tool asserts and one thing it refuses to assert.

  Q8  Is there anywhere in the output where you cannot tell whether something was observed
      or computed?

6. What to send back
--------------------

  - the complete raw output of every command, exactly as it appeared;
  - your answers to Q1-Q8 exactly as you wrote them, including any you are unsure about or
    believe are wrong;
  - anything that confused you, and any command that failed or behaved unexpectedly.

Do not summarise, and do not tidy the output.

Changes from revision 2.0
-------------------------

**P-V01-1.** Step 7 in revision 2.0 asked for a single `show`. Q2 asks *which* entries are
verified by content, and `status` reports counts, not names -- so revision 2.0 could not answer
its own question, and the reader of the second run had to invent nine extra `show` probes to
answer it. Step 7 now prescribes one `show` per entry. This is a repair of the protocol. The
tool's surface is unchanged: no listing command was added to compensate for a defect in the
test.

**P-V01-2.** Revision 2.0 forbade the reader to read anything under `/tmp/umbral-reader-v2`
except the tool binary and the reader's own directory, and then required the transcript to be
written to `/tmp/umbral-reader-v2/transcript.txt` -- outside that directory. The reader
resolved the contradiction correctly, but the instruction was wrong. This revision gives the
reader an explicit workspace (`/tmp/umbral-reader-v2/reader`) that contains both the directory
under observation and the transcript, so no instruction points outside what it is allowed to
touch.
