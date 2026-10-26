# PREREG41 — relational actions for imagination (steer towards / straight / away from the warmer side)
Written 25 Oct 2026 ~23:40 UTC before any run. As PREREG40 (stable labels) except the imagine brain's actions are relational:
a ∈ {towards warmer side, straight, away from warmer side}; transitions are counted per relational action; the chosen
relational action is converted to a steer using the current stable lateral label (level → straight). Greedy brain unchanged.
Worlds as before; seeds 30000+.
P41a: never-seen cold world: imagine (relational) ≥ greedy − 0.03.
P41b: in both worlds, imagine (relational) ≥ imagine (PREREG40, absolute actions) + 0.05.

## Outcome (recorded after running; seeds 30000–30011). The script's printed labels "P40a/P40b" correspond to P41b/P41a.
Hardland h48–72: greedy 0.625, imagine (relational) 0.565 (PREREG40 absolute: 0.546). Never-seen cold h24–48: greedy 0.606,
imagine 0.493 (PREREG40: 0.501). **P41a FAIL** (0.493 vs 0.606), **P41b FAIL** (+0.019, −0.008). Greedy value-steering
replicated a fifth time (0.61–0.63). Four imagination variants (clock, event, concept-only, steering absolute/relational with
and without stable labels) have not beaten one-step greedy steering.
