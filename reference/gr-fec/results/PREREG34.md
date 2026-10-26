# PREREG34 — evolve the brain's numbers instead of hand-picking them (Matej: "make it evolve organically")
Written 24 Oct 2026 ~14:06 UTC before any run.
**Still designed (architecture = the building blocks):** senses and body (as v4–v6), concept nodes grown by the novelty gate,
prediction-error learning, hunger-gated hyperedges, instinct pathway separate from learning, trust/roam/curiosity mechanisms,
pirouette rule, touch reflex.
**Evolved (12 genes, per worm, initial values uniform over wide ranges that include "off" and the wrong sign):**
g_odour, g_salt ∈ [−3, 3] (instinct gains); g_gut ∈ [−2, 2]; reward weights r_eat, r_hunger ∈ [−5, 5] (r = r_eat·pharynx −
r_hunger·hunger); trust_k ∈ [−5, 5]; roam ∈ [0, 5]; curiosity ∈ [0, 2]; log10 η ∈ [−5, −2]; log10 s0 ∈ [−3, −1];
log10 τ ∈ [−1.5, 1]; integration time ∈ [1, 60] s.
**Evolution:** 100 genomes, one worm each, a 24 h life per generation from newborn state; worlds alternate Hardland (even
generations) / Agarland (odd). Fitness = mean energy over life (sampled every 10 min). Top 20 survive unchanged and each has
4 mutated children (Gaussian, σ = 10 % of the gene's range, clipped). 20 generations (time-boxed). Seed 20000.
No lab assay is used anywhere in selection.
**Questions (descriptive, no pass/fail):** do elites evolve positive smell instinct, eat-is-good / hunger-is-bad rewards,
positive trust, roaming, curiosity? How do evolved values compare with my hand-picked v6 (g_odour 2.354, g_salt 0.706,
r 4/1, trust 2, roam 0/3, curiosity 0, η 5e-4, s0 0.01, τ 0.3, integ 10)?
**P34 (one prediction):** the elite mean fitness in the last two generations exceeds the first two generations' elite mean in
the same worlds.

## Outcome (recorded after running; ~14:02–14:26 evolution, 14:26–14:32 evaluation)
**P34 PASS:** elite mean fitness gen 0/1 → 18/19: harsh 0.705 → 0.874, rich 0.848 → 0.895. Population mean harsh 0.484 → 0.798.
Final elite medians (started uniform random): g_odour 1.72 (hand 2.35), g_salt −0.01 (hand 0.71), g_gut −1.55 (hand +1),
r_eat 4.45 (hand 4), r_hunger 3.97 (hand 1), **trust_k 2.92 (hand 2)**, roam 1.47 (was ≈ 4.5 for gens 3–18), curiosity 1.30,
η 0.01 (**upper range limit**), s0 0.020, τ 0.044, integration 1 s (**lower range limit** = no integration).
**Fair evaluation (e46, same protocol as PREREG33 (i), 20 elites × 5 worms, seeds 21000–21001):** Hardland energy h48–72
**0.773** (hand-built v6 0.682), odourless share **0.359** (0.158), cold-band share at end 0.68; Agarland h24–48 **0.892**
(hand-built 0.854, instinct-only 0.858).
Caveats: one evolutionary run; worlds had a fixed layout every generation, so the evaluation is in-distribution (no novel
world tested yet); two genes pinned at range limits (the ranges constrained evolution); evolution's own fitness (24 h
lifetime mean) differs from the 72 h evaluation metric.
