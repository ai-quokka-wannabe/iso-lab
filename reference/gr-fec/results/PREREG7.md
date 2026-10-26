# Pre-registration 7 — random causal order + improved CCSDS pipeline — written 2026-10-24 06:22:27 UTC, before confirmation data

Exploratory (3.0 dB, seeds 1000/1001, 12k frames): layered BP natural order 50 it FER 0.0495; same with a NEW random check
order every iteration 0.0458; also beat parallel random-order branches and same-order restarts; damping does not stack.
Confirmation data: CCSDS (128,64), AWGN, all-zero codeword. 3.0 dB seed 1100 (48 000 frames); 3.5 dB seed 1101 (96 000 frames).
Decoders (max 50 iterations each): FLOOD = flooding sum-product; FLOOD-D = flooding with damping 0.3;
LAY-FIX = layered, natural check order; LAY-RND = layered, new uniformly random check order every iteration.
PIPE = LAY-RND, then on failure: oscillation-guided guess-and-redecode — rank bits by hard-decision flips over the last 20
iterations (tie-break: smallest |posterior LLR|), for the top k = 4 bits try (flip current belief, then reinforce it) with a
saturated channel LLR (|50|), each attempt a fresh LAY-RND run of 50 iterations; stop at first syndrome-valid result.
PRIMARY (P8): LAY-RND has lower FER than LAY-FIX at BOTH SNRs, exact one-sided McNemar p < 0.01.
SECONDARY (S9): LAY-RND has lower FER than FLOOD-D at both SNRs (McNemar p < 0.01).
SECONDARY (S10): PIPE FER is at least 2x lower than FLOOD at both SNRs; undetected errors reported.
Average decoding work (iterations per frame incl. re-decodes) reported for all.
