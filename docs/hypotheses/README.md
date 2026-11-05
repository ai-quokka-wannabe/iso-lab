# Hypotheses About the Brain

A starting set of ideas about how a creature's brain should be built, written so that each can be tested
against the benchmark. They are hypotheses, not decisions: any of them may be refuted, replaced or joined by
better ones, with the reasons written in the journal.

![From an idea to a brain, as a flowchart: the maintainer's hunches and ideas from people become a plan, written
before any run; the plan is tested on the toy worm, in a notebook or on the Grid. An idea that survives is promoted
to a hypothesis or a task in STATE.md, and a module stays in a brain only if switching it off costs the worm on the
benchmark. One that fails or is unclear is kept under To Revisit or Blocked, with the next idea to try, and tried
again.](../figures/diagram_ideas.svg)

*How an idea travels through the lab. Drawn by [notebooks/figures.ipynb](../../notebooks/figures.ipynb).*

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
show it. Each says under *Tested so far*, where it has one, how far it has been tested; the rest are untested.

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
measure - it has carried a sense of food odour since Program ABI 11, and no other smell - is for the literature to say. For a brain on the worm's scale the questions
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
  abstract). Chemotaxis to salt uses two mechanisms in parallel: pirouettes, bursts of turns when things get worse
  (H2; [Pierce-Shimomura et al. 1999](https://doi.org/10.1523/JNEUROSCI.19-21-09557.1999)), and weathervaning,
  gradual curving up a gradient ([Iino and Yoshida 2009](https://doi.org/10.1523/JNEUROSCI.3633-08.2009)). Whether
  weathervaning, which that abstract describes as a response to a spatial gradient, needs only change signals is not
  yet read.
- **A remembered set point, not a map**, and **individuality**. Learning keeps a few slow numbers, such as a
  preferred temperature: worms grown at a temperature between 16 and 25 °C migrate to it on a thermal gradient, and
  adapting to a new one "takes several hours" ([Hedgecock and Russell 1975](https://doi.org/10.1073/pnas.72.10.4061),
  abstract). The same abstract reports that "Starved animals, in contrast, disperse from the growth temperature":
  hunger reshapes a set-point behaviour. That such numbers stand in place of a map of places is the hunch, not a
  sourced claim. And worms differ from one another persistently: within
  isogenic populations raised alike, a fraction of individuals have "consistent, non-genetic behavioral biases that
  persist across development", shaped by neuromodulatory systems
  ([Stern et al. 2017](https://doi.org/10.1016/j.cell.2017.10.041), abstract). A brain whose every individual is
  identical would fail an indistinguishability test that looks at individuals.

*Read so far*: the quotations above are from the papers' abstracts (Europe PMC); the full texts are not yet read.
The claims of graded neuromodulatory gains and of set points standing in place of a map have no source yet.
*Predicts*: on the toy worm, a hunger *gain* on steering (steering scaled by hunger) does better across fresh
layouts than the on/off hunger gate of findings.md § D4; on the benchmark, a Program needs per-individual parameters
to match the spread between reference worms, not only their mean.
*Refuted*: on the toy worm, if a graded hunger gain does no better than the on/off gate across fresh layouts, by a
rule set before the run; on the benchmark, if a Program with identical individuals matches the reference spread
between worms.
*Tested so far*: on the toy worm, the graded hunger gain beat the on/off gate across ten fresh layouts by the
registered rule (K1, +0.0377, interval +0.0046 to +0.0678), but was not shown better than never steering (journal of
2026-10-30, § H14's Graded Hunger Gain on Ten Fresh Layouts). The graded gain is also un-latched, unlike the gate;
in an ablation the gain `y[7]` in place of a step carries the lead (L1, +0.0440, +0.0096 to +0.0768), and no
contribution from removing the latch was detected (L2, −0.0063, −0.0275 to +0.0140, a latch effect up to +0.0140 not
excluded; § Graded or Un-latched? An Ablation of the Graded Hunger Gain). A step scaled by a constant 0.65 recovers
the lead (M2, +0.0423, +0.0116 to +0.0725) and grading adds nothing detectable (M1, +0.0018, −0.0298 to +0.0362;
§ Grading or Smaller Turns? A Scaled On/Off Gain), and again with the constant set to the mean gain the graded rule would have applied in the other arms' lives (P2
holds; P1, −0.0056, −0.0365 to +0.0218; § Grading or Smaller Turns, With the Constant Matched), and with three body
seeds per layout (Q2 holds; Q1, +0.0015, −0.0202 to +0.0216; § Graded, Matched and Un-latched With Three Body Seeds
per Layout): on the toy worm, the
gain helps by making turns smaller, and the graded half of the hunch is not supported there. The lead held with a second body seed (K3, +0.0441,
+0.0149 to +0.0737, seeds pooled; on the new worms alone +0.0506, +0.0161 to +0.0892), and the graded gain kept the
rich-world fix (K4; § The Graded Hunger Gain: a Second Body Seed, and the Rich Layouts). The matched step keeps it
too (W1), though the graded gain ends slightly above it in three rich layouts, where its steering below 0.3, its
grading and turn size are not separated (§ The Matched Scaled Step in the Rich Layouts). There the worms spend about
99% of their time below 0.3; zeroing the graded gain's steering below 0.3 is not shown to remove its edge over the
five layouts (Z1 fails), but removes it in the two where it was clearest, and leaves it not told apart from the
matched step (§ The Graded Gain Without Its Steering Below 0.3). With three body seeds that edge is not detected
pooled over five layouts; it varies with the layout, and where it exists an ungraded constant floor carries it
(descriptive; § A Constant Floor Below 0.3, With Three Body Seeds in the Rich Layouts). On ten fresh rich layouts that
floor was not shown to beat the matched step (§ The Constant Floor on Ten Fresh Rich Layouts). The benchmark half waits
on the benchmark.

![A forest plot of H14's toy-worm contrasts in two panels, each a difference in late energy with its 95% interval.
Ten fresh banded layouts, cold and warm pooled: the graded gain leads the greedy gate (K1, K3, and the new body seed
alone) and the un-latched step (L1), but is not shown better than never steering (K2 and the K3 run, both straddling
zero); a scaled on/off step is not told apart from the graded gain (M1, P1 and Q1 straddle zero) and itself leads the
un-latched step (M2, P2 and Q2 hold). Rich layouts: K4 and W1 stay above their -0.03 margin
in every layout; Z1, F1, F2 and H1 all straddle zero.](../figures/plot_toyworm_graded.svg)

*Every contrast above, with the journal section each is read from in the code that draws it; blue holds, orange
fails, grey is descriptive. The two panels have different scales. Drawn by
[notebooks/figures.ipynb](../../notebooks/figures.ipynb).*

### H15. The Pattern: a Small Core That Recurs

*A hunch*, from the maintainer (2026-10-30): that somewhere under the worm's behaviour, and under every animal's after
it, lies one recurring pattern of computation, the thing that would say what thought is and how it works. Put that
way it cannot be refuted, so the lab keeps a narrower, testable form of it and a standing watch for more.

- **The claim.** A small set of mechanisms, few enough to name in a sentence, earns its place by ablation in most of
  the behaviours one animal's benchmark measures, and the same set earns it again in the next animal's benchmark. The
  set is not fixed in advance; it is whatever keeps surviving. H3, H5, H7 and H14 each name candidates.
- **Why it might hold.** Nothing yet in the lab's own evidence. The hunch rests on the hope that brains share design
  across species; no source for that is cited here, and none is claimed.
- **Why it might not.** Each animal's brain may be a pile of special cases. On the toy worm so far, the more elaborate
  of two hunger mechanisms was not told apart from the plainer one (below).

*Predicts*: the ledger below converges on a short list of mechanisms that keep surviving ablation as the benchmark
grows; the list survives the move to the next animal.
*Refuted*: if, after the worm's benchmark is frozen and a lean Program scores well on it, no set of mechanisms small
enough to count as a core survives ablation in most of the behaviours measured; or if the next animal's best lean
brain keeps fewer than half of the worm's core. "Small enough", "most" and the scoring of "survives" are fixed in the
journal when the worm's benchmark is frozen, before any Program is scored on it; until then the refutation is not yet
a rule.
*The ledger*, one line per mechanism tested by a registered ablation, kept current by the standing watch in STATE.md:

All lines so far are toy-worm results about one behaviour, greedy steering under a hunger gate, from ablations with
one to three body seeds per layout and no benchmark; each is stated no more strongly than the journal section it cites.

- *A gain below 1 on steering while the hunger gate is on*: survives against the un-latched full-gain step (M2, +0.0423,
  +0.0116 to +0.0725, at 0.65; P2, +0.0496, +0.0198 to +0.0772; with three body seeds per layout, Q2,
  +0.0471, +0.0281 to +0.0644, and +0.0458, +0.0258 to +0.0654 on the new seeds alone; at 0.5572, the mean gain the graded rule would have
  applied in the 0.65 arm's lives; 0.5457 in the matched arm's own; the graded arm's own gain was not logged, and the
  match holds for the mean over layouts, not within each). In five rich layouts it costs nothing detectable against
  never steering (W1, lowest bound −0.0046; § The Matched Scaled Step in the Rich Layouts). Not shown better than
  never steering: the graded gain, whose lead over the un-latched step is of like size (L1 +0.0440), is not (pooled over
  two body seeds, −0.0305, −0.0725 to +0.0111). Journal of 2026-10-30, § Grading or Smaller Turns? A Scaled On/Off
  Gain, § Grading or Smaller Turns, With the Constant Matched, and § The Graded Hunger Gain: a Second Body Seed, and
  the Rich Layouts.
- *Grading that gain by the hunger reading*, tested together with the graded gain's small steering below 0.3: not
  detected to add anything in the banded layouts, three times (M1, +0.0018, −0.0298 to +0.0362;
  with the turn size closer to matched, P1, −0.0056, −0.0365 to +0.0218: an effect above +0.0218, about half the lead's point estimate, is excluded
  at a 95% bootstrap over ten layouts with one body seed; with three body seeds per layout, Q1, +0.0015, −0.0202 to
  +0.0216). § Grading or Smaller Turns? A Scaled On/Off Gain, § Grading or Smaller Turns, With the Constant Matched,
  and § Graded, Matched and Un-latched With Three Body Seeds per Layout.
- *Removing the gate's latch between decisions*: not detected to add anything (L2, −0.0063, −0.0275 to +0.0140).
  § Graded or Un-latched? An Ablation of the Graded Hunger Gain.
- *The graded gain's small steering below the gate's threshold*, in the rich layouts: not shown over five layouts (Z1,
  +0.0025, −0.0012 to +0.0053); its removal takes away the graded gain's edge in the two layouts where it was
  clearest (descriptive). § The Graded Gain Without Its Steering Below 0.3. With three body seeds, the graded gain's
  rich edge is not detected pooled over five layouts (+0.0014, −0.0039 to +0.0061), and neither a constant floor below
  0.3 (F1) nor grading beyond it (F2) is shown to matter as registered; descriptively, the edge varies by layout, and
  where it exists a constant floor carries it. § A Constant Floor Below 0.3, With Three Body Seeds in the Rich Layouts.
- *A constant steering below the gate's threshold*, in rich worlds: not shown on ten fresh rich layouts (H1, +0.0021,
  −0.0014 to +0.0052); eight of ten lean its way and one, 82300, is clearly worse (descriptive). § The Constant Floor on Ten Fresh Rich
  Layouts.

### H16. A Reflex Stack: Layered Wirings, No Learned Model

*A hunch*, brought by the maintainer from another Claude session, 2026-11-05. It arrives with that source's track
record: the same source's earlier hunch of graded slow knobs was not supported on the toy worm in its graded half
(H14, *Tested so far*).

- **The claim.** The worm's brain is roughly a layered control system of the kind Brooks described for a mobile robot,
  mixed with the direct sensor-to-motor wirings of Braitenberg's vehicles: a few wirings from senses to muscles give
  approach and avoidance; higher layers suppress lower ones; the world serves as its own model, so no map or learned
  model of the world is kept; and a few slow, hunger-like knobs and a few remembered numbers complete it. A Program
  built that way, and nothing more, gets through the worm's benchmark.
- **What counts as what**, fixed here before any design so that the line cannot move after a result. A *wiring* is a
  fixed mapping from senses to actions, its parameters set before a life and never changed in it; adaptation inside a
  sense pathway with a fixed, cited time constant is part of its wiring. A *slow knob* or *remembered number* is a
  scalar written by the creature's experience, by a fixed update rule, with a named counterpart in a cited source (a
  preferred temperature, after [Hedgecock and Russell 1975](https://doi.org/10.1073/pnas.72.10.4061); a hunger state);
  a stack may hold at most N = 8 of them in all. A *learned model of the world* is any state written by experience that
  is indexed by place or heading, or is updated to reduce the error of predicting a future sense value, or takes the
  experience-written state past K = 8 numbers. K and N are chosen now, as a bound of the hunch's own size, not derived
  from a source.
- **What the sources say, and what they do not.** Brooks's abstract describes "[l]ayers of control system" built "to
  let the robot operate at increasing levels of competence", made of "asynchronous modules that communicate over
  low-bandwidth channels", in which "Higher-level layers can subsume the roles of lower levels by suppressing their
  outputs" while "lower levels continue to function as higher levels are added"
  ([Brooks 1986](https://doi.org/10.1109/JRA.1986.1087032), *IEEE Journal on Robotics and Automation* 2(1):14-23;
  the abstract as Semantic Scholar serves it). The full text is openly reachable as MIT AI Memo 864
  ([MIT DSpace](https://dspace.mit.edu/bitstream/1721.1/6432/2/AIM-864.pdf), CC BY-NC by Semantic Scholar's record),
  the memo rather than the journal version; it was not read. The same abstract foresees the robot "building maps of
  its surroundings". The phrase "the world is its own best model" is commonly traced to a later paper,
  [Brooks 1990](https://doi.org/10.1016/S0921-8890(05)80025-9), *Elephants don't play chess*, *Robotics and Autonomous
  Systems* 6(1-2):3-15: a lead: its Crossref record was checked, the phrase itself not, so the lab is not sure of the
  attribution. Of Braitenberg's book ([Braitenberg 1984](https://openlibrary.org/isbn/0262022087),
  *Vehicles: Experiments in Synthetic Psychology*, MIT Press, Cambridge, Mass., x + 152 pages, ISBN 0-262-02208-7) only
  the catalogue record was read; that its vehicles are direct sensor-to-motor wirings giving approach and avoidance is
  how the book is commonly described, not checked here. That the worm's brain is built this way has no source; it is
  the hunch.
- **Why it might hold.** Only a lead, from a toy world, and none of it evidence about *C. elegans* (the journal of
  2026-11-04 says so of its own results). On the toy worm the lateral sense named the warmer side in 0.986 of scored
  decisions, and a turn towards the band's temperature also turned towards the nearest patch in 0.945; descriptively,
  the turn's correctness followed the map's choice, which pointed the band's way in 0.513 (0.424 to 0.619). Whether the
  map is what fails was registered as H-map and is *undecided* by its own rule, its interval reaching past 0.6
  (journal of 2026-11-04, § Does the Greedy Rule's Direction Fail? Each Steering Decision Scored;
  [gr-fec.md § 12](../research/gr-fec.md#12-why-the-demystification-stalled-a-post-mortem-2026-11-04)). So the senses
  and the geometry did not fail there; the map is not shown to have failed, and nothing shows that a worm without a map
  would succeed, which is this hunch's claim and untested.
- **How it differs from its neighbours.** H1 is one such wiring and H2 the switching between a few states; H12 keeps a
  small memory written while the creature lives, and H14 a remembered set point. H16 claims the sufficiency of the
  plainest of these: wirings, suppression between layers, slow knobs and a few numbers, with no learned model of the
  world at all.
- **Two neighbours, recorded and not part of the claim.** The same conversation likened the worm's switching between a
  few command states to the decide step of a sense-decide-act cycle such as Soar's
  ([Laird, Newell and Rosenbloom 1987](https://doi.org/10.1016/0004-3702(87)90050-6); only the Crossref record read).
  In the worm, brain-wide calcium imaging found a brain state that "evolves on a cycle" whose segments map "to the
  animals' major motor commands", assembling them into "a string of run-and-turn action sequence cycles"
  ([Kato et al. 2015](https://doi.org/10.1016/j.cell.2015.09.034), *Cell* 163(3):656-669; abstract, Europe PMC). One
  brain-wide, coordinated cycle sits uneasily beside independent layers that suppress one another: if that is how the
  worm selects its commands, a stack of layers may at best describe it coarsely, a point against the hunch's literal
  form. The conversation judged a joint-embedding predictive architecture
  ([Assran et al. 2023](https://arxiv.org/abs/2301.08243), arXiv record read) more likely to fit the fly or rat rung
  than the worm's.

*Predicts*: a reflex stack - wirings, suppression between layers, and at most N slow knobs and remembered numbers, as
defined above - reproduces the reference statistics at the worm's pace, and adding a learned model of the world to it
gains nothing the benchmark can detect.

*The search, fixed before any run.* "A better stack exists" must not dodge a result, so the stack is one family, the
architecture registered in STATE.md item 13 (a) before any life, fitted by one optimiser over five optimiser seeds with
a budget of evaluations per seed stated in item 13 (a) before the first fit, the same for every arm below; the
reported stack is the best of the five on the development data. A refutation holds for that family and budget, and a
new family is a new test, registered as such.

*Refuted*, each with its pass side:

- **R1a, sufficiency, on the frozen benchmark** (STATE.md item 2 (e5)). Refuting: the stack, so searched, fails the
  benchmark by its frozen pass rule - whatever any other Program does. Pass side: it passes.
- **R1b, whether a model is what is missing**, read only when R1a refutes. A Program of no more than ten times the
  stack's size, the stack with a registered learned model added (as R2), searched with the same budget, passes:
  evidence that the model is what the stack lacked. If none passes, R1b is undecided and R1a's refutation stands; if
  only a model-carrying Program of more than ten times the size passes, R1b is undecided too, and that is recorded
  beside the sizes, the lean rule preferring neither until a leaner one passes.
- **R2, adding a model.** The model is chosen and registered before the run: its kind (a map of places, or a predictor
  of a sense), its size and its update rule. Its *positive control*: it must be shown to work on its own terms - a
  map's accuracy computed by an evaluation tool from the world's ground truth, which never reaches the brain, above a
  chance baseline registered with it; a predictor's error below a no-change prediction's, with the interval of the
  difference excluding zero. If the control fails, R2 is undecided, never a pass. Refuting: with the control passed,
  the stack plus model scores better than the stack alone on the frozen benchmark beyond the score's interval. Pass
  side: the model's addition is not detected to help, with the size of help the interval excludes stated. "Score"
  means the frozen benchmark's score, which does not exist before (e5); before then R2 can be run only in R3's partial
  form, as a description, never as a verdict.
- **R3, a partial test before the freeze.** Until (e5) the benchmark has no pass rule, so nothing here can pass it.
  What can be run is the per-measure comparison (`benchmark::compare`, Cramér-von Mises per measure with Holm's
  correction) and the classifier test (`benchmark` (e1)) between Grid lives and real worms. The stack's free parameters
  are fitted against block 1's reference statistics, with the search above, and judged against a block of the seeded
  draw neither fitted on nor used before - not block 1, and not block 2, already used in item 2 (e3). Blocks 1 and 2
  were not told apart (e3), so a held-out block guards against fitting a block's sampling noise, not against fitting
  the population. The number of free parameters fitted is reported beside the 27 measures. Refuting, provisionally:
  more measures flagged than a threshold fixed in the journal before the run (item 13 (a)), after the full search,
  whatever any other Program does; a model-carrying Program given the same search is read as in R1b, provisionally. Pass side, provisionally: with at least a number
  of lives fixed in item 13 (a), no measure flagged and the classifier not telling the lives from the held-out block,
  reported with the alteration of each measure the test can detect at that number of lives. That alteration comes from
  item 2 (e4), still open, so until (e4) lands the pass side cannot be read: a non-detection with an unknown power is
  not a pass. Even read, it is a partial pass, not a benchmark pass, since the definition may still change. Reaction
  latencies are not in these recordings, which are of spontaneous behaviour, so they wait on stimulus assays
  ([pace.md § For the Lab](../research/pace.md#for-the-lab)).

*Baselines and controls*:

- the placeholder Program (`crates/placeholder-worm`, which undulates only), the floor;
- the stack with each wiring, layer, knob and remembered number pinned at baseline in turn, by the lab's ablation rule,
  a part being kept only if its removal costs the stack on a measure;
- the *sense-blind control*: the stack with each sense channel replaced by a surrogate signal that matches that
  channel's distribution of values and its autocorrelation over the stack's own lives (a phase-randomised surrogate of
  the recorded channel, amplitude-adjusted), with no relation to the world. It is "near the floor" if its count of
  flagged measures under R3 is at least halfway from the stack's count to the placeholder's. If it does not fall, the
  measures of these spontaneous recordings carry little that the senses drive, and a partial pass under R3 then says
  little about the wirings: it would test the rhythm and the switching of states, with the wirings left to the stimulus
  assays (item 11). That reading is written here, before any run;
- the stack plus a registered learned model, for R1b and R2;
- the real-against-real result of item 2 (e3), block 1 against block 2 not told apart, as the test's own control that
  two samples of real worms pass.

*Lean-brain cost*: an estimate, not yet a measurement - tens to hundreds of numbers and of the order of a hundred
arithmetic operations a tick, far inside H12's bound of about 100 000 numbers. Its size, and its compute and memory
per simulated second, are reported beside every score.

*The pace rule*: every wiring's latency and every knob's time constant is taken from a cited value in
[pace.md](../research/pace.md) (§ Reaction Latencies, § Time Constants of Internal States) and counted in the Grid's
31.25 ms ticks, rounded up to a whole tick, so that no wiring acts sooner than the worm: a latency under one tick takes
one tick, and every other is less than one tick slower than its source. A wiring whose latency pace.md does not yet source
takes, provisionally, the latency of the sourced pathway it is named after in item 13 (a)'s design, is marked
unsourced, and is not scored on its latency until pace.md sources it.

*Why it is a strong candidate for the lab's first scored creature*: it is cheap, every decision can be traced to the
one wiring or layer that made it, and it is the natural baseline any richer brain must beat on the benchmark.

*Waits on*: a Program that runs on the Grid with such a stack; the Disk reader, to extract the same features from its
lives, now on STATE.md's *Next* as item 3 (d2) by the maintainer's ruling of 2026-11-05 ("Reuse Master Control"); item
2 (e4), for R3's pass side; and, for R1 and R2, the frozen benchmark, item 2 (e5).
*Tested so far*: nothing.

## Adding a Hypothesis

A pool of candidates waits in [Matej's ideas and hunches](matej-hunches.md): unverified, to be triaged and
promoted here one by one.

Add a section with the same four parts: the idea, the evidence (public sources, linked), the behaviour it
predicts, and how the benchmark would show it. Record in the journal why it was added.

Bold ideas are welcome here - strange architectures, mathematics borrowed from far away, hunches no one has
tried on a worm. A hunch with little evidence yet is marked *a hunch* and says what would refute it; it is
tested like any other, and if it fails, that is written up as carefully as a success.
