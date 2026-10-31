# Experiment Plans for the Hunches

A pre-registered plan for every one of [Matej's ideas and hunches](../matej-hunches.md), so each can be tried and
none is quietly dropped - the maintainer's request, item 6 of [STATE.md](../../../STATE.md). A plan is written
before anything runs; when an experiment runs, its result goes to the journal, and a failed or inconclusive one is
kept and listed under STATE.md's *To Revisit*.

## The Format

Every plan has the same parts:

- **Question**: what the experiment answers, in one sentence.
- **Method**: what is built or measured, and how.
- **Baselines and controls**: what it is compared with, and the control that would expose a broken harness.
- **Refuted if**: the outcomes that would refute the hunch, fixed before the run.
- **Statistics**: the test or interval, chosen before the data is seen.
- **Seeds and repetitions**: how many, and why that many. The default is 20 seeds per condition: by a Monte Carlo of
  the one-sided two-sample t test at alpha 0.05 (100 000 draws, numpy generator seed 20261013), 20 per arm gives about
  80% power to see a difference of 0.8 standard deviations (the command is in the 2026-10-13 journal). A plan that
  needs more, or fewer, says why.
- **Cost**: compute and memory, in the terms of the rule *Lean brains*.
- **Waits on**: what must exist first - the benchmark, the feature tool, the arena, a sense the Grid lacks.
- **Leads**: each lead's status - found in Crossref by title and DOI, or read at the source (and for which number). A
  lead that cannot be found says so, and stays only as an unverified lead, never as evidence.

A plan says what it cannot settle yet. Numbers it needs from the literature are named, not guessed; they are read
out of their sources when the experiment is prepared.

## The Index

| Hunch | Title | Plan |
|-------|-------|------|
| A1 | A validation suite with pass bands fixed before any tuning | [planned](A1-A8.md#a1-a-validation-suite-with-pass-bands-fixed-before-any-tuning) |
| A2 | Score a population of seeds, never one worm | [planned](A1-A8.md#a2-score-a-population-of-seeds-never-one-worm) |
| A3 | A blind test with a pre-registered bar | [planned](A1-A8.md#a3-a-blind-test-with-a-pre-registered-bar) |
| A4 | Error fidelity: fail the way the animal fails | [planned](A1-A8.md#a4-error-fidelity-fail-the-way-the-animal-fails) |
| A5 | The exact trajectory difference as the first ablation measure | [planned](A1-A8.md#a5-the-exact-trajectory-difference-as-the-first-ablation-measure) |
| A6 | Tune by titration, not maximisation | [planned](A1-A8.md#a6-tune-by-titration-not-maximisation) |
| A7 | Three axes, the third being how much is grown rather than wired | [planned](A1-A8.md#a7-three-axes-the-third-being-how-much-is-grown-rather-than-wired) |
| A8 | A neurological examination below behaviour | [planned](A1-A8.md#a8-a-neurological-examination-below-behaviour) |
| A9 | Vital signs for a long-lived brain | [planned](A9-A15.md#a9-vital-signs-for-a-long-lived-brain) |
| A10 | Welfare and affect read from behaviour alone | [planned](A9-A15.md#a10-welfare-and-affect-read-from-behaviour-alone) |
| A11 | A sentience-indicator checklist, read with care | [planned](A9-A15.md#a11-a-sentience-indicator-checklist-read-with-care) |
| A12 | Experiment hygiene | [planned](A9-A15.md#a12-experiment-hygiene) |
| A13 | A stored quantity is not what its name says until a known case has been read | [planned](A9-A15.md#a13-a-stored-quantity-is-not-what-its-name-says-until-a-known-case-has-been-read) |
| A14 | Mine the field's failures first | [planned](A9-A15.md#a14-mine-the-fields-failures-first) |
| A15 | Computational psychiatry on a frozen copy | [planned](A9-A15.md#a15-computational-psychiatry-on-a-frozen-copy) |
| B1 | A nervous system, not a game client | [planned](B.md#b1-a-nervous-system-not-a-game-client) |
| B2 | Smell as an unlabelled fingerprint | [planned](B.md#b2-smell-as-an-unlabelled-fingerprint) |
| B3 | Senses switched on by rung | [planned](B.md#b3-senses-switched-on-by-rung) |
| B4 | How a reversal ends decides whether chemotaxis works | [planned](B.md#b4-how-a-reversal-ends-decides-whether-chemotaxis-works) |
| B5 | A fast agar-plate world for iterating | [planned](B.md#b5-a-fast-agar-plate-world-for-iterating) |
| B6 | Behaviour must not depend on the tick rate | [planned](B.md#b6-behaviour-must-not-depend-on-the-tick-rate) |
| B7 | Do not count your own echo | [planned](B.md#b7-do-not-count-your-own-echo) |
| B8 | A simulated body can be rolled back | [planned](B.md#b8-a-simulated-body-can-be-rolled-back) |
| B9 | The insect bet, and the bridge it owes | [planned](B.md#b9-the-insect-bet-and-the-bridge-it-owes) |
| C1 | Two chemotaxis mechanisms at once | [planned](C1-C12.md#c1-two-chemotaxis-mechanisms-at-once) |
| C2 | A bistable forward-backward switch | [planned](C1-C12.md#c2-a-bistable-forward-backward-switch) |
| C3 | Graded neurons and gap junctions | [planned](C1-C12.md#c3-graded-neurons-and-gap-junctions) |
| C4 | The connectome as a mask - or not | [planned](C1-C12.md#c4-the-connectome-as-a-mask---or-not) |
| C5 | A layered brain with its timescales written down | [planned](C1-C12.md#c5-a-layered-brain-with-its-timescales-written-down) |
| C6 | Blend the layers, do not override | [planned](C1-C12.md#c6-blend-the-layers-do-not-override) |
| C7 | Reflex interlocks the learning cannot reach | [planned](C1-C12.md#c7-reflex-interlocks-the-learning-cannot-reach) |
| C8 | A fast global stop and a slow appraisal; threat has an imminence axis | [planned](C1-C12.md#c8-a-fast-global-stop-and-a-slow-appraisal-threat-has-an-imminence-axis) |
| C9 | Coincidence detection as a learned prior | [planned](C1-C12.md#c9-coincidence-detection-as-a-learned-prior) |
| C10 | Many proposals, one commitment per cycle | [planned](C1-C12.md#c10-many-proposals-one-commitment-per-cycle) |
| C11 | Imagination through the action pathway | [planned](C1-C12.md#c11-imagination-through-the-action-pathway) |
| C12 | A step clock from brain rhythms | [planned](C1-C12.md#c12-a-step-clock-from-brain-rhythms) |
| C13 | Vigour from the average reward rate; one global slowing factor | [planned](C13-C24.md#c13-vigour-from-the-average-reward-rate-one-global-slowing-factor) |
| C14 | Poised near criticality, monitored | [planned](C13-C24.md#c14-poised-near-criticality-monitored) |
| C15 | A logistic-map arousal engine | [planned](C13-C24.md#c15-a-logistic-map-arousal-engine) |
| C16 | Golden-ratio spacing of rhythms | [planned](C13-C24.md#c16-golden-ratio-spacing-of-rhythms) |
| C17 | Memory timescales spaced by Feigenbaum's constant | [planned](C13-C24.md#c17-memory-timescales-spaced-by-feigenbaums-constant) |
| C18 | Spreading activation has a cliff | [planned](C13-C24.md#c18-spreading-activation-has-a-cliff) |
| C19 | Windowed tallies, not lifetime counts | [planned](C13-C24.md#c19-windowed-tallies-not-lifetime-counts) |
| C20 | Bounded loops for rhythmic behaviour | [planned](C13-C24.md#c20-bounded-loops-for-rhythmic-behaviour) |
| C21 | A noisy, log-compressed sense of elapsed time | [planned](C13-C24.md#c21-a-noisy-log-compressed-sense-of-elapsed-time) |
| C22 | Inhibition of return and foraging in memory | [planned](C13-C24.md#c22-inhibition-of-return-and-foraging-in-memory) |
| C23 | A stated, measured bound on what is held at once | [planned](C13-C24.md#c23-a-stated-measured-bound-on-what-is-held-at-once) |
| C24 | Instrument the state of attention | [planned](C13-C24.md#c24-instrument-the-state-of-attention) |
| D1 | Roaming and dwelling from continuous drives | [planned](D1-D6.md#d1-roaming-and-dwelling-from-continuous-drives) |
| D2 | Modulators that flip a smell's meaning | [planned](D1-D6.md#d2-modulators-that-flip-a-smells-meaning) |
| D3 | One modulator state, three effects | [planned](D1-D6.md#d3-one-modulator-state-three-effects) |
| D4 | Learning rate from uncertainty; surprise resets the current thought | [planned](D1-D6.md#d4-learning-rate-from-uncertainty-surprise-resets-the-current-thought) |
| D5 | A modulator design law | [planned](D1-D6.md#d5-a-modulator-design-law) |
| D6 | Receptors that tire, ten times faster than they recover | [planned](D1-D6.md#d6-receptors-that-tire-ten-times-faster-than-they-recover) |
| D7 | Roles for a handful of modulators | [planned](D7-D13.md#d7-roles-for-a-handful-of-modulators) |
| D8 | What not to build in chemistry | [planned](D7-D13.md#d8-what-not-to-build-in-chemistry) |
| D9 | Patience is confidence that reward will come | [planned](D7-D13.md#d9-patience-is-confidence-that-reward-will-come) |
| D10 | Watch for drift into an anxious regime | [planned](D7-D13.md#d10-watch-for-drift-into-an-anxious-regime) |
| D11 | Affect in several dimensions, emotions as regions | [planned](D7-D13.md#d11-affect-in-several-dimensions-emotions-as-regions) |
| D12 | Pain damped by an opioid-like system, as dynamics | [planned](D7-D13.md#d12-pain-damped-by-an-opioid-like-system-as-dynamics) |
| D13 | Bad things are learned faster than good ones | [planned](D7-D13.md#d13-bad-things-are-learned-faster-than-good-ones) |
| E1 | Habituation is ten properties, not one decrement | [planned](E1-E5.md#e1-habituation-is-ten-properties-not-one-decrement) |
| E2 | Novelty as mismatch, proven by an omission | [planned](E1-E5.md#e2-novelty-as-mismatch-proven-by-an-omission) |
| E3 | Three kinds of learning in three places | [planned](E1-E5.md#e3-three-kinds-of-learning-in-three-places) |
| E4 | Hebbian learning needs a stabiliser and a "used together" rule | [parked: plan failed verification](E1-E5.md#e4-hebbian-learning-needs-a-stabiliser-and-a-used-together-rule) |
| E5 | Memories that bind what, where, when and how it felt | [planned](E1-E5.md#e5-memories-that-bind-what-where-when-and-how-it-felt) |
| E6 | Separate, complete, consolidate, replay | to plan |
| E7 | Promote after spaced recurrence; merge when predictions agree; never delete | to plan |
| E8 | Two strengths: forgetting loses influence, not the trace | to plan |
| E9 | Attention as a conserved economy | to plan |
| E10 | The input reshapes the attractor landscape | to plan |
| E11 | Completion, with gates | to plan |
| E12 | Drives retrieve what could satisfy them | to plan |
| E13 | Curiosity as expected information gain | to plan |
| E14 | Sleep as a bounded replay pass, driven by pressure | to plan |
| E15 | Lessons about attractor memories | to plan |
| E16 | The spectrum as a health check | to plan |
| E17 | Maps at several scales | to plan |
| E18 | Innate core systems, and the smallest consistent hypothesis | to plan |
| E19 | Development by stage | to plan |
| E20 | Identity is the persisted pattern | to plan |
| F1 | Phase is the missing ingredient: complex amplitudes | to plan |
| F2 | Cognition on the tree-like side of the numbers | to plan |
| F3 | Memories as codewords of an error-correcting code | to plan |
| F4 | Stable thoughts as idempotents of an exceptional algebra | to plan |
| F5 | Self-similar groups as the symmetry of concept hierarchies | to plan |
| F6 | Critical brains as optimal expanders | to plan |
| F7 | Concepts consolidate only when local and global agree | to plan |
| F8 | Three legs: fixed primitives, critical dynamics, learned geometry | to plan |
| F9 | Analogies from mathematics, kept for their testable parts | to plan |
| F10 | Watch for a self-model | to plan |
| F11 | A scan for latent pathology | to plan |
| F12 | Predict the consequence, then refine | to plan |
| F13 | A memory that renormalises time, shaped like MERA | to plan |
| F14 | Thinking in loops: automata, their groups, graphs and knots | to plan |
| F15 | Rhythm and direction twisted together, as in the Hopf fibration | to plan |
| F16 | Harmonic analysis, and the Kakeya needle in a worm's turn | to plan |
| F17 | Metaphysics, kept for its testable parts | to plan |
| G1 | Think by predicting value over a grown concept graph; turn when the prediction falls | to plan |
| G2 | Walls come from events, not silence | to plan |
| G3 | Two kinds of concept node: intensity and tuned | to plan |
| G4 | Concepts grown by a novelty gate | to plan |
| G5 | Instinct and learning as separate pathways | to plan |
| G6 | Learned trust in instinct | to plan |
| G7 | Inner senses gate; they do not predict | to plan |
| G8 | Honest senses are noisy, so integrate before deciding | to plan |
| G9 | Noise below the decision threshold is blur; above it, search | to plan |
| G10 | Learning pays only when instinct is not enough | to plan |
| G11 | Judge a design by its worst life | to plan |
| G12 | Designed learning structure with evolved numbers | to plan |
| G13 | Every new tool given to evolution is first used to cheat | to plan |
| G14 | Steer by the learned value of the neighbouring concept | to plan |
| G15 | Imagination needs an accurate model and a body that can act on it | to plan |
| G16 | Complex edges as a Fourier code for timing | to plan |
| G17 | Which confidence gate works depends on how dense the answer space is | to plan |
| G18 | Walls as destructive interference | to plan |
| G19 | Insight as tunnelling | to plan |
| G20 | Spurious states as creativity, labelled | to plan |
| G21 | Frozen and chaotic phases of a mind | to plan |
| G22 | The portability test | to plan |
| G23 | A noise gene at every stage | to plan |
| G24 | The grand pattern: motifs that recur across independent evolutions | to plan |
| G25 | Calibrate on more than one behaviour | to plan |
| G26 | A printable mind | to plan |
| G27 | Optimise the optimiser with measurements | to plan |
| G28 | What transfers to the fly | to plan |
| G29 | The worm rediscovered the cognitive architectures | to plan |
| G30 | The worm model's realism gaps | to plan |
