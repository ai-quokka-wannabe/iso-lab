# Pre-registration 13 — chaos onset at a fixed absolute LLR scale — written 2026-10-24 07:42:07 UTC, before 4.0 dB data
PREREG6 failed: chaos onset alpha_c (median FTLE of failing frames crosses 0) = 1.012 (2.5 dB), 0.869 (3.0 dB), 0.756 (3.5 dB).
Post-hoc observation: alpha_c x (Eb/N0 linear) = 1.80, 1.73, 1.69 (mean 1.74) — onset at a nearly fixed ABSOLUTE LLR scale.
PREDICTION (P14): at Eb/N0 = 4.0 dB (linear 2.512), alpha_c = 1.74 / 2.512 = 0.693; claim passes if measured alpha_c lies in
[0.62, 0.76] (+-10%) using the PREREG5/6 protocol (CCSDS (128,64), sum-product BP, 50 it., warm 1-probe FTLE over it. 31-50,
median over failing frames, log-linear interpolation), seed 700, alpha grid 0.55, 0.6, 0.65, 0.7, 0.75, 0.8, 0.9, 48 000 frames each.
The Nishimori-line alternative (alpha_c = 1) would fail this test.
