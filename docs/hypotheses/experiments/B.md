# Plans for B: Body, Senses and World

Experiment plans for hunches B1 to B9 of [Matej's ideas and hunches](../matej-hunches.md), in the format of
[README.md § The Format](README.md#the-format). Section B is about the interface between a brain and its world, so
several of these plans test the world and the harness rather than a brain, and some wait on changes only the
world's own repositories can make - those go to them as issues when the time comes, by the rule *This repository,
and the world's by their own rules*.

The leads were looked up in Crossref by title on 2026-10-13. Found means the work exists as named, with the DOI
given; none was read in full, and where only the abstract was read, the plan says so. A lead that could not be found
says so.

Each refutation clause decides between three outcomes, by two tests fixed before any data.

- **Refuted**, by a test that finds something: a difference where the hunch says there is none, a value outside a
  band, or - where the hunch predicts an effect - the effect shown smaller than a smallest effect of interest the plan
  names (its interval lies wholly below that margin; a mere failure to find is not evidence of absence). Every
  refuting test is a finding, so a plan's refuting tests are corrected together: with k of them, each runs at alpha
  0.05/k (Bonferroni, as [A9-A15.md § A10](A9-A15.md#a10-welfare-and-affect-read-from-behaviour-alone) did), and a
  true hunch is wrongly refuted with probability at most 0.05.
- **Passed**, by a test that finds what the hunch predicts. The hunch passes only when every clause passes, which
  already bounds a wrongful pass at 0.05 (an intersection-union test), so pass tests run at alpha 0.05 uncorrected.
- **Inconclusive**, when neither test decides: reported so, never as a pass or a refutation, and kept under *To
  Revisit* with its power, to be run again with more seeds.

Seed counts are a floor. When a plan is prepared, its power is computed and stated for both decisions - the chance a
true hunch passes, and the chance a hunch whose effects are null is refuted - and the seeds are raised if either is
low. At the format's 20 seeds per arm a one-sided test has about 80% power at a standardised difference of 0.8
([README.md § The Format](README.md#the-format)); a margin-based refutation can be weaker still, and at 20 lives some
cannot fire at all against a true null (the third verification of
[#19](https://github.com/ai-quokka-wannabe/iso-lab/pull/19) showed this for B4's third clause, and noted it for B2).

Invariance clauses - B3's and B6's - compare by the frozen benchmark's per-feature comparisons, and pass by that
benchmark's own rule, which must then be an equivalence rule with a stated power, not a mere failure to find a
difference; if the frozen benchmark's rule is not, the clause says so when it is prepared, as the fourth verification
of #19 noted.

Costs are counted in simulated lives. What one life costs in compute and memory is not known until the arena is
proven (STATE.md item 3), so no plan gives absolute figures yet; each will when it is prepared. Seed counts follow
[README.md § The Format](README.md#the-format) unless a plan says why it differs.

## B1. A nervous system, not a game client

- **Question**: does every field a Program receives, and every action it sends, correspond to something a worm's
  sensory and motor biology can transduce or produce - at a comparable resolution and with a comparable delay - and
  does a blind field-by-field audit catch one that does not?
- **Method**: list every field of the senses (`TglSenses`) and of the actions (`TglActions`: the speed and turn
  intents, `joint_targets`, the call) in the Program interface at the pinned world ([PROGRAM_INTERFACE.md at
  c083262](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/c083262e473884cb60e83e95cddb851e4c7a8962/docs/PROGRAM_INTERFACE.md),
  also in `external/tron-grid-lite`, and its header). For each sense, name the worm's organ or neuron class that could
  transduce it; for each action, the worm's motor output that could produce it; each with a source read at the source,
  or marked as having none. Grade each field: carried by the animal; carried at a resolution the animal lacks; carried
  with a delay shorter than the animal's; not carried at all. The delay grades are read against the latencies of the
  pace note ([docs/research/pace.md](../../research/pace.md)); until the latencies they need are in it they are marked
  *pending*, not passed.
- **Baselines and controls**: a blind planted field. One agent prepares three copies of the field list, into which it
  plants none, one or two ground-truth fields (for example the body's position, or the distance to the nearest
  wall) at random places, and keeps the key; a second agent that has seen neither the key nor the preparation audits
  all three. A field the worm is known to have must pass: touch along the body, which six touch receptor neurons
  carry, with separate pathways for anterior and posterior touch ([Chalfie et al.
  1985](https://doi.org/10.1523/JNEUROSCI.05-04-00956.1985); the abstract was read, not the paper).
- **Refuted if**: the blind audit passes any planted field, or fails the known one. A field graded anything but *carried by the animal* is not a
  refutation of the hunch but a finding: it goes to the comparison space as a sense the Program must not rely on as
  given, and to the world's repository as an issue. Whether a Program may itself coarsen or delay a field the world
  gives too finely or too fast is a question for the maintainer, written in STATE.md, not decided by the audit.
- **Statistics**: none; a graded list, each grade with its source, and the planted-field key against the audit.
- **Seeds and repetitions**: not applicable; the three planted copies are the repetitions.
- **Cost**: reading, by two agents.
- **Waits on**: nothing for the senses, actions and resolution; the delay grades wait on the latencies still missing
  from the pace note (STATE.md item 1).
- **Leads**: found - Chalfie et al. 1985, "The neural circuit for touch sensitivity in *Caenorhabditis elegans*"
  (*The Journal of Neuroscience*, [doi:10.1523/JNEUROSCI.05-04-00956.1985](https://doi.org/10.1523/JNEUROSCI.05-04-00956.1985)).
  The other sources are the worm's sensory and motor biology, to be found and read field by field.

## B2. Smell as an unlabelled fingerprint

- **Question**: if each odour source carries a fixed multi-dimensional pattern, does a learning Program's recognition
  generalise to similar patterns and get confused by blends, as the hunch says real recognition should?
- **Method**: train the Program to approach a source with pattern P; then test it with patterns at increasing
  distances from P, and with blends of P and an untrained pattern Q in proportions from 0 to 1. Distance in pattern
  space is the Euclidean distance between the two patterns after each is scaled to unit length; the patterns'
  dimension is fixed when the Grid's chemical sense is specified, and if that sense makes another distance more
  natural, the change is written in the journal before any run. The hunch's world-side properties - intensity, its
  change over time, a left-right difference, blending mixtures, blurring old traces - are properties of the sense,
  not of a brain: they are added to the smell request in STATE.md *Needs From Other Repositories*, which the
  maintainer takes up, and this plan tests only the brain's side.
- **Baselines and controls**: an untrained Program tested on the same patterns - it fails, and the run is void, if
  its distance curve's 95% rho interval lies wholly below zero (a gradient without training comes from the patterns or the
  harness). A Program trained on Q, tested on the same P-centred patterns - it fails, and the run is void, if its 95% rho
  interval also lies wholly below zero (the P-trained gradient would then not be P's).
- **Refuted if**: the approach response is shown not to fall with distance from P - the one-sided 97.5% interval of
  Spearman's rank correlation between distance and response lies wholly above -0.3; or the response to blends is shown
  not to rise with the proportion of P - the one-sided 97.5% interval of its rho, between the proportion of P and the
  response, lies wholly below 0.3. Two refuting tests, each at 0.05/2; the margin, a rank correlation of 0.3 in size,
  is fixed here. Rho is computed over the 8 test points, from each point's mean response over its 20 lives, and its
  interval by resampling lives within each point. A clause passes if its one-sided 95% interval lies wholly on the
  predicted side of zero, and is inconclusive otherwise.
- **Statistics**: Spearman's rho over the point means, with bootstrap intervals, per curve.
- **Seeds and repetitions**: 20 seeds per test pattern; at least 8 distances and 8 blend proportions.
- **Cost**: 20 lives per test point, 16 points - about 320 test lives for each of the three Programs (P-trained,
  Q-trained, untrained), 960 in all, plus the training lives of the two trained Programs, whose number H9 will fix.
- **Waits on**: a Program that reads the Grid's chemical sense, which has carried food odour since Program ABI 11 (an
  odour other than food's waits on the world; STATE.md *Needs From Other Repositories*), and a Program that learns
  (H9).
- **Leads**: none named.

## B3. Senses switched on by rung

- **Question**: when the next rung of the ladder only adds sense channels, do all of the previous rung's tests still
  pass, and can they all still be run?
- **Method**: run the worm's frozen benchmark under four conditions: (a) the worm Program on the worm rung (its own
  score); (b) the same Program on the next rung's interface, receiving the extra channels with the arena's real
  readings but reading none of them; (c) as (b), with the extra channels fed noise; (d) a Program that uses the extra
  channels. Because the Program in (b) and (c) reads none of them, these two arms test the interface and the harness,
  not a brain: they can fail only if the added channels disturb the others (a framing, ordering or timing fault),
  which is what "only adds" forbids. Arm (d) is a different animal and may rightly score differently, so its score is
  reported, not compared; its role is to show that every worm measure can still be computed on the next rung.
- **Baselines and controls**: arm (a) is the baseline. Arm (c)'s noise goes to the Program of arm (b), so (b) against
  (c) checks that the content of the extra channels cannot reach a Program that ignores them.
- **Refuted if**: arm (b) or (c) differs from arm (a) on any benchmark measure by the per-feature comparisons the
  frozen benchmark will fix, with its correction for multiple comparisons, each of the two arms at a family-wise alpha
  of 0.05/2 (both clauses fire on finding); or any benchmark measure cannot be computed for arm (d) (exact).
- **Statistics**: the frozen benchmark's per-feature comparisons.
- **Seeds and repetitions**: 20 seeds per condition.
- **Cost**: four sets of 20 lives, 80 in all.
- **Waits on**: the frozen benchmark, a worm Program, and a second rung's senses on the Grid (the ladder of
  [PERCEPTION.md](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/c083262e473884cb60e83e95cddb851e4c7a8962/docs/PERCEPTION.md)).
- **Leads**: none named.

## B4. How a reversal ends decides whether chemotaxis works

- **Question**: does chemotaxis depend on whether backing up reorients the body - so that with reversals that do not
  reorient, reversals not followed by an omega turn stop helping, and chemotaxis fails on some seeds?
- **Method**: on the Grid's chain body the Program sends `joint_targets[7]`, and the interface says the gait - which
  joint bends when - is the Program's own (PROGRAM_INTERFACE.md § Actions, at the pinned commit). The two motor
  semantics of the hunch can therefore be two gait layers of the Program on the unchanged Grid: one whose reversal
  ends by bending the body onto a new heading, one that backs straight and resumes the old heading. Nothing in the
  world changes (*Realism over speed*). Step 1, before any brain: send each reversal as a scripted joint pattern, with
  no Program deciding, and measure the change of heading across it. Heading is the direction of the body's centroid
  travel over one whole undulation period, so the undulation itself does not enter it; the change is the angle between
  the heading over the last period before the reversal and over the first period after forward motion resumes. Its
  absolute value is used, because a reorienting reversal may turn either way; measurement noise biases an absolute
  value upward, so the same measure is first taken across forward runs with no reversal (the noise floor, 20 lives),
  comparing two undulation periods as far apart as a typical reversal lasts, so that heading drift is counted in the
  floor, and the floor's mean is subtracted before the thresholds are applied. Up to five patterns per semantics are
  written down before step 1 and tried in that order, stopping at the first that qualifies; the search is a finding,
  so each try is at alpha 0.05/5. A pattern reorients if the one-sided 99% interval of its mean change lies wholly
  above 30 degrees, and keeps the heading if it lies wholly below 10 degrees. Both thresholds are fixed here, and are
  checked against the real worm's heading change after a reversal once the recordings are read and before step 1 runs:
  if the worm's reversals do not fall on the reorienting side of 30 degrees, the thresholds are revised in the
  journal, with the worm's figure, before any scripted life. If no pattern reorients, the Grid's physics may be what
  prevents it, and it is written up as an issue for tron-grid-lite or master-control, with the patterns tried. If no
  pattern keeps the heading, the non-reorienting semantics does not exist on the Grid's body; that is a finding too,
  step 2 runs in the dish alone, and its result is reported as the dish's, not the Grid's. Step 2: a chemotaxis
  Program that steers by reversals alone (the pirouette strategy of [Pierce-Shimomura et al.
  1999](https://doi.org/10.1523/JNEUROSCI.19-21-09557.1999), with any other steering switched off, so reversals are
  its only lever), run with each gait layer in a chemical gradient. Per life, the score is +1 if it ends in the
  attractant zone, -1 in the control zone and 0 elsewhere; the chemotaxis index is the mean over lives. This form is
  written from memory of the population index in assays like those of [Bargmann et al.
  1993](https://doi.org/10.1016/0092-8674(93)80053-H), whose abstract, the only part read, does not give it; the whole
  form, zones and denominator included, is checked against the paper and fixed before any run. Measure under each
  semantics: the index; the heading change towards the gradient after each reversal not followed by an omega turn,
  over the same window as step 1; the share of lives that fail (a score of 0 or below); and the share of reversals
  followed by an omega turn, by the definition in [the behaviour-measures note](../../research/behaviour-measures.md).
- **Baselines and controls**: a Program with the same gait layers whose reversals are random in time, at the same rate:
  it fails, and the run is void, if its chemotaxis index's 95% interval lies wholly above zero under either
  semantics (taxis that does not come from informed reversals). The real worm, read from the recordings: the
  reorienting semantics must match the worm's heading change after a reversal and its reversal-to-omega share, by the
  frozen benchmark's per-feature comparisons; if it does not, the comparison is between two gaits neither of which is
  the worm's, and the run is void for this hunch until the gait matches.
- **Refuted if**: any of three effects the hunch claims, non-reorienting against reorienting, is shown smaller than
  its smallest effect of interest - its one-sided interval at 1 - 0.05/3 lies wholly below it (three refuting tests):
  (1) the chemotaxis index is lower, by at least 0.1; (2) the mean heading change towards the gradient after reversals
  not followed by an omega turn is lower, by at least 10 degrees; (3) the share of failing lives is higher, by at least
  10 percentage points. The margins are fixed here, before any data. Each clause passes if its one-sided test at alpha
  0.05 finds the effect (a permutation test on per-life scores for 1, on per-reversal changes permuting whole lives
  for 2, Fisher's exact test for 3), and is inconclusive otherwise. At 20 lives per arm, clause 3 cannot refute when
  both arms fail equally (the third verification of #19 enumerated every outcome: at 0 of 20 against 0 of 20 the
  Newcombe interval's upper bound is 0.119, above the margin), and clause 1 is likely in the same position; the seeds
  for step 2 are sized when it is prepared so that every clause can both pass and refute, and the power of each is
  stated.
- **Statistics**: the three tests above, with bootstrap intervals for 1 and 2 and a Newcombe interval for the
  difference of shares in 3; the omega share with a Wilson interval; the controls' rules as stated.
- **Seeds and repetitions**: step 1, 20 lives per scripted pattern; step 2, 20 lives per semantics, the format's
  default, and as many for the control.
- **Cost**: step 1, 20 lives for the noise floor and 40 to 200 Grid lives (20 per pattern tried, two to ten
  patterns); step 2, at least 80 lives in whichever world has the gradient, more if the power calculation asks.
- **Waits on**: step 1, the arena only (STATE.md item 3). Step 2 waits on a chemical gradient - the Grid's food odour,
  carried since Program ABI 11, or B5's dish - and a chemotaxis Program.
- **Leads**: none named for the hunch. The measures' sources: Pierce-Shimomura, Morse and Lockery 1999, "The
  fundamental role of pirouettes in *Caenorhabditis elegans* chemotaxis" (*The Journal of Neuroscience*,
  [doi:10.1523/JNEUROSCI.19-21-09557.1999](https://doi.org/10.1523/JNEUROSCI.19-21-09557.1999)) - cited elsewhere,
  read on its PMC page for [the behaviour-measures note](../../research/behaviour-measures.md); Bargmann, Hartwieg
  and Horvitz 1993, "Odorant-selective genes and neurons mediate olfaction in *C. elegans*" (*Cell*,
  [doi:10.1016/0092-8674(93)80053-H](https://doi.org/10.1016/0092-8674(93)80053-H)) - found in Crossref by DOI, only
  its abstract read.

## B5. A fast agar-plate world for iterating

- **Question**: can a small two-dimensional dish world, with the Grid's sensory and motor interface, reproduce the
  published assays fast enough to iterate on - and does a brain that passes there and fails on the Grid show what the
  dish leaves out?
- **Method**: build the dish's world as the hunch describes it - point sources with diffusing gradients that deplete
  and drift, a temperature gradient, food eaten only when moving forward near it, head-only sensing - but with two of
  the hunch's features moved to where they belong. Walls give touch, as the Grid's do, and the reversal at a wall is
  the Program's response to that touch; the cultivation set point is the animal's memory of where it was raised, so
  it lives in the Program, not in the dish. The dish speaks the Grid's interface, plus the senses an agar plate
  needs that the Grid lacks - a chemical sense, and a sense of the substrate's temperature (the Grid's thermal sense,
  `irradiance`, is radiant heat, not the temperature of what the body lies on) - in a provisional form, replaced by the
  world maintainers' own if they add them. Run A1's
  assays in the dish; move three brains from dish to Grid unchanged - the undulating control, B4's chemotaxis
  Program, and the best Program the lab has then - and run the same assays on the Grid. Before the comparison, write
  down and freeze the list of known differences between the two worlds.
- **Baselines and controls**: the same assays on real worms (A1's bands); a Program that only undulates (it must fail
  the assays that need steering, in both worlds; if it passes one, that assay is void in the world where it passed).
- **Refuted if**: a simulated second in the dish is not at least ten times cheaper in compute than on the Grid (not
  fast enough to iterate on); or more than one of A1's comparable assays cannot be run in the dish with its published
  protocol (geometry, timings, stimulus); or a brain's pass or fail differs between dish and Grid on more than one
  assay in five, every difference counted; or any difference is not explained by an entry of the frozen list (then a
  divergence does not show what the dish leaves out). Explained differences are the hunch's second half: they are
  reported as what the dish leaves out, but they still count towards the one in five.
- **Statistics**: compute per simulated second measured in both worlds; per assay, A1's rule in each world.
- **Seeds and repetitions**: 20 seeds per assay per world per brain.
- **Cost**: the dish's own build; 20 dish lives and 20 Grid lives per assay per brain, three brains.
- **Waits on**: A1's bands and the arena. A1's chemotaxis assays can be compared on the Grid only for odours
  it carries - food's alone, since Program ABI 11 (STATE.md *Needs From Other Repositories*); the rest run in the dish
  alone and the
  dish-to-Grid comparison covers the rest. The dish's temperature gradient is built because the hunch names it, but A1
  has no thermotaxis assay, so nothing is compared on it until one is planned with its source; a sense of the
  substrate's temperature on the Grid is not asked for until then. The dish is a tool, so it is
  written in Rust (the rule *Rust first*); it never scores a brain - scoring stays on the Grid.
- **Leads**: none named.

## B6. Behaviour must not depend on the tick rate

- **Question**: are a Program's behavioural statistics the same at two tick rates, and under fast-forward?
- **Method**: the Grid's tick is fixed: Master Control steps the world at 0.031 25 s, a constant in its source
  (`TICK_SECONDS` in `external/master-control/src/physics.rs` at the pinned commit), so the Grid itself cannot run at
  two rates. The rate test is therefore made where the rate can change - in B5's dish world at 32 Hz and 64 Hz, and
  in a Program's own internal integration step, at two steps per tick against one. Fast-forward: Master Control's
  heartbeat paces the ticks to the wall clock (`run` in `external/master-control/src/heartbeat.rs`), so the Grid has no
  fast-forward; in the dish, run the same seed paced to the wall clock and unpaced. A fast-forward on the Grid is a
  change for master-control, asked for there by issue if this experiment needs it when it is prepared.
- **Baselines and controls**: a scripted motor pattern with no Program, at 32 Hz and 64 Hz in the dish - its locomotion
  measures must agree by the same comparisons, or the dish's own physics depends on the rate and the run is void until
  the dish is fixed. An A/A control: the same rate twice with different seeds - if it shows a difference by the same
  comparisons, the run is void.
- **Refuted if**: any benchmark measure differs between the two rates, or between the two internal steps, by the
  per-feature comparisons the frozen benchmark will fix, with its correction for multiple comparisons, each of the two
  comparisons at a family-wise alpha of 0.05/2 (both fire on finding); or a paced and an unpaced dish run from the same
  seed differ at any tick (exact).
- **Statistics**: as above; exact equality for fast-forward.
- **Seeds and repetitions**: 20 seeds per rate, per internal step and per control arm; 20 paced-unpaced pairs.
- **Cost**: in the dish, 40 Program lives, 40 scripted lives, 20 A/A lives and 40 fast-forward lives; on the Grid, 40
  lives for the internal steps and 20 for their A/A control.
- **Waits on**: B5's dish, a Program, the frozen benchmark.
- **Leads**: none named; the rule *The animal's pace* is why it matters.

## B7. Do not count your own echo

- **Question**: does a Program discount sensory change it caused itself, keep a learned association when the stimulus
  is merely absent, and neither count one piece of evidence twice when it arrives by two paths nor discard it?
- **Method**: three probes. Reafference: let the Program move in a world that does not change, and measure the change
  in whatever internal belief the senses update; an efference copy should cancel it. Absence: after the Program has
  learned an association (a stimulus with food), give it a stretch with no stimulus at all, and measure the
  association's strength (its response to the stimulus) before and after. Reconvergence: deliver the same evidence by
  two sensory paths at once, and by each alone, and compare the belief update with the larger single-path update. For
  each probe, the margin delta is a tenth of its control's effect (below), a choice made here, before any data: an
  effect a tenth the size of the one the mechanism exists to prevent.
- **Baselines and controls**: reafference - the same Program with its efference copy switched off (the rule
  *Pre-register and ablate*); absence - the same stretch filled with unrewarded presentations of the stimulus
  (extinction); reconvergence - the larger single-path update itself. Each control must show its effect: its 95%
  interval lies wholly above zero, or the probe is void (it cannot see what it tests).
- **Refuted if**, for any probe, the difference lies outside plus or minus delta - with the efference copy on, the
  belief's change under self-motion; the loss of association over the blank stretch; the both-paths update minus the
  larger single-path update (above delta is double counting, below minus delta is discarding) - judged by
  bootstrap intervals. A probe is refuted if the one-sided interval at 1 - 0.05/3 lies wholly above delta or wholly
  below minus delta (three refuting tests); it passes if the 90% interval lies wholly inside the band (equivalence by
  two one-sided tests at alpha 0.05, uncorrected); anything else is inconclusive.
- **Statistics**: bootstrap intervals over seeds, as above.
- **Seeds and repetitions**: 20 seeds per probe and per control.
- **Cost**: short probe runs on a frozen copy, not lives; the absence probe needs one learning life per seed first.
- **Waits on**: a Program with an efference copy, a belief to update and an association it learns - none planned yet;
  this plan says what one would have to show.
- **Leads**: none named.

## B8. A simulated body can be rolled back

- **Question**: can a Program's state be snapshotted, restored and diffed so exactly that a resumed run is
  indistinguishable from an uninterrupted one - and can the same be done for the simulated body?
- **Method**: the hunch has two halves, and this plan runs the Program's now and scopes out the world's. The world's
  half waits: Master Control has no save and restore of world state at the pinned commit (none found in its `src/`;
  its "snapshot" is the per-tick world state it sends on the wire, not a restore point), so rolling back the body is a
  change for master-control, asked for there by issue when this half is prepared. The Program's half, by replay, as
  the interface's *Determinism and Replay* section describes: record the senses of a life; replay them into the
  Program uninterrupted; replay again, snapshotting the Program's state at three pauses; restore each snapshot into a
  fresh process and replay the remaining senses; compare the action sequences tick by tick. Time-travel debugging is
  the same comparison from any snapshot, stepped one tick at a time. Diffs: a field-by-field diff of two snapshots,
  which must be empty for a snapshot and its restored copy.
- **Baselines and controls**: an A/A control, two uninterrupted replays of the same recording - if they differ at any
  tick, the run is void. A corrupted-snapshot control: before a restore, set one state variable to a value other than
  the one it holds, for each of five variables chosen at random by seed - the diff must name that variable every time, and at least one
  of the five must make the continuation diverge, or the run is void (the snapshot does not reach the state, or the
  comparison cannot see a difference); whether each continuation diverges is reported per variable.
- **Refuted if**: a restored continuation differs from the uninterrupted one at any tick, or the diff of a snapshot and
  its restored copy is not empty, given that both controls pass.
- **Statistics**: exact equality.
- **Seeds and repetitions**: 20 seeds, a snapshot at three points in each life.
- **Cost**: 20 recorded Grid lives; per seed, replays rather than lives: the A/A pair, three restored continuations and
  five corrupted ones.
- **Waits on**: a Program and the arena (STATE.md item 3), for the recorded lives; the world's half, a save and
  restore in Master Control.
- **Leads**: none named.

## B9. The insect bet, and the bridge it owes

- **Question**: does a body-first Program adapt to a perturbed body - a weakened joint, a lost segment - within a time
  like the animal's?
- **Method**: perturb the body in mid-life and measure how long the Program takes to recover its locomotion measures
  to their bands - A1's bands, each measured over the window A1 fixes for it; recovered at the first window in which all
  of them are inside their bands, and a life that never recovers is given its remaining length. The animal-like time is
  read from a source on worms with an impaired body, to be found; without one, that comparison waits. World-free
  stand-ins can run sooner: a lesion of the Program's own motor output - one joint's target scaled down after the brain has
  set it, which the brain is not told, like a motor-neuron ablation; and a lost segment between lives rather than within
  one, by rezzing a shorter chain - the Program's model declares its `segment_count`, from one to `TGL_SEGMENTS_MAX`
  (PROGRAM_INTERFACE.md § The Body's Own Shape, at the pinned commit). They are reported as stand-ins, not as
  the hunch's test.
- **Baselines and controls**: the same Program unperturbed (it must stay within its bands in at least 19 lives in 20,
  or the bands do not fit it and the run is void); the same Program with its learning switched off.
- **Refuted if**, by two refuting tests each at 0.05/2: the lower bound of the one-sided 97.5% bootstrap interval of
  the median recovery time lies above the animal's figure; or recovery with learning switched off is shown not to be
  slower - the one-sided 97.5% bootstrap interval of the ratio of median recovery times, learning off over on, lies
  wholly below 1.2, the smallest slowing of interest, fixed here (the recovery is then not adaptation). The first
  clause passes if the one-sided 95% interval's upper bound lies at or below the animal's figure; the second if a
  one-sided Mann-Whitney test at alpha 0.05 finds learning-off slower. Each is inconclusive otherwise.
- **Statistics**: recovery time per seed, its median with bootstrap intervals; the ratio of medians with a bootstrap
  interval; the Mann-Whitney test above.
- **Seeds and repetitions**: 20 seeds per perturbation and condition.
- **Cost**: three sets of 20 lives per perturbation, stand-ins included.
- **Waits on**: for the hunch's own test, a way to perturb a body in mid-life, which the world does not have: the body
  is authored by the Program at rez and its limits are the Grid's, so a mid-life weakened joint or lost segment is a
  change for tron-grid-lite and master-control, asked for there by issue when this experiment is prepared - never made
  by the lab alone (*Realism over speed*). The motor-output stand-in waits only on a Program.
- **Leads**: found - Brooks 1991, "Intelligence without representation" (*Artificial Intelligence*,
  [doi:10.1016/0004-3702(91)90053-M](https://doi.org/10.1016/0004-3702(91)90053-M)); Pfeifer and Bongard 2006, *How
  the Body Shapes the Way We Think* (MIT Press,
  [doi:10.7551/mitpress/3585.001.0001](https://doi.org/10.7551/mitpress/3585.001.0001)). Not found as a work -
  Braitenberg 1984, *Vehicles*: Crossref has reviews of it (Stefik 1985, Dennett 1986), not the book; it stays an
  unverified lead.
