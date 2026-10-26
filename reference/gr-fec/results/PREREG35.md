# PREREG35 — organic brain v7: the architecture moves into the genome
Written 25 Oct 2026 before any run. Motivation (Matej): less designed/hard-coded, more organically grown.

## What moves from design into genes
The brain treats all 9 input channels alike — 8 senses (odour, salt, N, temperature, touch, pharynx, gut stretch, hunger
hormone) plus its own novelty trace — with temperature rescaled to (T − 20)/5. **Per channel (6 genes × 9):** grows concept
nodes or not, node width, event threshold; instinct gain (any sign); trust strength (instinct × learned value, any sign);
**reward weight** (what feels good or bad is evolved — nothing is designated "food" or "hunger"); turning bias.
**Global genes:** learning rate, how strongly the learned prediction enters the drive (any sign), integration time,
trend lag (1–20 s), turning sensitivity, exploration temperature, maximum turn rate, novelty-trace decay, and **two gate
channels** (which senses gate others as hyperedges; discrete genes, each switchable off).
Previous designed choices that are now evolvable: instinct only on smell/salt/gut; reward = eat − hunger; gating only by
hunger; trust only on smell/salt; roaming only via hunger × empty gut; curiosity only into the drive; nodes only on
chemicals and temperature; fixed widths.

## What remains designed (the substrate)
Senses and body; node growth by a novelty gate; prediction-error learning; the multiplicative trust form; drive → leaky
integration → trend → sigmoid turning; touch reflex with depression.

## Evolution
100 genomes (random start, all ranges include off and either sign), 24 h lives, 2 worker processes. **Worlds randomised
every generation** so a fixed map cannot be memorised: even generations a random harsh layout (odourless food band on a
randomly chosen side — cold *or* warm; smelly food on the other side; 3 decoy smells and 2 salt pans at random places),
odd generations a random rich layout (6 patches on a random side, 2 on the other). Selection by lifetime energy only; top
20 survive, 4 mutated children each; discrete gate genes reassigned with probability 0.1. 30 generations (time-boxed).
Seed 22000.

## Test in never-seen worlds (seeds 23000+)
Two new harsh layouts (food band cold; food band warm) and one new rich layout, 48 h, energy h24–48. Brains: organic v7
elites (20 × 5), hand-built v6 trust, e45 evolved-parameter elites, random walker (v6's mean turn rate).
P35a: v7 elite mean fitness, last two generations > first two generations.
P35b: in both new harsh layouts, v7 energy > hand-built v6 energy.
P35c: in the warm-band layout, v7 energy ≥ e45 evolved-parameter worms + 0.05 (e45 evolved only in a cold-band world).
P35d (descriptive): what evolution keeps — signs of reward weights, gate channels, trust channels, instincts, node growth.

## Amendment before the evolution run (smoke test only, no fitness seen)
(1) Random genomes with large learning rates made the delta rule diverge (overflow/NaN). Learning is changed to normalised
LMS: W += η·error·F / (1 + |F|²) — a stability property of the substrate (added to "what remains designed"); η range unchanged.
(2) Speed: ≈ 3.6 min per generation on 2 workers → 20 generations instead of 30 (same as PREREG34).

## Outcome part 1 — evolution (recorded after running; seeds 22000+)
**P35a PASS:** elite mean fitness first two generations 0.762 (harsh) / 0.799 (rich) → last two 0.851 / 0.893.
**Unplanned imbalance:** the random layout generator put the harsh-world food band on the warm side in 8 of 10 harsh
generations (cold side only in gens 0 and 10). Gen 10 (cold) elite fitness crashed to 0.601.
P35d (what evolution kept, elite medians gen 19): temperature instinct −2.02 → **+2.33** (largest move: "go warm"),
temperature reward +2.21, smell instinct +2.57 with trust +3.68 (learned trust kept), stretch instinct −2.67, learning rate
10^−2.47 → **10^−4.16** (learning nearly switched off), integration 43 s, gate channels rarely on (9 of 40 slots: pharynx 5,
salt 3, temperature 1). Evolution statistics (`experiments/a50_evolution_stats.py`): 90 % of the harsh-world gain by gen 4;
84 % of genes still above half their initial diversity (largely neutral, wasted search).

## Outcome part 2 — never-seen worlds (seeds 23000–23002, 50 worms each, energy h24–48)
| layout | organic v7 | hand-built v6 | e45 evolved numbers | random |
|---|---|---|---|---|
| harsh, food band cold | 0.442 | 0.431 | **0.836** | 0.379 |
| harsh, food band warm | 0.816 | 0.810 | **0.828** | 0.345 |
| rich, new | 0.843 | 0.890 | **0.892** | 0.427 |
**P35b PASS** only nominally (v7 beats v6 by 0.011 and 0.006 — not meaningful). **P35c FAIL** (v7 0.816 vs e45 0.828 warm).
The e45 brains (fixed v6 architecture, evolved numbers, learning rate at its maximum, trained in a fixed cold-band world)
generalise to both unseen bands; v7 memorised "go warm" (cold band 0.442 ≈ hand-built).
Mind printout of a v7 elite after 24 h in the cold world (`results/a49_mind_v7_cold.log`): instinct temperature +3.0 (go warm),
reward "hunger feels good" +5.0 (a neutral, drifted gene — learning was nearly off), energy 0.00 (starved). Interpretability
made the failure legible.
