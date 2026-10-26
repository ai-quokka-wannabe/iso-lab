# PREREG22 — worm-level hypergraph agent, judged by classic C. elegans assays
Written 24 Oct 2026, 10:25 UTC, before any agent code was run.

## World
Circular plate, radius 25 mm. Worm: point, speed 0.2 mm/s, time step 1 s, heading noise σ = 0.05 rad/step,
reflecting edge. Fields: odour (attractant, Gaussian peak 1 at (15, 0), σ 12 mm); salt (same shape, used in the salt
assay); temperature T(x) = 20 + 5·x/25 °C on the thermal plate, 20 °C elsewhere. Sensor noise: odour/salt σ 0.01,
temperature σ 0.05 °C.

## Agent (the hypergraph)
- **Nodes:** tuned sensory nodes (odour 8, salt 8 Gaussian bins over 0–1, width 0.15; temperature 11 bins centred 15…25 °C,
  width 1 °C); internal nodes: food-memory trace m (1 on food, decays τ = 300 s), hunger h = 1 − exp(−t_off_food/1800 s);
  valence node V; motor nodes forward / reverse / turn; touch node.
- **Innate edges (genome):** odour bins → V with weight = bin centre (attractant); salt bins → V with 0.3 × bin centre
  (mild naive attraction); temperature bins → V = 0; touch → reverse = 0.9.
- **Learned edges:** sensory → V by prediction error (Rescorla–Wagner): W += η (v − W·s) s, η = 5e-4 per s, with actual
  valence v = +1 on food, else −h. Learning is on during all phases.
- **Thinking:** drive D(t) = W·s(t) + m(t). Trend dD = D(t) − D(t − 4 s).
- **Acting:** probability of a pirouette (reversal + new random heading) per step
  P = 0.3 · sigmoid((−dD/s0 − 1)/τ), τ = exploration temperature (default 0.3).
- **Touch:** tap → reverse with probability equal to the touch edge weight; each tap depresses it by 12 %; it recovers
  towards 0.9 with τ_rec = 300 s (built-in plasticity).
- **Calibration (only free parameter):** s0 ∈ {0.003, 0.01, 0.03} chosen by odour chemotaxis index on calibration seed
  2200, then frozen. All assays use fresh seeds ≥ 2201. 100 worms per condition.

## Assays and pass criteria
Chemotaxis index CI = (N_target − N_control)/N_total, counting worms within 8 mm of the target (15, 0) or control
(−15, 0) spot at the end (20 min).
- **A1 odour chemotaxis (innate):** CI ≥ 0.3; random-walk baseline |CI| < 0.15.
- **A2 thermotaxis memory:** cultivate 2 h on food at T_c = 15 or 25 °C, then 20 min on the thermal plate without food.
  Pass: mean final temperature (T_c = 25 group) − (T_c = 15 group) ≥ 3 °C. *Not coded specifically — must emerge.*
- **A3 salt associative learning:** condition 1 h without food either in uniform salt (0.8) or without salt (mock), then
  salt chemotaxis. Pass: CI(mock) ≥ 0.15 and CI(salt-starved) ≤ CI(mock) − 0.3. *Not coded specifically.*
- **A4 habituation:** 30 taps at 10 s intervals, then 10 min rest, one tap. Pass: reversal rate tap 30 ≤ 50 % of tap 1;
  after rest ≥ 80 % of tap 1. *Built-in plasticity — a sanity check, not a discovery.*
- **A5 foraging (area-restricted search):** 30 min on food, then moved to an empty plate. Pass: pirouette rate in
  minutes 0–5 ≥ 2 × minutes 15–20.
- **A6 ablation — no learning (η = 0):** must FAIL A2 and A3 (shows the learned edges do the work).
- **A7 baseline — random walker** with the agent's mean pirouette rate: must fail A1–A3, A5.
- **A8 exploration temperature (edge-of-chaos hunch):** A1 CI at τ ∈ {0.03, 0.1, 0.3, 1, 3, 10}. Prediction: the best CI
  is at an interior τ (not 0.03, not 10).
- **Exploratory:** (i) worms cultivated at 20 °C *without* food: do they move away from 20 °C (as starved real worms do,
  Mohri et al. 2005)? (ii) Kato-style state sequence not tested in v0.

## Honesty note
Sensory tuning, the valence node, the motor programme and the learning rule are designed in (like a genome). The
claim tested is that one mechanism — predict valence over a learned concept graph, turn when the prediction falls —
reproduces several distinct worm behaviours, two of them (A2, A3) without assay-specific code. Complex edge phases are
not used in v0 (no task here needs order).

## Outcome of v0 (recorded after running; seeds 2201–2311)
A1 PASS (CI +0.67; random walk +0.05). A2 FAIL (diff +2.03 °C < 3; right direction). A3 PASS (mock +0.77, salt-starved
+0.01). A4 PASS (0.90 → 0.20 → 0.74). A5 FAIL (12.4 vs 8.8 pirouettes/min). A6 as predicted (A2 −0.08, A3 +0.62/+0.51).
A7 random walker fails everything. A8 nominal PASS (best τ = 1) but **refuted on replication** (f33, 8 seeds: CI falls
monotonically with τ: 0.753, 0.746, 0.721, 0.586). Exploratory Mohri: starved-at-20 °C worms end farther from 20 °C
(|T−20| 3.99 vs 2.18 fed) — qualitatively like real worms.
Forensics (f33): A2 learned thermal edges ≤ 0.19 and only near 25 °C, because always-on "absence" bins (odour = 0,
salt = 0) overshadow temperature during cultivation; with 60 min 66–72 % of worms reach the home side (diff 4.5 °C).
A5: s0 = 0.003 makes turning noise-driven (10/min); s0 = 0.03 gives ratio 2.97 and ~1.8/min (realistic).

# PREREG23 — worm agent v1 (two fixes, fresh seeds)
Written before running v1.
1. **Rectified senses:** odour and salt bins centred at 0 are removed (activation forced to 0): a sensor does not fire
   for absence.
2. **Calibration on two assays:** s0 ∈ {0.003, 0.01, 0.03, 0.1} chosen on calibration seeds 4000–4001 by maximising
   min(CI_A1 / 0.3, ratio_A5 / 2).
Everything else unchanged. Re-run A1–A8 with seeds 4101+ and the same pass criteria (A8 judged on 8 seeds per τ:
interior best τ with mean CI above both ends by > 2 SE). Thermotaxis stays at 20 min (no re-gridding).
