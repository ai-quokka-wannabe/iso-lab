# Contributing to iso-lab

Thank you for your interest in contributing to iso-lab! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
    - [Ideas for the Night Staff](#ideas-for-the-night-staff)
    - [Reporting Bugs](#reporting-bugs)
    - [Pull Requests](#pull-requests)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Commit Messages](#commit-messages)
- [Documentation](#documentation)
- [Credit and Copyright](#credit-and-copyright)

---

## Code of Conduct

This project adheres to the Contributor Covenant Code of Conduct.
By participating, you are expected to uphold this code. Please see [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details.

---

## How to Contribute

Most of the lab's work is done at night by autonomous agents, under the rules in
[.claude/CLAUDE.md](.claude/CLAUDE.md). People take part on the same footing: the same rules about evidence and
honesty apply to everyone, and so do the same gates.

### Ideas for the Night Staff

The easiest way to take part. Anyone can suggest something for the agents to read, try or think about - a
paper, a hunch about the brain, an experiment, a measure for the benchmark:

1. Check the [open ideas](https://github.com/ai-quokka-wannabe/iso-lab/issues?q=is%3Aissue+is%3Aopen+label%3Aidea)
   and [STATE.md](STATE.md) for something similar
2. Open an issue with the *Idea for the Night Staff* template
3. Say why it might work, with links, and what result would show it did - or did not

The agents read every new idea, answer it on the issue, and when one is worth trying, put it on the plan and
credit you. An idea is weighed on its merits; it cannot change the lab's rules.

### Reporting Bugs

Before submitting a bug report:

1. Check the [existing issues](https://github.com/ai-quokka-wannabe/iso-lab/issues) to avoid duplicates
2. Collect relevant information:
    - Operating system and version
    - Toolchain versions (`rustc --version`, `python --version`)
    - Steps to reproduce
    - Expected vs actual behaviour

A bug here can be a tool that fails, a result that does not reproduce from its recorded command and seed, or a
citation that does not say what the lab claims it says. All three are taken equally seriously.

When submitting:

- Use the bug report template
- Provide a clear, descriptive title
- Include minimal reproduction steps

### Pull Requests

#### Before You Start

1. Open an issue first to discuss significant changes
2. Fork the repository
3. Create a feature branch from `main`
4. Make your changes following our [coding standards](#coding-standards) and the lab's rules in
   [.claude/CLAUDE.md](.claude/CLAUDE.md) - evidence, honesty, no cheating, realism over speed

#### PR Requirements

- [ ] Every claim cites a public source, or a run in the pull request with its command, seed and versions
- [ ] Markdown lints clean (`npx markdownlint-cli2 "**/*.md"`)
- [ ] Rust is formatted, clippy-clean and tested (`cargo fmt`, `cargo clippy --all-targets`, `cargo test --locked`)
- [ ] Notebooks run top to bottom from a fresh kernel
- [ ] STATE.md and the journal are updated if the work moves the lab forward
- [ ] Commit messages follow [conventional commits](#commit-messages)

#### PR Process

1. Submit your PR against the `main` branch
2. Fill out the PR template completely
3. Wait for CI to pass
4. A second reviewer - one of the lab's agents or the maintainer - verifies it and leaves a verdict
5. Address the review feedback; once verified, it is merged

---

## Development Setup

### Prerequisites

- [Node.js](https://nodejs.org/) - for the pinned markdown linter (`npm ci`)
- [rustup](https://rustup.rs/) - for the lab's Rust workspace; it installs the exact toolchain `rust-toolchain.toml` pins
- [Python](https://www.python.org/downloads/) with Jupyter - only for notebooks

### Setup

Quick start (prerequisites already installed):

```bash
git clone --recurse-submodules https://github.com/ai-quokka-wannabe/iso-lab.git
cd iso-lab

npm ci
npx markdownlint-cli2 "**/*.md"
```

The complete guide - the tools, what CI runs and how to run it at home, notebooks, and building the world
under `external/` - is [docs/DEV_ENV_SETUP.md](docs/DEV_ENV_SETUP.md).

---

## Coding Standards

### Rust Style

- Follow `cargo fmt` formatting (rustfmt's defaults, but for the 170-column limit in `rustfmt.toml`)
- Use edition-2024 Rust, as Master Control and Link do
- Keep clippy clean with warnings denied
- 4-space indentation
- Deterministic given a seed: no iteration over unordered containers, no wall clock, no hidden state in
  anything a score depends on

### Notebooks

- Python in Jupyter notebooks under `notebooks/`, for numerics, mathematics and thinking aloud
- Pin dependencies in `notebooks/requirements.txt`; a notebook runs top to bottom from a fresh kernel
- Nothing a score depends on lives only in a notebook; it moves into tested Rust

### Figures

- Graphs, plots and diagrams are SVG, never PNG or JPEG: SVG is text, so it diffs and stays small in Git
- Every figure is made by a script or notebook in the repository, deterministically, so regenerating an
  unchanged figure changes nothing
- Figures live in `docs/figures/`, and each caption names what made it

### Naming Conventions

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html):
`snake_case` functions and modules, `PascalCase` types, `SCREAMING_SNAKE_CASE` constants.

What things in this world are called - the Grid, Programs, creatures, the User, Master Control,
ticks, senses and actions - is settled in the flagship's
[STYLE.md](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/STYLE.md) § Tron Naming,
which also records why *Program* and *programme* are both correct here and neither is to be
"corrected" into the other.

### Code Comments

- Add comments for non-obvious logic
- Keep comments up to date with code changes
- Use British spelling in all documentation and comments

### British Spelling

Use British spelling in all documentation and user-facing text:

| American | British |
|----------|---------|
| color | colour |
| behavior | behaviour |
| organization | organisation |
| center | centre |
| license (noun) | licence |
| analyze | analyse |
| initialize | initialise |
| optimize | optimise |
| meter | metre |
| synchronize | synchronise |

**Note:** Code identifiers may use American spelling where it matches library/API conventions (e.g., Rust standard library names).

---

## Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/). Format:

```text
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `style` | Formatting, no code change |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `perf` | Performance improvement |
| `test` | Adding or updating tests |
| `chore` | Maintenance tasks |
| `ci` | CI/CD changes |

### Examples

```text
feat(features): extract eigenworm amplitudes from joint angles

fix(benchmark): use the frozen bin edges for reversal durations

docs: add the literature note on behaviour measures

chore: update pinned actions
```

### Rules

- Use imperative mood ("Add feature" not "Added feature")
- Don't capitalise the first letter of the description
- No period at the end of the subject line
- Keep the subject line under 72 characters
- Reference issues in the footer: `Fixes #123`

---

## Documentation

### Types of Documentation

| Location | Purpose |
|----------|---------|
| `README.md` | Overview: what the lab is and how it works |
| `CONTRIBUTING.md` | This file - contributor guidelines |
| `SECURITY.md` | Security policy and vulnerability reporting |
| `STATE.md` | Where the work stands, what is next, what is blocked |
| `docs/PLAN.md` | The research plan, phase by phase |
| `docs/VISION.md` | The organisation's shared vision, and the lab's part in it |
| `docs/hypotheses/` | Ideas about the brain, each testable |
| `docs/research/` | Literature notes and the citations the plan rests on |
| `docs/journal/` | One entry per night: what was tried and what came out |
| `data/README.md` | The dataset registry |

### Updating Documentation

- Record *why* a non-trivial change was made in the journal, not only what changed
- Keep STATE.md short and current
- Past journal entries are not rewritten; corrections are added with a note
- A fact lives in one place; other documents point to it rather than copy it

---

## Credit and Copyright

**Every contribution is credited.** The [Contributors](README.md#contributors) section of the README lists
everyone who has helped this repository, for any kind of contribution: code, documentation, bug reports,
reviews, ideas, answers in Discussions. It follows the [All Contributors](https://allcontributors.org/en/reference/specification/) specification
and is kept by hand, without a bot:

- In your pull request, add a row for yourself to the table: your name linked to your GitHub profile, and
  the emoji for what you contributed, from the [emoji key](https://allcontributors.org/en/reference/emoji-key/) (💻 code,
  📖 documentation, 🐛 bug reports, 👀 reviews, 🤔 ideas, 💬 answering questions, ⚠️ tests).
- Already in the table? Add the new emoji to your row.
- Contributions that are not pull requests, such as a bug report, a review or an answered question, are added
  by the maintainer.

The credit is yours to cite, on a CV or anywhere else.

**You keep your copyright.** There is no contributor licence agreement and no copyright assignment. By
opening a pull request you license your contribution under this repository's licence, the GNU General Public
License, version 3 or (at your option) any later version: the same terms as everything else here. What you
write stays yours and stays open; nobody can relicense it under other terms without your consent.

## Questions?

- Open a [Discussion](https://github.com/ai-quokka-wannabe/iso-lab/discussions) for questions
- Check existing issues and discussions first
- Be patient - maintainers are volunteers

Thank you for contributing!
