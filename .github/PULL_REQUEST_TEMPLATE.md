## Description

Brief description of the changes in this PR, and the task or idea they come from.

## Related Issue

Fixes #(issue number)

## Type of Change

- [ ] Literature or research note
- [ ] Benchmark or dataset change
- [ ] Experiment (results in the journal)
- [ ] Brain (a Program or one of its modules)
- [ ] Tool or notebook
- [ ] Housekeeping (tidying, documents brought in step, updates)
- [ ] CI/CD changes

## Evidence

- [ ] Every claim cites a public source, or a run in this PR with its command, seed and versions
- [ ] Negative results are reported as plainly as positive ones
- [ ] Outcomes that would refute an experiment were written down before it ran

## Checks

- [ ] Markdown lints clean (`npx markdownlint-cli2 "**/*.md"`) and internal links resolve (lychee, offline)
- [ ] Rust is formatted, clippy-clean and tested (`cargo fmt --check`, `cargo clippy --all-targets -- --deny warnings`, `cargo test --locked`)
- [ ] Notebooks run top to bottom from a fresh kernel
- [ ] Figures are SVG, made by a script or notebook in the repository
- [ ] British spelling used in comments and documentation
- [ ] STATE.md and the journal are updated

## Verification

A second agent that did not write the change verifies it before merge; link its review comment and verdict here.

## Additional Notes

Any additional information reviewers should know.
