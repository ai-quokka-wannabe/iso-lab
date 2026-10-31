# The Plan

The goal is a creature on the Grid whose behaviour cannot be told apart from a real *Caenorhabditis elegans*:
the first AI brain the lab develops, and the first rung of a ladder of animals. That sentence only means
something once *told apart* is defined, so the plan builds the measuring stick before it builds anything to
measure.

## What "Indistinguishable" Will Mean

Biologists describe a worm's behaviour with hundreds of quantitative measures taken from tracking video: body
posture, speed, the rhythm of its undulation, how often it reverses or makes an omega turn, how long it runs
in one direction, how it switches between roaming and dwelling
([Yemini et al. 2013](https://doi.org/10.1038/nmeth.2560); [Brown et al. 2013](https://doi.org/10.1073/pnas.1211447110)).
Posture alone is captured remarkably well by a handful of basic shapes, the "eigenworms"
([Stephens et al. 2008](https://doi.org/10.1371/journal.pcbi.1000028)).

The working definition, to be refined in Phase 0 and then frozen:

- **The measures**: a fixed set of behavioural features, computed the same way from real worm recordings and
  from the Grid creature's recordings. They include the timing of behaviour - how long a response takes,
  how long a state lasts - so that the creature's speed of thought is held to the real worm's too.
- **The test**: a classifier two-sample test ([Lopez-Paz and Oquab 2017](https://arxiv.org/abs/1610.06545)).
  Train a classifier to tell real worms from Grid worms using only those features; if it cannot beat chance
  by more than its confidence interval allows, the two are indistinguishable *on those measures*. Per-feature
  comparisons of the distributions say where they differ when they do.
- **The honesty clause**: the definition is frozen before any brain is scored against it, and every change
  to it is justified in the journal and followed by re-scoring everything under the new definition.

### The Bodies Differ

A real worm is a millimetre-long, soft-bodied animal crawling on agar with 95 body-wall muscles. The Grid's
worm is a chain of eight rigid segments joined by seven servos, metres long, walking a neon floor. So the
comparison is made in the body's own units - body lengths, undulation periods - and posture is compared
through eigen-shapes fitted to the joint angles the Grid worm has. Behaviours that depend on a sense the Grid
does not offer yet (smell, hence chemotaxis) are listed as not comparable and stay on the list.

## Phases

### Phase 0: The Benchmark

1. Choose the reference data: public recordings of wild-type worms, with licence and checksum recorded in
   [data/README.md](../data/README.md).
2. Write down how behaviour is measured, with sources.
3. Define the comparison space between the two bodies.
4. Build the feature-extraction tool, with tests, and compute the wild-type reference statistics.
5. Implement the test, check it on held-out real recordings (real against real must come out
   indistinguishable), and freeze the definition.

![The benchmark's pipeline: recordings of real worms are downloaded by fetch-block, measured by features and
summarised as reference statistics; a Program's lives on the Grid are recorded as Disks, to be measured the same way
through a planned Disk reader; a frozen benchmark will compare the two, measure by measure. Dashed boxes are not yet
built.](figures/diagram_benchmark.svg)

*The benchmark's pipeline; a dashed box is not yet built. Drawn by
[notebooks/figures.ipynb](../notebooks/figures.ipynb).*

### Phase 1: The Arena

Live headless lives in the cloud: Master Control (the world) and the Grid's creature host (rendering the
senses through Mesa's software Vulkan, lavapipe), built from the commits `external/` pins, and a Program (the
brain) written in this lab. [rc-worm](https://github.com/ai-quokka-wannabe/rc-worm), the organisation's
human-steered worm, is the example of how a Program is shaped; the lab writes its own. The Disk each life
leaves is the recording; the Phase 0 tool reads features from it. A flat arena first. The senses a Program
has today, from
[PROGRAM_INTERFACE.md](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/docs/PROGRAM_INTERFACE.md):
eyes as sample lists (the `elegans` preset is two body-referenced light receptors), hearing, touch,
vestibular sense, warmth, and proprioception (joint angles and the servos' loads).

### Phase 2: A First Brain

A Program built from the smallest set of modules from [the hypotheses](hypotheses/README.md) that could
produce the most basic behaviours - undulating forward, reversing when touched - scored against the benchmark,
with the result published whatever it is.

### Phase 3: Closing the Gap

Module by module: pick the behaviour where the creature and the worm differ most, form or revise a
hypothesis, change one module, re-score. Never tune the world, the body, the senses or the benchmark to close
a gap.

Two habits keep the results honest:

- **Forbidden outcomes first.** Before a change is run, write down the outcomes that would refute it, in the
  journal. A run that produces one is a failed hypothesis, reported as such.
- **Ablation for attribution.** Every module and every modulator has a switch that pins it at its baseline.
  When a change moves the score, run it once with all switches on and once with each switched off in turn,
  so the improvement is attributed to the part that caused it, not assumed.

### Phase 4: Up the Ladder

When the worm is done, the next rung - an insect - with its own benchmark.

## Prior Work to Learn From

- **OpenWorm** and its **c302** framework model the worm's nervous system at several levels of detail
  ([Gleeson et al. 2018](https://doi.org/10.1098/rstb.2017.0379)).
- A data-driven model couples a detailed brain, body and environment in one closed loop
  ([Zhao et al. 2024](https://doi.org/10.1038/s43588-024-00738-w)).
- The recordings and the tracking software behind the Open Worm Movement Database
  ([Javer et al. 2018](https://doi.org/10.1038/s41592-018-0112-1)).

This lab differs in what it reproduces: the worm's *behaviour*, on a body that is not a worm's, through senses
rendered by the Grid - not the worm's cells.
