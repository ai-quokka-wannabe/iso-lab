Review all changed or specified files for the lab's style rules.

Check against these rules (from `.claude/CLAUDE.md` § Writing, `CONTRIBUTING.md` § Coding Standards and the
lint configuration):

## Rust

- `cargo fmt --check` and `cargo clippy --all-targets -- --deny warnings` are clean
- Names follow the Rust API Guidelines: `snake_case` functions and modules, `PascalCase` types,
  `SCREAMING_SNAKE_CASE` constants
- Deterministic given a seed: no iteration over unordered containers, no wall clock, no hidden state in
  anything a score depends on
- A new source file starts with the licence header naming *The AiQuokkaWannabe Authors*

## Notebooks

- The first cell carries the licence header
- Dependencies are pinned in `notebooks/requirements.txt`
- Outputs are small; no embedded datasets
- Nothing a score depends on lives only in the notebook

## Figures

- Graphs, plots and diagrams are SVG, never PNG or JPEG
- Each is produced by a script or notebook in the repository, and regenerating it changes nothing

## YAML (GitHub Actions)

- 4-space structure indentation
- 2-space continuation from list items
- Blank lines between top-level keys and between jobs
- No inline comments
- GitHub-owned actions only, pinned by full commit SHA

## JSON

- 4-space indentation

## Markdown

- ATX-style headings
- Dash lists (`-`), numeric ordered lists (`1.`)
- Fenced code blocks with language specified
- Lines at most 170 characters

Report every violation with file path, line number, and what's wrong. If no files are specified via $ARGUMENTS, check all files modified since the last commit.
