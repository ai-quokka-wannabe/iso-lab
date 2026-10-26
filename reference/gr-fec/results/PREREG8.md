# Pre-registration 8 — servo 'anti-freeze' loop — written 2026-10-24 06:35:28 UTC, before confirmation data
Exploratory (3.0 dB, seeds 1000/1001): heating oscillating frames (channel or message temperature) always hurt;
Langevin kicks >= 0.3 hurt; re-melt frozen frames + channel cooling (gain 0.2) rescued 16, lost 1 vs LAY-RND.
SERVO = layered BP, new random check order every iteration, 50 iterations; from iteration 10, a frame whose hard decisions
have not flipped and whose unsatisfied syndrome has not changed for 3 iterations ('frozen') has its channel LLR scale
multiplied by 1.2 (clamped <= 2) and its check messages reset (re-melt from channel evidence).
PRIMARY (P9): SERVO has lower FER than LAY-RND (same check-order random stream) at BOTH 3.0 dB (seed 1200, 48 000 frames)
and 3.5 dB (seed 1201, 96 000 frames), exact one-sided McNemar p < 0.01.
Reported: rescued/lost counts, undetected errors.
