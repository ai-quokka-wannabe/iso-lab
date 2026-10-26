# Pre-registration — written 2026-10-23 20:44:50 UTC, after seeing only 6 of 36 lifts in batch 1 (seed 11)

Primary hypothesis H1 (to be tested on an INDEPENDENT confirmation batch, seed 12, CCSDS (128,64) protograph lifts,
Eb/N0 = 4.0 dB, 150k frames per code, sum-product BP, 50 iterations):
  Spearman(oll3_mean, FER) < 0 — codes whose Tanner graph has higher (less negative) mean 3-step lazy-walk
  Ollivier-Ricci curvature (alpha=0.5) have lower FER.
Secondary H2: the partial correlation of rank(oll3_mean) with rank(log FER), controlling for ranks of total
  6-cycle and 8-cycle counts (closed non-backtracking walks), is negative.
Decision rule: one-sided permutation p < 0.05 for H1 on the confirmation batch. Batch 1 is exploratory only.
No other feature will be claimed from the confirmation batch.
