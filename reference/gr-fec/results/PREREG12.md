# Pre-registration 12 — 'multiverse' OSD — written 2026-10-24 07:30:22 UTC, before confirmation data
Exploratory (3.0 dB, seed 2500, 48k frames, 2558 BP failures): single blended OSD-2 hybrid FER 0.00219; MV2 = best (channel
likelihood) of {blended OSD-2 (w=0.25, BP posterior at it. 20), OSD-2 ordered by BP posterior at it. 5} gave 0.00056;
MV6 (adds channel, posterior it.20, blend minus 0.5*mean|LLR|*flips over last 8 it., blend w=1) gave 0.00025.
ML bracket from OSD-3 certificates on seed 2500: [0.00006, 0.00038].
BP stage: random-order layered BP, 20 iterations (posterior at it. 5 from the same random-order stream).
PRIMARY (P13): fresh seeds 2600 (3.0 dB, 96 000 frames) and 2601 (3.5 dB, 192 000 frames): MV2 hybrid FER < single blended
OSD-2 hybrid FER, exact one-sided McNemar p < 0.01 at both SNRs, AND MV2 FER <= 0.5 x single-blend FER at both SNRs.
SECONDARY (S13): at 3.5 dB, MV6 FER <= MV2 FER (reported with McNemar).
SECONDARY (S14): gravity gate (gap/action >= 0.5 of the winning universe) on MV2: accepted-but-wrong <= max(2, BP undetected).
