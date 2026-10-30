# The Comparison Space

Which measures of a real worm's behaviour can be compared with the Grid's worm, in which units, and which cannot be
compared yet and why. This is Phase 0's third step ([docs/PLAN.md § Phase 0](../PLAN.md#phase-0-the-benchmark)): a
**draft**, not frozen. It fixes nothing a score depends on; the feature-extraction tool (STATE.md item 2) will test
every *comparable* below against the data, and the benchmark is frozen only after that, as the plan says.

The sources were read in full on PubMed Central on 2026-10-14, unless a line says otherwise: Yemini et al. 2013
([doi:10.1038/nmeth.2560](https://doi.org/10.1038/nmeth.2560), PMC3962822) and Flavell et al. 2013
([doi:10.1016/j.cell.2013.08.001](https://doi.org/10.1016/j.cell.2013.08.001), PMC3942133). Stephens et al. 2008 and
Brown et al. 2013 were read for [the behaviour-measures note](../research/behaviour-measures.md). The Grid's facts
are read from the world's sources at the commits `external/` pins, and were last re-checked at tron-grid-lite
`ee5332d`, master-control `53e5acb` and link `9b1b771` (journal of 2026-10-30, § A Coherence Audit, and What It
Mended).

## The Two Bodies

| | The real worm (the reference recordings) | The Grid's worm |
|---|---|---|
| Body | Soft, continuous; about a millimetre long | A chain of at most eight rigid segments joined by at most seven servos (`SEGMENTS_MAX` in link's `src/protocol.rs`), its length the Program's choice of `segment_count` and `segment_spacing` |
| What is recorded | A skeleton of 49 points per frame, from video (Yemini et al. 2013) | Every tick, each segment's position, yaw and pitch (`CreatureState` and `SegmentPose` in link's `src/protocol.rs`), on the Disk |
| Rate | 20 to 30 frames per second, "new videos" 30 (Yemini et al. 2013); 32.57 in record 1011832 ([data/README.md § The Phase 0 Survey](../../data/README.md#the-phase-0-survey)) | 32 ticks per second (`TICK_SECONDS` = 0.031 25 s in master-control's `src/physics.rs`) |
| Gaps | 46% of frames without a skeleton in record 1011832. Yemini et al. name three causes - a dropped frame, a stage movement, a failed extraction - and how the 46% splits among them is not yet known | None: every tick has every pose |
| Bending | Unbounded by the recording; local bends can be large | Each servo bounded: the Grid's limit for the class is 0.9 rad, about 52 degrees (`roster.cpp` in tron-grid-lite), under the world's ceiling of pi/2 (`WORLD_MAX_JOINT_ANGLE` in master-control's `src/roster.rs`) |
| Plane | Crawling on agar, two-dimensional | Three-dimensional poses; on the flat arena a trailing segment's pitch is zero, while the head "pitches as its own vertices meet the floor" (link's `SegmentPose` and `CreatureState`) |
| Surroundings | On a bacterial lawn (OP50), after a 30-minute wait, tracked for 15 minutes | A flat, foodless arena |

## Common Units

- **Length** in body lengths, L: the skeleton's arc length for the worm, the chain's length for the Grid's worm.
  Speeds are in L per second, as Yemini et al.'s motion-state thresholds already are.
- **Time** in seconds of simulated time, which the rule *The animal's pace* equates with the animal's real time, and
  in undulation periods where a measure is about the rhythm.
- **Posture at a common resolution.** The Grid's worm reports eight tangent directions (one yaw per segment) and seven
  joint angles; the worm's 49-point skeleton is finer. So posture is compared at the coarser one: the worm's skeleton
  is resampled to eight arcs of equal length, each arc's direction taken as that of its chord - what a rigid segment
  reports as its yaw, and expected, not yet shown, to be steadier than the local tangent at one point of a 49-point
  skeleton - and the mean angle
  removed, as
  Stephens et al. 2008 remove it from their 100 angles ("an overall rotation mode was removed"); the Grid's worm gives
  the same eight angles directly. Eigen-shapes are fitted on the real worms' eight angles and both bodies are
  projected on them. How much of the worm's posture survives at eight samples is itself a measure the tool reports:
  Stephens et al. find four eigenworms carry over 95% of the variance at full resolution, and Yemini et al. use six
  for 97%.
- **Body parts by fraction of length.** Yemini et al. define the head as the first sixth of the skeleton (points 1 to
  8) and measure each bend between edges a twelfth of it long on either side. The Grid's segments are eighths, so
  head, midbody and tail are redefined for both bodies as the same fractions of the resampled eight-arc skeleton, and
  Yemini et al.'s sixth-and-twelfth definitions are not used directly.

## The Measures

*Comparable* means the measure can be computed on both bodies in the units above, pending the tool's check; *with a
caveat* names what must be settled first; *not yet* names what is missing. The grades are about the two bodies only.
Every one of them also waits on the open questions at the end, food and the habituation window first, and no
locomotion measure counts as comparable until food is settled.

| Family | Measure (Yemini et al. 2013 unless named) | Status | Why |
|---|---|---|---|
| Morphology | Length, width, area | Not comparable | The Grid's body is authored by the Program at rez; its shape is a choice, not a behaviour |
| Posture | Eigen-shape amplitudes (Stephens et al. 2008) | Comparable | On the eight-angle resampling above |
| Posture | Bend angles at head, midbody and tail | With a caveat | At the eight-arc resolution; a real bend beyond the Grid's 0.9 rad per joint cannot be reproduced, and in one block's first measurement 9.8 to 68.6% (median 35.8%, 34 recordings, some of other lengths; not yet the reference statistic) of the worm's skeletonised frames had such a bend at some joint (journal of 2026-10-15, § The Joint-Limit Issue, Drafted; the first measurement is in the journal of 2026-10-14, *How Often the Reference Worms Bend Beyond the Grid's Joint Limit*) |
| Posture | Amplitude, wavelength, track length, eccentricity | With a caveat | Wavelength from eight samples resolves only waves longer than two segments; the tool reports whether the worm's waves are |
| Posture | Coils, kinks | With a caveat | Yemini et al. find coils from frames whose segmentation failed ("by scanning the video frame annotations"); the Grid's recordings never fail, so a coil needs a geometric definition, the body touching itself, applied to both. A coil also needs bends the 0.9 rad limit may forbid |
| Locomotion | Speed and angular speed of head, midbody, tail | Comparable | In L per second and radians per second |
| Locomotion | Forward, backward and paused states, by Yemini et al.'s thresholds | Comparable | The thresholds are fractions of length per second over at least half a second ([the behaviour-measures note](../research/behaviour-measures.md#locomotion-states)) |
| Locomotion | Reversals: frequency, duration, distance | Comparable | Built on the states above |
| Locomotion | Crawling amplitude and frequency | Comparable | From the bend time series; frequency in hertz and per undulation |
| Locomotion | Foraging (the nose's fast, aperiodic movements) | Not yet | Yemini et al. measure it as the angle between the head's first and second twelfths (skeleton points 1 to 4 and 5 to 8), up to 10 Hz. The Grid's first joint sits an eighth of the body back, and its head segment cannot bend within itself |
| Locomotion | Omega and upsilon turns | With a caveat | Defined from the body's bends - an omega turn's angle "must initially exceed 30° at the first but not the last third of the body" - which eight arcs can express; the 0.9 rad limit and the worm's segmentation gaps both bias the comparison |
| Path | Range from the track's centroid | With a caveat | In L; it depends on the arena's size in body lengths, which differs between a plate and the Grid until the arena is fixed |
| Path | Path curvature | Comparable | In radians per L |
| Path | "Dwelling" as occupancy of grid squares | With a caveat | Yemini et al.'s grid squares are a worm's width across; in body lengths they are comparable, but the name is not Flavell et al.'s dwelling state |
| States | Roaming and dwelling (Flavell et al. 2013) | With a caveat | See below |
| Motifs | Behavioural motif dictionary (Brown et al. 2013) | With a caveat | Fitted on eight-angle postures, if the eigen-shapes carry the motifs at that resolution |
| Responses | Chemotaxis, thermotaxis, touch responses | Not yet | The reference recordings are of spontaneous behaviour, with no stimulus; and the Grid has no chemical sense (STATE.md *Needs From Other Repositories*) |

## Roaming and Dwelling in Fifteen Minutes

The question the plan left open. Flavell et al. 2013 recorded worms "for 90 minutes at 3 frames per second" on
large lawns, averaged speed and angular speed over 10-second intervals, and segmented them with a two-state hidden
Markov model. Dwelling durations fit a single exponential with τ = 482 s; roaming durations fit two, τ = 75 s and
520 s, and "85% of roaming states have a mean lifetime of 75 sec".

A reference recording lasts 15 minutes, 900 s, by the protocol (most do; see *Unequal lengths and frame rates* below). So:

- **The share of time in each state** can be estimated from 15 minutes, with the same 10-second bins and a two-state
  model, but each recording holds few states: a dwelling state averages 482 s and a roaming state about 142 s (0.85 ×
  75 s + 0.15 × 520 s), so 900 s spans about 1.4 dwelling-roaming cycles on average.
- **State durations cannot be compared from 15 minutes.** A large share of dwelling states and of long roaming states
  are cut off by the recording's start or end, so their durations are censored; only the short roaming states (τ = 75
  s) are seen whole often enough. A survival analysis that treats cut-off states as censored could estimate the
  dwelling time constant, but how well at 900 s per worm is for the tool to show, not for this note to assume.
- **The states are defined on food.** Flavell et al. describe them as foraging states on a bacterial lawn; whether a
  worm on the Grid, without food, should show them at all is part of the next question.

## Open Questions Before Freezing

- **Food.** The reference worms are on food after a 30-minute wait (Yemini et al. 2013), and the Grid's arena has
  none. Roaming and dwelling are defined as foraging states on food, and Flavell et al. 2013's introduction,
  summarising earlier work, says "the proportion of time spent roaming increases when food is limited or low in
  quality" and that serotonin and dopamine "promote food-related behaviors such as eating, egg-laying and slow
  locomotion"; so food changes what the benchmark measures, though the size of the change for each measure is still to
  be read from the primary sources. A Grid worm compared with on-food worms is compared with an animal in another
  condition. One pointer for later: Stephens et al. 2008 filmed worms "rinsed of E. coli", on plates with no lawn
  mentioned, so their eigenworms describe worms off food. Each paper's variance share comes from eigen-shapes fitted
  to its own data, so the two figures say nothing about whether postures on and off food match. *A hunch*: posture
  depends on food less than locomotion does. It would be tested by projecting both datasets on one shared basis and
  comparing the distributions of the amplitudes, and refuted if they differ by the benchmark's own comparison. Three
  ways forward, none chosen: compare with the reference's N2 recordings made without food (28 records by their
  metadata, which may overlap the males, larvae and recordings without the wait; [data/README.md § The Phase 0
  Survey](../../data/README.md#the-phase-0-survey); their protocol not yet read); ask the world for food and a feeding
  signal, which hunch B1's "am I taking in energy now?" also needs; or restrict the benchmark to measures shown not to
  differ between on and off food in the worm, from a source still to be found. At the maintainer's request the world's
  half is asked for by issue: food on the floor in
  [master-control#63](https://github.com/ai-quokka-wannabe/master-control/issues/63), a sense of the body's own intake
  in [tron-grid-lite#153](https://github.com/ai-quokka-wannabe/tron-grid-lite/issues/153).
- **Which speed the locomotion states are computed from: decided (journal of 2026-10-30, § One Instrument for Both
  Bodies' Locomotion States).** Both bodies' forward, backward and paused states are computed by the lab's one
  pipeline. Yemini et al.'s rule and thresholds are applied unchanged (`features::states`) to the lab's eight-arc
  midbody speed and length with the published choices (`locomotion-states --published-choices`), on the reference
  skeletons, and it will be applied to the Grid's poses alike once a path reads them. The published `motion_modes` are not the reference states.
    - *Why, and the options weighed*: in the journal entry above. In short, mixing instruments would score a
      measurement bias as behaviour, and the other single-instrument options would add parameters with no source.
    - *What it costs*: the reference statistics differ from Yemini et al.'s published states, with more frames in no
      state and every state's share lower, paused most; the difference is reported beside them.
    - *What it does not close*: the reference skeletons' tracking noise and gaps (§ The Two Bodies).
    - *What would reopen it*: the check of durations and reversal rates (STATE.md item 2(c)), if the lab's speed misses
      the published states there for a cause other than the spacing.
- **The habituation window.** Yemini et al. tracked 25 wild-type worms for two hours without the usual wait and found
  speed decaying with τ = 19 minutes, midbody amplitude with τ = 48 minutes and crawling frequency with τ = 19
  minutes. The reference recordings start 30 minutes after the worm is moved, so a Grid life must be compared over the
  matching window, or the creature's own settling must be shown to follow the worm's; which, is fixed here before
  freezing, with the settling time constants of [the pace
  note](../research/pace.md#time-constants-of-internal-states).
- **Missing frames.** Yemini et al. drop frames that were dropped because the processor could not keep up with the
  video, blurred by a stage movement, or whose extraction failed, and note that coiling worms "suffer a large portion
  of segmentation failures". So measures that depend on large bends may be biased in the reference and not in the
  Grid's recordings. The tool must report how the gaps split by cause and, per measure, how much it depends on frames
  likely to be lost, and the benchmark must say whether the Grid's frames are masked to match.
- **The joint limit.** If a real worm's bend over an eighth of its length often exceeds 0.9 rad, the Grid's body
  cannot be indistinguishable on posture whatever its brain does. That would be a finding about the body, for the
  world's maintainers by the rule *This repository, and the world's by their own rules*, never a reason to tune the
  benchmark. A first measurement is in the bend-angle row of *The Measures* above.
- **The arena.** Path measures depend on the arena's size in body lengths; a plate and the Grid's arena must be
  matched, or path measures restricted to those that do not reach the edge.
- **Which recordings.** The N2 subset and how it is drawn are fixed by the tool's task (STATE.md item 2), before any
  statistic is computed: block 1 of the seeded draw ([data/README.md § The Reference
  Draw](../../data/README.md#the-reference-draw)).
- **Unequal lengths and frame rates.** The recordings are not all 15 minutes long, nor filmed at one frame rate: 5 and
  120 minutes are among those read so far ([the journal of 2026-10-14 § How Long the Reference Recordings
  Are](../journal/2026-10-14.md#how-long-the-reference-recordings-are)), and frame rates from 25.4 ([§ Reading the
  Features Files](../journal/2026-10-14.md#reading-the-features-files)) to 32.57 frames per second ([data/README.md §
  The Phase 0 Survey](../../data/README.md#the-phase-0-survey)). Measures are taken in seconds, and the benchmark
  must say how recordings of unequal length enter it, by a fixed window of each, by excluding those far from 15
  minutes, or otherwise, before any statistic is computed.
