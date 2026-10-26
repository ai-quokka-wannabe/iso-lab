# PREREG38 — imagination as random walks on the concept graph (H-AI14)
Written 25 Oct 2026 before any run. Brain = embodied v6 with learned trust (separate instinct, trust_k 2, s0 0.01, integ 10)
plus `src/imagine.py`: learned transition counts between states (temperature concept × direction of change) for
"keep going" vs "turn"; every 10 s, K = 8 imagined walks of H = 6 steps per first action, scored by learned concept values
(discount 0.9); imagined advantage ΔQ = Q(turn) − Q(keep going) adds turns (prob 0.05·tanh⁺(2ΔQ)/s) or suppresses them
(prob 0.5·tanh⁻(2ΔQ)). Constants chosen in advance, not tuned.
Comparison in one batch: v6 trust (no imagination) vs v6 trust + imagination; Hardland (fixed layout of PREREG31) 72 h and
a never-seen cold-band layout (seed 23000) 48 h; 100 worms; seeds 27000+.
P38a: Hardland energy h48–72: imagination ≥ no-imagination + 0.05.
P38b: never-seen cold band energy h24–48: imagination ≥ no-imagination + 0.05.
Descriptive: print a sample of imagined walks as named concepts (interpretability).

## Outcome (recorded after running; seeds 27000–27011)
Hardland energy h48–72: plain 0.630, imagination 0.605 (**P38a FAIL**, −0.025). New cold band h24–48: plain 0.446,
imagination 0.437 (**P38b FAIL**, −0.009). Printed imagined walks barely move ("22.4 °C → 22.4 °C → … → 21.6 °C"): with
clock-based 10 s steps the worm moves ≈ 2 mm (≈ 0.2 °C), so H = 6 steps look ≈ 1 min / ≈ 1 °C ahead, while the cold food
band is ≈ 5 °C away — imagination cannot see it. Diagnosed directly from the printable walks.

# PREREG38b — event-based imagination (transitions between *different* concepts)
Written before running. One change: a transition is recorded only when the state (temperature concept × direction) changes,
together with the action taken since the last change (turned at least once or not); imagined walks step from concept change
to concept change (H = 8 changes ≈ several °C). Everything else as PREREG38; fresh seeds (offset +100). Same criteria P38a/b.

## Outcome of 38b (seeds 27100–27111)
Hardland h48–72: plain 0.625, imagination 0.616 (**P38a FAIL**, −0.009). New cold band h24–48: plain 0.442, imagination
0.504 (**P38b PASS**, +0.062). Printed walks: most "events" are flickers of the direction label (↑ · ↓) caused by sensor noise
(σ 0.05 °C vs ≈ 0.2 °C real change per 10 s), not moves between concepts.

# PREREG38c — state = temperature concept only (written before running)
One change from 38b: the direction component is dropped; a state is the temperature concept alone, so only real concept
changes are events. Fresh seeds (offset +200). Same criteria.

## Outcome of 38c (seeds 27200–27211)
Hardland h48–72: plain 0.581, imagination 0.653 (**P38a PASS**, +0.072). New cold band h24–48: plain 0.418, imagination 0.438
(**P38b FAIL**, +0.020). Imagined walks now travel across concepts (e.g. 19.7 → 20.7 → 21.5 → 22.3 → 23.0 °C).
Across 38/38b/38c single-seed differences range −0.025…+0.072 while plain baselines vary 0.58–0.63 (Hardland) and
0.42–0.45 (cold) between seeds — replication needed.

# PREREG38d — replication of 38c with three seeds per world (written before running)
Same brains as 38c; seeds offsets +300, +400, +500 (3 independent pairs per world). P38d: in each world the mean
(imagination − plain) over the 3 pairs ≥ +0.03, and positive in ≥ 2 of 3 pairs.

## Outcome of 38d — replication (seed offsets +300, +400, +500)
Hardland h48–72 differences −0.019, +0.009, −0.030 (mean −0.013) → **FAIL**. New cold band h24–48 differences +0.018, −0.002,
+0.046 (mean +0.021, 2/3 positive) → **FAIL** (below +0.03). The 38c Hardland gain (+0.072) was seed luck.
Interpretation: the worm can only choose "keep going" or "turn to a random heading", and its concepts include no heading,
so imagination cannot steer towards what it imagines; transitions without heading are nearly symmetric. Imagination needs
a body that can act on it (directed turns) and a sense of its own heading (proprioception).
