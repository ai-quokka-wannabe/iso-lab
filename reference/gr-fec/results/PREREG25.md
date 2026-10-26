# PREREG25 — grown grounding: concept nodes created by a novelty gate (worm v2)
Written 24 Oct 2026 before any run of e36.

Change from v1: no fixed sensor bins. Each modality (odour, salt, temperature) has a pool of up to 30 nodes per worm,
starting empty. Each step, the raw (noisy) reading x of a modality is compared with that modality's prototypes:
- **Novelty gate:** if the nearest prototype is farther than the tuning width (odour/salt 0.15, temperature 1.0 °C —
  the v1 widths) a new node is created at x, with the genome's innate edge to valence (odour: x; salt: 0.3·x;
  temperature: 0).
- **Events, not silence:** odour/salt nodes are only created when the reading exceeds 3 × its sensor-noise sd
  (temperature is always an event).
- Otherwise the nearest prototype moves towards x by 0.005·(x − prototype).
- Activation of each node: Gaussian of distance with the tuning width; edges learned exactly as in v1.
All other settings as v1 (s0 = 0.03, τ = 0.3, η = 5e-4). Assays A1–A5 with PREREG22 criteria, fresh seeds 8100+.
P25: ≥ 4 of A1–A5 pass. S25: A2 (thermotaxis memory) and A3 (salt learning) both pass — the learned behaviours survive
when the concepts themselves are learned. Reported: nodes created per modality.

## Outcome (recorded after running, seeds 8101–8150)
A1 FAIL (CI +0.05), A2 PASS (+6.75 °C, better than v1's +4.34), A3 FAIL (mock +0.16, salt −0.08), A4 PASS, A5 PASS
(×2.43). P25 FAIL (3/5), S25 FAIL. Cause: grown nodes are *tuned* ("this much odour"); the first odour node is born at the
starting concentration, so climbing the gradient lowers its activation and the innate "more is good" prior is lost.
Real chemosensory neurons (e.g. AWC) code intensity monotonically; thermosensory AFD is tuned near the remembered
temperature. Nodes created: ~1 odour, ~1 salt, ~6.5 temperature per worm.

# PREREG25b — two kinds of node
Written before running. Each chemical modality (odour, salt) gets one innate **intensity node** (activation = reading,
clipped at 0; innate edge 1.0 for odour, 0.3 for salt) *plus* grown tuned nodes as in PREREG25 but with innate edge 0
(learned only). Temperature: grown tuned nodes only (as PREREG25). Everything else unchanged; fresh seeds 8200+.
P25b: ≥ 4 of A1–A5 pass; S25b: A2 and A3 both pass.

## Outcome of 25b (seeds 8201–8250)
A1 PASS (+0.31, barely), A2 PASS (+6.74 °C), A3 FAIL (mock +0.10, salt-starved −0.12), A4 PASS, A5 FAIL (×1.97). 3/5.
Forensic: v1's overlapping bins gave an effective innate odour slope of 2.354 (salt 0.706); 25b's intensity nodes had
1.0 / 0.3 — a 2.35× weaker genome, so the comparison was not gain-matched.

# PREREG25c — gain-matched two kinds of node (final iteration of this line)
Written before running. As 25b, with intensity-node innate edges 2.354 (odour) and 0.706 (salt), measured from v1.
Fresh seeds 8300+. Same criteria. Whatever the outcome, this line stops here for this sprint.
