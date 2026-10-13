# Hypotheses About the Brain

A starting set of ideas about how a creature's brain should be built, written so that each can be tested
against the benchmark. They are hypotheses, not decisions: any of them may be refuted, replaced or joined by
better ones, with the reasons written in the journal.

## The Shape of the Brain

The working assumption is that the brain is **modular and heterogeneous**: a few compartments with different
jobs, each built the way its job needs rather than all of one architecture - as in animals, where a reflex
arc, a rhythm generator and the circuits that set behavioural state work in very different ways. Three
principles go with it:

- **Every module's state can be read.** Its numbers mean something a person can inspect, so a behaviour can
  be traced to the module that produced it.
- **Senses only.** Every module sees the world through the creature's senses and nothing else.
- **Earned complexity.** A module is added, or made richer, only when a behaviour the benchmark measures
  demands it.

## The Hypotheses

Each states the idea, why biology suggests it, what behaviour it should produce, and how the benchmark would
show it. All are untested.

### H1. A Fast Reflex Arc

Direct, weighted pathways from senses to motor commands - touch at the head drives reversal, touch at the tail
drives forward acceleration - running before and alongside every slower module, and able to override them.
The worm's touch circuit is the classic example ([Chalfie et al. 1985](https://doi.org/10.1523/JNEUROSCI.05-04-00956.1985)).
*Predicts*: touch-evoked reversals with short latency and the probabilities real worms show.

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
an ordinary differential equation ([Chen et al. 2018](https://arxiv.org/abs/1806.07366)).
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

## Adding a Hypothesis

Add a section with the same four parts: the idea, the evidence (public sources, linked), the behaviour it
predicts, and how the benchmark would show it. Record in the journal why it was added.
