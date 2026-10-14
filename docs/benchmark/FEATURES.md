# The Comparison Space

Which measures of a real worm's behaviour can be compared with the Grid's worm, in which units, and which cannot be
compared yet and why. This is Phase 0's third step ([docs/PLAN.md § Phase 0](../PLAN.md#phase-0-the-benchmark)): a
**draft**, not frozen. It fixes nothing a score depends on; the feature-extraction tool (STATE.md item 2) will test
every *comparable* below against the data, and the benchmark is frozen only after that, as the plan says.

The sources were read in full on PubMed Central on 2026-10-14, unless a line says otherwise: Yemini et al. 2013
([doi:10.1038/nmeth.2560](https://doi.org/10.1038/nmeth.2560), PMC3962822) and Flavell et al. 2013
([doi:10.1016/j.cell.2013.08.001](https://doi.org/10.1016/j.cell.2013.08.001), PMC3942133). Stephens et al. 2008 and
Brown et al. 2013 were read for [the behaviour-measures note](../research/behaviour-measures.md). The Grid's facts
are read from the world's sources at the commits `external/` pins (tron-grid-lite `c083262`, master-control
`03e71ce`, link `75383e7`).

## The Two Bodies

| | The real worm (the reference recordings) | The Grid's worm |
|---|---|---|
| Body | Soft, continuous; about a millimetre long | A chain of at most eight rigid segments joined by seven servos (`SEGMENTS_MAX` in link's `src/protocol.rs`), its length the Program's choice of `segment_count` and `segment_spacing` |
| What is recorded | A skeleton of 49 points per frame, from video (Yemini et al. 2013) | Every tick, each segment's position, yaw and pitch (`CreatureState` and `SegmentPose` in link's `src/protocol.rs`), on the Disk |
| Rate | About 30 frames per second (32.57 in record 1011832, [data/README.md § The Phase 0 Survey](../../data/README.md#the-phase-0-survey)) | 32 ticks per second (`TICK_SECONDS` = 0.031 25 s in master-control's `src/physics.rs`) |
| Gaps | 46% of frames without a skeleton in record 1011832; segmentation fails most where the body coils | None: every tick has every pose |
| Bending | Unbounded by the recording; local bends can be large | Each servo bounded: the Grid's limit for the class is 0.9 rad, about 52 degrees (`roster.cpp` in tron-grid-lite), under the world's ceiling of pi/2 (`WORLD_MAX_JOINT_ANGLE` in master-control's `src/roster.rs`) |
| Plane | Crawling on agar, two-dimensional | Three-dimensional poses; on the flat arena pitch is zero (link's `SegmentPose`) |
| Surroundings | On a bacterial lawn (OP50), after a 30-minute wait, tracked for 15 minutes | A flat, foodless arena |

## Common Units

- **Length** in body lengths, L: the skeleton's arc length for the worm, the chain's length for the Grid's worm.
  Speeds are in L per second, as Yemini et al.'s motion-state thresholds already are.
- **Time** in seconds of simulated time, which the rule *The animal's pace* equates with the animal's real time, and
  in undulation periods where a measure is about the rhythm.
- **Posture at a common resolution.** The Grid's worm reports eight tangent directions (one yaw per segment) and seven
  joint angles; the worm's 49-point skeleton is finer. So posture is compared at the coarser one: the worm's skeleton
  is resampled to eight equal arcs, its tangent angle taken at each arc's midpoint, and the mean angle removed, as
  Stephens et al. 2008 remove it from their 100 angles ("an overall rotation mode was removed"); the Grid's worm gives
  the same eight angles directly. Eigen-shapes are fitted on the real worms' eight angles and both bodies are
  projected on them. How much of the worm's posture survives at eight samples is itself a measure the tool reports:
  Stephens et al. find four eigenworms carry over 95% of the variance at full resolution, and Yemini et al. use six
  for 97%.
- **Body parts by fraction of length.** Yemini et al. define the head as the first sixth of the skeleton (points 1 to
  8) and measure bends over a twelfth of it. The Grid's segments are eighths, so head, midbody and tail are redefined
  for both bodies as the same fractions of the resampled eight-arc skeleton, and Yemini et al.'s sixth-and-twelfth
  definitions are not used directly.

## The Measures

*Comparable* means the measure can be computed on both bodies in the units above, pending the tool's check;
*with a caveat* names what must be settled first; *not yet* names what is missing.

| Family | Measure (Yemini et al. 2013 unless named) | Status | Why |
|---|---|---|---|
| Morphology | Length, width, area | Not comparable | The Grid's body is authored by the Program at rez; its shape is a choice, not a behaviour |
| Posture | Eigen-shape amplitudes (Stephens et al. 2008) | Comparable | On the eight-angle resampling above |
| Posture | Bend angles at head, midbody and tail | With a caveat | At the eight-arc resolution; a real bend beyond the Grid's 0.9 rad per joint cannot be reproduced, so the share of real frames beyond it is to be measured first |
| Posture | Amplitude, wavelength, track length, eccentricity | With a caveat | Wavelength from eight samples resolves only waves longer than two segments; the tool reports whether the worm's waves are |
| Posture | Coils, kinks | With a caveat | A coil needs bends the 0.9 rad limit may forbid, and real coils are where segmentation fails, so the worm's count is biased low |
| Locomotion | Speed and angular speed of head, midbody, tail | Comparable | In L per second and radians per second |
| Locomotion | Forward, backward and paused states, by Yemini et al.'s thresholds | Comparable | The thresholds are fractions of length per second over at least half a second ([the behaviour-measures note](../research/behaviour-measures.md#locomotion-states)) |
| Locomotion | Reversals: frequency, duration, distance | Comparable | Built on the states above |
| Locomotion | Crawling amplitude and frequency | Comparable | From the bend time series; frequency in hertz and per undulation |
| Locomotion | Foraging (the nose's fast swings) | Not yet | The Grid's head is one rigid segment; its swing is the first joint's, a sixth of the worm's resolution at the nose |
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

A reference recording lasts 15 minutes, 900 s. So:

- **The share of time in each state** can be estimated from 15 minutes, with the same 10-second bins and a two-state
  model, but each recording holds only one or two dwelling states on average (900 s against a mean of 482 s).
- **State durations cannot be compared from 15 minutes.** Most dwelling states and the long roaming states run past
  the recording's end, so their durations are cut off; only the short roaming states (τ = 75 s) are seen whole often
  enough. A survival analysis that treats cut-off states as censored could estimate the dwelling time constant, but
  how well at 900 s per worm is for the tool to show, not for this note to assume.
- **The states are defined on food.** Flavell et al. describe them as foraging states on a bacterial lawn; whether a
  worm on the Grid, without food, should show them at all is part of the next question.

## Open Questions Before Freezing

- **Food.** The reference worms are on food after a 30-minute wait (Yemini et al. 2013), and the Grid's arena has
  none. Roaming and dwelling are defined as foraging states on food (Flavell et al. 2013), and whether the worm's
  speed and reversals also differ off food is still to be read from a source; until then a Grid worm compared with
  on-food worms may be compared with an animal in another condition. Three ways forward, none chosen: compare with the
  reference's N2 recordings made without food (28 records by their metadata, [data/README.md § The Phase 0
  Survey](../../data/README.md#the-phase-0-survey); their protocol not yet read); ask the world for food and a feeding
  signal, which hunch B1's "am I taking in energy now?" also needs; or restrict the benchmark to measures shown not to
  differ between on and off food in the worm, from a source still to be found. This is for the maintainer to decide,
  and it is listed in STATE.md.
- **The habituation window.** Yemini et al. tracked 25 wild-type worms for two hours without the usual wait and found
  speed decaying with τ = 19 minutes, midbody amplitude with τ = 48 minutes and crawling frequency with τ = 19
  minutes. The reference recordings start 30 minutes after the worm is moved, so a Grid life must be compared over the
  matching window, or the creature's own settling must be shown to follow the worm's; which, is to be fixed with the
  pace note (STATE.md item 1).
- **Missing frames.** The worm's gaps fall where the body coils, so every measure that depends on large bends is
  biased in the reference and not in the Grid's recordings. The tool must report, per measure, how much it depends on
  frames that segmentation tends to lose, and the benchmark must say whether the Grid's frames are masked to match.
- **The joint limit.** If a real worm's bend over an eighth of its length often exceeds 0.9 rad, the Grid's body
  cannot be indistinguishable on posture whatever its brain does. That would be a finding about the body, for the
  world's maintainers by the rule *This repository, and the world's by their own rules*, never a reason to tune the
  benchmark.
- **The arena.** Path measures depend on the arena's size in body lengths; a plate and the Grid's arena must be
  matched, or path measures restricted to those that do not reach the edge.
- **Which recordings.** The N2 subset and how it is drawn are fixed by the tool's task (STATE.md item 2), before any
  statistic is computed.
