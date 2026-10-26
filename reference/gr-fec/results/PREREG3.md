# Pre-registration 3 — written 2026-10-24 04:33:43 UTC, before generating the cheap-probe data
Same code, decoder, channel and frames as PREREG2 (identical seeds 100 -> 200, so decoder trajectories are identical).
Cheap estimator: no per-iteration probing; at checkpoint t0 only, K cold-start power-iteration probes (finite-difference JVP).
Model: hist + lamc_K (single feature, no trend), fitted on seed 100, frozen, tested on seed 200, 3.0 dB, t0 = 10.
PRIMARY (P3): with K = 5, AUC(hist + lamc5) − AUC(hist) has bootstrap 95% CI lower bound > 0, AND its point estimate is
>= 50% of the PREREG2 P1 gain (i.e. >= 0.019).
Reported, not claimed: K = 1, 2, 3, 8 and checkpoints t0 = 5, 15.
