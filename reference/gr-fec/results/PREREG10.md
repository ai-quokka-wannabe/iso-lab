# Pre-registration 10 — blended-reliability OSD — written 2026-10-24 07:08:49 UTC, before confirmation data
Exploratory (seeds 2100/2101): after 20 iterations of random-order layered BP, OSD-2 on the failures with ordering by
|channel LLR| + 0.25 |BP posterior| and channel-likelihood candidate selection gave hybrid FER 0.00146 vs 0.00296 (3.0 dB) and
0.00029 vs 0.00048 (3.5 dB) for standard channel-ordered OSD-2. w = 0.25 was chosen on that data.
Confirmation: fresh seeds 2200 (3.0 dB, 48 000 frames) and 2201 (3.5 dB, 96 000 frames), same BP stage.
PRIMARY (P11): at both SNRs, BLEND (w = 0.25) hybrid FER < CHANNEL-ordered OSD-2 hybrid FER, exact one-sided McNemar p < 0.01
on the frames passed to OSD.
