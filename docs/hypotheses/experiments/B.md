# Plans for B: Body, Senses and World

Experiment plans for hunches B1 to B9 of [Matej's ideas and hunches](../matej-hunches.md), in the format of
[README.md § The Format](README.md#the-format). Section B is about the interface between a brain and its world, so
several of these plans test the world and the harness rather than a brain, and some wait on changes only the
world's own repositories can make - those go to them as issues when the time comes, by the rule *This repository,
and the world's by their own rules*.

The leads were looked up in Crossref by title on 2026-10-13. Found means the work exists as named, with the DOI
given; none was read in full. A lead that could not be found says so.

Costs are counted in simulated lives. What one life costs in compute and memory is not known until the arena is
proven (STATE.md item 4), so no plan gives absolute figures yet; each will when it is prepared. Seed counts follow
[README.md § The Format](README.md#the-format) unless a plan says why it differs.

## B1. A nervous system, not a game client

- **Question**: does every field a Program receives correspond to something a worm's sensory biology can transduce, at
  a comparable resolution - and does a field-by-field audit catch one that does not?
- **Method**: list every field of the senses structure in the Program interface at the pinned world
  ([PROGRAM_INTERFACE.md](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/docs/PROGRAM_INTERFACE.md) and
  its header), and for each, name the worm's organ or neuron class that could transduce it, with a source read at the
  source, or mark it as having none. Grade each field: carried by the animal; carried at a resolution the animal
  lacks; not carried at all.
- **Baselines and controls**: a planted field - a copy of the field list with one ground-truth entry added (the body's
  position) - must be flagged by the audit; a field the lab already knows the worm has (touch along the body) must
  pass.
- **Refuted if**: the audit passes the planted field, or fails the known one. A field graded *not carried at all* is
  not a refutation of the hunch but a finding: it goes to the comparison space as a sense the Program must not rely
  on, or to the world's repository as an issue.
- **Statistics**: none; a graded list, each grade with its source.
- **Seeds and repetitions**: not applicable.
- **Cost**: reading.
- **Waits on**: nothing; it can be done now, and is the cheapest plan in section B.
- **Leads**: none named; the sources are the worm's sensory biology, to be found and read field by field.

## B2. Smell as an unlabelled fingerprint

- **Question**: if each odour source carries a fixed multi-dimensional pattern, does a learning Program's recognition
  generalise to similar patterns and get confused by blends, as the hunch says real recognition should?
- **Method**: train the Program to approach a source with pattern P; then test it with patterns at increasing
  distances from P, and with blends of P and an untrained pattern Q in proportions from 0 to 1.
- **Baselines and controls**: an untrained Program (no gradient expected); a Program trained on Q, tested the same way
  (the gradient must centre on Q, not P).
- **Refuted if**: the approach response does not fall with distance from P - Spearman's rank correlation between
  distance and response has a 95% interval that does not lie below zero; or the response to blends does not change
  monotonically with the proportion of P, by the same test.
- **Statistics**: Spearman's rho with a bootstrap 95% interval, per curve.
- **Seeds and repetitions**: 20 seeds per test pattern; at least 8 distances and 8 blend proportions.
- **Cost**: 20 lives per test point, about 320 lives per curve pair.
- **Waits on**: a chemical sense on the Grid (STATE.md *Needs From Other Repositories*, which the maintainer takes
  up), and a Program that learns (H9).
- **Leads**: none named.

## B3. Senses switched on by rung

- **Question**: when the next rung of the ladder only adds sense channels, do all of the previous rung's tests still
  pass?
- **Method**: run the worm's frozen benchmark on a worm Program that receives the next rung's extra channels but
  ignores them, and on one that uses them.
- **Baselines and controls**: the worm Program on the worm rung (its own score); the extra channels fed with noise (an
  added channel must not break the worm by its mere presence).
- **Refuted if**: the Program that ignores the new channels, or the one fed noise on them, differs from its own rung
  on any benchmark measure by the per-feature comparisons the frozen benchmark will fix, with its correction for
  multiple comparisons - then the rungs do not only add, and earlier tests do not stay valid.
- **Statistics**: the frozen benchmark's per-feature comparisons.
- **Seeds and repetitions**: 20 seeds per condition.
- **Cost**: three sets of 20 lives.
- **Waits on**: the frozen benchmark, a worm Program, and a second rung's senses on the Grid (the ladder of
  [PERCEPTION.md](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/docs/PERCEPTION.md)).
- **Leads**: none named.

## B4. How a reversal ends decides whether chemotaxis works

- **Question**: does chemotaxis depend on whether backing up reorients the body - so that with reversals that do not
  reorient, reversals not followed by an omega turn stop helping?
- **Method**: two parts, because how a reversal moves the body is the world's physics, not the Program's choice, and
  the lab never changes the world to suit a brain (*Realism over speed*). On the Grid, measure what a reversal does:
  the change of heading from just before a reversal to just after it, for a Program that reverses, against the real
  worm's (read from the recordings). In B5's dish world, where both semantics can be built, run the same chemotaxis
  Program under reversals that end with a change of heading and reversals that end on the old heading, and measure the
  chemotaxis index and the share of reversals followed by an omega turn under each.
- **Baselines and controls**: in the dish, a Program whose reversals are random in time (no taxis expected under
  either semantics); the real worm's reversal-to-omega share and heading change after a reversal.
- **Refuted if**: in the dish, the chemotaxis index under the non-reorienting semantics is not lower than under the
  reorienting one - a one-sided two-sample t test across seeds at alpha 0.05 does not find it lower. The Grid
  measurement is not a refutation but a finding: if the Grid's reversals reorient differently from the worm's, it is
  written up as an issue for tron-grid-lite or master-control.
- **Statistics**: the chemotaxis index per seed and the test above; the heading change per reversal against the real
  worm's, compared by the frozen benchmark's per-feature comparisons; the omega share with a Wilson interval.
- **Seeds and repetitions**: 20 seeds per semantics in the dish, the format's default; on the Grid, every reversal of
  20 lives.
- **Cost**: 40 dish lives, 40 for the control, and 20 Grid lives.
- **Waits on**: B5's dish with a chemical gradient, a chemotaxis Program, and for the Grid part a Program that
  reverses. Chemotaxis on the Grid itself waits on a chemical sense.
- **Leads**: none named; the omega-turn definition is in [the behaviour-measures
  note](../../research/behaviour-measures.md).

## B5. A fast agar-plate world for iterating

- **Question**: can a small two-dimensional dish world, with the Grid's sensory and motor interface, reproduce the
  published assays fast enough to iterate on - and does a brain that passes there and fails on the Grid show what the
  dish leaves out?
- **Method**: build the dish as the hunch describes, behind the same interface as a Program on the Grid; run A1's
  assays there; move a brain from dish to Grid unchanged and run the same assays on the Grid.
- **Baselines and controls**: the same assays on real worms (A1's bands); a Program that only undulates (it must fail
  the assays that need steering, in both worlds).
- **Refuted if**: a simulated second in the dish is not at least ten times cheaper in compute than on the Grid (it is
  not fast enough to iterate on); or more than one of A1's comparable assays cannot be run in the dish with its
  published protocol (geometry, timings, stimulus); or a brain's pass or fail differs between dish and Grid on more
  than one assay in five, where a difference does not count only if it is traced to a difference between the two
  worlds written down before the comparison.
- **Statistics**: compute per simulated second measured in both worlds; per assay, A1's rule in each world.
- **Seeds and repetitions**: 20 seeds per assay per world.
- **Cost**: the dish's own build, and one Grid life per seed per assay.
- **Waits on**: A1's bands and the arena. The dish is a tool, so it is written in Rust (the rule *Rust first*); it
  never scores a brain - scoring stays on the Grid.
- **Leads**: none named.

## B6. Behaviour must not depend on the tick rate

- **Question**: are a Program's behavioural statistics the same at two tick rates?
- **Method**: the Grid's tick is fixed: Master Control steps the world at 0.031 25 s, a constant in its source
  (`TICK_SECONDS` in `external/master-control/src/physics.rs` at the pinned commit), so the Grid itself cannot run at
  two rates. The test is therefore made where the rate can change - in B5's dish world at 32 Hz and 64 Hz, and in a
  Program's own internal integration step, at two steps per tick against one.
- **Baselines and controls**: the same rate twice with different seeds (the run-to-run spread the comparison is judged
  against).
- **Refuted if**: any benchmark measure differs between the two rates by the per-feature comparisons the frozen
  benchmark will fix, with its correction for multiple comparisons.
- **Statistics**: as above.
- **Seeds and repetitions**: 20 seeds per rate.
- **Cost**: 40 lives in the dish per comparison; the internal-step comparison, 40 Grid lives.
- **Waits on**: B5's dish, a Program, the frozen benchmark.
- **Leads**: none named; the rule *The animal's pace* is why it matters.

## B7. Do not count your own echo

- **Question**: does a Program discount sensory change it caused itself, and avoid counting one piece of evidence
  twice when it arrives by two paths?
- **Method**: two probes. Reafference: let the Program move in a world that does not change, and measure the change in
  whatever internal belief the senses update; an efference copy should cancel it. Reconvergence: deliver the same
  evidence by two sensory paths at once, and by each alone, and compare the belief update.
- **Baselines and controls**: the same Program with its efference copy switched off (the rule *Pre-register and
  ablate*): the self-motion probe must then show a change.
- **Refuted if**: with the efference copy on, self-motion in a still world moves the belief by an amount whose 95%
  interval excludes zero; or the update from both paths at once exceeds the larger single-path update by an amount
  whose 95% interval excludes zero (double counting).
- **Statistics**: bootstrap 95% intervals over seeds.
- **Seeds and repetitions**: 20 seeds per probe.
- **Cost**: short probe runs on a frozen copy, not lives.
- **Waits on**: a Program with an efference copy and a belief to update - none planned yet; this plan says what one
  would have to show.
- **Leads**: none named.

## B8. A simulated body can be rolled back

- **Question**: can a Program's state be snapshotted, restored and diffed so exactly that a resumed run is
  indistinguishable from an uninterrupted one?
- **Method**: run a life, snapshot the Program at a pause, continue; restore the snapshot into a fresh process and
  continue from it with the same inputs; compare the two continuations.
- **Baselines and controls**: two uninterrupted runs from the same seed (the A/A control of A5).
- **Refuted if**: the restored continuation differs from the uninterrupted one at any tick, given that the A/A control
  does not.
- **Statistics**: exact equality.
- **Seeds and repetitions**: 20 seeds, a snapshot at three points in each life.
- **Cost**: two lives per seed.
- **Waits on**: a Program, and A5's lock-step condition for fresh lives; replaying recorded senses into a Program, as
  the Program interface describes, can test the Program's half sooner.
- **Leads**: none named.

## B9. The insect bet, and the bridge it owes

- **Question**: does a body-first Program adapt to a perturbed body - a weakened joint, a lost segment - within a time
  like the animal's?
- **Method**: perturb the body in mid-life and measure how long the Program takes to recover its locomotion measures
  to their band. The animal-like time is read from a source on worms with an impaired body, to be found; without one,
  the comparison waits.
- **Baselines and controls**: the same Program unperturbed; the same Program with its learning switched off (it should
  not recover, or recover more slowly).
- **Refuted if**: the Program does not return to band within the animal's recovery time, read from its source; or it
  recovers as fast with learning switched off (the recovery is not adaptation).
- **Statistics**: recovery time per seed, its median with a bootstrap 95% interval, against the animal's figure.
- **Seeds and repetitions**: 20 seeds per perturbation.
- **Cost**: three sets of 20 lives per perturbation.
- **Waits on**: a way to perturb a body in mid-life, which the world does not have: the body is authored by the
  Program at rez and its limits are the Grid's, so a mid-life weakened joint or lost segment is a change for
  tron-grid-lite and master-control, asked for there by issue when this experiment is prepared - never made by the lab
  alone (*Realism over speed*).
- **Leads**: found - Brooks 1991, "Intelligence without representation" (*Artificial Intelligence*,
  [doi:10.1016/0004-3702(91)90053-M](https://doi.org/10.1016/0004-3702(91)90053-M)); Pfeifer and Bongard 2006, *How
  the Body Shapes the Way We Think* (MIT Press,
  [doi:10.7551/mitpress/3585.001.0001](https://doi.org/10.7551/mitpress/3585.001.0001)). Not found as a work -
  Braitenberg 1984, *Vehicles*: Crossref has a review of it (Dennett 1986), not the book.
