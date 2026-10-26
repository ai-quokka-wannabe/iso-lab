# PREREG43 — optimise the optimiser: equal-budget race GA vs OPT2 (Matej: "optimise the hell out of the optimiser")
Written 26 Oct 2026 before any race run.
**Speed-up 1 (implementation, verified):** compiled brain step (`src/fastbrain.py`, numba): 2.3× faster for the whole
simulation; equals the original to 1e-16 at step 1, with differences growing chaotically (≈ 1e-9 by step 600) — floating-point
summation order, the same sensitivity measured in the FEC phase; statistically equivalent, not bit-identical.
**Race:** organic v8 genome (evolve3), compiled brain, same initial population (seed 32000), same world schedule (three 24 h
lives per generation: harsh cold, harsh warm, rich; fitness = minimum), same budget = 43,200 worm-hours (= 6 GA generations).
- **GA:** current optimiser — top 20 + 4 children each with fixed Gaussian mutation σ = 10 % of each gene's range; full lives.
- **OPT2:** (a) self-adaptive per-gene mutation (each genome carries its own step sizes, Schwefel log-normal rule);
  (b) successive halving: at hour 6 of each life the worse half *in that world* is culled; survivors live to 24 h (culled
  genomes keep their 6 h score). Saves ≈ 37 % worm-hours per generation → ≈ 9–10 generations in the same budget.
Evaluation: final elites (10 × 5) of each in the never-seen worlds of PREREG35 (48 h).
P43a: OPT2's best elite-mean fitness within the budget ≥ GA's + 0.03.
P43b: never-seen worst-case energy OPT2 ≥ GA.
Also reported: fitness vs worm-hours curves; OPT2's median step size over generations (does it shrink on neutral genes?).

## Outcome (recorded after running; GA 08:23–08:45, OPT2 08:45–09:10 UTC; evals after)
Elite mean fitness vs worm-hours — GA: 0.153, 0.219, 0.244, 0.384, 0.538, **0.557** (43,200). OPT2 (4,500 worm-hours per
generation, 37.5 % saving): 0.174, 0.204, 0.297, 0.309, 0.418, 0.388, 0.447, **0.531**, 0.516, 0.485 (45,000).
**P43a FAIL** (best OPT2 0.531 < GA 0.557 + 0.03). Self-adaptive step sizes barely moved (median 0.100 → 0.095).
Never-seen worst case: GA 0.498 (cold 0.574, warm 0.620, rich 0.498); OPT2 0.513 (0.687, 0.549, 0.513). **P43b PASS**
(marginal, 0.513 ≥ 0.498).
Caveats: every generation draws new world layouts, so fitness at equal worm-hours is confounded by layout difficulty; one run
each. Lessons for the optimiser: (1) the 2.3× implementation speed-up is real and free; (2) successive halving saves compute
but its extra generations did not convert into fitness here — noisy single-life evaluation at 6 h is a weak predictor;
(3) self-adaptation needs many more generations (or CMA-ES-style covariance learning) to pay; (4) to compare optimisers
cleanly, evaluate all candidates on a *fixed* panel of layouts.
