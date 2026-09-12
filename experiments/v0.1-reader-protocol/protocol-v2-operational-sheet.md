UMBRAL v0.1 — READER PROTOCOL v2 — OPERATIONAL INSTRUCTIONS
===========================================================

What this is
------------

You will use a small command-line tool on a directory of your own, and then answer
eight questions about what you saw. It is a test of the tool's output, not of you.

Before you start
----------------

Do not read the project's source code, its documentation, or its experiment records.
Use only the tool and the output it produces.

If you have already seen any of those, say so. That matters for how the result is
recorded, and saying so does not invalidate anything.

If something is unclear, ask. The exchange will be recorded. Questions about what the
output *means* will not be answered while the test is running.

1. The tool
-----------

    /tmp/umbral-reader-v2/bin/umbral

It is already built. You do not need to compile anything.

2. Your directory
-----------------

Prepare a directory that looks like a working directory in use. It must contain:

  - several ordinary files, at least two of which have byte-identical contents;
  - at least one subdirectory;
  - at least one symbolic link, and at least one symbolic link pointing to a path
    that does not exist;
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

Run them in this order. Record the exit status of each one, and the complete output —
stdout and stderr — copied exactly as it appeared.

   1.  <binary> init <dir>

   2.  <binary> observe <dir>

   3.  Make three or four changes of your own choosing inside <dir>: create, edit,
       rename, delete. Nobody will tell you which.

   4.  <binary> observe <dir>

   5.  <binary> status <dir>

   6.  <binary> changes <dir>

   7.  <binary> show <dir> <a path inside <dir> that you choose>

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

  Q8  Is there anywhere in the output where you cannot tell whether something was
      observed or computed?

6. What to send back
--------------------

  - the complete raw output of every command, exactly as it appeared;
  - your answers to Q1-Q8 exactly as you wrote them, including any you are unsure
    about or believe are wrong;
  - anything that confused you, and any command that failed or behaved unexpectedly.

Do not summarise, and do not tidy the output.
