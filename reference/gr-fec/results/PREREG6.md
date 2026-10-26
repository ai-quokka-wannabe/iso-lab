# Pre-registration 6 — chaos onset at the Nishimori line — written 2026-10-24 05:07:51 UTC, before data at 2.5/3.5 dB

Exploratory (3.0 dB, seed 500, PREREG5 data): median FTLE of failing frames crosses zero at alpha_c = 0.869 (log-linear
interpolation). FER is flat for alpha in [0.85, 1.2]; a parabola fit of log FER puts the minimum at 1.148, so the
"chaos onset = FER optimum" version does NOT hold tightly and is not claimed.
Claim tested instead: chaos onset sits near the Nishimori line alpha = 1 (a location fixed by theory, not estimated).
Data: CCSDS (128,64), sum-product BP, 50 it., seed 600, Eb/N0 = 2.5 dB (12 000 frames) and 3.5 dB (24 000 frames),
alpha grid 0.6, 0.75, 0.85, 0.92, 1.0, 1.08, 1.2, 1.4; FTLE and failure classes as in PREREG5.
PRIMARY (P6): at BOTH SNRs, alpha_c (zero crossing of median FTLE of failing frames, log-linear interpolation; if several
crossings, the first from the hot side) lies in [0.8, 1.25].
SECONDARY (S7): at both SNRs, the FER at alpha = 1.0 is within the 95% Wilson interval of the lowest FER on the grid
OR is the lowest (Nishimori optimality reproduced).
SECONDARY (S8): PREREG5 monotone trends (FTLE up with alpha, fixed-point share down) hold at both SNRs (Spearman over grid).
