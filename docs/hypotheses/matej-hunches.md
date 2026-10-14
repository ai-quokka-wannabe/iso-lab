# Matej's Ideas and Hunches

The maintainer's ideas and hunches about animal minds, collected for the night staff. The more ideas the lab has
to try, the better its chances; so they are many, and some are wild.

How to read them:

- **Every entry is a hunch**, not a result. It earns a place in the lab only by being tested: write the outcomes
  that would refute it, run it, and report what came out, as the rules *Be bold* and *Pre-register and ablate*
  in [.claude/CLAUDE.md](../../.claude/CLAUDE.md) say.
- **The leads are unverified.** Under each entry, *Leads* names published work the idea touches, by author and
  year only. None has been checked yet, and some author lists may be wrong. Verify each against its source -
  Crossref, arXiv, the paper itself - before citing it anywhere else in the lab, as
  [docs/research/README.md](../research/README.md) requires. A lead that cannot be verified is dropped.
- **Numbers are candidates.** Where an entry gives a number from the literature, it is a starting point to be
  read out of the source, never a value to use as remembered.
- **Promote, do not copy.** A hunch that survives a first look becomes a numbered hypothesis in
  [README.md](README.md) or a task in [STATE.md](../../STATE.md), in the lab's own words and with verified sources.

Some entries overlap with H1-H9; they are kept, because they say something more specific.

## A. Measuring the Worm

### A1. A validation suite with pass bands fixed before any tuning

A battery of quantitative worm assays with numeric pass bands written down before a brain is tuned: the
chemotaxis index, pirouette rate and its dependence on the change in concentration over time, run durations,
speed, body-wave frequency, reversal rate, the share of reversals followed by an omega turn and the heading
change of an omega turn, the bearing after a pirouette, time spent dwelling on and roaming off food and how fast
the switch happens when food is removed, tap habituation and its recovery, and conditioned chemotaxis. The rule:
real-worm data beats any prototype.
*Why*: a ready-made skeleton for the benchmark's worm features, beside Phase 0's feature set.
*Test*: read every band out of its source; an assay the Grid cannot run yet (anything needing smell) is listed as
not comparable.
*Leads*: Bargmann and Horvitz 1991; Pierce-Shimomura et al. 1999; Iino and Yoshida 2009; Gray et al. 2005;
Wakabayashi et al. 2004 (a Crossref search finds a paper on how long forward runs last; whether it is the
source meant here is to be checked); Rankin et al. 1990.

### A2. Score a population of seeds, never one worm

Biology measures behaviour over many animals; a single simulated worm can score perfectly, or badly, on one seed.
Score each measure over an ensemble of seeds, require its mean and its spread to fall in the band, and treat a
seed that fails alone as a bug to investigate, not noise to average away.
*Why*: the lab is deterministic given a seed, so seed sweeps are cheap and catch rare failures.
*Test*: outlier seeds traced to a mechanism; the ensemble spread compared with the spread between real worms.

### A3. A blind test with a pre-registered bar

Show a judge short segments - half real worm, half Grid worm - normalised identically, sampled by seed, with the
selection rules fixed before assembly (skip start-up, consecutive segments, a cap per recording, the same filters
on both sides). Two judges, one expert and one naive; residual cues disclosed in advance. Fix the verdict bands
before the test, from a binomial bound. The judge's list of tells becomes the work queue. Run it rarely, as a
milestone gate; between milestones use per-feature statistics.
*Why*: a human-readable complement to the classifier two-sample test of [the plan](../PLAN.md).
*Test*: it is the test.

### A4. Error fidelity: fail the way the animal fails

Indistinguishability is judged over the whole distribution of behaviour, and failures are half of it. Sort every
non-success into animal-plausible errors (keep, and match their rates to the real worm's) and machine-class errors
(stereotyped loops, impossible recoveries, failures at the wrong latency - drive to zero). Build the error
classifier before the mechanism.
*Why*: a worm that succeeds like a worm but fails like a program is spotted at once.
*Test*: error-class rates against real recordings.

### A5. The exact trajectory difference as the first ablation measure

With a deterministic harness, compare an ablated brain with its baseline by the exact difference of the
trajectories before any summary statistic: coarse statistics can miss most of the changes a mechanism makes.
Repeats within a condition must be byte-identical; never score speed while a variable-speed part runs; a
mechanism whose removal changes nothing is a statement about the test until a targeted test exercises it.
*Test*: an A/A control that differs means the run is invalid.

### A6. Tune by titration, not maximisation

The benchmark score is exactly the proxy that will be overfitted. Tune like a dose: wide cheap screening, freeze
the knobs that do nothing, move one or two at a time, rank by a lower confidence bound, prefer a broad plateau to
a sharp peak, keep a sealed held-out probe set (a score that rises while the held-out set stays flat is the
signature of gaming), and accept a setting only if it beats the default by more than the measured spread.
Forbidden outcomes are hard failures, never soft penalties. Probe emergent properties rather than ones a single
parameter sets directly.
*Leads*: Goodhart's law in Strathern's phrasing; Skalse et al. 2022 on proxies.

### A7. Three axes, the third being how much is grown rather than wired

Judge a brain on three axes read together: the diagnostic battery, statistical similarity to the real worm on
held-out data, and the share of behaviour that comes from learned structure rather than designer-written rules.
Keep a manifest of every innate unit, connection, reflex and constant, so the innate share can be counted. For a
worm much is wired by nature - so the third axis becomes "what is wired, what is learned", stated honestly.
*Test*: a change that improves the first two axes by adding a special case is a regression on the third.

### A8. A neurological examination below behaviour

Examine the brain the way a neurologist examines a nervous system: vital signs first, then reflexes and gating,
short-term holding, priming, attention, inhibition, habituation, and long-run stability last. Every probe reads
internal state, which no clinic can. Each failed probe ticks a column of a lesion matrix; a diagnosis needs two
converging failures and a spared control. Run on a frozen copy, restored between probes, with every band
pre-registered.
*Why*: a behavioural mismatch with the worm can then be traced to a module instead of tuned blindly.
*Test*: validate the battery by deliberate lesions - each lesion must produce its predicted failure pattern, and
only that; a probe that ignores its own lesion measures something else.
*Leads*: Rankin et al. 2009; Beggs and Plenz 2003; Casali et al. 2013; Turrigiano et al. 1998; Goddard 1969;
Hedge, Powell and Sumner 2018.

### A9. Vital signs for a long-lived brain

A dozen task-free readings on control charts against the brain's own history: how many units are active and how
concentrated the activity is (top share, participation ratio), the fraction of units pinned at any bound, the
branching ratio of activity from one step to the next (sub-critical, with margin), how often a hub wins on
unrelated cues, the distribution of each unit's summed outgoing weight, the growth rate of learned weights,
habituation load, each modulator's mean, variance and reactivity, and a perturbational complexity index (one
cue, then neutral input: too low is stereotyped, too high has no persistence).
*Why*: early warning of runaway excitation, capture or drift, long before behaviour visibly breaks.
*Leads*: Beggs and Plenz 2003; Casali et al. 2013.

