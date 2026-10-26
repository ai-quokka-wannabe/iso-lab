# The gr-fec Sandbox: A Worm-Level Mind, Tested

What Matej's research sandbox gr-fec learned about a worm-level mind, written down so the lab keeps all of it. gr-fec
began on 23 October 2026 as a study of the geometry of error-correcting codes and turned, on 24 October, to an "AI
branch": an embodied, senses-only *C. elegans*-level agent with an interpretable concept-hypergraph mind, grown,
tested and evolved in about twenty pre-registered experiments over three days.

**How to read this note.** gr-fec is the maintainer's own work, unpublished, done in a toy world: simulated agar
plates, a few dozen concepts, a hundred worms, one second per step. Every number below is gr-fec's, from its own
ledger and pre-registrations, not the lab's; for the lab each is a lead to reproduce on the Grid, never evidence, as
[the rules](../../.claude/CLAUDE.md) require. The literature gr-fec names is listed as it names it and is unverified
here; check each item at its source before citing it, as [README.md](README.md) says. The ideas this note leaves the
lab are written as hunches in [Matej's ideas and hunches § G](../hypotheses/matej-hunches.md#g-from-the-gr-fec-sandbox).

gr-fec's own honesty, kept: about two in five of its pre-registered predictions passed (34 PASS against 54 FAIL across
22 outcome files, by its own count); two apparent wins were seed luck caught by replication; and its worm models are,
in its own words, close to classic klinokinesis and Rescorla-Wagner models - the contribution is the integration and
the design rules, not a new learning mechanism.

## 1. The Rules It Worked By

- Pre-register every prediction before the run, record PASS or FAIL afterwards, never relabel a verdict.
- Replicate surprising wins on fresh seeds; never reuse a seed block for a new claim.
- Every result needs a referee: ground truth, a Monte Carlo or a null control. Try to break a result before building
  on it. Flag "mirage or real?", including on the maintainer's own ideas.
- **No cheating**: the brain receives information only through simulated senses, World → Body → Brain. Designing the
  genome, body and world with knowledge of the world is allowed.
- Less hard-coding, more organic growth and evolution - **but the mind stays a printable hypergraph**: every concept,
  edge, hyperedge and gene readable by name; no language model or opaque network as the brain.
- A ledger row for every result, labelled confirmed, refuted, null, suggestive or measured.

These are close to the lab's own rules; gr-fec arrived at them independently.

## 2. The Embodied Worm: World, Body, Brain

### 2.1 Worlds

- **Lab plates**: radius 25 mm with a reflecting rim. Odour and salt as static Gaussian bumps (peak 1 at (15, 0) mm,
  σ 12 mm) or uniform; a neutral odour N; a thermal plate with T = 20 + 5·x/25 °C (15 to 25 °C); a bacterial lawn
  b in [0, 1]; optional decoys. No diffusion and no depletion on lab plates.
- **Agarland, the rich world**: radius 50 mm, the same gradient spread over 15 to 25 °C; eight food patches of radius
  6 mm, six of them on the warm side, depleting as they are eaten (by intake/30) and regrowing over about two hours;
  odour from the remaining food (σ 10 mm); three barren salt pans. Each worm has its own copy of the food - no
  competition.
- **Hardland, the harsh world**: four *odourless* lawns in the cool band (about 16.5 °C); two smelly lawns on the warm
  side that deplete fast and regrow over six hours; three decoy smells over empty ground; two salt pans. Here the
  smell instinct is a trap and only learning finds the food.
- **Random layouts**: harsh worlds with the odourless band on a random side, rich worlds with the patches on a random
  side; never-seen test layouts from fixed seeds.

### 2.2 Body

- Constant speed 0.2 mm/s, heading noise σ 0.05 rad/s; a pirouette draws a uniformly random new heading. No slowing
  on food.
- Metabolism, per second: intake = 0.02·b·(1 − G); gut G grows by intake and empties at 0.004·G; energy E gains
  0.003·G·(1 − E) and burns 1/5400; a hunger hormone follows (1 − E) with a 120 s low-pass. Newborns start with G = 0
  and E = 0.6. *Caution*: at that burn a full worm empties in an hour and a half and a newborn in under an hour, far
  faster than a real worm starves; the constants have no source.
