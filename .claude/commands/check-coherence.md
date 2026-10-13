Audit the documentation for coherence: `README.md`, `CONTRIBUTING.md`, `STATE.md`, everything under `docs/`,
`data/README.md` and `.claude/CLAUDE.md`, against each other and against the tree as it is. Adopted from the
flagship's own command.

Run the numbered checks in order. Each returns exactly one of: **pass**, **finding** (quote the
two clauses side by side, each with file and line), or **ask the maintainer** (when the tree
cannot say which clause is right).

## 1. Contradictions between clauses that were each right when written

A number, a name, a rule or a status stated differently in two places: a phase marked started in one file and
not in another, a task in both *Next* and *Done*, a step number cited by a document after the steps were
renumbered, a job name the ruleset requires by its exact name spelled differently in a document.

## 2. Orphaned claims about the tree

A file, directory, crate, notebook, dataset, script, workflow, job, test or command a document names that does
not exist, or exists under another name. Verify by listing, not by memory.

## 3. Facts stated in two places

A fact lives in one place and the others point to it (`FILE § Section`). A second copy is a finding, whether or
not it still agrees; propose the reference that replaces it. The one deliberate exception is the Long Arc block
in `docs/VISION.md`, mirrored word for word across the organisation.

## 4. Scope drift

A document describing what the lab was going to do rather than what it does: a plan step that is done but still
says "will", a result reported in the journal that STATE.md has not caught up with, a hypothesis marked untested
that the journal has tested.

## 5. Stale "today" sections

`STATE.md` § Where We Are, `docs/VISION.md` § What Exists Today, `.claude/CLAUDE.md` § CI today and any
paragraph shaped like a handoff, against the workflows and the tree as they are now.

## 6. References lychee cannot judge

A section referenced by name (`FILE § Section Name`) whose heading has since been renamed, or
a claim of the form "see X" where X no longer says that.

## Rules of the audit

- Report only what you verified in the tree. A finding names files and lines.
- Do not restate a rule's content in the report; cite where it lives.
- Be willing to conclude that the documents are coherent. A coherence audit that manufactures
  findings to look thorough is worse than useless, because it trains the reader to ignore the
  next one.
- Findings are reported, not fixed, unless the maintainer asked for fixes in the same breath. At night, a
  finding becomes a housekeeping task in STATE.md.
