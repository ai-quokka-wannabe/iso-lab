# PREREG36 — organic v7b: select for generality (three balanced lives per genome per generation)
Written 25 Oct 2026 before any run. Brain = organic v7 (`src/evolve2.py`), unchanged. Fresh random start, seed 25000
(independent of PREREG34/35). 100 genomes; **each generation every genome lives three 12 h lives**: harsh world with the
food band cold (side −1), harsh world with the food band warm (side +1), rich world — each a newly randomised layout;
fitness = mean of the three lifetime mean energies. Top 20 survive + 4 mutated children each. 14 generations (time-boxed,
≈ 5 min/generation). Full populations and fitness logged every generation.
Test in the never-seen worlds of PREREG35 (same layouts and seeds 23000–23002, 50 worms, 48 h, energy h24–48), elites 10 × 5.
P36a: v7b in the cold-band world ≥ 0.70 (v7 0.442).
P36b: final elite median learning rate ≥ 10 × v7's (10^−4.16 → ≥ 10^−3.16).
P36c: final elite median |temperature instinct| < 1 (no side memorised; v7 +2.33).
P36d: v7b ≥ e45 − 0.05 in all three never-seen layouts (e45: 0.836, 0.828, 0.892).
Descriptive (grand-pattern check): is learned trust in smell positive again (third independent run)? Gene–fitness
correlations from full populations.

## Outcome (recorded after running; evolution 12:21–13:37 UTC, test after)
Evolution: elite mean fitness 0.710 → 0.814 (gen 12) / 0.779 (gen 13); by world at gen 13: cold 0.640, warm 0.855, rich 0.842.
**P36b FAIL:** learning rate went to the range minimum (10^−5.00; v7 10^−4.16) — learning switched off.
**P36c FAIL:** temperature instinct +2.78 ("go warm") despite balanced cold/warm lives.
Never-seen worlds (energy h24–48): v7b cold **0.576** (v7 0.442, v6 0.431, e45 0.836), warm **0.853** (best of all brains), rich
0.858 (e45 0.892). **P36a FAIL** (0.576 < 0.70). **P36d FAIL** (cold −0.26 vs e45).
Grand-pattern check: learned trust in smell −2.53 (e45 +2.92, v7 +3.68) — but with learning off the learned values stay ≈ 0,
so the trust gene is selection-neutral and drifts: **inconclusive**, not a contradiction.
Gene–fitness correlations (all 1,400 individuals): strongest |r| ≈ 0.2–0.3 (exploration temperature −0.32, touch bias −0.26,
eating reward +0.24, neutral-odour genes ±0.22); 16 of 73 genes |r| < 0.05 (selection-neutral).
Diagnosis: (1) with fitness = *mean* over lives, a warm instinct wins 2 of 3 world types, so selection prefers a strong
instinct compromise to learning; (2) 12 h lives are short and the organic brain's normalised learning over ~350 features is
slow, so learning's benefit arrives too late to pay; (3) 73 genes × 14 generations is a large, under-explored space.