- **The senses**, a noisy vector y the brain sees and nothing else:

  | Channel | What it is | Noise σ |
  |---------|------------|---------|
  | y0 | odour | 0.01 |
  | y1 | salt | 0.01 |
  | y2 | neutral odour N | 0.01 |
  | y3 | temperature | 0.05 °C |
  | y4 | touch: 1 on a tap | 0.05 |
  | y5 | pharynx: the rate of eating | 0.02 |
  | y6 | gut stretch: G | 0.02 |
  | y7 | hunger hormone | 0.02 |
  | y8 | temperature 1 mm left of the head minus right (steering bodies only) | 0.02 °C |

  No sensory adaptation in any channel. A steering body also takes a command s in [−1, 1], turning 0.15·s rad/s.

### 2.3 Brain, version by version

- **v0 and v1**: fixed Gaussian bins as concept nodes - eight for odour, eight for salt, eleven for temperature.
  Innate weights rise with concentration. Value learning by Rescorla-Wagner, W += η(v − W·s)s with η = 5e-4; a drive
  D = W·s plus a food memory; a pirouette with probability 0.3·σ((−ΔD/s0 − 1)/τ), where ΔD is the change of the drive
  over 4 s, τ = 0.3 and s0 calibrated. Touch reverses with a probability that falls by 0.88 per tap and recovers over
  five minutes (habituation built in). v1 stopped any node firing for the *absence* of a smell.
- **v2**: concept nodes grown by a novelty gate - a node is born when no prototype is near enough, and only for
  readings above three times the sensor's noise; otherwise the nearest prototype drifts towards the reading. Intensity
  nodes ("how much") added beside tuned nodes ("which value").
- **v3**: interoceptive nodes (energy, hunger, gut fullness), hunger-gated copies of the external features as
  hyperedges, and eating worth more when hungry. v3b used hunger and fullness only as gates.
- **v4 and v4b, the embodied worm**: the earlier cheats removed - the brain had been told "on food", rewarded from
  simulator truth, handed hunger as a formula and held still while "cultivated". Now: tuned pools of up to 30 nodes
  per sense, three intensity nodes, hunger-gated copies; reward wired by the genome to *sensed* eating and hunger,
  r = 4·pharynx − hunger; prediction W·F updated by the error; drive = prediction + gut stretch. v4b added a leaky
  integrator of the drive (τ 10 s) and a 10 s trend.
- **v5**: the instinct (2.354·odour + 0.706·salt) taken out of the learner into a separate fixed pathway that the error
  never corrects.
- **v6**: **learned trust in instinct** - each instinct multiplied by t = clip(1 + k·(learned value of its cue,
  plain and hunger-gated), 0, 1.5), with k = 2 - and curiosity, a novelty trace (τ 5 min) of node births added to the
  drive. Roaming: turning divided by 1 + roam·hunger·(1 − gut stretch).
- **Evolved v6 (e45)**: the v6 architecture with twelve genes over wide ranges that include "off" and the wrong sign
  (smell and salt instinct, gut drive, reward for eating, penalty for hunger, trust, roaming, curiosity, learning rate,
  s0, τ, integration time).
- **Organic v7**: the architecture itself in the genome - nine channels treated alike (eight senses and the brain's own
  novelty), each with genes for node growth, width, event threshold, instinct, trust, *reward* (what feels good is
  evolved) and turning bias, plus global genes and two evolvable gate channels: 73 genes, up to 351 features, learning
  by normalised least mean squares.
- **v8**: v7 plus genes for value-steering, innate lateral steering and steering noise.
- **Steering brains**: a state of the current temperature concept and a left-right label; *greedy* steering compares the
  learned value of the neighbouring warmer and colder concepts every 10 s and steers towards the better side;
  *imagine* runs eight random walks of eight steps over learned event transitions per action and steers by the best.

### 2.4 The assays, as gr-fec scored them

| Assay | Protocol | Pass |
|-------|----------|------|
| A1 odour chemotaxis | 100 worms from the centre, 20 min; CI = (target − control)/total within 8 mm | CI ≥ 0.3, a random walker below 0.15 |
| A2 thermotaxis memory | 2 h on food at 15 or 25 °C, then 20 min on the thermal plate | the two groups ≥ 3 °C apart |
| A3 salt learning | 1 h starved in salt against mock, then a salt CI test | mock ≥ 0.15, salt ≤ mock − 0.3 |
| A4 tap habituation | a tap every 10 s for 300 s, 600 s rest, one tap | tap 30 ≤ half of tap 1; after rest ≥ 0.8 of tap 1 |
| A5 foraging | 30 min on food, then an empty plate | turns per minute in minutes 0-5 ≥ twice those in minutes 15-20 |

