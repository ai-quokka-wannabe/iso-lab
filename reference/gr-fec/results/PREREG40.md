# PREREG40 — stable concept labels for imagination (hysteresis), imagination vs greedy steering
Written 25 Oct 2026 ~23:20 UTC before any run. Same as PREREG39 except the lateral label: the lateral signal is smoothed by
a leaky average (τ 10 s) and the label changes only with hysteresis (to "left warmer" when the smoothed value > +0.02 °C,
to "right warmer" when < −0.02 °C, to "level" when |value| < 0.005 °C; otherwise unchanged). Applied to both greedy and
imagine brains. Worlds as PREREG39 (Hardland 72 h; never-seen cold band 48 h); seeds 29000+.
P40a: in both worlds, imagine (stable labels) ≥ imagine (PREREG39, flickering labels) + 0.05.
P40b: in the never-seen cold world, imagine ≥ greedy − 0.03 (imagination catches up with one-step greed).

## Outcome (recorded after running; seeds 29000–29011)
Hardland h48–72: greedy 0.623, imagine 0.546 (flickering labels: 0.556). Never-seen cold h24–48: greedy 0.613, imagine 0.501
(flickering: 0.475). **P40a FAIL** (−0.010, +0.026). **P40b FAIL** (0.501 vs 0.613). Printed walks now move coherently across
concepts, so label noise was not the only problem: the transition model pools each steering action's effect over all
headings and cannot represent the local rule "steer towards the warmer side when warm is good", which greedy uses directly.
A noisy multi-step world model loses to a correct one-step rule. Greedy steering replicated a fourth time (≈ 0.61–0.62).
