# iso-lab - working rules for the agents

You are working in **iso-lab**, the open research lab of the ai-quokka-wannabe organisation. Its goal: a
creature on the Grid whose behaviour cannot be told apart from a real *Caenorhabditis elegans* by the
measures biologists use, reached one honest step at a time. The repository is public and people watch it;
write every commit, pull request and journal entry for them.

The worm is the first AI brain the lab develops, not the last. When it is done, the next animal up the ladder
gets its own benchmark and its own brain. Build the worm's modules as parts a bigger brain could reuse, but
judge them only on the worm.

These rules bind every run. When a rule and a task conflict, the rule wins: stop, write the conflict down in
STATE.md, and leave the task for the maintainer.

## Each run, in order

1. **Start from `main`.** Fetch, check out `main`, pull. Main is always the latest merged state.
2. **Read** [STATE.md](../STATE.md), [docs/PLAN.md](../docs/PLAN.md) and the newest entry in
   [docs/journal/](../docs/journal/).
3. **Pick one bounded task**: the top of STATE.md's *Next* list, unless it is blocked; then the next one.
   It must be about forty-five minutes of work. A bigger task is split, and the split written into STATE.md,
   before you start.
4. **Branch** `claude/<short-name>` from `main`.
5. **Do the work.** For anything long - a build, a download, an experiment - run it in the background
   (`nohup ... &`) and poll it; foreground commands time out. Record every experiment's exact command, seed,
   inputs and versions.
6. **Review your own diff as a sceptic before anything leaves the machine.** Is every claim supported by a
   cited source or a run in this pull request? Is any number reported without its command? Is a negative
   result hidden or softened? Does it break a rule below? Fix what you find.
7. **Check locally** what CI will check: `npm ci && npx markdownlint-cli2 "**/*.md"`, the offline link
   check, and the tests of any code you touched.
8. **Write it down.** Update STATE.md (move the task to *Done* with a one-line result, add what the work
   showed should come next, list blockers). Add to tonight's journal entry,
   `docs/journal/YYYY-MM-DD.md` (UTC date; append if it exists).
9. **Commit, push, open a pull request.** In the cloud the GitHub CLI's `gh pr` commands fail (GraphQL is
   blocked); use the REST API through `gh api`, for example
   `gh api -X POST repos/ai-quokka-wannabe/iso-lab/pulls -f title=... -f head=claude/<name> -f base=main -f body=...`.
10. **Wait for the gates**: the `CI Success` check and CodeQL. Read any Copilot review comments
    (`gh api repos/ai-quokka-wannabe/iso-lab/pulls/<n>/comments` and `.../reviews`) and act on the ones that
    are right; say in a comment why you set any aside.
11. **Merge** when every gate is green:
    `gh api -X PUT repos/ai-quokka-wannabe/iso-lab/pulls/<n>/merge -f merge_method=merge`. The branch is
    deleted on merge.
12. **If a gate stays red**, try at most three fixes. Then leave the pull request open, record it under
    *Blocked* in STATE.md with what you tried, and stop. Never leave `main` red, and never weaken a check
    to make it pass.
13. **Next task, or stop.** One task per branch and pull request, landed or honestly parked. Then go back to
    step 1 for the next task, as long as the prompt that started the session allows it - a night session
    works until the hour its prompt names, or until nothing unblocked is left on the *Next* list.

## The rules

- **No cheating.** A creature's brain gets only what its body senses through the Program interface - the
  same senses, at the same resolution, with the same delays. Ground truth from the world (positions, the
  Disk, the logs) is for evaluation tools only and never reaches a brain, not even "temporarily for
  debugging". A shortcut no real animal has is not a result.
- **Realism over speed.** Never tune the world, the body or the senses to make a brain look better. The
  benchmark is defined and frozen *before* a brain is scored against it; any change to it needs a written
  justification in the journal, and earlier scores are re-run under the new definition, not compared across
  it.
- **Honesty.** Never invent data, results, quotations or references. Every factual claim about biology or
  prior work cites a public source with a link; say how sure you are. A failed experiment is written up as
  carefully as a successful one. If you are unsure whether something is true, say so in the text.
- **Pre-register and ablate.** Before running a change to a brain, write in the journal the outcomes that
  would refute it. Give every module and modulator a switch that pins it at baseline, and attribute an
  improvement by switching each off in turn - never by assumption.
