# Pre-registration 17 — search-limited vs iteration-limited failures — written 2026-10-24 08:34:58 UTC, before data
Account (L45): short-code waterfall failures are SEARCH failures (wandering/oscillating among wrong states); long-code
waterfall failures are ITERATION-LIMITED (still converging when the cap hits).
Measure: for frames unconverged at iteration 50 (layered BP, fixed natural order, sum-product), progress ratio
rho = (# unsatisfied checks at it. 50) / (# unsatisfied checks at it. 25).
PRIMARY (P17): median rho of AR4JA (8192,4096) failures at 0.9 dB (1500 frames, seed 3000) is lower than median rho of
CCSDS (128,64) failures at 3.0 dB (24 000 frames, seed 3001), AND the fraction of failures with rho < 0.5 ('still progressing')
is at least 2x higher for AR4JA.
