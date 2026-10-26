# Pre-registration 14 — hardware-like chain — written 2026-10-24 08:14:16 UTC, before confirmation data
Chain: 6-bit channel LLRs (step 0.5, clip 15.5), normalised min-sum x0.75 layered BP (fixed natural order, 20 it., quantised messages)
-> MV2 OSD-2 (blend |L| + 0.25 |posterior it.20|; posterior it.5) -> gravity gate r >= 0.5.
Exploratory (seeds 3400/3401): accepted-wrong 0/0; retransmissions 0.133x / 0.086x min-sum BP alone; MV2 FER 0.40x / 0.29x blend.
Confirmation: fresh seeds 3500 (3.0 dB, 96 000 frames), 3501 (3.5 dB, 192 000 frames).
PRIMARY (P15), both SNRs: accepted-but-wrong <= max(2, BP undetected) AND retransmissions <= 0.2 x min-sum-BP-only detected FER
AND MV2 FER <= 0.5 x blend-OSD-2 FER.
