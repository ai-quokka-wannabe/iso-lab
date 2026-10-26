# Pre-registration 5 — Nishimori line and decoder chaos — written 2026-10-24 05:01:42 UTC, BEFORE any data

Matej's hunch: bring thermodynamics/statistical physics in. Physics basis: in spin glasses the Nishimori line
(assumed temperature = true temperature) is optimal and free of glassy behaviour; below it (colder) lies frustration,
where BP is known to stop converging; above it (hotter) lies a paramagnetic regime where BP converges to a
(possibly wrong) fixed point.

Set-up: CCSDS TC LDPC (128,64), sum-product BP, 50 iterations, AWGN at Eb/N0 = 3.0 dB, all-zero codeword.
Decoder temperature mismatch via LLR scaling: decoder input = alpha * true LLR, alpha = sigma_true^2 / sigma_assumed^2.
alpha = 1: Nishimori line (matched). alpha > 1: decoder COLDER than channel (overconfident). alpha < 1: HOTTER.
alpha grid: 0.5, 0.7, 0.85, 1.0, 1.2, 1.5, 2.0; 12 000 frames each, seed 500.
Chaos measure per frame: FTLE = mean over iterations 31-50 of log(stretching factor of the BP map), warm-started
1-probe power iteration each iteration. Failure end-states (iterations 41-50): "fixed point" = max message change < 1e-3
and no hard-decision flips; "oscillating" = any flips; "drifting" = otherwise.

PRIMARY (P5): among failing frames, FTLE increases with alpha: Spearman(alpha, FTLE) over all failing frames > 0,
one-sided permutation p < 0.01, AND the oscillating fraction of failures at alpha = 2.0 exceeds that at alpha = 0.5.
SECONDARY (S5): FER(alpha) is minimised inside [0.85, 1.2], with the best FER in that window below FER at alpha = 0.5 and
at alpha = 2.0 (non-overlapping 95% Wilson intervals).
SECONDARY (S6): the fixed-point share of failures decreases with alpha (Spearman over the 7 grid points < 0).
Reported, not claimed: undetected errors, posterior-entropy ("decoder temperature") trajectories.
