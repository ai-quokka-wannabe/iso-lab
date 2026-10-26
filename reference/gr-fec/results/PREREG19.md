# PREREG19 — gravity gate on an associative memory in Hamming space (H-AI1)
Written 24 Oct 2026 before any run of e30.

Setup: memory of P random ±1 patterns of length N = 256 (P = 16 and P = 64). Recall = nearest stored pattern
(maximum correlation with the soft cue = ML decoding over the memory "codebook"). Soft cue y = 2·x/σ² with
x = source + Gaussian noise σ. Sources, one third each: (S) a stored pattern; (F) a "flying cat": a stored pattern
with 15–35 % of its bits changed (related but never stored); (R) a random never-stored pattern.
Recall is *correct* only for S cues recalled to their own pattern; any recall of an F or R cue is a hallucination.
Scores (higher = more trustworthy): cos = correlation of y with recalled pattern / (|y|·√N); gap = metric(best) −
metric(runner-up) with metric = <y, pattern>/2 (log-likelihood difference); action = Σ|y_i| over bits where the
recall disagrees with sign(y); r = gap / action (action floored at 1e-9).
σ levels: 0.6, 0.8, 1.0. 6000 cues per (P, σ). Seed 1900.

P19 (main): AUROC for separating correct recalls from hallucinations, r ≥ cos − 0.01 and r ≥ gap − 0.01 in at least
5 of the 6 (P, σ) cells.
S19 (stronger): r beats cos by ≥ 0.02 in at least 4 of 6 cells.
Reported: accepted-correct share when the threshold lets through ≤ 1 % of hallucinations, for each score.

## Outcome of the original design (recorded after running)
All four scores reached AUROC ≈ 1.000 in all cells: P19 "passes" 6/6 and S19 fails 0/6, but the design was at the
ceiling (N = 256 with σ ≤ 1 makes recall trivially separable), so it is **uninformative**. Fixed before any further run:

# PREREG19b — same test in a hard regime
N = 64; σ ∈ {1.2, 1.6, 2.0}; P ∈ {16, 64}; flying-cat cues change 10–30 % of bits; everything else identical;
fresh seed 1901. Same criteria P19 / S19. If AUROCs are again ≥ 0.999 everywhere the test is declared uninformative.

## Outcome of PREREG19b (recorded after running)
P19 FAILS (2/6), S19 FAILS (0/6): the gap/action ratio does not beat plain fit (cos) on a memory. Forensics
(f30, seed 1902) split hallucinations into *novelty* (cue matches nothing stored: cos/action win, e.g. flying cat
σ = 2.0: cos 0.876 vs gap 0.825) and *confusion* (stored cue recalled as the wrong memory: gap wins, 0.960 vs cos 0.896).

# PREREG19c — a two-wall detector
Score = logistic regression on (cos, log gap), fitted on seed 1903 data and frozen; tested on fresh seed 1904,
same cells as 19b (N = 64, σ ∈ {1.2, 1.6, 2.0}, P ∈ {16, 64}).
P19c: AUROC(two-wall) ≥ max(AUROC(cos), AUROC(r)) + 0.005 in ≥ 4 of 6 cells.
