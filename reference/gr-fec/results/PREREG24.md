# PREREG24 — randomness entering thought through the senses (Matej, 24 Oct 2026)
Written before any run of e35.

Agent: worm v1 (rectified senses, s0 = 0.03), made **deterministic inside**: no heading noise; turn iff the pre-registered
drive rule says "falling" ((−dD/s0 − 1) > 0, no random draw); a turn rotates the heading by a fixed 2.4 rad; starting
headings evenly spaced over the 100 worms. The only randomness left is sensor noise, scaled by k ∈ {0, 0.1, 0.3, 1, 3,
10, 30} × the v1 levels (odour σ 0.01·k). Seeds 6000+ (4 seeds per k, 100 worms each), 20 min.
Worlds: (W1) single odour peak 1.0 at (15, 0); (W2) trap world: the same peak plus a decoy peak 0.6 at (−10, 0),
σ 6 mm. Score: chemotaxis index target (15, 0) vs control at the decoy position (−10, 0); "stuck" share = worms that
moved < 2 mm net in the last 5 min or sit within 6 mm of the decoy.

P24a: in both worlds, CI at k = 0 is lower than the best k by > 0.1 (noise helps when it is the only randomness).
P24b: the best k is interior (not 0, not 30) in both worlds.
P24c: the best k in W2 (traps) ≥ the best k in W1.

## Outcome (recorded after running)
W1: k = 0 CI +0.61 (16 % stuck); best k = 3, CI +0.87 ± 0.01; k = 30 CI −0.06. W2 (trap): k = 0 CI −0.06; best k = 1,
CI +0.29 ± 0.01; k = 30 −0.21. **P24a PASS (both worlds), P24b PASS (both), P24c FAIL** (trap world optimum k = 1 is lower
than the single-peak optimum k = 3). Unexplained detail: a dip at k = 0.1–0.3, worse than no noise at all (W1 0.48, 0.42 vs
0.61; W2 −0.22, −0.16 vs −0.06). Note k = 1 is the noise level v1 already used.