### A10. Welfare and affect read from behaviour alone

Infer the creature's state the way welfare science infers an animal's - from the outside: approach latency to
novelty, activity over a day, time sheltering, play, grooming, sleep regularity, exploration, and judgement-bias
tasks (an ambiguous cue read optimistically or pessimistically). The hunch is that play appears only in good
conditions, so its absence is a signal (a claim to check in the welfare literature).
*Why*: compares like with like, without reading the brain's internals.
*Test*: judgement bias should shift after positive or negative manipulations, as in real animals.
*Leads*: Mendl, Burman and Paul 2010; Boissy et al. 2007; Burghardt 2005; Pellis and Pellis 2009.

### A11. A sentience-indicator checklist, read with care

Run published indicator criteria for sentience - nociceptors, integrative regions and their connections,
affective behaviour, motivational trade-offs, conditioned place preference and aversion, self-protection,
analgesia restoring normal behaviour - against the architecture. Caution: indicators can be met by construction,
which cuts both ways as evidence.
*Leads*: Birch, Schnell and Clayton 2020; Birch 2024; the Cambridge Declaration on Consciousness 2012.

### A12. Experiment hygiene

Fix thresholds, baselines, seeds and outcome rules before running; never relabel a verdict afterwards - run a
new pre-registered follow-up instead. Check that every baseline is in its working regime; compute theoretical
ceilings as anchors; a zero coefficient of variation can mean invariance, total failure or tie-breaking, so look;
report several metrics so a finding is not buried; run memory tests below capacity. The most informative control
is often one matched on correlation structure but lacking the hypothesised structure.

### A13. A stored quantity is not what its name says until a known case has been read

Before anything consumes an internal quantity - a baseline, a set point, a tonic level - print it and check it
against cases where the right value is known. Names cover several timescales at once ("familiar" can mean
habituated in seconds, resting in tens of seconds, or known for days); give each its own variable.

### A14. Mine the field's failures first

Before building a mechanism, read how earlier cognitive architectures died. The expensive deaths were unbounded
state and commitments that could not be revoked; the other failure was a mechanism flexible enough to explain any
result after the fact. A model should forbid something.
*Test*: name one behaviour the model forbids.
*Leads*: post-mortems of LIDA, Soar and ACT-R's associative learner; Johnson-Laird, Herrmann and Chaffin 1984;
Laird, Lebiere and Rosenbloom 2017.

### A15. Computational psychiatry on a frozen copy

Because the brain's state can be read, restored and lesioned, disorder-like signatures (perseveration, loosened
association, blunted affect) can be watched as they happen and localised. Guard rails: a matching symptom is not
the disorder; a knob that makes it vanish is Goodhart unless it corrects the mechanism and leaves unrelated probes
alone. For the worm: compare ablations with the published phenotypes of mutant and ablated worms.

## B. Body, Senses and World

### B1. A nervous system, not a game client

The world simulates body and environment; the brain is only the nervous system - raw sensor readings in, muscle
commands out. No identities, labels, threat levels, health bars, waypoints or "food remaining"; the brain knows
only "am I taking in energy now?", and hunger, health, fatigue and recognition are derived inside it. If an animal
of that size could not perceive it, the brain does not get it.
*Test*: audit the interface field by field against what the animal's sensory biology can transduce. (This is the
lab's rule *No cheating*; the field-by-field audit is the addition.)

### B2. Smell as an unlabelled fingerprint

When the Grid has smell, each source could carry a fixed multi-dimensional chemical pattern, with intensity, its
change over time and a left-right difference. Similar sources have similar patterns, mixtures blend, old traces
blur; the brain must learn which pattern means what.
*Test*: perturb the fingerprints; real recognition should generalise to similar ones and be confused by blends.

### B3. Senses switched on by rung

A graduated sensorium for the ladder of animals: the worm gets a scent channel or two, a few touch zones,
temperature and its rate, a scalar noxious sense, scalar light, proprioception and a feeding signal, with
forward, backward and turn; an insect adds vibration, more touch, a directional light array; a rodent adds
image-forming vision, hearing, limbs and voice. Each rung only adds channels, so earlier tests stay valid.
*Test*: every rung's tests still pass on the next rung.

### B4. How a reversal ends decides whether chemotaxis works

