# How Worm Behaviour Is Measured

The measures the benchmark will be built from, and where each comes from. Every claim below was read from the
paper's text in PubMed Central on 2026-10-13, through NCBI's E-utilities or the article's PMC page; all five
papers were read in full. How sure the lab is: high for the quoted definitions and numbers, which are the
papers' own; lower for the paragraphs headed *For the lab*, which are the lab's reasoning and untested.

## Posture: a Few Eigen-Shapes

[Stephens et al. 2008](https://doi.org/10.1371/journal.pcbi.1000028) describe the worm's shape by the
tangent angle along its body and find that the shapes of crawling worms live in a space of few dimensions:
"over 95% of the total variance in angle along the body is accounted for by just four eigenvalues". The four
shapes are the *eigenworms*.

- **The data**: tracking microscopy of single worms; the equations of motion were built from 12 worms, 5
  trials each, 4000 frames a trial, and the phase-space densities from worms sampled at 32 Hz.
- **Undulation as a phase**: the first two eigenworm amplitudes trace a rotation, so their angle is a phase
  φ of the body wave. Its rate, the phase velocity ω = dφ/dt, predicts the worm's crawling speed; the phase
  grows steadily during forward crawling, with "occasional, abrupt reversals".
- **Robust to condition**: the same four modes still capture about 95% of the shape variance during a thermal
  response, where the distribution of shapes is very different.
- **For the lab**: posture and the rhythm of undulation become comparable between a real worm and a body of
  seven joint angles by fitting eigen-shapes to each and comparing in phase and in periods of the wave rather
  than in millimetres. How many modes seven joints can carry is for the comparison space to work out.

## The Feature Database

[Yemini et al. 2013](https://doi.org/10.1038/nmeth.2560). The Open Worm Movement Database record read in the
survey names its protocol as this paper's method
([data/README.md § The Phase 0 Survey](../../data/README.md#the-phase-0-survey)).

- **What was recorded**: 1218 N2 young-adult hermaphrodites over three years, 2009 to 2012, and at least 20
  worms of each mutant strain filmed, each "spontaneously behaving on food", given 30 minutes to habituate and
  then tracked for 15 minutes. Videos entered the comparisons only if they ran at least 20 frames per second for
  14 to 15 minutes with at least 20% of frames segmented, which left "a mean of 24 worms per strain with a
  minimum of 12".
- **What was measured**: "measures of morphology, posture, locomotion, and path dynamics", with behavioural
  events such as reversals and omega turns generating secondary measures such as event frequency, time spent
  and distance covered. "Altogether, the permutation of these measures yielded 702 distinct feature
  measurements."
- **How strains were compared**: each feature against wild type by a Wilcoxon rank-sum test, with Fisher's
  exact test for measurements found only in one group.
- **Event definitions are thresholds** (see *Locomotion States* below). The benchmark has to fix such
  thresholds in units both bodies share before any comparison.
- **A difference worth noting**: the paper counts 1218 N2 worms recorded 2009 to 2012; the survey counted
  1792 N2 records on Zenodo, recorded 2009 to 2016. The two are not the same set, and which records the
  paper used is not yet traced.

## Behavioural Motifs

[Brown et al. 2013](https://doi.org/10.1073/pnas.1211447110), read on its PMC page (PMC3545781), carry the
eigenworms over to mutants: "four basic shapes, or eigenworms, previously described for wild-type worms, also
capture mutant shapes", and the representation is used "to build a dictionary of repetitive behavioral motifs
in an unbiased way". Measuring the distance between an individual's behaviour and each motif in the dictionary
gives a fingerprint that compares mutants with wild type and with each other.

- **For the lab**: a motif dictionary is a candidate measure that needs no hand-set event thresholds; whether
  motifs fitted to real worms make sense on seven joint angles is for the comparison space to judge.

## Locomotion States

Yemini et al. 2013 define the motion states by thresholds in units of the worm's own length:

- **Forward**: a period of more than half a second in which the worm "travels at least 5% of its mean length
  over the entire period" and its speed "is at least 5% of its length, per second, in each frame".
- **Backward** (a reversal): the same, with the midbody speed negative.
- **Paused**: a period of more than half a second in which the forward and backward speeds "do not exceed 2.5%
  of its length, per second, in each frame".
- **Omega and upsilon turns**: computed from the bends of the skeleton; an omega turn's body angle
  "must initially exceed 30° at the first but not the last third of the body".

Because the thresholds are fractions of body length per second and durations in seconds, they carry over to a
body of any size, which is what the comparison space needs.

[Pierce-Shimomura et al. 1999](https://doi.org/10.1523/JNEUROSCI.19-21-09557.1999), read on its PMC page
(PMC6782915), divide each track "into periods of smooth swimming (runs) and periods of frequent turning
(pirouettes)"; in a chemical gradient, "the initiation of pirouettes was correlated with the rate of change of
concentration (dC/dt) but not with absolute concentration", and the average heading after a pirouette was up
the gradient. Chemotaxis itself is not comparable on the Grid yet: the Grid has carried a chemical sense since Program ABI 11, but
no Program reads it and no feature measures chemotaxis ([STATE.md](../../STATE.md)); the run and pirouette structure
of a track is.

### How the Toolbox Turns States Into Durations and Rates

Read at the source on 2026-10-30, the OpenWorm analysis toolbox at commit `181ab13`, as in the journal of 2026-10-15
(§ The Published Locomotion States, Read at Their Source):
[`features/events.py`](https://github.com/openworm/open-worm-analysis-toolbox/blob/181ab13/open_worm_analysis_toolbox/features/events.py),
`EventListWithFeatures` (SHA-256 `c0349183…c124`), and
[`features/locomotion_features.py`](https://github.com/openworm/open-worm-analysis-toolbox/blob/181ab13/open_worm_analysis_toolbox/features/locomotion_features.py),
`MotionEvent` (`334a66de…bc23`).

- **Each mode has its own event list.** `MotionEvent` runs `EventFinder` once per mode, forward, backward and paused,
  on the midbody speed, and builds an `EventListWithFeatures` from each. The per-frame `motion_modes`, where a later
  mode overwrites an earlier one, are built separately. So durations and rates come from each mode's own events, and
  an event that the per-frame states partly overwrite still counts whole in its own list.
- **Event spans include untracked frames.** `EventFinder.get_start_stop_indices` extends the first event back to frame
  0 when every value before it is undefined, and the last event forward to the final frame likewise; `remove_gaps`
  joins runs across short gaps. So an event's span counts the untracked frames inside it, and an edge event's span
  takes in the whole untracked lead-in or lead-out.
- **Durations**: `event_durations = (end_frames - start_frames + 1) / fps`, for every event, those cut off by the
  recording's ends included, and with the untracked frames above counted. The times between events are `(next start - end - 1) / fps`.
- **Distance**: per event, the sum over its frames of `|midbody speed| / fps`, skipping frames without a speed.
- **Total time**: `num_video_frames / fps`, with `num_video_frames` set to the length of the midbody speed. That is
  every frame of the recording, those without a skeleton included.
- **Frequency**: `num_events_for_stats / total_time`, in events per second, so the reversal rate is backward events
  per second of recording. `num_events_for_stats` leaves out an event already under way at the first frame and one
  still under way at the last, judged on the extended spans, so an event whose first tracked frame comes well after
  an untracked start counts as partial. It does so only when there are at least two events: a lone event touching an
  edge is counted.
- **No events**: when a mode has none, `is_null` is set and its frequency and time ratio are undefined (NaN), not
  zero. A recording with no reversals has an undefined reversal rate in the toolbox.
- **Time ratio**: `nansum(event_durations) / total_time`, partial events included.

Two cautions. The docstring names a MATLAB original (`+seg_worm/+feature/+event/event.m`), and whether the reference
records' stored frequencies were computed by this Python or by that MATLAB is not yet shown; the stored motion modes
were reproduced exactly from this rule (journal of 2026-10-15). And the edge rule's exception for a lone event may be an
accident of the code rather than a choice: that is unverified, to be settled by reading `event.m`. The lab's own
durations and rates should follow it only where matching the published values is the point.

## Roaming and Dwelling

[Flavell et al. 2013](https://doi.org/10.1016/j.cell.2013.08.001): worms on food "spontaneously switch between
two discrete foraging states called roaming and dwelling". Both include forward locomotion, reversals and
turns, but in combinations specific to each state that "last several minutes and change through abrupt
transitions". The states were scored from the speed and angular speed of each animal, filmed for 90 minutes on
large (600 cm²) homogeneous bacterial lawns.

- **For the lab**: the 15-minute recordings of the feature database are short against states lasting several
  minutes. Checked in [the comparison space](../benchmark/FEATURES.md#roaming-and-dwelling-in-fifteen-minutes):
  the share of time in each state can be estimated from 15 minutes, the states' durations cannot.

## What This Note Does Not Cover Yet

- The reaction latencies and the time constants of internal states that the pace rule asks for, each from a
  cited source: the next literature task in [STATE.md](../../STATE.md).
- How sure the lab is that each measure carries over to the Grid's body: the comparison space's job.
