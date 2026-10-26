# PREREG39 — imagination with a body that can act on it: directed steering + lateral temperature sense
Written 25 Oct 2026 ~22:55 UTC before any run.
Body (`src/steer.py`, DirBody): adds sense y[8] = T(1 mm left of head) − T(1 mm right of head) + noise σ 0.02 °C (head-swing
sampling) and accepts a steering command s ∈ {left, straight, right}: heading += 0.15·s rad per second (random pirouettes
still possible). Brain = embodied v6 with learned trust, plus one of:
- **plain:** never steers (steering available but unused);
- **greedy (control):** steers towards the side whose neighbouring temperature concept has the higher learned value (one-step);
- **imagine:** state = (temperature concept, lateral sign −/0/+ with threshold 0.01 °C); event-based transition counts per
  steering action; every 10 s, K = 8 imagined walks of H = 8 concept events per action (later steps "straight"), scored by
  learned concept values (discount 0.9); steer = argmax Q if max − min > 0.02, else straight.
Constants fixed in advance. Worlds: Hardland (72 h, energy h48–72) and the never-seen cold-band layout (seed 23000, 48 h,
energy h24–48); 100 worms; seeds 28000+.
P39a: imagine ≥ plain + 0.08 in both worlds (threshold above the ≈ ±0.03 seed spread seen in PREREG38d).
P39b: imagine ≥ greedy + 0.03 in both worlds (multi-step imagination beats one-step greed).
Descriptive: printed imagined walks with lateral labels.

## Outcome (recorded after running; seeds 28000–28012)
Hardland h48–72: plain 0.672, greedy 0.629, imagine 0.556. New cold band h24–48: plain 0.388, **greedy 0.625**, imagine 0.475.
**P39a FAIL** (Hardland −0.116; cold +0.088). **P39b FAIL** (imagine below greedy in both). Printed imagined walks: the lateral
label flickers L → R → L with head swings and sensor noise, so most "events" are label noise; transition model ≈ noise.
Unexpected: one-step greedy steering on learned concept values gives +0.237 in the never-seen cold world.

# PREREG39b — replication of the greedy-steering effect in the never-seen cold world (written before running)
plain vs greedy, cold-band layout (seed 23000), 48 h, two fresh seed pairs (28100+, 28200+).
P39c: greedy − plain ≥ +0.10 in both pairs.

## Outcome of 39b (seeds 28100–28201)
Pair 0: plain 0.451, greedy 0.620 (+0.170). Pair 1: plain 0.450, greedy 0.623 (+0.173). **P39c PASS** — greedy value-steering is
a robust gain of ≈ +0.17–0.24 in the never-seen cold world (three seed pairs including PREREG39).
