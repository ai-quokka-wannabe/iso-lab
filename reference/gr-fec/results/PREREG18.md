# Pre-registration 18 — the optimum sits on the chaotic side of the chaos onset — written 2026-10-24 09:17:29 UTC, before data
Post-hoc pattern (all settings so far): FER-optimal decoder temperature lies on the CHAOTIC side of the onset (failures' median
FTLE > 0 at the optimum): CCSDS SP 2.5/3.5 dB, R36 3.0 dB, R48 (pending), CCSDS min-sum 3.0 dB (beta_c ~ 0.57, beta* 0.7).
Test: CCSDS (128,64), flooding normalised min-sum, 50 it., beta grid 0.4..1.0 step 0.1, 12 000 frames per beta, at Eb/N0 = 2.5 dB
(seed 910) and 3.5 dB (seed 911, 24 000 frames).
PRIMARY (P18), at both SNRs: (a) the FER-minimising grid beta* has median FTLE of failures > 0, and (b) beta* > beta_c
(log-linear zero crossing of median FTLE), and (c) median FTLE at beta* < 0.4 ('mildly' chaotic).
