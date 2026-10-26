# Pre-registration 16 — the 'temperature' of min-sum is its normalisation factor — written 2026-10-24 08:34:58 UTC, before data
Theory: normalised min-sum is homogeneous of degree 1, so channel-LLR scaling alpha has no effect (verified: identical
convergence for 1500 frames under x3.7). The only temperature-like knob is beta (check-message scale relative to channel).
Prediction from the sum-product phase picture (PREREG5): small beta = weak messages = 'hot' -> failures freeze; beta -> 1
= overconfident messages = 'cold' -> failures oscillate chaotically.
Set-up: CCSDS (128,64), flooding normalised min-sum, 50 it., AWGN 3.0 dB, seed 900, beta in {0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0},
12 000 frames each (same noise for all beta); FTLE and end-state classes exactly as PREREG5.
PRIMARY (P16): over the beta grid, Spearman(beta, median FTLE of failures) > 0 AND Spearman(beta, fixed-point share of
failures) < 0, AND oscillating share at beta = 1.0 exceeds that at beta = 0.4.
SECONDARY (S16): FER is minimised at an interior beta (0.5 <= beta* <= 0.9).
