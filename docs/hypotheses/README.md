# Hypotheses About the Brain

A starting set of ideas about how a creature's brain should be built, written so that each can be tested
against the benchmark. They are hypotheses, not decisions: any of them may be refuted, replaced or joined by
better ones, with the reasons written in the journal.

## The Shape of the Brain

The working assumption is that the brain is **modular and heterogeneous**: a few compartments with different
jobs, each built the way its job needs rather than all of one architecture - as in animals, where a reflex
arc, a rhythm generator and the circuits that set behavioural state work in very different ways. Five
principles go with it:

- **Every module's state can be read.** Its numbers mean something a person can inspect, so a behaviour can
  be traced to the module that produced it.
- **Senses only.** Every module sees the world through the creature's senses and nothing else.
- **Earned complexity.** A module is added, or made richer, only when a behaviour the benchmark measures
  demands it.
- **Lean.** The real worm manages with 302 neurons ([White et al. 1986](https://doi.org/10.1098/rstb.1986.0056));
  a module earns its cost in compute and memory as well as in behaviour.
- **At the animal's pace.** The brain's speed of thought is indistinguishable from the real animal's. Every
  module works on the time scales measured in the real worm - its reaction latencies, the time constants of
  its states - counted in simulated time, neither faster nor slower.

## The Hypotheses

Each states the idea, why biology suggests it, what behaviour it should produce, and how the benchmark would
show it. All are untested.

### H1. A Fast Reflex Arc

Direct, weighted pathways from senses to motor commands - touch at the head drives reversal, touch at the tail
drives forward acceleration - running before and alongside every slower module, and able to override them.
The worm's touch circuit is the classic example ([Chalfie et al. 1985](https://doi.org/10.1523/JNEUROSCI.05-04-00956.1985)).
*Predicts*: touch-evoked reversals with the latencies and probabilities real worms show.

### H2. Behaviour as a Small Set of States

Locomotion organised as a few states - forward run, reversal, turn, pause - with transition probabilities that
senses and internal state reshape. Pirouettes, the bursts of reversals and turns worms use to reorient,
follow this pattern ([Pierce-Shimomura et al. 1999](https://doi.org/10.1523/JNEUROSCI.19-21-09557.1999)).
*Predicts*: the run-length and reversal-rate distributions of the reference data; the transition matrix is
itself readable.

### H3. A Slow Internal State That Reshapes Everything Else

A low-dimensional internal state (hunger, arousal, threat) that changes slowly and modulates the other modules:
the state transitions of H2, the reflex gains of H1, and which senses get attention. In the worm, serotonin and
the neuropeptide PDF set the opposing states of dwelling and roaming
([Flavell et al. 2013](https://doi.org/10.1016/j.cell.2013.08.001)), and tyramine coordinates the escape
response ([Pirri et al. 2009](https://doi.org/10.1016/j.neuron.2009.04.013)).
*Predicts*: roaming and dwelling as distinct, long-lived regimes with the durations real worms show.

### H4. Rhythm Through the Body

Undulation produced by a rhythm generator coupled to proprioception, so the bend travels down the body as each
segment feels its neighbour bend; in the worm, proprioceptive coupling within motor neurons drives forward
locomotion ([Wen et al. 2012](https://doi.org/10.1016/j.neuron.2012.08.039)). On the Grid the joint angles
and servo loads are the proprioception.
*Predicts*: posture eigen-shape statistics and undulation frequency that adapt to load the way the worm's do.

### H5. Settling Into Attractors

Decisions and memories as attractor states of a recurrent dynamical system that the creature's state settles
towards. Classical Hopfield networks store memories this way
([Hopfield 1982](https://doi.org/10.1073/pnas.79.8.2554)); modern continuous Hopfield networks store far more
and turn out to share their update rule with transformer attention
([Ramsauer et al. 2020](https://arxiv.org/abs/2008.02217)); continuous-time formulations treat the settling as
an ordinary differential equation ([Chen et al. 2018](https://arxiv.org/abs/1806.07366)). How such memories
are written - Hebbian learning, Boltzmann machines, dense associative memory - is H9.
*Predicts*: behaviour that persists and switches the way an attractor landscape would - more relevant for
later rungs than for the worm, and tested only when a measured behaviour asks for it.

### H6. Small, Bounded Learning

Plasticity that is small, bounded and visible: the worm habituates to repeated taps
([Rankin et al. 1990](https://doi.org/10.1016/0166-4328(90)90074-O)).
*Predicts*: tap habituation curves - response probability falling with repeated stimulation and recovering
with rest - like the worm's.

### H7. Poised Dynamics

Recurrent dynamics kept near the boundary between order and chaos, where neural activity shows avalanches
with power-law sizes ([Beggs and Plenz 2003](https://doi.org/10.1523/JNEUROSCI.23-35-11167.2003)), checked as
a health measure of the recurrent modules rather than as a behaviour.
*Predicts*: nothing directly behavioural; a diagnostic to watch as the brain grows.

### H8. Brain Chemistry as Modelled Substances

H3's internal state made concrete: a small set of neuromodulators, each modelled as a substance rather than a
mood. Each is a broadcast quantity with three state variables:

- a **phasic level**, the burst this moment's input causes;
- a **tonic level**, a slow running average with its own time constant;
- a **receptor sensitivity** that wears down with use and recovers slowly (tachyphylaxis), partly restored
  during rest.

What a modulator does is set generic parameters of the other modules through a bounded, documented map: a
transition rate of H2, a reflex gain of H1, the frequency or amplitude of H4's rhythm, the learning rate of H6.
Its input comes only from quantities the creature already senses or computes, its dynamics are deterministic,
and every modulator has a switch that pins it at baseline, so its contribution can be measured by turning it
off.

The worm suggests the cast. Dopamine slows the worm when it meets food and serotonin slows a hungry worm
further ([Sawin et al. 2000](https://doi.org/10.1016/S0896-6273(00)81199-X)); serotonin and PDF hold the
dwelling and roaming states ([Flavell et al. 2013](https://doi.org/10.1016/j.cell.2013.08.001)); tyramine
coordinates the escape response ([Pirri et al. 2009](https://doi.org/10.1016/j.neuron.2009.04.013)); and the
worm has a sleep-like state between larval stages, lethargus ([Raizen et al. 2008](https://doi.org/10.1038/nature06535)),
which suggests a slowly rising pressure discharged by quiescence.

**No clamp on feeling.** The chemistry spans the whole range a real animal has, the aversive and stressful
included: threat, stress, malaise and avoidance are part of what an animal is, and part of what the benchmark
measures. Worms learn to avoid the smell of bacteria that made them ill
([Zhang et al. 2005](https://doi.org/10.1038/nature04216)), and cellular stress puts them into a protective
sleep-like state ([Hill et al. 2014](https://doi.org/10.1016/j.cub.2014.08.040)). Bounds exist only where
biology has them - a receptor saturates, a store runs empty - never as a ceiling on how bad a creature may
feel.

*Predicts*: the slowing responses, the long-lived roaming and dwelling regimes, escape coordination, and
quiescence bouts in the proportions real worms show - and, under ablation, the deficits that worms lacking
each signalling pathway show.

### H9. Associative Memory, Written by Hebb's Rule

A family to explore together, as candidates for how a creature remembers. Memories as attractors of a
recurrent network (H5), written into its connections by Hebbian learning: connections between cells that are
active together grow stronger ([Hebb 1949](https://doi.org/10.4324/9781410612403), cited in its modern
reprint), kept bounded and stable by normalisation, as in Oja's rule
([Oja 1982](https://doi.org/10.1007/BF00275687)). Boltzmann machines make such a network stochastic: it learns
a probability distribution and samples from it rather than settling on one answer
([Ackley, Hinton and Sejnowski 1985](https://doi.org/10.1207/s15516709cog0901_7)) - a candidate for behaviour
that varies rather than repeats. Dense associative memories sharpen the network's energy function so that the
same number of neurons stores far more patterns ([Krotov and Hopfield 2016](https://arxiv.org/abs/1606.01164)),
the line that leads to the modern Hopfield networks of H5.

The worm does learn by association: it learns to avoid the smell of bacteria that made it ill
([Zhang et al. 2005](https://doi.org/10.1038/nature04216)). Which of its learned behaviours the Grid can
measure - it has no smell yet - is for the literature to say. For a brain on the worm's scale the questions
are concrete: which behaviour such a memory would carry, how few neurons it needs, what it costs to run,
sampled or settled, and whether its state can still be read.

*Predicts*: behaviour that changes with experience the way the worm's does, with the change traceable to
learned connections that can be read; tested once the benchmark has a measure of learning.

### H10. A Quantum-Like Brain

*A hunch*, the maintainer's: the states and connection strengths of a brain are complex numbers rather than real
ones - phases as well as magnitudes - and perhaps numbers further up the hypercomplex tower (quaternions and
beyond). Complex-valued associative memories exist: units that take one of several phases, as in the phasor
networks of [Noest 1988](https://doi.org/10.1103/PhysRevA.38.2196) and the multistate memory of
[Jankowski, Lozowski and Zurada 1996](https://doi.org/10.1109/72.548176), with quaternion-valued Hopfield networks
after them ([Kobayashi 2017](https://doi.org/10.1016/j.neucom.2017.02.044)); these are cited for what they
propose, not read in full - and the first test's notebook says its update rule follows the first two, an
attribution unchecked until they are read. In the worm, the undulation of [Stephens et al. 2008](https://doi.org/10.1371/journal.pcbi.1000028)
is already one phase. Related ideas wait in [Matej's ideas and hunches](matej-hunches.md), F1 among them.
*Predicts*: a complex or hypercomplex module that does more than a real-valued one of the same compute and memory -
stores more, or produces a measured behaviour the real one cannot.
*Refuted, module by module*: when it scores no better than a real-valued module of the same compute and memory,
judged by a statistic fixed before the run; and, under ablation, when its gain survives being made real-valued.
*Tested so far*: the first toy test, phasor against real associative memory at the same memory budget, in
[notebooks/h10_phasor_memory.ipynb](../../notebooks/h10_phasor_memory.ipynb): **refuted at that level** - the
phasor network was no better (its interval included zero) at twice the multiply-adds. The 2026-10-13 journal has
the numbers; the test is kept, to return to.

### H11. A Hebbian Hopfield Transformer

*A hunch*, the maintainer's: one module that is at once a Hopfield network, a Boltzmann machine, a Hebbian memory,
trained by backpropagation, and a transformer block. The bridge exists: the attention of a transformer is one update
of a modern Hopfield network, a softmax - a Boltzmann distribution - over stored patterns
([Ramsauer et al. 2020](https://arxiv.org/abs/2008.02217)), and linear attention is a fast-weight memory written by an
outer-product, Hebbian rule ([Schlag, Irie and Schmidhuber 2021](https://arxiv.org/abs/2102.11174)). The hybrid
writes each token's pattern into a classical Hopfield weight matrix by Hebb's rule as the sequence arrives
([Hopfield 1982](https://doi.org/10.1073/pnas.79.8.2554)), so its memory stays a fixed size however long the
sequence, where a transformer's grows with every token; it reads by letting the query settle for a few steps of
noisy (during training), temperature-controlled Hopfield dynamics, the mean-field form of a Boltzmann machine's
sampling ([Ackley, Hinton and Sejnowski 1985](https://doi.org/10.1207/s15516709cog0901_7)); and backpropagation
trains everything slow - the projections, the temperature, the decay, the rest of the block - through the
unrolled settling. It is pretrained, then learns on its own: once backpropagation has shaped the slow weights they
are frozen, and what is new in each life - which key goes with which value - is written by the Hebbian rule alone,
with no gradient and no teacher. For a worm-sized brain the draw is the fixed memory: H9's associative memory with H5's
attractors, at a cost that does not grow with time.
*Predicts*: on a recall task, better recall than a one-step Hebbian memory (linear attention) given the same
memory, with the gain carried by the settling, and holding up better when the frozen model must learn on its own
more than it was pretrained for.
*Refuted*: when it recalls no better than linear attention with the same number of memory cells, judged by the
statistic fixed before the run; and, under ablation, when its gain survives removing the settling.
*Tested so far*: two toy tests of associative recall, in
[notebooks/h11_hebbian_hopfield_transformer.ipynb](../../notebooks/h11_hebbian_hopfield_transformer.ipynb): **refuted
at that level** - the hybrid recalled 18% against 99.8% for linear attention of the same memory, its settling cost 66
points against one step of the same read, and frozen and given streams up to three times as long it stayed 0.79 to 0.84 below. The 2026-10-14
journal has the numbers and a hunch about why (the settling falls into the mixture states of Hebb's rule). A pilot of
v2, written by the delta rule ([its notebook](../../notebooks/h11v2_delta_pilot.ipynb)), recalled 0.89 and its
settling did no harm that the pilot could detect, but it did not beat a DeltaNet memory of the same size. A third test,
recall from a noisy cue ([its notebook](../../notebooks/h11v3_noisy_cue.ipynb)), refuted v2's design there too: 0.59
against DeltaNet's 0.94, the settling harmful. A fourth, v4, a DeltaNet whose cue settles in a Hopfield memory of
its keys ([its notebook](../../notebooks/h11v4_keyclean.ipynb)), recalled 0.75 against 0.94 and its settling helped at
no noise level. In none of the four did the hybrid beat its matched control, or the settling its own ablation. A pilot of a fifth,
v5, with binary keys ([its notebook](../../notebooks/h11v5_binary_keys.ipynb)), did not change that.

### H12. A Plain Worm Brain

*A hunch*, written at the maintainer's request after H11's five tests: the brain that finally passes the worm's
benchmark will look plain. The worm's 302 neurons and their connections were mapped by
[White et al. 1986](https://doi.org/10.1098/rstb.1986.0056). The hunch *assumes*, without a source yet, that most of
the worm's behaviour is innate and its wiring laid down in development rather than learned - how much the wiring varies
between animals is for the literature to say (a later developmental study,
[Witvliet et al. 2021](https://doi.org/10.1038/s41586-021-03778-8), is relevant and not yet read). If the assumption
holds, the "pretraining" is evolution's, and most of the Program will be fixed structure found against the benchmark,
with a little learning on top. Its parts: a rhythm generator coupled to the body (H4); a handful of behavioural states
switched at the worm's pace (H2); a few slow state variables standing for neuromodulators (H3, H8); a small memory
of fixed size, written by a local rule while the creature lives, as H11's tests found a frozen fast-weight memory can
([docs/research/findings.md § M2](../research/findings.md#m2-a-frozen-self-written-memory-keeps-binding-pairs-in-streams-longer-than-it-was-trained-on---observed));
and senses at the worm's own resolution. No attractor settling and no transformer, unless one earns its place
([§ M3](../research/findings.md#m3-hopfield-settling-did-not-earn-its-place-in-five-versions---tested-v1-v3-v4-pilot-v2-v5)):
of the order of 300 units and tens of thousands of numbers, cheap enough for a microcontroller - an estimate, not yet
a measurement. Much of what looks like intelligence would come from the loop of brain, body and Grid.
*Predicts*: a Program built from these parts alone reproduces the reference statistics - turning, reversals, roaming
and dwelling bouts, habituation - at the worm's pace, once the benchmark exists.
*Refuted*: when no Program within about 100 000 numbers and a microcontroller's compute passes the benchmark where
one beyond that bound does; the richer parts are then credited by ablation, one by one.
*Tested so far*: nothing; it needs the benchmark.

### H13. Mechanism-Matched Modules, Trained as a Whole, Then Ground Down

*A hunch*, the maintainer's (2026-10-17): build each part of the brain from the kind of mechanism that best matches
that part's job - an oscillator for undulation, a switch for roaming and dwelling, a depressing synapse for
habituation, a Hebbian store for memory - at the level of method rather than neuron (Marr's algorithmic level;
[Marr 1982, reissued 2010](https://doi.org/10.7551/mitpress/9780262514620.001.0001)); train the slow parameters of all the modules together
against the behaviour, leaving within-life learning local; then grind: ablate each module, drop what does not earn
its place, simplify, and repeat, checking every step against held-out recordings. It extends the working assumption
above and hunches A6, C4, C5 and E3 of [Matej's ideas and hunches](matej-hunches.md) with two things they do not
state: joint training of the whole heterogeneous brain, and the grind as a loop.
*Predicts*: the loop recovers a brain's mechanisms from its behaviour statistics alone, finds by ablation the modules
the behaviour does not need, and the held-out check flags a module that only memorises the development set.
*Refuted*: at the level of a proof of concept, when on synthetic worms with a known hidden brain the loop fails the
criteria written in the journal before the run (2026-10-17, *H13's Proof of Concept on Synthetic Worms*); on the
real worm, only once the benchmark is frozen, as the rules require.
*Tested so far*: a proof of concept in four rounds, on synthetic worms only
([round 1](../../notebooks/h13_modular_poc.ipynb), [rounds 2 to 4](../../notebooks/h13_rounds_2_to_4.ipynb); the
journal of 2026-10-17 has the numbers). Ablation told real modules from the decoy in every run (round 1's under the
chosen reading of the noise), and pruning it left a better fit each time; recovery of the hidden brain is refuted -
first, as round 2 showed, for want of statistics, then, with five more, for one optimiser seed of three passing where two
were needed; a fifth round, targeting what remained, passed for three seeds of five where four were needed; a diagnosis found noise in the
fitter a real part of the remaining bias, with more simulated worms per candidate passing four of five but still
missing the switching rate in three; and a precision test on fresh populations found the fitter's noise the larger part of
what remained; eight seeds on a noise-free target then showed the fitter biased on average, not merely noisy, and a
polish test weakened the reading that the search's smoothing makes it; a longer search failed its rule, though post hoc
it brought the outlying seeds in; and with 128 worms per candidate and
360 generations the whole loop passed criterion 1 for five seeds of five, with the switching rate still biased high but
inside the tolerance; on five fresh populations it passed for all five, with no bias in the switching rate shown; a tap
protocol with a rest did not separate habituation's two parameters, and a profile showed why: the statistics see almost
only their combination; none of the five tap
protocols tried at intervals of 10 s or more did better, but a train of taps every 2 s separated the two in 8 of 8 fits on the near noise-free target with X pinned (in the
whole loop on five fresh 30-worm populations it did not carry, 1 of 5; the interval's realism is unchecked, see
[the journal](../journal/2026-10-22.md)).

### H14. Graded Slow Knobs, Change-Detecting Senses and Individuality

*A hunch*, brought by the maintainer from a conversation with another Claude session (2026-10-30). It sharpens H12 with
three claims that H12 does not make; the rest of that conversation's picture is already H2, H3, H4, H6 and H12.

- **Slow state as a graded knob, not a switch.** Hunger and the roaming-dwelling state act as continuous, brain-wide
  gains on turning and roaming rather than as gates that switch a behaviour on or off. Serotonin and PDF set roaming
  and dwelling (H3; [Flavell et al. 2013](https://doi.org/10.1016/j.cell.2013.08.001)); whether their effect on
  turning rates is graded is the claim, not yet read at a source.
- **Senses that report change.** The salt-sensing pair ASE splits into an ON-cell (ASEL, stimulated by increases in
  NaCl) and an OFF-cell (ASER, by decreases), and both responses are transient, "indicating that ASE neurons report
  changes in concentration rather than absolute levels" ([Suzuki et al. 2008](https://doi.org/10.1038/nature06927),
  abstract). Navigation then needs only two rules on such signals: pirouettes when things get worse (H2;
  [Pierce-Shimomura et al. 1999](https://doi.org/10.1523/JNEUROSCI.19-21-09557.1999)) and weathervaning, gradual
  curving up a gradient ([Iino and Yoshida 2009](https://doi.org/10.1523/JNEUROSCI.3633-08.2009)).
- **A remembered set point, not a map**, and **individuality**. Learning keeps a few slow numbers, such as a
  preferred temperature, rather than a map of places. And worms differ from one another persistently: within
  isogenic populations raised alike, a fraction of individuals have "consistent, non-genetic behavioral biases that
  persist across development", shaped by neuromodulatory systems
  ([Stern et al. 2017](https://doi.org/10.1016/j.cell.2017.10.041), abstract). A brain whose every individual is
  identical would fail an indistinguishability test that looks at individuals.

*Read so far*: the two quotations above are from the papers' abstracts (Europe PMC); the full texts are not yet
read, and the claim of graded neuromodulatory gains has no source yet.
*Predicts*: on the toy worm, a hunger *gain* on steering (steering scaled by hunger) does better across fresh
layouts than the on/off hunger gate of findings.md § D4; on the benchmark, a Program needs per-individual parameters
to match the spread between reference worms, not only their mean.
*Refuted*: on the toy worm, if a graded hunger gain does no better than the on/off gate across fresh layouts, by a
rule set before the run; on the benchmark, if a Program with identical individuals matches the reference spread
between worms.
*Tested so far*: nothing.

## Adding a Hypothesis

A pool of candidates waits in [Matej's ideas and hunches](matej-hunches.md): unverified, to be triaged and
promoted here one by one.

Add a section with the same four parts: the idea, the evidence (public sources, linked), the behaviour it
predicts, and how the benchmark would show it. Record in the journal why it was added.

Bold ideas are welcome here - strange architectures, mathematics borrowed from far away, hunches no one has
tried on a worm. A hunch with little evidence yet is marked *a hunch* and says what would refute it; it is
tested like any other, and if it fails, that is written up as carefully as a success.
