# Development Environment Setup

How to check, run and extend the lab from nothing, on Windows or Linux, exactly as CI does. The night shift's
cloud machine is set up as [NIGHT_SHIFT.md](NIGHT_SHIFT.md) describes; this page is for a person at their own
desk.

---

**The short version**, for someone who has done this before:

```text
git clone --recurse-submodules https://github.com/ai-quokka-wannabe/iso-lab.git
cd iso-lab
npm ci
npx markdownlint-cli2 "**/*.md"
```

That needs Git and Node.js, nothing else, until the lab has Rust or notebooks to run. Everything below is the
long version.

---

## Prerequisites

| Tool | Version | Where to get it |
|------|---------|-----------------|
| Git | any recent | <https://git-scm.com/downloads> |
| Node.js | an LTS release, only for the markdown linter (`npm ci`) | <https://nodejs.org/> |
| lychee | v0.24.2, the version CI pins, for the link check | <https://github.com/lycheeverse/lychee/releases> |
| rustup | any recent; the toolchain will be pinned by `rust-toolchain.toml` once the Cargo workspace exists, and rustup installs exactly that on first use | <https://rustup.rs/> |
| Python | 3.10 or newer, with Jupyter, only for notebooks | <https://www.python.org/downloads/> |

The world's repositories are submodules under `external/`. Clone with `--recurse-submodules`, or initialise
them afterwards with `git submodule update --init --recursive`.

---

## Checking, exactly as CI does

CI runs one job, *Quick Checks*, behind the `CI Success` gate. At home:

```text
npm ci --ignore-scripts
npx markdownlint-cli2 "**/*.md"
lychee --offline --include-fragments --no-progress --root-dir "$(pwd)" --exclude-path node_modules --exclude-path external "./**/*.md"
```

CI also refuses any file with a carriage return: line endings in the repository are LF, and `.gitattributes`
lets Git convert them for your working copy. The external links are checked weekly, on a schedule, by
`.github/workflows/links.yml`.

When the lab has Rust, its CI job runs `cargo fmt --check`, `cargo clippy --all-targets -- --deny warnings`
and `cargo test --locked`; run the same three before pushing.

---

## Notebooks

Notebooks live under `notebooks/`, with their dependencies pinned in `notebooks/requirements.txt`:

```text
python -m venv .venv
.venv/bin/pip install -r notebooks/requirements.txt        (Windows: .venv\Scripts\pip ...)
.venv/bin/jupyter lab
```

Before committing one, check that it runs top to bottom from a fresh kernel:

```text
.venv/bin/jupyter nbconvert --to notebook --execute --inplace notebooks/<notebook>.ipynb
```

---

## The World Under `external/`

The Grid and Master Control are built from the commits `external/` pins, each by its own guide:

- Master Control: [its DEV_ENV_SETUP.md](https://github.com/ai-quokka-wannabe/master-control/blob/main/docs/DEV_ENV_SETUP.md)
- The Grid: [its DEV_ENV_SETUP.md](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/docs/DEV_ENV_SETUP.md),
  and [RUNNING_THE_GRID.md](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/docs/RUNNING_THE_GRID.md) to
  run a world with a creature in it

Each carries the Link it builds with; `external/link` is here for reading the wire's specification. On a
machine without a GPU, the Grid renders the senses through Mesa's software Vulkan, lavapipe
(`mesa-vulkan-drivers` on Ubuntu).

---

## Troubleshooting

### `external/` is empty

The submodules were not initialised: `git submodule update --init --recursive`.

### The linter reports files under `external/` or `node_modules/`

Run it from the repository root, where `.markdownlint-cli2.jsonc` tells it to skip them.

### CI says a file contains a carriage return

A tool wrote CRLF into a file Git stores as LF, or a stray `\r` sits inside a line. Re-save the file with LF
line endings.