If backing up does not reorient the body, reversals not followed by an omega turn stop helping the worm find a
source, and chemotaxis fails on some seeds. Body mechanics and the meaning of motor commands decide whether the
circuit's statistics produce taxis.
*Test*: chemotaxis and the reversal-to-omega fraction under each motor semantics, against real data.

### B5. A fast agar-plate world for iterating

A small two-dimensional dish world - point sources with diffusing gradients that deplete and drift, a temperature
gradient with a cultivation set point, food eaten only when moving forward near it, walls that trigger a
reversal, head-only sensing - running far faster than real time, with the same sensory and motor interface as the
Grid, so a brain moves from dish to Grid unchanged.
*Why*: rapid iteration against published assay protocols, with the Grid kept for the real scoring.
*Test*: the published assays reproduced in the dish; a brain that passes there and fails on the Grid shows what
the dish leaves out.

### B6. Behaviour must not depend on the tick rate

Reflex durations scale with real elapsed time; behaviour statistics must be invariant across tick rates and
fast-forward.
*Why*: the lab's *animal's pace* counts time in simulated seconds; this is how to check it holds.
*Test*: the same statistics at two tick rates.

### B7. Do not count your own echo

Sensory change caused by the animal's own movement is not news about the world (reafference; an efference copy
cancels it), and the absence of a stimulus is not evidence against a learned association. Evidence is never
fabricated, double-counted or discarded; double counting shows up where two paths reconverge.
*Test*: a reconvergence probe - two sources of the same evidence should not add up to more than either justifies.

### B8. A simulated body can be rolled back

In a real body an action cannot be undone; in a simulated one, every moment can be snapshotted, audited and
rolled back. Keep that advantage on purpose: snapshots at every pause, diffs between them, time-travel debugging.

### B9. The insect bet, and the bridge it owes

A hunch held by many: sensorimotor competence in a body, at insect scale, is the universal ingredient, and
abstraction comes later. The honest counterpoint: behaviour-based robotics stalled exactly at the scale-up step.
Body-first is the lab's bet too - so plan to measure the bridge rather than assume it.
*Test*: adaptation to a perturbed body (a weakened joint, a lost segment) within an animal-like time.
*Leads*: Brooks 1991; Pfeifer and Bongard 2006; Braitenberg 1984.

## C. Circuits, Rhythms and Dynamics

### C1. Two chemotaxis mechanisms at once

A biased random walk - turning more often when the concentration is falling - and continuous weathervane
steering toward the side where it is higher, both running together; hunger shifts each on its own timescale.
*Test*: signed curvature against bearing during runs; pirouette rate binned by the change in concentration.
*Leads*: Pierce-Shimomura et al. 1999; Iino and Yoshida 2009; Ferrée et al. 1999.

### C2. A bistable forward-backward switch

Forward and backward command interneurons inhibit each other, forming a two-state switch that sensory input tips;
touch at the head drives backing, touch at the tail drives forward.
*Test*: direction-specific escape, and the effects of simulated ablations.
*Leads*: Chalfie et al. 1985; Zhen and Samuel 2015.

### C3. Graded neurons and gap junctions

Most worm neurons do not spike: one continuous-time equation per neuron (leak, sigmoid synapses, symmetric
diffusive gap junctions), time constants of tens to hundreds of milliseconds, synapse signs taken from the
transmitter (glutamate is often inhibitory in the worm). Compare with calcium imaging through a model of the
fluorescence, not raw voltage.
*Test*: removing gap junctions should hurt coordination more than removing as many chemical synapses.
*Leads*: Goodman et al. 1998; Liu et al. 2018; Beer 1995; Cook et al. 2019.

### C4. The connectome as a mask - or not

Three positions to settle by experiment rather than taste: the wiring diagram as a mask on which connections may
carry weight; the connectome only as a sanity check on a functional abstraction; or extracted circuits of a dozen
neurons grown toward all 302.
*Test*: all three on the benchmark and on ablation predictions.
*Leads*: White et al. 1986; Cook et al. 2019; Varshney et al. 2011; Witvliet et al. 2021; Ripoll-Sánchez et al. 2023.

### C5. A layered brain with its timescales written down

Six layers at different speeds: reflexes on raw senses; a behaviour sequencer with a readable transition matrix;
attractor settling; a low-dimensional modulator state; senses perturbing the settling all the time, so the
creature is always mid-thought; and small bounded learning during each settle. A fast reflex path and a slow
deliberative path.
*Why*: the lab's modular direction, with timescales to start from.

### C6. Blend the layers, do not override

Higher layers blend with reflexes rather than replacing them - output as a mix weighted by an authority that falls
with arousal, novelty and low confidence, so reflexes take over when startled. The reflex layer always runs and
controls alone if higher layers fail.
*Test*: with higher layers disabled the animal still avoids harm and chemotaxes crudely.
*Leads*: Brooks 1986; Arkin 1998.

### C7. Reflex interlocks the learning cannot reach

From safety engineering: put protective reflexes below, and out of reach of, the learned layers; several
independent detectors; fail safe on uncertainty; a watchdog budget per tick; a self-test at start-up; a protective
action that never increases another forbidden quantity; log what the actuator did, not what was commanded. If a
slow layer can retune a fast reflex's threshold, the reflex no longer protects.
*Test*: no learned path writes a reflex threshold.

### C8. A fast global stop and a slow appraisal; threat has an imminence axis

A fast stop signal that halts every candidate action but says nothing about what to do next, paired with
reselection; and a slower appraisal. Threat states differ by imminence: distal and uncertain, proximal and
imminent, certain but distant.
*Test*: stop latency unaffected by deliberation load; different behaviour under near and distant threat.
*Leads*: Aron et al. 2006; LeDoux and Brown 2017; Mobbs et al. 2007.

### C9. Coincidence detection as a learned prior

