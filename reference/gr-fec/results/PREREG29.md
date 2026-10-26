# PREREG29 — a worm's life: let it live and watch its mind develop (no lab, no calibration)
Written 24 Oct 2026 before any run of e40. Brain and body exactly as embodied v4b (src/embodied.py, integ 10 s,
s0 0.01, hyperedges on); nothing is tuned. The brain still receives only the sensory vector y.

## World ("Agarland", src/lifeworld.py)
Arena radius 50 mm; temperature T = 20 + 5·x/50 (15 °C west, 25 °C east). 8 bacterial patches (radius 6 mm): 6 on the warm
side (x ≈ +15…+40, ≈ 22–24 °C), 2 on the cool side (x ≈ −25…−35). Each patch has food F ∈ [0, 1] per worm-world: eaten
F −= intake/30 per s, regrowth F += (1 − F)/7200 per s. Patch odour = Σ F·exp(−d²/2·10²), clipped at 1. 3 salt pans
(σ 8 mm, peak 1) in barren places with no food. Each worm lives in its own copy of the world (same layout).
100 worms per condition, born at the centre (20 °C), body state as a newborn (gut 0, energy 0.6). Life = 48 h.

## Conditions
L learning worm (v4b genome); F no-learning twin (η = 0); R random walker (same mean turn rate as L).

## Observed during life (every 2 h)
Mean energy, share of time on food, nodes per modality, the brain's learned value as a function of temperature and of
salt level (read from its own weights), mean turn rate.

## Exams on clones (hours 0, 6, 24, 48): copies are taken to standard lab plates; the living worms continue untouched
Thermal plate (radius 25 mm, 15–25 °C) 20 min → final temperature; salt chemotaxis plate → CI; odour chemotaxis plate → CI.

## Predictions
P29a: mean energy over hours 24–48: L > F > R.
P29b: thermal exam at 48 h: L clones end ≥ 1.5 °C warmer than newborn (hour 0) clones (food lives in the warm zone).
P29c: salt exam at 48 h: L clones' salt CI lower than newborn's by ≥ 0.2 (salt pans are barren).
P29d: node growth slows: nodes added in hours 24–48 < half of those added in hours 0–24.

## Outcome (recorded after running; seeds 15000–15002; forensic seed 15100)
P29a FAIL: energy h24–48 L 0.757, **F (no learning) 0.858**, R 0.499 — learning made the worms *worse*.
P29b FAIL: thermal exam h48 17.55 °C vs newborn 20.10 °C — learning worms prefer the *cool* side (food lives on the warm side).
P29c FAIL (direction right): salt CI 0.40 → 0.27 (needed −0.2). P29d PASS: nodes +22.9 (h0–24), +2.0 (h24–48).
Forensics: the learned value of temperature is most negative at 22–23 °C — the food zone. Cause: the innate odour edge
(born 2.35) sits inside the learned prediction; near patches the odour saturates, so the smell "promises" ≈ 1.7–2.4 while a
sated meal delivers ≈ 0.6. The negative prediction error is absorbed by the co-active temperature nodes: the warm zone is
learned as *disappointing* (Rescorla–Wagner conditioned inhibition). Worms then drift to the cool side (78 % by h12),
where the two cool patches are depleted (0.83) while warm patches stay full (0.98).
Lesson: instinct and learning must be separate pathways — an innate drive must not sit in the pool that prediction
errors correct.
