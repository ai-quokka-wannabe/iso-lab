# Pre-registration 15 — chaos-onset law on two new codes — written 2026-10-24 08:56:34 UTC, before third-SNR data
Measured (fit data): R36_200 (3,6)-regular n=200: alpha_c = 0.985 (2.0 dB, K = 1.561), 0.811 (2.5 dB, K = 1.443).
R48_200 (4,8)-regular n=200: alpha_c = 0.971 (2.5 dB, K = 1.728), 0.825 (3.0 dB, K = 1.645). (CCSDS: K = 1.80, 1.73, 1.69, 1.68.)
Observations already visible: K is code-dependent (not universal) and drifts downward with SNR on every code.
Two competing predictions for the third SNR (same protocol as PREREG13; alpha grid 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.15):
 (a) CONSTANT-K law: alpha_c = mean K / EbN0_lin.  R36 @ 3.0 dB: 0.753 [0.677, 0.828];  R48 @ 3.5 dB: 0.753 [0.678, 0.829]
 (b) DRIFTING-K law (K linear in Eb/N0 dB, extrapolated): R36 @ 3.0 dB: K = 1.325 -> alpha_c = 0.664 [0.598, 0.731];
     R48 @ 3.5 dB: K = 1.562 -> alpha_c = 0.698 [0.628, 0.767]
Each passes if the measured alpha_c lies in its +-10% window. Reported: which law fits both codes.
Also re-tested on these new codes (S15): phase picture — Spearman(alpha, FTLE) > 0 and Spearman(alpha, fixed share) < 0.
