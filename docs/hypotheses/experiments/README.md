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
  lead that cannot be found is dropped and says so.

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
| C13 | Vigour from the average reward rate; one global slowing factor | to plan |
| C14 | Poised near criticality, monitored | to plan |
| C15 | A logistic-map arousal engine | to plan |
| C16 | Golden-ratio spacing of rhythms | to plan |
| C17 | Memory timescales spaced by Feigenbaum's constant | to plan |
| C18 | Spreading activation has a cliff | to plan |
| C19 | Windowed tallies, not lifetime counts | to plan |
| C20 | Bounded loops for rhythmic behaviour | to plan |
| C21 | A noisy, log-compressed sense of elapsed time | to plan |
| C22 | Inhibition of return and foraging in memory | to plan |
| C23 | A stated, measured bound on what is held at once | to plan |
| C24 | Instrument the state of attention | to plan |
| D1 | Roaming and dwelling from continuous drives | to plan |
| D2 | Modulators that flip a smell's meaning | to plan |
| D3 | One modulator state, three effects | to plan |
| D4 | Learning rate from uncertainty; surprise resets the current thought | to plan |
| D5 | A modulator design law | to plan |
| D6 | Receptors that tire, ten times faster than they recover | to plan |
| D7 | Roles for a handful of modulators | to plan |
| D8 | What not to build in chemistry | to plan |
| D9 | Patience is confidence that reward will come | to plan |
| D10 | Watch for drift into an anxious regime | to plan |
| D11 | Affect in several dimensions, emotions as regions | to plan |
| D12 | Pain damped by an opioid-like system, as dynamics | to plan |
| D13 | Bad things are learned faster than good ones | to plan |
| E1 | Habituation is ten properties, not one decrement | to plan |
| E2 | Novelty as mismatch, proven by an omission | to plan |
| E3 | Three kinds of learning in three places | to plan |
| E4 | Hebbian learning needs a stabiliser and a "used together" rule | to plan |
| E5 | Memories that bind what, where, when and how it felt | to plan |
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
