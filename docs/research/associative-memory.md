# Associative Memory for a Worm-Sized Brain

Four families of associative memory, surveyed as candidates for the worm's brain: Hebbian learning and Oja's rule,
the classical Hopfield network and its capacity, modern Hopfield networks and dense associative memory, and Boltzmann
machines. For each, the note says what it is, with its equation; what it would cost on the worm's scale, 302 neurons
([White et al. 1986](https://doi.org/10.1098/rstb.1986.0056)), by the rule *Lean brains*; which of the worm's
behaviours it could carry; how it would be ablated and tested; and how it bears on
[H5](../hypotheses/README.md#h5-settling-into-attractors), [H6](../hypotheses/README.md#h6-small-bounded-learning),
[H9](../hypotheses/README.md#h9-associative-memory-written-by-hebbs-rule) and
[E4](../hypotheses/experiments/E1-E5.md#e4-hebbian-learning-needs-a-stabiliser-and-a-used-together-rule).

**How the sources were read**, on 2026-11-03 (STATE.md's item 4 of that night):

| Source | Read |
|--------|------|
| The Nobel Committee for Physics, [Scientific Background to the Nobel Prize in Physics 2024](https://www.nobelprize.org/uploads/2024/11/advanced-physicsprize2024-3.pdf), and the [press release](https://www.nobelprize.org/prizes/physics/2024/press-release/) | in full, at nobelprize.org |
| [Krotov and Hopfield 2016](https://arxiv.org/abs/1606.01164), "Dense Associative Memory for Pattern Recognition" | in full, arXiv v2 |
| [Ramsauer et al. 2020](https://arxiv.org/abs/2008.02217), "Hopfield Networks is All You Need" | the abstract and § 2 up to Theorem 3 (the energy, the update, Theorems 1-3), arXiv v3; Theorem 4 only in its opening sentence and its bounds not read; Theorem 5 onwards and the appendix not read |
| [Ackley, Hinton and Sejnowski 1985](https://doi.org/10.1207/s15516709cog0901_7), "A Learning Algorithm for Boltzmann Machines" | its sections on the decision rule and the learning rule, in a scan hosted on [Hinton's university page](https://www.cs.toronto.edu/~hinton/absps/cogscibm.pdf); the scan's text layer is damaged, so its equations are written below in the form the paper's prose describes, not copied |
| [Hopfield 1982](https://doi.org/10.1073/pnas.79.8.2554), PNAS | **abstract only**, through Europe PMC (PMC346238); the full text could not be fetched from PubMed Central, Europe PMC or PNAS from the cloud machine. Its model is described below from the Nobel committee's account, which is cited for it |
| [Oja 1982](https://doi.org/10.1007/BF00275687), *Journal of Mathematical Biology* 15, 267-273 | **not read**: its metadata only, through Crossref; the publisher's copy is closed |
| [Amit, Gutfreund and Sompolinsky 1985](https://doi.org/10.1103/PhysRevLett.55.1530), *Physical Review Letters* 55, 1530-1533 | **not read**: its metadata only, through Crossref; the publisher's page refused the machine |

How sure the lab is: high for the quoted phrases and the equations of the sources read, which are theirs; medium for
the equations given in their commonly published form from sources not read here, each marked; lower for the
paragraphs headed *For the worm*, which are the lab's reasoning, and for the costs, which are the lab's arithmetic
from the equations (`python3` one-liners, given in the journal of 2026-11-03, § Associative Memory, Surveyed).

## Why This Line of Work

The Nobel Prize in Physics 2024 went to John J. Hopfield and Geoffrey Hinton "for foundational discoveries and
inventions that enable machine learning with artificial neural networks", in the committee's words
([press release](https://www.nobelprize.org/prizes/physics/2024/press-release/), 8 October 2024). The press release
says Hopfield "created an associative memory that can store and reconstruct images and other types of patterns in
data", and that Hinton "used the Hopfield network as the foundation for a new network that uses a different method:
the Boltzmann machine". The committee's scientific background describes the Boltzmann machine as "a stochastic
extension of Hopfield's model from 1982" ([Scientific
Background](https://www.nobelprize.org/uploads/2024/11/advanced-physicsprize2024-3.pdf)). Three of the four
families here are therefore the prize's own subject; the fourth, Hebbian learning, is the rule both rest on: the
background names Hebb's 1949 proposal and calls the Hopfield network's correlation-based couplings "the Hebb rule"
(Scientific Background).

A prize says the work mattered to physics and to machine learning. It says nothing about whether any of it fits a
worm, which is what the rest of this note weighs.

## 1. Hebbian Learning and Oja's Rule

**What it is.** A connection between two cells active together grows stronger
([Hebb 1949](https://doi.org/10.4324/9781410612403), as cited in H9; not read). For a unit with input x, weights w and
output y = w · x, the plain rule is

```text
Δw = η y x,
```

which grows without bound. Oja's rule adds a decay proportional to the output's square,

```text
Δw = η y (x - y w),
```

which, for a small enough η, keeps |w| near 1 and turns w towards the input's first principal component - the
result the paper's title names, "Simplified neuron model as a principal component analyzer". **The equation is given
in the form commonly published for it, not read from Oja 1982 here** (medium confidence).

**Cost on the worm's scale.** A local rule: one multiply-add per weight per update, and no memory beyond the weights.
On every possible directed pair of 302 neurons, 90 902 weights, a full update is about 91 000 multiply-adds; on a
sparse wiring it is one per synapse. This is the cheapest family by far, and the only one whose cost scales with the
wiring rather than with the number of memories.

**What it could carry.** On its own, Hebb's rule is not a memory but a way of writing one: it sets the weights the
other three families read. Of the worm's behaviours, the candidates are those that change with paired experience:
learning to avoid the smell of bacteria that made it ill ([Zhang et al. 2005](https://doi.org/10.1038/nature04216),
as cited in H9), which the Grid could test through its one smell, food odour (Program ABI 11). Tap habituation, H6's
target, is a decrement of a response with repetition, not an association; a Hebbian rule is not the natural model
for it (*for the worm*, the lab's reading).

**Ablated and tested.** The switches are the ones E4 names: the rule off (weights frozen at their initial values),
the stabiliser off (Hebb alone), and the "used together" rule off. E4's pilot shows that on a graded network with a
resting drive b, a loop ignites itself once its weight passes `w*` = 1 - 2b, and that Oja's rule bounds a weight at 1,
above `w*`, so it does not prevent ignition
([E1-E5.md § E4](../hypotheses/experiments/E1-E5.md#e4-hebbian-learning-needs-a-stabiliser-and-a-used-together-rule);
asserted in `crates/hebbian`). E4 itself calls these "assertions, not tests", and what they leave "a design rule, not
a finding": they hold for that one toy model class, with its chosen constants. In that class, Oja's bound of 1 lies
above `w*`, which suggests that a guard against runaway has to bound the loop gain below ignition; whether that rule
matters for a worm is untested, and waits on a Program and the frozen benchmark.

**Ties.** H9 (Hebb's rule writes the memory; Oja keeps it bounded) and E4, whose pilot asserts, in its toy model
class, where Oja's bound lies against the ignition point; E4's test of the rule waits on a Program. H6 only through the learning rate a modulator would set (H8).

## 2. The Classical Hopfield Network and Its Capacity

**What it is.** N binary units, symmetric couplings set by Hebb's rule from the stored patterns, asynchronous threshold
updates, and an energy that the updates can only lower. In the Nobel committee's account of Hopfield 1982
([Scientific Background](https://www.nobelprize.org/uploads/2024/11/advanced-physicsprize2024-3.pdf)), a unit
s_i ∈ {0, 1} takes the field

```text
h_i = Σ_{j≠i} w_ij s_j,
```

and is set to 1 if h_i > 0 and to 0 otherwise, and the energy

```text
E = - Σ_{i<j} w_ij s_i s_j
```

"is a monotonically decreasing function under the dynamics of the network". Stored patterns are minima; a corrupted
cue rolls to the nearest one. Hopfield's own abstract calls this a "content-addressable memory which correctly yields
an entire memory from any subpart of sufficient size" ([Hopfield 1982](https://doi.org/10.1073/pnas.79.8.2554),
abstract).

**Capacity.** With ±1 units and couplings T_ij = Σ_μ ξ_i^μ ξ_j^μ, the number of random patterns that can be stored and
reliably retrieved "is of the order of K_max ≈ 0.14N" ([Krotov and Hopfield 2016](https://arxiv.org/abs/1606.01164),
§ 2, citing Hopfield 1982 and Amit, Gutfreund and Sompolinsky); for perfect recovery of a memory their eq. (6) gives
about N / (2 ln N). Beyond the limit, "several neighboring memories in the configuration space will merge together"
into a state "which has nothing to do with any of the stored memories" (§ 2). The figure 0.14 is taken from Krotov and
Hopfield, not from Amit, Gutfreund and Sompolinsky, which was not read; the committee cites the analysis of capacity
by spin-glass methods to Amit, Gutfreund and Sompolinsky, "Spin-glass models of neural networks", *Physical Review A*
32, 1007-1018 (1985) ([doi:10.1103/PhysRevA.32.1007](https://doi.org/10.1103/PhysRevA.32.1007); found through Crossref, not
read).

**Cost on the worm's scale** (the lab's arithmetic). At N = 302: 0.14 N ≈ 42 patterns at the usual error threshold,
and N / (2 ln N) ≈ 26 recalled perfectly. The weights are 45 451 symmetric pairs; one sweep of updates is about 91 000
multiply-adds. Wired on all pairs, it is a dense matrix; the worm's wiring is not all-to-all, and a memory confined to
a subset of the 302 would hold proportionally fewer patterns.

**What it could carry.** A few dozen discrete states, settled into and held: H5's attractors. Of the worm's behaviours,
the lab's candidates are long-lived regimes that persist and switch - roaming and dwelling
([Flavell et al. 2013](https://doi.org/10.1016/j.cell.2013.08.001), as cited in H8) - but these are two or three
states, far below any capacity question. A worm-sized brain does not need Hopfield's capacity; it may need its
settling (*for the worm*, a judgement).

**Ablated and tested.** Couplings zeroed (no attractors), couplings frozen after the first life (no learning), and
asymmetry added (the energy argument lost). The test is H5's: the durations and switching rates of the locomotion
states against the reference (STATE.md item 2), with and without the attractor module. A capacity check is a unit test
in Rust, not a benchmark question.

**Pace.** Asynchronous settling takes sweeps, and each sweep is time. By *The animal's pace*, the number of sweeps per
decision is fixed from the worm's measured decision times ([pace.md](pace.md)), not from what converges fastest.

**Ties.** H5 directly; H9 for how the couplings are written; E4's ignition threshold is the graded twin of a pattern
becoming a fixed point.

## 3. Modern Hopfield Networks and Dense Associative Memory

**What it is.** Krotov and Hopfield replace the quadratic energy with

```text
E = - Σ_μ F(ξ^μ · σ),
```

with F(x) = x^n, or a rectified polynomial; n = 2 is the classical network
([Krotov and Hopfield 2016](https://arxiv.org/abs/1606.01164), eqs. 2-3). Each memory's well becomes sharper, and the
capacity grows as K_max = α_n N^(n-1) (eq. 5). An exponential F was introduced by Demircigil et al.
(2017), with a capacity of N = 2^(d/2) binary patterns - both as Ramsauer et al. report them (§ 2; Demircigil et al.
not read). Ramsauer et al. make the state continuous and take the logarithm of that energy plus a quadratic term,
with energy

```text
E = - lse(β, Xᵀξ) + ½ ξᵀξ + β⁻¹ log N + ½ M²
```

and update

```text
ξ_new = X softmax(β Xᵀξ),
```

where the columns of X are the stored patterns ([Ramsauer et al. 2020](https://arxiv.org/abs/2008.02217), eqs. 2-3;
here N is the number of patterns and d their dimension). Their own capacity bound is their Theorem 3, for random patterns
on a sphere. The network "can store exponentially (with the dimension of
the associative space) many patterns, retrieves the pattern with one update", and its update "is equivalent to the
attention mechanism used in transformers" (abstract). Besides fixed points for single patterns it has "metastable
states averaging over a subset of patterns" and a global fixed point averaging all of them (abstract).

**Cost on the worm's scale** (the lab's arithmetic). The capacity is paid for somewhere. Written as a tensor, a cubic
energy over 302 units has 302³ ≈ 27.5 million couplings, far beyond the worm's wiring. Written as Ramsauer's update,
the network keeps each memory as a column of X: K memories of 302 numbers, and about 2 K × 302 multiply-adds and K
exponentials a retrieval - 25 000 multiply-adds for 42 memories. Krotov and Hopfield's own duality reads the same
thing as a network "with one hidden layer" (abstract): in the lab's reading, in effect one hidden unit per memory, so the memories are
neurons too. The exponential capacity is a statement about the dimension of the patterns, not a saving on neurons, and
a worm has no use for exponentially many memories.

**What it could carry.** One-step retrieval and the metastable averages are the attractive parts: a single settling
step, cheap enough for every tick, and a state that blends similar memories rather than choosing one. A candidate for
recognising a learned smell context (*for the worm*, a hunch, untested). H11, the lab's Hebbian Hopfield transformer,
was refuted (journal of 2026-10-14), which counts against reaching for this family first.

**Ablated and tested.** β pinned low (only the global average, no memory), n = 2 (the classical network), and the
memory matrix cleared after each life. The test is the same benchmark measure as the module's behaviour asks for,
with the module's neurons counted in the brain's size by *Lean brains*: hidden units per memory are not free.

**Ties.** H5 names Hopfield 1982, Ramsauer et al. and Chen et al. 2018; Krotov and Hopfield are cited in H9, which calls dense associative memory "the line that leads to
the modern Hopfield networks of H5". Neither has been tested on the worm.

## 4. Boltzmann Machines

**What it is.** A Hopfield network made stochastic. Each state s has a probability from the Boltzmann distribution,

```text
P(s) ∝ exp(-E / T),   E = - Σ_{i<j} w_ij s_i s_j - Σ_i θ_i s_i,
```

with T "a fictive temperature" ([Scientific
Background](https://www.nobelprize.org/uploads/2024/11/advanced-physicsprize2024-3.pdf)). Ackley, Hinton and
Sejnowski's units turn on with a probability that is a logistic function of their energy gap divided by T (their eq.
5), and the network reaches "thermal equilibrium", after which "the relative probability of two global states will
follow the Boltzman [sic] distribution". Learning lowers an information-theoretic distance G between the distribution of
the visible units in the world and in the free-running network; its gradient needs only two co-occurrence
statistics, so each weight changes as

```text
Δw_ij = ε (p_ij - p'_ij),
```

with p_ij the probability that units i and j are both on when the visible units are clamped by the environment, and
p'_ij the same when the network runs freely (their eq. 10, written from their prose; the scan's equations are
damaged). In their own simulations they did not use eq. 10's proportional step: each weight was moved by a fixed
weight-step according to the sign of p_ij - p'_ij. They note that the rule "uses only locally available information". The committee adds that "each step of
the algorithm involves time-consuming equilibrium simulations for two different ensembles", and that the machine was
"initially of limited use" until the restricted Boltzmann machine.

**Cost on the worm's scale** (the lab's arithmetic). The same 45 451 symmetric weights as a Hopfield network on 302
units, plus a random draw per unit update; but each decision is a sample, and each learning step two runs to
equilibrium. Annealing - starting hot and cooling, which the paper recommends - multiplies the sweeps. The cost lies
in time, not in neurons, and that is where *The animal's pace* bites: a brain may run only as many sweeps between
ticks as the worm's decision times allow ([pace.md](pace.md)).

**What it could carry.** Behaviour that varies rather than repeats (H9's own words): a worm on the same patch does not
make the same turn twice. The sample's variability is set by T, a single knob a modulator could set (H8). The lab's
candidate: the rates and durations of the locomotion states as a distribution, not a sequence (*for the worm*, a
hunch). The test is whether the distribution matches, which is the benchmark's question exactly.

**Ablated and tested.** T → 0 (the Hopfield network it extends); T pinned at its baseline (no modulation of
variability); learning off (weights frozen). The determinism rule holds: the samples come from a seeded generator, so a
life replays exactly.

**Ties.** H9 names the Boltzmann machine as the candidate for "behaviour that varies rather than repeats"; H5 as its deterministic limit at T → 0, in the lab's
reading; E4's stabiliser question returns as a temperature question, untested.

## For the Lab

The lab's reading, at the confidence of a judgement:

- **The worm does not need capacity.** Every family here was built to store many patterns. A worm-sized brain may need
  two or three settled states and one learned association; at that load the classical network's 42 patterns are
  ample, and dense memory's capacity buys nothing while costing hidden units.
- **What the families offer the worm is a mechanism, not storage**: settling (classical Hopfield), one-step blending
  (modern Hopfield), sampling (Boltzmann), and a local way to write any of them (Hebb and Oja). Each is a module with
  a switch, judged on the benchmark like any other.
- **E4 gives a design rule, not a finding.** Its pilot asserts, in one toy model class with chosen constants, that
  Oja's bound of 1 lies above the ignition point `w*` = 1 - 2b, which suggests that a Hebbian loop needs its gain
  bounded below ignition by something more than Oja's rule. The rule is untested on a worm.
- **Everything waits on a Program and a frozen benchmark**, as E4 does. Before then, the toy problems this note
  suggests are unit tests and notebooks, not results.

## Gaps, to Be Read

- Hopfield 1982 in full: the paper's own equations and its asynchronous update; PNAS and PubMed Central refused the
  cloud machine. Next lead: a library copy, or PubMed Central's scanned page images.
- Oja 1982: the rule's own form and its condition on η. No open copy found.
- Amit, Gutfreund and Sompolinsky 1985, *PRL* 55, 1530, and *Physical Review A* 32, 1007, which the Nobel committee
  cites: the 0.14 (often given as 0.138) read at the source.
- Ramsauer et al. 2020, the appendix: the conditions of Theorem 3, the capacity bound.
- Krotov and Hopfield's later papers on the hidden-unit form of dense memory, which bear on its neuron count; not
  looked up.
