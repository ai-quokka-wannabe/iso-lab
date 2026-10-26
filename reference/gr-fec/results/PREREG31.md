# PREREG31 — a worm's life in a harsh world where instinct is not enough
Written 24 Oct 2026 before any run. Brain/body exactly v5 (embodied, instinct pathway separate, integ 10 s, s0 0.01,
hyperedges). Nothing tuned. Brain sees only y. 100 worms per condition, born at the centre (20 °C), life 72 h; exams on
clones at h0, 24, 48, 72 (thermal, salt, odour lab plates as before).

## World "Hardland" (src/lifeworld.py: HardWorld) — arena radius 50 mm, T = 20 + 5·x/50 °C
- 4 **odourless** lawns (radius 6 mm) in the cool band at x = −35 mm (≈ 16.5 °C), y = −24, −8, 8, 24; eaten F −= intake/30,
  regrowth (1 − F)/7200 per s. They emit no smell.
- 2 **smelly** lawns on the warm side at (25, 15), (30, −20); deplete fast (F −= intake/10), regrow slowly ((1 − F)/21600).
  Odour = F·exp(−d²/2·10²).
- 3 **decoy** smells from empty ground at (35, 0), (15, −35), (10, 30): strength 1, σ 10 mm, no food. Odour clipped at 1.
- 2 barren salt pans at (0, 40), (−5, −42), σ 8 mm.

## Conditions: L learning (v5), F no-learning twin (η = 0), R random walker (L's mean turn rate). Seeds 17000–17002.

## Predictions
P31a: energy h48–72: L ≥ F + 0.05, and F > R.
P31b: thermal exam: L clones at h72 end ≥ 1.5 °C cooler than at h0 (towards the odourless-food band).
P31c: share of time on odourless lawns h48–72: L ≥ 2 × F.
P31d: odour exam h72: L CI ≤ F CI − 0.1 (decoys teach that smell is unreliable).

## Outcome (recorded after running; seeds 17000–17002)
Energy h48–72: L 0.404, F 0.277, **R 0.369**. P31a FAIL only on "F > R": learning beats instinct (+0.127, threshold 0.05
met), but the instinct-only worm is *worse than random* — decoys trap it. P31b FAIL (right direction): thermal exam
19.52 → 18.26 °C (−1.27, needed −1.5). P31c PASS: odourless-food share L 0.027 vs F 0.000 (but R 0.058). P31d FAIL: odour
exam L +0.80 vs F +0.83.
Forensics (learned values read from weights): smell learned as disappointing (learned part −0.45 at full odour) — but the
fixed instinct adds +2.35, so learning cannot override it; cool band learned only weakly positive (+0.06) because learners
rarely go there (cool-band share 0.02 → 0.10; random walker 0.17–0.22). Two bottlenecks: (1) learned value cannot
outweigh a fixed instinct; (2) too little exploration to discover food the instinct does not know.
