# PREREG28 — embodied worm (v4): brain sees the world only through simulated senses
Written 24 Oct 2026 before any run of e39. Motivated by Matej: no cheating — hunger etc. must come from simulated biology.

## Architecture (src/embodied.py) — strict interfaces
- **World:** plate radius 25 mm; fields odour, salt, neutral odour N, temperature, bacterial lawn density b(x) ∈ [0, 1];
  taps. Experimenter operations only: move worms between plates (lawn, assay plates), apply taps. Never touches body or brain state.
- **Body (hidden from the brain):** position, heading, speed 0.2 mm/s, heading noise; metabolism per second:
  gut G: + 0.02·b·(1 − G) (eating), − 0.004·G (digestion); energy E: + 0.003·G·(1 − E), − 1/5400 basal.
  Sensory neurons → signal vector y (noisy): odour, salt, N (σ 0.01), temperature (σ 0.05 °C), touch (tap event),
  pharynx = eating rate / 0.02 (σ 0.02), gut stretch = G (σ 0.02), hunger hormone = 1 − E, low-passed τ 120 s (σ 0.02).
  Motor: executes brain commands (pirouette = random new heading; reversal on touch reflex).
- **Brain (sees only y):** grown tuned nodes per chemical/temperature channel (novelty gate, events > 3σ only, widths as
  v2/v3), intensity nodes for odour/salt/N (genome edges 2.354 / 0.706 / 0); hunger-gated copies of all external-sense
  nodes (hyperedges); **reward from the genome:** r = 4·pharynx − hunger (sensed only); prediction-error learning
  (η 5e-4) on external-sense features only (inner senses gate, never predict — v3b lesson); drive D = prediction +
  1.0·gut stretch (innate); pirouette rule as v1 with exploration τ 0.3; touch reflex with depression as before.
  s0 chosen by the PREREG23 two-assay rule over {0.01, 0.03, 0.1} on calibration seeds 13000–13001.

## Lab procedures (realistic)
All worms start well fed: 30 min on a full lawn at 20 °C, moving freely. Cultivation = 2 h on a full lawn at T_c.
Starvation = time on a plate without food. Tests 20 min, learning always on (it cannot be switched off in an animal).
- A1 odour chemotaxis; A2 thermotaxis memory (T_c 15 / 25 °C); A3 salt learning (1 h starvation in salt 0.8 vs mock);
  A4 tap habituation; A5 foraging (from lawn to empty plate). Criteria as PREREG22. **P28a: ≥ 4 of 5 pass.**
- Controls: no-learning brain must fail A2, A3; random-walk brain must fail A1–A3, A5.
- Mohri: 2 h at 20 °C fed (lawn) vs starved (no food), then thermal plate; report |T − 20|.
- **Hunger gating with a neutral odour N (H-AI13, redesigned):** training = 6 rounds of [starve 0–150 min (random) → 10 min
  on a lawn scented with N 0.8]; then fed (30 min lawn) or starved (2 h) → N chemotaxis 10 min. Hyperedge brain vs
  pairwise-only brain. **P28b:** hyper CI(starved) − CI(fed) ≥ 0.15 and > the pairwise difference by ≥ 0.1.
- Also reported: body variables at test start (true hunger, gut) — for the reader only, never given to the brain.

## Outcome of v4 (recorded after running; seeds 13000–13301)
s0 = 0.03. A1 PASS +0.57 (random walk −0.03); A2 FAIL +2.85 °C (needed 3); A3 PASS +0.24 / −0.12; A4 PASS 0.91 → 0.19 → 0.81;
A5 FAIL ×1.42 (5.13 vs 3.60 /min) → **P28a FAIL (3/5)**. Controls as predicted. Mohri |T−20| fed 0.92 vs starved 3.40.
Gating (N odour): hyperedges fed +0.40 → starved +0.52 (diff +0.12); pairwise +0.49 → +0.28 (diff −0.21); difference between
models 0.33 (≈ 3 SE). **P28b FAIL** (hyper diff < 0.15) though the model contrast is as predicted. A coding error
(bool.astype) crashed the first gating run; fixed and re-run with the same seeds.
Forensics: A5 baseline turning 3.6/min comes from sensed gut-stretch noise (σ 0.02) differenced over 4 s (σ ≈ 0.028 ≈ s0);
v1–v3's food memory was noise-free (a hidden cheat). A2: a sated worm's meal is worth ≈ 0.6 (sensed), not 1.0.

# PREREG28b — v4b: brain-side temporal integration
Written before running. One brain change: the drive is passed through a leaky integrator (τ = 10 s) and the trend is
the integrated drive now minus 10 s ago (instead of raw drive minus 4 s ago). Everything else identical; s0
re-calibrated by the same rule over {0.003, 0.01, 0.03} (the integrated trend is smaller); fresh seeds (offset +1000).
Criteria P28a, P28b unchanged.