Adding inputs only gives OR; a unit that fires only when a few specified inputs are active together gives AND.
Make the structure innate and which inputs learned.
*Test*: it must beat additive summation on a probe that needs a conjunction.

### C10. Many proposals, one commitment per cycle

Elaborate candidate actions in parallel and reversibly, then commit exactly one per cycle. Stop on quiescence or a
budget fixed in advance, with a falling bound (the confidence needed to stop drops as time passes); priority is
salience times relevance, with a small tax on losers and a floor against starvation; prevent rumination with the
deadline, not with a monitor.
*Test*: a detector for cycling content; every step logged.

### C11. Imagination through the action pathway

Simulate a candidate action through the same pathway as a real one, with the motor output gated off; a few noisy
sweeps, then a choice, like hippocampal sweeps at a maze junction.
*Test*: decision latency grows with the number of sweeps and with novelty.
*Leads*: Johnson and Redish 2007; Battaglia, Hamrick and Tenenbaum 2013.

### C12. A step clock from brain rhythms

The atomic deliberation step as a theta cycle holding a handful of gamma packets, the reportable decision as an
ignition a little later; budgets in cycles, not wall time.
*Test*: decision latencies cluster at multiples of the cycle, if the hunch holds.
*Leads*: Buzsáki 2006; Lisman and Jensen 2013.

### C13. Vigour from the average reward rate; one global slowing factor

The optimal latency of an action is inversely proportional to the square root of the average reward rate: tonic dopamine sets how
fast, independently of what. Separately, one global speed parameter can explain many effects that look separate.
*Test*: latency distributions shift with recent reward; one parameter checked against several findings at once.
*Leads*: Niv, Daw, Joel and Dayan 2007; Kail 1991.

### C14. Poised near criticality, monitored

Run near the boundary between order and chaos and measure it: avalanche sizes, autocorrelation time, dynamic
range, the largest Lyapunov exponent. Tune by hand at the worm's scale; at larger scale add slow homeostatic gain
control beside fast learning. (This sharpens H7.)
*Test*: does behavioural realism peak near criticality?
*Leads*: Langton 1990; Beggs and Plenz 2003; Turrigiano and Nelson 2004.

### C15. A logistic-map arousal engine

One parameter of the logistic map, driven by internal state, sweeps the creature from rest (a fixed point)
through rhythmic regimes (period two, four, eight) to the edge of chaos and chaos (panic, or wide exploration).
Personality is the baseline parameter; a slow drift toward the edge of chaos when calm gives personality shaped by
experience. Deterministic variability without random draws. Extension: coupled maps as gait rhythm generators.
*Test*: fit dwell-time distributions and transition statistics to real worm data; it must beat a plain
state-to-temperature mapping, not just add noise.
*Leads*: Feigenbaum 1978; Strogatz 2015.

### C16. Golden-ratio spacing of rhythms

Space internal rhythm frequencies by powers of the golden ratio, the ratio hardest to lock, so independent
rhythms stay independent as their number grows.
*Test*: with many coupled generators at realistic coupling, measure unwanted locking under golden-ratio spacing
against other spacings. (The maintainer reports small tests that were inconclusive - unrecorded, not evidence.)
*Leads*: Pletzer, Kerschbaum and Klimesch 2010.

### C17. Memory timescales spaced by Feigenbaum's constant

A hunch: the timescales of successive memory layers - immediate, working, episodic, long-term - stand in a ratio
near Feigenbaum's delta, about 4.67.
*Test*: fit the ratios of real decay and persistence times; the prediction can fail cleanly.

### C18. Spreading activation has a cliff

Purely excitatory spreading changes behaviour abruptly when connectivity crosses a threshold, and hubs that
receive from many neighbours without the source paying climb to their bounds. Cures: lateral inhibition, a
conserved budget divided among targets, a logarithmic penalty on fan-out.
*Test*: run at ten and a hundred times the size and look for the cliff; two-hop priming well below one-hop.
*Leads*: Shrager, Hogg and Huberman 1987; Collins and Loftus 1975; Anderson 1974.

### C19. Windowed tallies, not lifetime counts

Statistics the animal acts on should be tallies over a recent window of real choices; lifetime counts stop
hearing new evidence. One primitive - halve every slot when the total reaches the window - replaces many
counters.
*Test*: after a contingency reversal, the shift should follow the animal's time course.

### C20. Bounded loops for rhythmic behaviour

Represent behavioural programmes as trees of named sub-programmes with a return stack; every loop runs on a
finite budget, can be pre-empted by new input, and logs its iterations; and a global duty-cycle bound holds even
for loops rebuilt from bounded parts.

### C21. A noisy, log-compressed sense of elapsed time

Read elapsed time through a pacemaker whose intervals grow geometrically with Weber-like noise, over an exact
internal tick; an exact noiseless counter is unlike any animal.
*Test*: timing error proportional to the interval.
*Leads*: Taatgen, van Rijn and Anderson 2007.

### C22. Inhibition of return and foraging in memory

After selection the winner briefly inhibits itself, so the next can win; search behaves like foraging - exploit
a patch, leave as it depletes; massed repetition satiates, spaced repetition strengthens.
*Why*: the worm's local search and dispersal is literally patch depletion and leaving.
*Leads*: Posner and Cohen 1984; Hills, Jones and Todd 2012.

### C23. A stated, measured bound on what is held at once

Give the focus an explicit capacity, distinct from "everything active"; for a worm, a few-second window of
sensory integration.
*Test*: a span probe, and decay with lag.
*Leads*: Cowan 2001.

### C24. Instrument the state of attention

Two cheap readings: how peaked the activity is, and how stable the active set is from one step to the next
(overlap across consecutive steps). Report responsiveness as components, never summed.
*Test*: stability should drop at changes in the environment.

