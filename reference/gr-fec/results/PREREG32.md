# PREREG32 — learned trust in instinct + curiosity from the novelty gate (worm v6), harsh world
Written 24 Oct 2026 before any run. Brain = v5 plus:
- **Trust (hyperedge instinct × learned value):** instinct for cue c ∈ {odour, salt} is multiplied by
  t_c = clip(1 + 2·(w_c + h·w̃_c), 0, 1.5), where w_c, w̃_c are the learned (plain and hunger-gated) weights of that cue's
  intensity node and h the sensed hunger. Gut-stretch instinct unchanged.
- **Curiosity:** novelty trace ν += number of concept nodes born this step; ν decays with τ 300 s; drive D += 1.0·ν.
Constants (2, 1.5, 300 s, 1.0) fixed now, not tuned.
Conditions in Hardland (PREREG31 world, 72 h, 100 worms, seeds 18000+): T = trust only, C = curiosity only, TC = both.
Safety check: TC in Agarland (PREREG29 world, 48 h) vs its own instinct-only twin F (same seeds offset).
Reference values from PREREG31 (other seeds): L5 0.404, F 0.277, R 0.369 energy h48–72; L5 odourless share 0.027.
P32a: TC energy h48–72 ≥ 0.454 (L5 + 0.05) and ≥ 0.419 (R + 0.05).
P32b: TC odourless-food share h48–72 ≥ 0.054 (2 × L5).
P32c (descriptive): T and C each between L5 and TC.
P32d (safety): in Agarland, TC energy h24–48 ≥ F − 0.02.

## Outcome (recorded after running; seeds 18000–18004)
Hardland energy h48–72: **T (trust only) 0.607**, TC (trust + curiosity) 0.572, C (curiosity only) 0.406
(references from PREREG31: L5 0.404, R 0.369, F 0.277). Odourless-food share: T 0.115, TC 0.095, C 0.021 (L5 0.027).
Cool-band share at h72: T 0.41, TC 0.37, C 0.09.
**P32a PASS** (TC 0.572 ≥ 0.454). **P32b PASS** (0.095 ≥ 0.054). **P32c**: C ≈ L5 (curiosity alone adds nothing); T above TC
(curiosity slightly *hurts* in combination) — the descriptive expectation "T and C between L5 and TC" was wrong for T.
**P32d PASS** (rich world: TC 0.854 vs instinct-only 0.858 — trust and curiosity do not harm a good life).
Exams: T clones' odour CI fell 0.85 → 0.64 (trust in the smell lowered); thermal exam unchanged (20.2 → 20.1 °C) although
41 % of living T worms reside in the cold band — the lab exam does not capture what they learned (to investigate).
Caveat: L5/F/R references come from PREREG31's seeds, not re-run with v6 seeds.
