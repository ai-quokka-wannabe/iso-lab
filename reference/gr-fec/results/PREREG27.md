# PREREG27 — worm v3: everything learned so far, re-running all worm experiments
Written 24 Oct 2026 before any run of worm v3.

## Model (src/worm3.py)
- **External senses:** odour, salt, temperature. Grown tuned nodes (novelty gate, widths 0.15/0.15/1.0 °C, ≤ 30 per
  modality, created only for events > 3× noise; temperature always an event), innate edge 0. Intensity nodes for odour
  and salt with innate edges 2.354 / 0.706 (gain-matched, PREREG25c).
- **Inner senses (interoception) as graph nodes:** energy e (on food rises (1−e)/600 per s; off food decays with
  τ 1800 s), hunger h = 1 − e; gut fullness g (on food rises (1−g)/120 per s; off food decays τ 300 s). Both sensed with
  noise σ 0.02. Gut fullness has an innate edge +1 to the drive (replaces the v1 food-memory trace).
- **Hyperedges:** every external-sense node also enters gated by sensed hunger (feature × h), with its own learned
  weight (innate 0). Pairwise-only ablation removes these.
- **Reward:** on food v = 0.6 + 0.8·h (eating is worth more when hungry); off food v = −h. Learning as before
  (η = 5e-4, prediction error on all features).
- Drive, pirouette rule, touch habituation, plate and noise levels as v1. s0 chosen by the PREREG23 two-assay rule over
  {0.01, 0.03, 0.1} on calibration seeds 10000–10001. Test seeds 10100+.

## Re-runs (criteria unchanged from the original pre-registrations)
- A1–A5 (PREREG22 criteria). P27a: ≥ 4 of 5 pass.
- A6 no-learning ablation must fail A2 and A3; A7 random walker must fail A1–A3, A5.
- A8 exploration temperature τ sweep, 8 seeds: report (no interior optimum expected, as in L58).
- Mohri: worms starved at 20 °C end farther from 20 °C than fed ones (report).
- Senses-only randomness (PREREG24 design on v3, made deterministic inside): P27b: in both worlds CI(k = 0) < best − 0.1
  and best k interior.

## New: does hunger gating need hyperedges? (H-AI13)
Training: 20 sessions of 5 min on food in uniform salt 0.8, hunger set uniformly at random (0–1) at the start of each
session, separated by 5 min off food without salt. Test: salt chemotaxis (20 min, no food) with hunger set to 0.1 or 0.9
(gut empty). P27c: with hyperedges CI(h = 0.9) − CI(h = 0.1) ≥ 0.15; pairwise-only |difference| < 0.1.
(Reason: an additive hunger term is constant across the plate, so it cannot change gradient-following.)

## Outcome of v3 (recorded after running, seeds 10000–10611)
s0 = 0.03. A1 PASS +0.62; **A2 FAIL −1.11 °C (reversed)**; A3 PASS (+0.42 / −0.18); A4 PASS; A5 FAIL (×1.38) → P27a FAIL
(3/5). A6, A7 as predicted. A8: plateau 0.54–0.59 to τ = 1, then falls (as L58). Mohri: |T−20| fed 2.70, starved 3.27.
P27b FAIL: deterministic v3 needs no noise in W1 (CI +1.00 at k = 0); W2 k = 0 +0.42 vs best +0.46. P27c FAIL: no
hunger effect with or without hyperedges (+0.01 / +0.00).
Forensics: (1) gut fullness, an always-on predictor during feeding, overshadows external cues and, because a sated
meal is worth 0.6 < gut prediction ≈ 1, drives the home-temperature edge negative — the v0 absence-node problem in a
new place. (2) Learning during the gating test lets hungry worms learn "salt is bad" fast (v = −0.9), cancelling the
effect. (3) Smooth intensity coding removes the rugged landscape that made sensor noise useful in v1/PREREG24.

# PREREG27b — worm v3b: inner senses as gates, not predictors
Written before running. Changes: (1) hunger and gut fullness are removed from the additive prediction; they act only
through the hunger-gated copies of external senses (hyperedges) and gut fullness through the innate drive term
(D = prediction + g, as v1's food memory). (2) In the gating test learning is frozen during the 20-min test (the test
measures what was learned, not new learning). Everything else as PREREG27; fresh seeds 11000+; same criteria
P27a, P27c. The senses-only noise test is not repeated (its v3 result stands, with the forensic reading above).
