# PREREG30 — a worm's life v5: instinct and learning in separate pathways
Written 24 Oct 2026 before any run. Everything as PREREG29 (world, 48 h life, 100 worms, exams on clones, conditions
L / F / R) except one brain change:
- **Instinct pathway (genome, fixed):** I = 2.354·odour + 0.706·salt (intensity) + 1.0·gut stretch. Never corrected by learning.
- **Learning pathway:** P = W·F over all learned features (tuned nodes, intensity nodes — now starting at weight 0 —
  and hunger-gated copies); error = r − P with r = 4·pharynx − hunger (sensed). The instinct is NOT part of the error.
- Behaviour: drive D = I + P → leaky integrator (τ 10 s) → trend → pirouette rule (s0 0.01, τ 0.3), as v4b.
Note (recorded before running): the "residual" alternative, error = r − (P + I), is the v4b arrangement that failed in
PREREG29. Fresh seeds 16000–16002.
Predictions (same thresholds as PREREG29): P30a energy h24–48: L > F > R. P30b thermal exam h48 ≥ h0 + 1.5 °C.
P30c salt CI h48 ≤ h0 − 0.2. P30d nodes added h24–48 < ½ of h0–24.

## Outcome (recorded after running; seeds 16000–16002). First launch crashed on an initialisation-order bug (no
results produced); fixed and re-run with the same seeds.
P30a FAIL: energy h24–48 L 0.854 ≈ F 0.860 > R 0.490 (learning no longer hurts, but does not help).
P30b FAIL: thermal exam 20.15 → 18.77 °C. P30c FAIL: salt CI 0.80 → 0.70. P30d PASS: nodes +17.7 then +1.5.
Living position: L stays on the warm side with the food (mean x +22 mm, like F) — the v4b drift to the cold side is gone.
Forensics: learned temperature values stay within ±0.15 (v4b: down to −1.0); salt values −0.16 at most vs instinct +0.706.
Energy sits at ≈ 0.85 all life: the worm is permanently well fed, so sensed reward (4·pharynx − hunger) is ≈ 0 everywhere
and there is almost nothing to learn. The world is too rich; instinct alone suffices.