## D. Chemistry and Affect

### D1. Roaming and dwelling from continuous drives

Hunger, rising with metabolism and falling with feeding, scales reflex parameters continuously - speed, reversal
rate, steering gain - instead of switching a state; the discrete labels are only for observers, with hysteresis.
Drives gate reflexes without replacing them.
*Test*: roaming-bout lifetimes, the share of time dwelling, the latency to roam after food is removed, and
mutant-like manipulations, against real worms.
*Leads*: Ben Arous et al. 2009; Fujiwara et al. 2002; Flavell et al. 2013 and Sawin et al. 2000 are already
cited and verified in [README.md § H8](README.md#h8-brain-chemistry-as-modelled-substances).

### D2. Modulators that flip a smell's meaning

Opposing modulators and state-dependent valence: serotonin begins and extends dwelling, a neuropeptide roaming;
dopamine slows the worm on food and serotonin slows a starved worm further (Sawin et al. 2000, as cited in
[README.md § H8](README.md#h8-brain-chemistry-as-modelled-substances)); octopamine promotes roaming (a claim to
check - no lead yet); a gas repellent when fed becomes
attractive when starved; starvation lowers avoidance of noxious stimuli; long starvation brings a brain-wide
quiescent state.
*Test*: reproduce a valence flip from internal state alone - a strong test that the chemistry is real (and of
H8).
*Leads*: Rengarajan et al. 2019; Ghosh et al. 2016; Skora et al. 2018.

### D3. One modulator state, three effects

The same low-dimensional modulator state reshapes the behavioural transition matrix, sets the regime of the
settling dynamics, and weights the senses (a hungry animal attends to smell, a frightened one to touch). Emotion
not as a computed label but as a dynamical regime.
*Test*: move one modulator; all three effects should move together.

### D4. Learning rate from uncertainty; surprise resets the current thought

An acetylcholine-like signal of expected uncertainty sets the learning rate; a phasic noradrenaline-like burst on
surprise knocks memory out of its current attractor and reopens exploration; tonic noradrenaline sets the balance
of exploring and exploiting; dopamine split into tonic wanting and phasic prediction error.
*Test*: learning speed scales with how volatile the world is; a surprise shortens dwell in the current attractor.
*Leads*: Yu and Dayan 2005; Aston-Jones and Cohen 2005; Berridge 2007.

### D5. A modulator design law

Each modulator is a broadcast value with local receptor multipliers; it reads one input computed from outcomes
the brain already has, sets one or two generic knobs through a bounded map, and carries one pre-registered
prediction and one forbidden outcome. Name it by its computational role first and its molecule second. Give each
its own knob and its own probe, so single ablations can tell them apart - many configurations pass any battery
otherwise.
*Test*: a modulator counts only if removing it worsens its own probe and improves no other.
*Leads*: Doya 2002; Dayan 2012; Prinz, Bucher and Marder 2004.

### D6. Receptors that tire, ten times faster than they recover

The hunch is that receptor sensitivity desensitises roughly ten times faster than it recovers, and that sleep
restores much of it - the ratio is the maintainer's guess, with no source yet; find one or fit it. Use the ratio,
not absolute constants.
*Test*: repeated identical rewards lose effect and recover after a gap or a sleep. (Sharpens H8's tachyphylaxis.)

### D7. Roles for a handful of modulators

A starting set of computational roles to map onto the worm's own modulators: a prediction-error burst that sets
the learning rate, with a novelty bonus that habituates; a tonic level that sets vigour; a noradrenaline-like gain
whose burst commits and resets focus at a change point; an acetylcholine-like switch between encoding and
retrieval, so recall never writes its own products back as evidence; a serotonin-like patience; a social-salience
term; and sleep pressure rising with learning load. Pre-registered forbidden outcomes for them: uncorroborated
events writing more than corroborated ones, writes during retrieval, a novelty bonus that does not fall with
repeats, old focus dominating after a change point, sleep pressure falling without sleep.

### D8. What not to build in chemistry

No single arousal scalar - it conflates opposite modes; no hand-drawn inverted-U curves - if one appears it must
emerge from two opposed monotone effects; arousal should amplify the existing priority order (winner takes more),
because uniform scaling does nothing but risk saturation; no modulator that only annotates or that chooses
content.
*Leads*: Mather and Sutherland 2011; Easterbrook 1959; Teigen 1994.

### D9. Patience is confidence that reward will come

Model serotonin's patience as learned confidence that a pending goal will be fulfilled, setting how long to
persist, rather than as a fixed discount.
*Why*: the worm's decision to stay on or leave a food patch is serotonin-modulated.
*Test*: persistence grows with a history of fulfilment and shrinks when goals go unmet.
*Leads*: Miyazaki et al. 2018; Lottem et al. 2018.

### D10. Watch for drift into an anxious regime

The hunch: a diffuse value signal gating plasticity is among the oldest working patterns in neurorobotics, and
modulator systems that include threat may drift toward anxious, inhibited behaviour over long runs (no lead yet;
both claims to check).
*Why*: the lab does not clamp feeling, so this drift is a vital sign to watch, not a reason to clamp.
*Test*: the long-run distribution of behavioural states in a neutral world.
*Leads*: Sporns and Alexander 2002; Krichmar 2008.

### D11. Affect in several dimensions, emotions as regions

A modulator state of several axes - energy and satiety, threat, arousal, prediction error, seeking, affiliation,
pain, circadian phase - with the classic emotional systems as regions or trajectories in it rather than
dimensions: play as a region (safe, fed, pain-free, a peer near, mid arousal); rage as threat with seeking blocked;
grief as falling affiliation. Valence and arousal alone cannot tell fear from rage.
*Why*: for rodent-scale rungs; the worm needs only a few axes.
*Leads*: Panksepp 1998; Russell 1980; Barrett 2017; LeDoux 2012.

### D12. Pain damped by an opioid-like system, as dynamics

An endogenous analgesia system damps pain at high levels and under stress; prolonged grief reaches a
self-limiting shutdown; the same system underlies liking and social reward. Implemented as chemistry with its
own dynamics, never as a fixed cap - a cap would be a clamp.

### D13. Bad things are learned faster than good ones

The hunch (no lead yet): negative events are learned faster, more strongly and for longer than positive ones;
trust builds slowly;
memories are not deleted but become harder to reach; retrieval is triggered by association, context or place,
relevance to a goal, and emotional salience.
*Test*: acquisition against extinction curves; reinstatement of fear.

## E. Learning and Memory

### E1. Habituation is ten properties, not one decrement

Real habituation has a list of parametric properties - a negatively accelerated decrement, spontaneous recovery,
faster re-habituation across series, frequency and intensity effects, habituation below zero, stimulus
specificity, dishabituation tested on the original stimulus, habituation of dishabituation, and long-term
habituation across sessions. A decrement alone fails most of them; stimulus specificity is the cheapest test
that separates habituation from fatigue. Use a fixed number of trials, never run-to-criterion.
*Why*: tap habituation is a classic worm preparation for exactly these properties (sharpens H6).
*Leads*: Thompson and Spencer 1966; Rankin et al. 2009; Groves and Thompson 1970; Stanley 1976; Wang and Arbib
1992.

### E2. Novelty as mismatch, proven by an omission

Novelty is a prediction error against a learned model of the stimulus, not "not seen recently". The proof is the
omission response: after a regular pattern, withhold the expected stimulus - a comparator notices, a decaying
filter cannot. In oddball tests include a control with many equally rare stimuli.
*Leads*: Sokolov 1963; Näätänen et al. 2007.

### E3. Three kinds of learning in three places

Habituation and sensitisation in the reflex layer; classical conditioning by co-activation in attractor memory;
operant conditioning as a prediction-error learner reshaping the behaviour sequencer; latent learning for free,
as attractors form during exploration without reward. Learning stays local - no backpropagation.
*Test*: blocking and overshadowing; latent learning.
*Leads*: Schultz, Dayan and Montague 1997; Rescorla and Wagner 1972; Tolman 1932.

### E4. Hebbian learning needs a stabiliser and a "used together" rule

Unbounded Hebbian growth is a positive feedback loop: pay for a stabiliser (normalisation as in Oja's rule, a
sliding threshold as in BCM, or homeostatic scaling). Count a pair as co-active only when both ends were used,
not merely resting high - otherwise hubs at rest grow without input. Keep association separate from evidence;
cue competition (blocking) discounts redundant sources.
*Test*: co-activate a pair many times, then show unrelated cues - the pair must never capture the activity, and
its weight must saturate. (Sharpens H9.)
*Leads*: Oja 1982 (already cited and verified in [README.md § H9](README.md#h9-associative-memory-written-by-hebbs-rule));
Bienenstock, Cooper and Munro 1982; Turrigiano 2008; Zenke and Gerstner 2017.

### E5. Memories that bind what, where, when and how it felt

Each memory attractor binds place, sensory signature, body state, modulator state, a time tag and valence. A
partial cue completes it; the recalled memory pulls the modulator state toward the stored one (mood-congruent
recall) and is recalled more easily where it was formed.
*Leads*: Clayton and Dickinson 1998; Teyler and DiScenna 1986; Whittington et al. 2020.

### E6. Separate, complete, consolidate, replay

A sparse separation step before recurrent completion; a fast specific store beside a slow general one;
high-arousal events consolidated first; replay during sleep moving memories between stores; pruning by age,
rarity and low emotion.
*Test*: interference between similar memories with and without separation; retention after sleep against none.
*Leads*: McClelland, McNaughton and O'Reilly 1995; Foster 2017.

### E7. Promote after spaced recurrence; merge when predictions agree; never delete

Generalise from an episode only after several spaced recurrences in different contexts; decay salient events more
slowly by the size of their salience, not its sign; merge two representations when the merged form is shorter or
when they predict the same future; demote, never wipe.
*Leads*: Crutchfield and Shalizi 1999; Stolcke and Omohundro 1993.

### E8. Two strengths: forgetting loses influence, not the trace

A storage strength that grows slowly and does not decay, and a retrieval strength that rises fast and falls with
disuse; a hard successful retrieval strengthens more than an easy one.
*Why*: savings in relearning - an animal phenomenon that a two-strength memory reproduces.
*Test*: spaced against massed practice; relearning savings.
*Leads*: Bjork and Bjork 1992.

### E9. Attention as a conserved economy

A fixed total of importance circulates; units are paid for being used and charged rent only while in focus, so
monopolies correct themselves; a simple controller holds total activity in a band; integer units make
conservation checkable.
*Test*: conservation holds under random input; no unit monopolises.

### E10. The input reshapes the attractor landscape

During retrieval, weight each stored pattern by its alignment with the current input, so only matching memories
become deep wells: blended or shifting cues settle on one clean memory instead of smearing. It buys robustness to
mixed cues, not capacity.
*Why*: the worm's senses are often mixed - two gradients, a gradient and a touch.
*Leads*: Betteti et al. 2025; Mongillo, Barak and Tsodyks 2008. (Sharpens H5.)

### E11. Completion, with gates

Complete a partial cue only to memories one or two steps away, with a threshold, a cap and a penalty on fan-out,
so indirect evidence never outweighs direct input.
*Test*: a false-memory probe; completion with many associates against one.

### E12. Drives retrieve what could satisfy them

Active drives raise memories that could satisfy them, as a bounded multiplier that never admits a memory below
threshold.
*Test*: satiation removes the bias; ablation changes only drive-congruent recall.

### E13. Curiosity as expected information gain

Rank what to explore by how much a look is expected to move the belief - for a simple success-and-failure belief,
perhaps twice its posterior variance (a candidate to derive) - which falls away on pure noise, so an unlearnable random source stops attracting.
Never rank by novelty or entropy alone; agents get hooked on coin flips.
*Test*: a fresh, little-known target beats a saturated one; a noise source's score goes to zero.
*Leads*: Oudeyer and Kaplan 2007; Kakade and Dayan 2002; Orseau, Lattimore and Hutter 2013.

### E14. Sleep as a bounded replay pass, driven by pressure

Sleep pressure rises with learning load and falls only with sleep; sleep is a triggered replay pass with a hard
cap, restoring receptors and paying down pressure; replay may never write more than real experience.
*Why*: worms sleep - developmentally timed and after stress.
*Test*: paired copies, one allowed to sleep and one kept awake, with verdict rules fixed in advance.
*Leads*: Borbély et al. 2016; Tononi and Cirelli 2014; Raizen et al. 2008 is already cited and verified in
[README.md § H8](README.md#h8-brain-chemistry-as-modelled-substances).

### E15. Lessons about attractor memories

Matej's experience with attractor memories, unrecorded and not evidence until reproduced here: keep storage flat rather than tree-shaped; for exact recall use a
fixed high inverse temperature; at low temperature, structured memories keep their category - so temperature
chooses the level of abstraction ("gist" recall); greedy coarse-to-fine descent compounds its errors; adding
category prototypes to the store hurts recall of the specific ones; annealing did worse than a fixed temperature.
*Test*: each claim reproduced in a notebook before anything is built on it.
*Leads*: Ramsauer et al. 2020; Mézard, Parisi and Virasoro 1987.

### E16. The spectrum as a health check

The eigenvalue spectrum of the stored patterns' similarity matrix as a cheap monitor of a memory's health -
structured stores show regular cascades that symmetry breaking splits predictably.

### E17. Maps at several scales

Place codes at geometrically spaced scales identify position over a large range, like grid-cell modules; a
head-direction ring attractor once heading must be inferred.
*Why*: for rodent rungs.
*Leads*: O'Keefe and Nadel 1978; Hafting et al. 2005; Stensola et al. 2012.

### E18. Innate core systems, and the smallest consistent hypothesis

A few separable innate systems bootstrap later learning; generalising by the smallest hypothesis consistent with
the examples gives sharp learning from few exposures.
*Test*: generalisation gradients tighten from one exposure to three.
*Leads*: Spelke and Kinzler 2007; Tenenbaum and Griffiths 2001.

### E19. Development by stage

Targets from developmental data with error bars, fixed before seeing runs; tuned on one probe set and judged on
another; re-tuned per stage, because the optimum drifts. For the worm: larval stages, adult and dauer each with
their own targets.

### E20. Identity is the persisted pattern

A long-lived creature is its persisted state: never wipe it, store it in a format that grows new fields without
migrations, keep an active tier and a cold one (forgetting as demotion), and snapshot at every pause.
*Test*: old snapshots load into new builds bit-faithfully.

## F. Wild Hunches

These are the boldest. Each is worth a notebook before it is worth anything else.

### F1. Phase is the missing ingredient: complex amplitudes

A network of co-active candidates is a classical shadow of quantum mechanics - superposition as many candidates,
amplitude as activation times confidence, sampling as collapse, order effects as non-commuting measurements -
missing only interference, which needs complex phase. Renou et al. 2021 propose a test that would rule out
real-valued quantum theory under stated assumptions, and experiments reporting it followed (to be found and
verified); so if minds share physics' kind of mathematics, a real-valued brain would be a shadow. The one
interpretable phase is timing: phase coding against a brain rhythm. Climbing the Cayley-Dickson ladder,
quaternions lose commutativity, octonions associativity, and sedenions have zero divisors - two non-zero
amplitudes multiplying to nothing. A rule of thumb: go complex only when a measured gap forces it. (This feeds
STATE.md's quantum-like Program, H10.)
*Test*, pre-registered: interference where two paths reconverge; order effects on a neutral probe; the exponent
of the selection rule; a merge probability between two phase-carrying states as cos²(Δθ/2).
*Leads*: Renou et al. 2021; Busemeyer and Bruza 2012; Pothos and Busemeyer 2013; Baez 2002.

### F2. Cognition on the tree-like side of the numbers

Ostrowski's theorem gives two kinds of completion of the rational numbers - the real and the p-adic. The hunch:
physics lives on the real side with continuous symmetry, cognition on the tree-like (ultrametric) side with
discrete, hierarchical symmetry - categorical perception, sharp concept boundaries, discrete reasoning steps, a
few working-memory slots. The usable part: discrete state machines and attractor labels on top of continuous
modulator state.
*Test*: ultrametric structure in the creature's learned similarity between states.

### F3. Memories as codewords of an error-correcting code

Draw memory states from a highly symmetric error-correcting code (the extended binary Golay code is the
classic): recall from any few features would be equivalent, a few corrupted features corrected, more detected.
*Test*: noise tolerance at the code's distance; invariance of recall to which features are given, against
correlation-matched controls.
*Leads*: Conway and Sloane 1999.

### F4. Stable thoughts as idempotents of an exceptional algebra

In the 27-dimensional exceptional Jordan algebra over the octonions, a stable thought would be an idempotent
reached by repeated squaring, and binding a three-way triple product. The boldest of the algebra hunches; no
engineering use is proposed.
*Test*: block structure of that dimension in learned representations; fixed points of repeated squaring.
*Leads*: Jordan, von Neumann and Wigner 1934; Albert 1946; Baez 2002.

### F5. Self-similar groups as the symmetry of concept hierarchies

Concept hierarchies as the action of self-similar automaton groups (Grigorchuk's group is the famous one) on
rooted trees: growth between polynomial and exponential, statistics similar across scales, recall respecting
subtrees, and two forced kinds of relation - one that goes deeper and one that stays at a level. Methodology
lesson from the maintainer's first tries (unrecorded, not evidence): a small dense graph saturates too fast to show any growth regime, so use a large sparse
tree and a positive control.
*Leads*: Grigorchuk 1980; Bartholdi, Grigorchuk and Nekrashevych 2003; Nekrashevych 2005.

### F6. Critical brains as optimal expanders

A brain at criticality might wire like an optimal expander graph - mixing information fastest - with the zeros of
its graph's zeta function on the Ramanujan circle, and its layers as scales of a renormalisation flow.
The maintainer's first tries came out null (unrecorded, not evidence); the test is cheap to repeat at the worm's scale.
*Leads*: Lubotzky, Phillips and Sarnak 1988; Terras 2011.

### F7. Concepts consolidate only when local and global agree

A concept should consolidate only if all the contexts it appeared in agree - a local-global coherence condition,
like the Hasse principle in number theory - where a flat network would average the disagreements away.
*Test*: consolidation of a concept with consistent against inconsistent contexts, below memory capacity.

### F8. Three legs: fixed primitives, critical dynamics, learned geometry

A blueprint for any viable mind: fixed discrete primitives (the reflexes), drive dynamics poised at a bifurcation,
and learned geometric representations; a mind with all three should show an autonomy that a single-leg system
lacks.
*Test*: remove each leg in turn and measure the richness of behaviour.

### F9. Analogies from mathematics, kept for their testable parts

A set of analogies between mathematics and mind - flow as superconductivity and distraction as vortices,
attention as an energy gap, memory as a causal loop rather than a stored file, creativity as raised cognitive
temperature, sleep and waking as phases. The testable parts: a cognitive temperature tied to exploration (see
E15), sleep and waking as a phase transition with its signatures, and the order in which functions return after
anaesthesia.

### F10. Watch for a self-model

A creature can want only what it can model. A safety threshold worth watching as the ladder climbs: behavioural
signs of self-modelling. Keep plasticity bounded, and no loop that learns to change the architecture itself.
*Test*: a measurable self-model criterion checked before each new rung.

### F11. A scan for latent pathology

Runtime detectors catch only what has already happened. A periodic offline walk over the creature's connections,
during sleep, could look for configurations that almost match a known bad pattern - an incipient runaway clique,
a kindling-prone pair - before anything fires.
*Test*: near-miss configurations predict later capture events.

### F12. Predict the consequence, then refine

Before acting, predict the likely consequence and refine the action; meanwhile, tag what to store as short- or
long-term important, and refine procedures offline. A sketch of prediction and consolidation together, for
higher rungs.

### F13. A memory that renormalises time, shaped like MERA

The multi-scale entanglement renormalisation ansatz (MERA) is a tensor network from quantum many-body physics: layers
of small tensors that disentangle neighbours and then coarse-grain them, so that structure at every scale is kept at a
cost that grows only logarithmically. The hunch: a creature's memory built the same way over *time* - recent moments
in fine detail, older ones merged layer by layer into fewer, coarser summaries - would hold the worm's many time
scales at once, from a reversal's seconds to the hours of food search and lethargus, in a small fixed or slowly
growing memory. The fast-weight memories tried in H11 forget with a single decay rate; this would not.
*Test*: on a task whose answer depends on events at several time scales, a MERA-shaped temporal memory against a
stack of fast-weight memories with several decay rates, at the same memory and compute; refuted if it does no better.
*Leads*: Vidal 2007; Vidal 2008; Swingle 2012.

### F14. Thinking in loops: automata, their groups, graphs and knots

Much of thinking is loops and iterations, so the mathematics of loops may describe it. Four lenses: behaviour as a
finite automaton (H2's small set of states), and the Krohn-Rhodes theorem's decomposition of any finite automaton into
a cascade of simple groups and resets - a way to split a learned brain into a reversible, rhythm-like part and a memory
part (F5 takes the automaton groups another way); the wiring diagram as a graph whose cycles and motifs are where loops
could live; and knot and braid theory for the closed paths the body traces in its space of shapes, which several
rhythms together braid - invariants that could become benchmark measures of rhythm, not only a way to build a brain.
*Test*: each lens is refuted where it explains nothing that a plain count of states, or a plain phase measure, does
not already: the decomposition of a trained brain predicts no behaviour better than its state count; a braid invariant
of the worm's posture separates no strains or conditions that phase alone does not.
*Leads*: Krohn and Rhodes 1965; White et al. 1986; Stephens et al. 2008; Birman 1974.

### F15. Rhythm and direction as a Hopf fibration

The Hopf fibration splits the three-sphere - the unit quaternions, or pairs of complex numbers of unit length - into
circles, one over each point of the ordinary sphere: every state is a direction on the sphere plus a phase around its
circle. The hunch: a creature's state held this way keeps what it does (a direction - a heading, a choice among
behaviours) and the rhythm it does it with (the phase of the body wave) in one object, the two separable by
construction yet moving together, as steering and undulation do in the worm. The same geometry is the Bloch sphere of a
two-level quantum state, which ties it to H10's complex-valued brain.
*Test*: a module whose state lives on the three-sphere, read out through the fibration, against one holding a plain
angle and a separate phase, at the same compute and memory, on a task that couples steering to the body's rhythm;
refuted if the fibred state does no better.
*Leads*: Hopf 1931; Urbantke 2003.