Plus controls: a no-learning twin and a random walker; a temperature sweep; and a starvation test after Mohri et al.
2005 (fed against starved worms' distance from 20 °C). The pass bars are gr-fec's own, not fitted to published effect
sizes - unlike the lab, whose benchmark comes from real recordings.

## 3. What Came Out

### 3.1 The lab worm, v0 to v6

| Version | A1 | A2 | A3 | A4 | A5 | Passed |
|---------|----|----|----|----|----|--------|
| v0 | +0.67 | +2.03 °C (fail) | pass | pass | fail | 3 of 5 |
| v1 | +0.61 | +4.34 °C | +0.33 / −0.08 | pass | ×2.7 | 5 of 5 (cheats still in) |
| v3 | +0.62 | −1.11 °C (reversed) | pass | pass | ×1.38 (fail) | 3 of 5 |
| v3b | +0.63 | +3.10 °C | pass | pass | ×1.53 (fail) | 4 of 5 |
| v4 embodied | +0.57 | +2.85 °C (fail) | pass | pass | ×1.42 (fail) | 3 of 5 |
| v4b + integration | +0.74 | +4.07 °C | +0.37 / −0.13 | pass | ×5.82 | **5 of 5, no cheats** |
| v6 trust | +0.84 | +3.88 °C | +0.69 / −0.18 | pass | ×5.6 | 5 of 5, uncalibrated |

Each pass after a failure came one diagnosed fix later, on fresh seeds - forking paths; gr-fec asks for v4b's 5 of 5 to
be replicated before it is relied on.

### 3.2 Lives in worlds

| World, late energy | Learner | No-learning twin | Random walker | Reading |
|--------------------|---------|------------------|---------------|---------|
| Agarland, v4b | 0.757 | **0.858** | 0.499 | learning hurt: the food zone learned as "disappointing" |
| Agarland, v5 | 0.854 | 0.860 | 0.490 | damage gone; a fed worm has nothing to learn |
| Hardland, v5 | 0.404 | 0.277 | 0.369 | learning beats instinct, which is now a trap |
| Hardland, v6 trust | **0.607**, replicated **0.682** | 0.292 | 0.387 | trust lets experience turn the lying smell down |

In Hardland, v6's worms read from their weights had halved their trust in smell and learned a map of places - a peak of
value at 16-17 °C over the odourless food, a valley at 18-21 °C - which is why a lab exam started at 20 °C, the valley
floor, showed no direction. Curiosity as a novelty drive did not help.

### 3.3 Evolution

| Brain | Never-seen cold | warm | rich | Worst case |
|-------|-----------------|------|------|------------|
| e45: designed v6 + evolved numbers | 0.836 | 0.828 | 0.892 | **0.828** |
| organic v7c: architecture in the genome, worst-of-three fitness | 0.690 | 0.703 | 0.780 | 0.690 |
| organic v7b: mean-of-three fitness, 12 h lives | 0.576 | 0.853 | 0.858 | 0.576 |
| organic v8: steering genes | 0.551 | 0.826 | 0.779 | 0.551 |
| hand-built v6 trust | 0.431 | 0.810 | 0.890 | 0.431 |
| random walker | 0.379 | 0.345 | 0.427 | - |

- **e45 beat the hand design in both worlds** (harsh 0.773 against 0.682; odourless-food share 36% against 16%). From
  random starts it rediscovered the smell instinct, "eating is good" and learned trust; it dropped the salt instinct and
  the smoothing; it switched on roaming, curiosity and fast learning. One run, fixed layouts, two genes at their range
  limits.
- **v7 memorised the world schedule**: an unplanned imbalance (eight of ten harsh worlds warm) let it evolve "go warm"
  and turn learning down fiftyfold; it starved in never-seen cold worlds. A printout of one elite's mind showed why: a
  "go warm" instinct of +3.0 and a drifted gene saying hunger feels good (+5.0).
- **v7b** (mean over three balanced lives) again switched learning off. **v7c** (worst of three 24 h lives, learning
  kept on by a floor) made the best organic generalist and found a new trick: a "go warm" instinct with strongly
  negative temperature trust, so experience could override it.
- **v8** given steering built an innate warm bias plus maximal steering noise - "wobble" - and did worse.
- **The recurring motif**: learned trust in smell came out positive in three of four evolutions where learning was on
  (+2.9, +3.7, +3.3; −1.8 in v8), and neutral where learning was off.
- **Statistics of evolution**: 90% of the harsh-world gain by generation 4; 84% of genes still drifting at the end;
  in v7b 16 of 73 genes selection-neutral, the strongest gene-fitness correlations |r| 0.2 to 0.3 (decisive turning,
  eating reward).

### 3.4 Steering and imagination

- Imagination as random walks over learned concept transitions (H-AI14) failed six times: walks stepped by clock time
  stood still; event steps flickered with sensor noise; with no heading sense the worm could not act on what it
  imagined; with stable labels and relational actions it still lost. One apparent win (+0.072) was seed luck, caught by
  a three-seed replication.
- **Greedy value-steering** won: with a head-swing lateral sense, steering towards the neighbouring concept of higher
  learned value gave +0.17, +0.17 and +0.24 over not steering in never-seen cold worlds, replicated five times (about
  0.62 against 0.39-0.45).

### 3.5 Noise

- With the worm deterministic inside and sensor noise its only randomness, an interior noise optimum appeared in both a
  single-peak and a trap world (CI +0.61 at no noise, +0.87 at three times the default; the trap world from −0.06 to
  +0.29) - but a little noise did worse than none: below the turning threshold noise only blurred the orbits.
- With a smooth intensity code a fully deterministic worm reached CI +1.00: noise was needed only for a rugged
  representation.
- No edge-of-chaos optimum in decision temperature (refuted on eight seeds), unlike the decoders.

### 3.6 Complex edges

On a timing task, one complex number per edge, the average of e^(iω·lag), predicted a never-experienced composed delay
as 6.97 against a true 7, with two numbers where a real lag vector needed sixteen; on a bimodal delay it pointed at the
mean and flagged itself with low confidence (|z| 0.35 against 0.90). Efficiency and a free confidence signal, not new
capability; the maintainer decided to keep complex edges where relations have timing.

## 4. The Principles It Drew

gr-fec's own synthesis, with its own status words:

1. **Think by predicting value over a grown concept graph; act when the prediction falls** - solid in the toy world.
2. **Walls come from events, not silence** - three independent failures, three fixes; a design rule.
3. **Instinct and learning are separate pathways** - solid.
4. **Learned trust in instinct is the key hyperedge** - the strongest convergent motif.
5. **Inner senses gate; they do not predict** - solid; hunger gating by hyperedge itself only suggestive.
6. **Honest senses are noisy, so integrate before deciding** - solid as a constraint, open as an optimum.
7. **Noise helps only above the decision threshold, and only on a rugged landscape** - solid.
8. **Learning pays only when the world is unpredictable within a life and instinct is not enough** - solid.
9. **Judge a design by its worst life** - solid.
10. **Designed learning structure with evolved numbers beats evolving everything** - for now.
11. **A body that steers by local comparison beats multi-step imagination at worm level** - solid for gradient worlds.
12. **Planning pays only when the world model is accurate** - solid lesson.
13. **Answer-space density decides which hallucination detector works** - solid.
14. **Interpretability is the debugging tool** - demonstrated repeatedly.
15. **Pre-register, replicate, and expect to be wrong.**
16. **Every new tool given to evolution is first used to cheat** - recurring.
17. **Optimise the optimiser with measurements, not intuition** - a compiled brain gave 2.3 times; textbook upgrades
    did not help at that scale.

gr-fec also compared these with the cognitive architectures and found most rediscovered - Psi theory and MicroPsi,
OpenCog's weighted hypergraph and OpenPsi, Soar and ACT-R utility learning, Dyna's model-based planning and its failure
mode, uncertainty-based arbitration between controllers (Daw, Niv and Dayan 2005), the Baldwin effect (Hinton and
Nowlan 1987) - with "walls from events" the one it could not find stated as a rule. It noted its citations there were
from memory.

## 5. Refuted or Weakened - Not to Be Re-Claimed

The gravity gate on a sparse memory; a two-wall detector trained on rare confusions; an edge-of-chaos optimum in
decision temperature; a universal benefit of sensor noise; inner senses as predictors; curiosity as a novelty drive;
imagination as random walks (six variants); self-adaptive mutation and successive halving at that scale; a "quantum"
mind, kept as metaphor only.

## 6. Problems to Avoid, Found in gr-fec

Flagged by gr-fec itself: the noise-free food memory of v0 to v3 was a hidden cheat; a pass refuted on replication
(the temperature sweep), a win that was seed luck (imagination), a pass that was only nominal (v7 in never-seen worlds);
an unbalanced random world generator; per-generation layouts confounding fitness comparisons; two runs crashed on bugs
and re-run with the same seeds; swapped printed labels in one script; habituation built in, so its pass is a sanity
check, not a discovery; turning three to five times a real worm's rate; no slowing on food, no reversal-then-omega
sequence, no sensory adaptation, no quiescence.

Found on reading its code, not flagged there: one evaluation script seeds its body generator with Python's string
hash, which is randomised per process, so those runs are not reproducible without a fixed hash seed; the evolution
splits each population between two workers with different body seeds, so a genome's score depends on its half; and the
metabolism constants starve a newborn in under an hour.

## 7. The Other Half: What the Decoders Taught

gr-fec's first phase, on decoding error-correcting codes, left ideas that bear on brains:

- **Decoder temperature has two failure phases**: too stiff, it freezes at a confident wrong answer; too soft, it
  wanders chaotically. Error rates were lowest just on the soft side of the onset of chaos (0.96 to 1.42 times the
  onset in 11 of 13 sweeps), and the phase picture collapsed onto one curve when the knob was divided by its own onset,
  a post hoc finding. "Oscillation is search, freezing is death." The worm did not show it in decision temperature.
- **The gravity gate**: accept an answer only when its margin to the runner-up is large against the cost of the answer
  itself (the ratio of the two) - after Forney 1968. On decoders it let no wrong answer through; on a small language
  model's factual answers it was the best detector, modestly; on a sparse memory it failed, because there errors are
  novelty rather than confusion. A candidate "I am not sure" gate for a brain's decisions.
- **Exploration without information loss**: redrawing the decoder's update order each iteration helped short codes that
  are search-limited and hurt long ones that are iteration-limited.
- **The portability test**: the frozen-chaotic picture held across three codes and three decoder implementations, one of
  them quantised hardware, so it belongs to the abstract layer; random update order did not survive quantisation, so it
  belongs to the physical one.
- **Measurement, not steering**: geometry worked as an instrument and failed as a control - curvature of the code's
  graph predicted nothing, and an exploratory correlation of −0.89 became +0.17 on a fresh batch.

## 8. What gr-fec Hands On

Its prioritised open threads, as it left them:

1. A fixed evaluation panel - several fixed layouts per world type with common random numbers - for every comparison.
2. Evolve on top of the best structure (e45 with greedy value-steering), judged by the worst of several lives, then
   release designed parts into the genome step by step.
3. Covariance-matrix adaptation on the e45 genome; freeze the neutral quarter of genes; cull on several short lives.
4. Imagination only where one-step greed must fail - detours, barriers, valleys to cross.
5. The grand-pattern tally: three or more independent evolutions, counting recurring motifs.
6. Realism: turning rates, slowing on food, reversal-then-omega, sensory adaptation, quiescence.
7. Compare internal state flow with whole-brain imaging (Kato et al. 2015), then the fruit fly.
8. Unrun: frozen-chaotic phases of a language model's generation, hunger gating with more power, a noise gene per stage.

For the lab, the differences matter as much as the overlap: the Grid's body is eight rigid segments with servos,
not a point with a heading; the lab's benchmark comes from real recordings, not from pass bars; its brains run at the
worm's pace in simulated time; and its code is Rust. gr-fec's worm is best read as a design to rebuild and test on the
Grid, its numbers as predictions to check there.

## Leads Named by gr-fec (Unverified)

Biology: Hedgecock and Russell 1975; Saeki et al. 2001; Mohri et al. 2005; Zhang, Lu and Bargmann 2005;
Pierce-Shimomura et al. 1999; Kato et al. 2015; White et al. 1986; Cook et al. 2019; Douglass et al. 1993; Priplata et
al. 2003; Faisal, Selen and Wolpert 2008; O'Keefe and Recce 1993; Fiser, Berkes, Orbán and Lengyel 2010; Ginsburg and
Jablonka 2019; the FlyWire connectome 2024; a 2024 brain-body-environment worm model from Beijing (gr-fec: "verify
citation").

Learning and cognition: Rescorla and Wagner 1972; Daw, Niv and Dayan 2005; Sutton 1990; Hinton and Nowlan 1987;
Hopfield 1982; Amit, Gutfreund and Sompolinsky 1985; Sompolinsky, Crisanti and Sommers 1988; Ramsauer et al. 2020;
Kanerva 1988; Harnad 1990; Huh et al. 2024; Crosby et al. 2019; Pathak et al. 2017; Eliasmith 2012; Busemeyer and Bruza
2012; Tegmark 2000.

Codes and physics: Forney 1968; Sourlas 1989; Chertkov and Chernyak 2006; Kadowaki and Nishimori 1998; Santoro et al.
2002; Sundararajan, Winstead and Boutillon 2014; Trouillon et al. 2016; Sun et al. 2019; Plate 1995; Schoenholz et al.
2017; Holtzman et al. 2020; Basu et al. 2021; Kadavath et al. 2022; Farquhar et al. 2024.