- **Public sources only.** Cite published papers, public datasets and public documentation. Never refer to
  unpublished or private material.
- **Licences.** Before using a dataset, model or piece of code, find its licence and record it in
  [data/README.md](../data/README.md) (datasets) or the pull request (code). Use only what may be
  redistributed alongside GPL-3.0 code, or keep it an external download fetched by script with a checksum.
  Quote papers in short excerpts only.
- **This repository only.** Push only here. When the work needs a change in tron-grid-lite, master-control,
  link or rc-worm - a new sense, a world fix - write it under *Needs from other repositories* in STATE.md,
  with the evidence; the maintainer carries it over.
- **The world as pinned.** The world lives in `external/` as submodules pinned by commit: tron-grid-lite (the
  Grid) and master-control (the world server), each carrying the Link it builds with; `external/link` is a
  copy for reading the wire's specification, never what gets built. Fetch them with
  `git submodule update --init --recursive`. A bump of `external/` - Dependabot proposes them weekly - changes
  the world: before merging one, re-run the arena's reference measurements and say in the journal what
  moved, and never compare scores across two worlds without saying so.
- **The lab's own Programs.** The brains the lab builds live in this repository. rc-worm is the example of a
  Program's shape, not a dependency.
- **Rust first.** Anything another run builds on - the feature extraction, the benchmark, the Programs, the
  tools - is written in Rust, like Master Control and Link: one Cargo workspace at the repository root with
  its crates under `crates/`, the toolchain pinned to an exact version in `rust-toolchain.toml` (as
  master-control does), `cargo fmt` clean, `cargo clippy --all-targets -- --deny warnings` clean, and tested.
- **Notebooks for thinking.** Python in Jupyter notebooks, under `notebooks/`, is welcome for numerics and
  mathematics, for brainstorming a hypothesis, and for quick rough calculations. Pin a notebook's
  dependencies in `notebooks/requirements.txt`; it must run top to bottom from a fresh kernel
  (`jupyter nbconvert --to notebook --execute --inplace <notebook>`); commit it with its outputs, kept small.
  A notebook explores, it does not decide: nothing the benchmark or a score depends on lives only in a
  notebook - once a calculation matters, it moves into tested Rust.
- **Budget.** The cloud machine has about 4 CPUs, 16 GB of memory and 30 GB of disk, and no GPU. Download
  only what the current task needs; delete raw data you no longer need before the run ends. Never start a
  loop that cannot finish within the run.
- **No secrets.** Nothing that looks like a key, token or password goes into a file, a commit or a log.

## Writing

- British English throughout (*behaviour*, *licence* as a noun, *analyse*).
- The organisation's vocabulary: *the Grid*, *Programs* (creatures' brains), *Users* (humans), *Master
  Control* (capitalised, never abbreviated), *Link* (the library; `link` is its repository), *Disk* (a
  recording).
- Markdown must pass markdownlint with this repository's configuration (line length 170, dashes for lists).
- Commit messages: conventional commits (`docs: ...`, `feat: ...`, `test: ...`, `fix: ...`), a body that
  says what and why, and the trailer `Co-Authored-By: Claude <noreply@anthropic.com>`.
- A new source file starts with the licence header naming *The AiQuokkaWannabe Authors*, like every file in
  the organisation; in a notebook, the first cell carries it.
- Code you add gets a CI job, and that job joins the `needs` of `CI Success` in both workflows, so the gate
  covers it. Notebooks are exempt; the Rust they lead to is not. Workflows may use only GitHub-owned actions,
  pinned by full commit SHA - the repository refuses any other - so install toolchains in a `run` step
  (`rustup` reads `rust-toolchain.toml`). When you add the Cargo workspace or `notebooks/requirements.txt`,
  add the matching ecosystem to [.github/dependabot.yml](../.github/dependabot.yml).

## Where things are

- [STATE.md](../STATE.md) - the shared memory between runs. Keep it short and current.
- [docs/PLAN.md](../docs/PLAN.md) - the phases; change it only with a reason written in the journal.
- [docs/hypotheses/](../docs/hypotheses/) - ideas about the brain, each testable.
- [docs/journal/](../docs/journal/) - one entry per night; past entries are not rewritten (corrections are
  added with a note).
- [data/README.md](../data/README.md) - the dataset registry.
- [docs/NIGHT_SHIFT.md](../docs/NIGHT_SHIFT.md) - the routine that starts each run.
