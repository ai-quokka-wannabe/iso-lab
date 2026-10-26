> **Resuming? Read `docs/HANDOFF.md` first** (also loaded via `CLAUDE.md`). Principles learned: `docs/principles.md`.

# gr-fec — geometry of FEC/EDAC signal spaces

Research sandbox. Start with `docs/brief.md` (§0 rules, §7 hypotheses, §10 literature, §12 sprint-1 results) and `results/LEDGER.md`.

- `src/ldpc.py` — code construction (random regular, 4-cycle removal), girth, vectorised sum-product BP, AWGN LLRs
- `src/alist.py` — alist reader (handles zero padding), GF(2) rank
- `src/curvature.py` — Ollivier–Ricci (multi-step lazy walks, exact OT), resistance curvature, non-backtracking cycle features
- `experiments/` — t0x sanity checks, e0x simulations, a0x analyses
- `results/` — raw JSON, logs, analysis outputs, PREREG.md, LEDGER.md

Requirements: Python 3.11+, numpy, scipy, networkx, POT (`pip install pot`).
CCSDS matrix: `results/CCSDS_64_128.alist` (from the AFF3CT configuration_files repository).

Large decoder-state arrays (`results/e05_*.npz`) are not included in the zip; regenerate them exactly with
`python3 experiments/e05_stopping.py <EbN0> <seed> <frames> [cheap]` using the seeds in `results/PREREG2-4.md`
(100/101/200/201 full; 100/101/200/300/301 cheap). Decoding is deterministic given the seed.

## AI branch ("physics of thought", 24 Oct 2026)
Start with `docs/ai_branch.md` §0: hallucination detection with the decoder's gravity gate, and a worm-level embodied
agent (World → Body → Brain, `src/embodied.py`) that passes classic C. elegans assays. Ledger rows L53–L71.
