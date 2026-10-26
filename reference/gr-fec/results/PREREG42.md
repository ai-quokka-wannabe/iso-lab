# PREREG42 — organic v8: steering genes (does evolution rediscover value-steering?)
Written 26 Oct 2026 before any run. Brain = organic v7 (`src/evolve2.py`, learning-rate range 10^−3…10^0 as v7c) plus
`src/evolve3.py`: body = DirBody (head-swing lateral temperature sense, directed steering ±0.15 rad/s). Three new genes:
k_value ∈ [−5, 5] (steer by lateral signal × learned value slope dV/dT — value-steering), k_innate ∈ [−5, 5] (steer by the
lateral signal alone — innate warm/cold seeking), log10 steering noise ∈ [−3, 0] (rad/s). Steer = clip(tanh(k_value·L·V′ +
k_innate·L) + noise, −1, 1), L = smoothed lateral signal / 0.02 °C, V′ from the brain's own learned temperature concepts.
Evolution as v7c: fresh seed 31000, 100 genomes, three 24 h lives (harsh cold, harsh warm, rich; new random layouts),
fitness = minimum; 10 generations; full logging. Test: never-seen worlds of PREREG35, elites 10 × 5, 48 h.
P42a: final elite median k_value > +1 and |k_innate| < 1 (value-steering rediscovered, no innate side preference).
P42b: never-seen worst-case energy ≥ 0.74 (v7c 0.69 + 0.05).
P42c: learned trust in smell > 0 (grand-pattern tally, 4th run).

## Outcome part 1 — evolution (recorded after running; seed 31000, 10 generations, 06:44–08:23 UTC)
Elite min-fitness 0.087 (gen 0: random steering genes make most worms spin in mm-sized circles and starve) → 0.587 (gen 9;
cold 0.614, warm 0.727, rich 0.769). Final elite medians: **k_value +1.82** (value-steering kept; gene–fitness r +0.18),
**k_innate +2.54** (innate warm-seeking steering; r +0.24), steering noise 10^−0.03 ≈ 0.9 rad/s (range maximum; r +0.30 —
the strongest fitness correlate), temperature instinct −2.59 (cold-seeking via turning — opposing the steering), learning rate
10^−2.36 (r −0.15), learned trust in smell −1.76.
**P42a FAIL** (k_value > 1 holds but |k_innate| < 1 fails). **P42c FAIL** (trust in smell negative with learning on — grand-pattern
tally now 3 of 4 runs with learning on). P42b (never-seen worst case ≥ 0.74) pending (queued after PREREG43).
Reading: evolution combined value-steering with an innate warm-steering bias, an opposing cold-seeking drive and maximal
steering wobble (random exploration) — a messier solution than the hand-built "greedy value-steering" and one that again
leans on side-specific instincts.

## Outcome part 2 — never-seen worlds (e61, 50 worms, 48 h, energy h24–48)
v8: cold 0.551, warm 0.826, rich 0.779 — worst case **0.551** (< 0.74; v7c 0.690). **P42b FAIL.** Adding evolvable steering made
the organic brain *less* general than v7c: evolution used it for an innate warm-steering bias plus maximal wobble.
