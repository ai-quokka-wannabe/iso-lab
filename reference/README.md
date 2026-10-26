# Reference Archives

Archives kept here as they were given, for the lab to read and to reproduce. They are not the lab's code: they are
never edited, they are exempt from the lab's style, licence-header and *Rust first* rules, and the markdown and link
checks skip them. Whatever the lab takes from them is rebuilt in its own Rust or a notebook, and tested there.

## gr-fec

[gr-fec/](gr-fec/) is the maintainer's research sandbox of 23 to 26 October 2026: first the geometry of
error-correcting-code decoders, then an embodied, senses-only worm-level mind grown, tested and evolved in about twenty
pre-registered experiments. What it learned is written up for the lab in
[docs/research/gr-fec.md](../docs/research/gr-fec.md), and its ideas are § G of
[Matej's ideas and hunches](../docs/hypotheses/matej-hunches.md#g-from-the-gr-fec-sandbox).

- **Start with** [gr-fec/docs/HANDOFF.md](gr-fec/docs/HANDOFF.md): its state, code map, exact commands and gotchas.
  Then [gr-fec/docs/principles.md](gr-fec/docs/principles.md) and [gr-fec/docs/ai_branch.md](gr-fec/docs/ai_branch.md);
  every result is a row of [gr-fec/results/LEDGER.md](gr-fec/results/LEDGER.md) and every prediction a
  `results/PREREG*.md` with its outcome.
- **To run it**: Python 3.11 or newer with the packages in [gr-fec/requirements.txt](gr-fec/requirements.txt), from
  inside `reference/gr-fec/`. Its paths under `/home/claude/` are those of the cloud machine it was written on. The
  large decoder-state arrays are not included; its README says how to regenerate them from their seeds.
- **Known problems**, beyond the ones it records itself: `experiments/e48_organic_eval.py` seeds a generator with
  Python's string hash, so its runs reproduce only with `PYTHONHASHSEED` fixed; the evolution scripts give each half of
  a population a different body seed; and its metabolism constants starve a newborn worm in under an hour. The details
  are in [docs/research/gr-fec.md § 6](../docs/research/gr-fec.md#6-problems-to-avoid-found-in-gr-fec).
- **What was changed on publishing**: one personal detail removed from `docs/HANDOFF.md`, marked where it was; one log,
  `results/e31.log`, had the carriage returns of a progress display turned into line breaks. Nothing else.
- **Licence**: contributed by the maintainer and published under this repository's licence, GPL-3.0-or-later,
  copyright The AiQuokkaWannabe Authors. Two data files came from elsewhere: `results/CCSDS_64_128.alist` from the
  AFF3CT project's configuration files, as its README says, under that project's licence; the origin of
  `results/AR4JA_4096_8192.qc` is not recorded in the archive.
