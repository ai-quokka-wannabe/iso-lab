# Pre-registration 11 — 'gravity gate' for OSD answers — written 2026-10-24 07:20:42 UTC, before confirmation data
Context: BP-only failures are DETECTED (syndrome != 0); OSD always outputs a codeword, so its wrong answers are UNDETECTED.
Exploratory (seeds 2300/2301): hybrid UER 2.0e-3 (3.0 dB) / 3.4e-4 (3.5 dB). Gate statistic r = gap/action, where gap = channel
likelihood-metric difference between runner-up and best OSD-2 candidate (well-depth gap) and action = sum |LLR| over bits where the
best codeword disagrees with the channel hard decision (tunnelling cost). AUC for flagging wrong answers 0.996/0.999.
Threshold r >= 0.5 chosen on that data: 0/95 and 0/32 wrong answers accepted.
GATED HYBRID: random-order layered BP (20 it); on failure, blended OSD-2 (w = 0.25); accept OSD output only if r >= 0.5, else
declare a detected failure.
Confirmation: fresh seeds 2400 (3.0 dB, 96 000 frames), 2401 (3.5 dB, 192 000 frames).
PRIMARY (P12), at BOTH SNRs: (a) number of accepted-but-wrong OSD outputs <= max(2, number of BP-only undetected errors on the
same frames), AND (b) gated detected FER <= 0.3 x BP-only (20 it) detected FER.
Note stated in advance: Monte-Carlo can only bound UER at ~1e-5 here; space-telecommand targets (~1e-9) need importance sampling.
