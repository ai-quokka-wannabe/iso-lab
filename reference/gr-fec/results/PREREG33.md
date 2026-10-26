# PREREG33 — day-2 steps for worm v6 (written 24 Oct 2026 before any run; budget 1 h)
Brain/body = v6 (`src/embodied.py`, separate instinct, integ 10, s0 0.01). Nothing tuned. 100 worms per condition.

(i) **Replication in one batch (Hardland, 72 h, seeds 19000+):** F instinct-only, R random walker (L5's mean turn rate),
L5 learner, T trust (k 2), TC trust + curiosity. P33a: T > L5 + 0.1 and T > R + 0.1 in energy h48–72; T odourless share ≥ 2 × L5.
(ii) **Reading their minds:** for every learning condition log, every 4 h, the learned value of temperature (15–25 °C) and
odour (0–1) and the mean trust factors t_odour, t_salt. Descriptive; question: why no cold preference in the T thermal exam?
(iii) **Better exploration — hunger-driven roaming (TR = T + roaming):** turn probability × 1/(1 + 3·h·(1 − gut stretch))
(hungry, empty-gutted worms make longer runs, as real worms roam). Constant 3 fixed now. P33b: TR energy h48–72 ≥ T.
(iv) **Lab assays on v6 (T):** A1–A5 with PREREG22 criteria, s0 fixed at 0.01 (no calibration), fresh seeds 19500+.
P33c: ≥ 4 of 5 pass.
(v) **A world that changes during life:** Hardland for 36 h, then Agarland (rich, honest smells) for 36 h; conditions T and F
(seeds 19600+). P33d: in h60–72 T energy ≥ F − 0.03 (trust recovers), and T's t_odour at h72 > at h36.

## Outcome (recorded after running)
Plan change mid-run (Matej: "don't hardcode things in the brain; let it evolve"): job B (TC, TR roaming, changing world)
was stopped unfinished and replaced by PREREG34 (evolution). Jobs A and C completed.
(i) **P33a PASS** — replication (seeds 19000–19003), energy h48–72: T 0.682, L5 0.426, R 0.387, F 0.292; odourless share
T 0.158 vs L5 0.034 (R 0.061).
(ii) Minds of T worms: trust in smell 0.70 (h4) → 0.55 (h72), salt ≈ 0.9; learned temperature value: peak +0.23/+0.19 at
16–17 °C (the cold food band), valley −0.34…−0.39 at 18–21 °C (the barren middle), gentler towards warm. The lab thermal
exam starts worms at 20 °C — the bottom of their learned valley, where the slope is shallow both ways — explaining the null
thermal exam: they learned a map of places, and the exam starts them in its worst spot.
(iii) Roaming (TR) — not run (job B stopped). (v) Changing world — not run (job B stopped).
(iv) **P33c PASS** — lab assays on v6 trust (s0 0.01, no calibration): A1 CI +0.84; A2 +3.88 °C; A3 mock +0.69 / salt −0.18;
A4 0.92 → 0.18 → 0.81; A5 ×5.61 → 5/5.
