# PREREG26 — complex vs real edges for timing and composition (H-AI11)
Written before any run of e37.

Task (second-order conditioning with timing, worm-style): training episodes "odour A, then odour B after lag 3 ± 1"
(200 episodes, no food) and separately "B, then food after a lag" (200 episodes). Test: A alone — when is food expected?
(A → food is never experienced; it must be composed.) Lag cases: (U) unimodal 4 ± 1 → truth: composed lag 7 ± spread;
(M) bimodal 2 or 8 (equally likely) → truth peaks at 5 and 11. Lags uniform jitter in {−1, 0, +1}. Window L = 16 steps.
Edges learned from the episodes (empirical statistics, seed 9000):
- R0 scalar real: strength only (P(B within L | A)). Composition = product. Cannot time.
- RL real lag vector, L = 16 numbers per edge. Composition = convolution (O(L²)).
- C1 one complex number per edge, z = mean e^{iω·lag}, ω = 2π/16 (2 real numbers). Composition = product (O(1)).
  Decoded time = arg(z)/ω; |z| = timing confidence.
- C3 three harmonics (6 real numbers). Composition = elementwise product; decode = argmax of truncated inverse Fourier.
P26a (sanity): R0 gives no timing.
P26b: in case U, C1 predicts the composed lag within ±0.5 of 7 using 2 numbers per edge vs RL's 16.
P26c: in case M, C1 misplaces the peaks (predicts ≈ the mean, 8) but signals it with low |z| (< 0.5 × case U);
RL finds both peaks (5 and 11); C3 finds at least one true peak (±1).
