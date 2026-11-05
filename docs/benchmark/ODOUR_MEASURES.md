# Odour Chemotaxis: the Measures, a Draft

A written definition of the candidate measures for the odour assay. This is STATE.md item 11 (d2), from
[the stimulus assays' plan](STIMULI.md); the journal of 2026-11-05, § The Odour Measures, Drafted, records how it was
written. It is a **draft**, made as [SALT_MEASURES.md](SALT_MEASURES.md) was for salt: nothing here is frozen, no track
was downloaded for it, and nothing is computed. It is frozen only through the cycle in [STIMULI.md](STIMULI.md)'s
opening paragraph, and before any brain is scored.

What the reference holds, its licence, and whether a flow chamber can stand for the Grid's still air are in
[STIMULI.md § Air](STIMULI.md#air-odour-chemotaxis-and-aerotaxis); this note does not repeat them. The sources here are
two:

- the paper, Chen, Wu, Gershow and Leifer 2023 ([doi:10.7554/eLife.85910](https://doi.org/10.7554/eLife.85910),
  PMC10425172), its methods' section on behavioural analysis and the caption of its Figure 7, read on Europe PMC;
- the archive's analysis script `Chen_flow_2022/demo_code/odor_flow_worm.m` (18,570 bytes, CC BY 4.0 with the record,
  Figshare [doi:10.6084/m9.figshare.21737303](https://doi.org/10.6084/m9.figshare.21737303), version 9), fetched alone by
  range requests and read as text, never run (the journal has the command and its checksum). A citation `.m:N` is its line N.
  Its header still says "figure 5", the preprint's numbering, so that it is the exact code behind the published
  Figure 7 is *likely, not confirmed*.

## Where the Paper and the Script Disagree

The paper's rules are short, and the script does not always follow them. Each difference matters to a measure, so all
are listed before any measure is defined.

| Rule | The paper (methods) | The script | Read as |
|---|---|---|---|
| Smoothing | "a third-order polynomial in a 0.5 s time window" | Savitzky-Golay, `smooth(..., filt,'sgolay',poly_degree)` with `poly_degree = 3`, `filt = 7` frames at `fr = 1/14` s (`.m:50-52`, `.m:74-75`) | The same rule: 7 frames at 14 Hz is 0.5 s. Confirmed for odour |
| Smoothing for curvature | not stated | `filt = 14*2`, a nominal 2 s window (an even span, which MATLAB's `smooth` may adjust; not checked), applied within each run (`.m:315`, `.m:338-339`) | A second, wider filter |
| Turn sampling | turns from "the angle change between displacement vectors over 1 s time window" | positions every `delta_t + 1` = 15 frames, 15/14 = 1.07 s, from the *unsmoothed* `Path` (`.m:239`, `.m:253`, `.m:261-265`) | Near the paper; unsmoothed in the script |
| Turn threshold | "angle changes larger than 60°" | `thre_pr = 120` (`.m:289`); 60 is `thre_grad`, the bearing's bin edge (`.m:290`) | **Differs**: 60° in the text, 120° in the script |
| Turn rate | "Probability of observing a sharp turn per time" (Figure 7e) | turns over samples in each bin, labelled "turn probability per second" (`.m:292-305`) | Per 1.07 s sample, not per second |
| Curvature | "the angle between displacement vectors over 1 mm displacement in space" | the angle between vectors 15 frames apart, divided by the first vector's length in mm (`.m:347`, `.m:359`) | **Differs**: over a fixed time, then per mm, not over a fixed distance |
| Curvature's conditioning | "conditioned on bearing", "gray-scale quadrants" (Figure 7f) | the bearing multiplied by the gradient's size, split at -10 and +10 (`.m:361`, `.m:385-387`) | **Differs**: bearing and steepness mixed |
| Track duration | "more than 1 min" | `mint = 60*3`, 3 min (`.m:54`, `.m:67`) | **Differs** |
| Track displacement | "displacement larger than 3 mm" | mean squared distance from the track's mean position over `minx^2`, `minx = 300` pixels (`.m:55`, `.m:68-70`): a radius of gyration over 9.5 mm | **Differs** |
| Start concentration | tracks starting above "70% of the maximum odor concentration" removed | `conc_thre = 0.8*max(max(M))` at the smoothed start (`.m:59`, `.m:76`) | **Differs**: 70% against 80% |
| Pixel size | 32 µm (STIMULI.md) | `pix2mm = 1/31.5`, 31.7 µm (`.m:57`) | Close; the script's is the more exact |
| The local gradient | "the odor landscape constructed with the OSA" | the gradient of the landscape map after a 100 by 100 pixel box filter (`.m:241`, `.m:318`), 3.2 mm square | A smoothing scale the paper does not state |

Two more facts from the script. The angle function it calls, `angles`, is not in the file: it comes from the authors'
`Chemotaxis_function` repository (`.m:12-13`, `.m:18`), so its sign convention and range are *unknown*. And the bearing's
distribution of Figure 7d is not plotted by this script; its code was not found.

## The Shared Steps

The same pipeline for both bodies, as for salt. Where a step is salt's rule unchanged, it points to
[SALT_MEASURES.md § The Shared Steps](SALT_MEASURES.md#the-shared-steps) rather than copying it.

| Step | Reference tracks | A Grid life | Shared with salt |
|---|---|---|---|
| O1. Position *r*, the centroid | the tracker's `Path` (pixels) | the mean of the chain's segment midpoints, from the poses on the Disk | salt P1, with its Q4 |
| O2. Time and smoothing | 14 Hz, smoothed by Savitzky-Golay, order 3, 7 frames | the 32 Hz centroid interpolated linearly to *t* = *j*/14 s, then the same filter | salt P2's resampling, with the filter now known (for odour) |
| O3. Gradient direction *û* | the direction of ∇*C* of the authors' measured landscape at the centroid, after the box filter | the direction of ∇*C* of the world's air field of the assay's source at the centroid, rebuilt from the input log by an evaluation tool, after a box filter of the same size in body lengths | none: salt fits a plane; here the field is measured |
| O4. Heading over a distance | displacement vectors over a fixed distance ℓ | the same | salt P5's rule, with its own ℓ per measure |
| O5. Bearing β | the angle from *û* to the heading, counter-clockwise, in (-180°, 180°]; 0° is up the gradient | the same | salt's bearing, with *g* = +1 (butanone attracts) |
| O6. Turns over 1 s | heading change between displacement vectors 14 frames (1 s) apart | the same, on O2's samples | none: a time rule, not salt P7's distance rule |

- **O3 uses only the gradient's direction.** The Grid has no calibrated concentration: its air carries the world's own
  unit, and what a Program senses is a bound fraction at each receptor type, with no tie to parts per million
  (STIMULI.md § Air). The direction of ∇*C* does not depend on the unit, provided the world's field is proportional to some
  concentration; nothing else about *C* enters any measure below. The field is read for evaluation only; it never
  reaches a brain (the rule *No cheating*). The box filter's 3.2 mm is 2.83 *L* at the lab's reference body length of
  1123 µm (SALT_MEASURES.md, Q2), an assumption carried into every figure here.
- **Track filters** are applied to both bodies by one rule, the paper's (Q2 below, decided): kept if longer than 1 min
  of simulated time and displaced more than 3 mm (2.67 *L* at 1123 µm); removed if they start where *C* exceeds 70%
  of the field's maximum. That last is a ratio of two concentrations, so it needs no unit either. "Displacement" is
  ambiguous in the paper; the draft reads it as the largest distance from the start, which is the lab's reading.
- **The Grid's side** needs an odour source apart from food, placed and held steady (STATE.md *Needs From Other
  Repositories*, item (3)), and a Disk reader (STATE.md item 3 (d2)). The world's odour fades in about ten seconds
  after a meal (STIMULI.md § Air), so no measure here can be taken around a food orb.

## The Candidate Measures

Each is one number per unit, where the unit is the open question Q1. They are the paper's three measures against bearing,
plus speed from salt, and nothing that carries the concentration's size.

### N1. Centroid Speed

Salt's M1 unchanged ([SALT_MEASURES.md § M1](SALT_MEASURES.md#m1-centroid-speed)), on O2's samples. It is the
control that a difference in the others is not only a difference of pace.

### N2. Turn Rate by Bearing (the Biased Random Walk)

*τ* = log(*R*_away / *R*_towards), where *R* is the number of turn onsets (O6, |Δθ| > 60°, a run of consecutive
samples beyond the threshold counted once, as salt P7 does) divided by the summed sample time, over the samples whose
preceding displacement has |β| > 120° (away) or |β| < 60° (towards). These are the script's bins (`.m:290-300`) and the
paper's Figure 7e. A positive *τ* means the worm turns more often when heading down the gradient.

- *Inputs*: O1-O3, O5, O6.
- *Differs from the script* in three ways, each a choice of this draft: 60° (the paper's text) not 120° (the script);
  per second of time, not per sample; turns counted once at onset, so a slow turn is not counted twice, which is what
  the paper says its 1 s window is for ("To avoid double counting turns when the animal turns slowly").
- *Registered check*: the script's own rule (15-frame samples from the unsmoothed track, 120°, per sample) is also
  computed on the reference, to see that the lab's pipeline reproduces the shape of Figure 7e before its own rule is
  trusted. It is a check of the pipeline, not a second primary.
- *Depends on*: Q1-Q4.

### N3. Curvature Against Bearing (Weathervaning)

Salt's M3 with the paper's distance: O4 with ℓ = 1 mm, 0.89 *L* at 1123 µm ("over 1 mm displacement in space"),
κ in degrees per body length, and *w* the mean of -sign(β)·κ over the samples with 45° < |β| < 135°, within runs (between
N2's turns). The bearing alone conditions it, not the bearing times the gradient's size as in the script, because the
size needs a calibrated concentration the Grid does not have. A positive *w* means the worm veers up the gradient when
heading across it.

- *Inputs*: O1-O5, N2's turns.
- *The reference's own*: the curvature's distribution "conditioned on bearing" by quadrant (Figure 7f), its skew
  across the gradient read as weathervaning. The distribution by quadrant is reported beside *w*, as description.
- *Depends on*: Q1-Q3, Q5. Its sign rests on the script's `angles` (Q3) only for the check against Figure 7f; the lab's
  own κ is signed by its own convention, counter-clockwise positive, on both bodies.

### N4. The Bearing's Distribution

*b* = the mean of cos β over the unit's samples, from -1 to 1, with the distribution in twelve 30° bins as
description (Figure 7d, "Peak around zero is consistent with chemotaxis").

- *Comparable only between like landscapes* (STIMULI.md § Air): the bearing's distribution reflects the landscape's
  shape and the start positions, not only the worm. A Grid assay with a radial source does not compare with the
  chamber's cone. N4 is kept as a measure only for a Grid landscape built to the reference's shape; until then it is
  description.
- *Depends on*: Q1, Q2, Q5.

### Left Out

- **Drift velocity against the gradient's steepness** (Figure 9): it is in the chamber's butanone units, which the
  Grid's air has no calibration to (STIMULI.md § Air).
- **A chemotaxis index**: the authors compute none, calling it a "coarser readout"; salt's M5 is not carried over.
- **Turn rate by the sign of dC/dt** (salt's M2): the paper does not report it, and it is not added here without a
  reference statistic to compare it with.

## The Open Questions of Item 11 (d2)

- **Q1, the unit of the test. Decided by the maintainer's ruling in the session of 2026-11-05: "Plate/experiment".**
  Figure 7 rests on "six experiments, ∼100 worms per experiment". The unit is the experiment, about six per landscape,
  not the tracks, so each track's experiment must be known. Whether the track files keep an experiment per
  track is not known: their insides were not read. If they do, the unit is set by the data; if not, the route is salt's
  Q1 (a), a letter to the authors (SALT_MEASURES.md § The Open Questions). Six units give a wide interval at best; that
  limit is stated beside any odour result. On the Grid, an experiment would be one world holding several worms, the
  same choice as salt's plate.
- **Q2, paper or script. Decided by the maintainer's ruling in the session of 2026-11-05: "Paper primary".** The paper's
  text is the reference; the script's rules are a registered check. Six rules differ (the table above, marked **Differs**). This draft takes
  the paper's text as primary, because it is what the authors published and reviewers read, and the script as a check
  that the lab's pipeline reproduces their figure. If the script turns out to be the code behind Figure 7, the
  reference statistics computed by the paper's rules are the lab's, not the authors' published values, and are
  reported as such.
- **Q3, the authors' `angles`.** Its sign and range are unknown, and only the checks against Figures 7e and 7f need it.
  It is in the authors' `Chemotaxis_function` repository, the same one salt's Q3 waits on; one session with it
  attached closes both.
- **Q4, tracking noise over 1 s. Open; the lab's to decide before any track is fetched.** The authors give the 1 s
  window as meant "to mitigate effects of small displacements from tracking noise"; this question is whether it does so at the
  worms' run velocity, which the authors give as "∼0.1 mm/s" (the last sentence of the paper's Results, beside
  the larvae's ∼3 mm/s). One second at that speed is 0.1 mm, 3.15 pixels at 31.7 µm. Take three collinear samples
  3.15 pixels apart, the middle one shared by both displacement vectors, with an error of up to one pixel at each, an
  assumption as in salt's Q7. Noise alone can then turn the heading change by up to 70.4° if the pixel is a radius, or
  92.5° if it is one pixel on each axis (the journal of 2026-11-05, § The Odour Measures, Drafted, has the command).
  Both are beyond the 60° threshold. Doubling salt Q7's single-vector bound, 2 arcsin(2 / 3.15) = 78.8°, gives a looser
  figure that cannot be reached, because the shared middle sample cannot turn both vectors to their worst at once.
  The smoothing lowers the noise by an amount not yet measured. Salt solved this with a
  heading over a distance (salt P5, ℓ = 0.25 *L*); taking that here would leave the paper's rule. The candidates are the
  paper's 1 s rule, salt's distance rule, or the 1 s rule with samples dropped while the displacement is under ℓ.
  The choice is made in a later run, from the bound and the literature, not from the tracks.
- **Q5, the landscape on the Grid.** A placed source in still air gives a radial field; the reference's is a cone
  carried by a flow. The rates and curvatures conditioned on bearing are argued to compare across the two (STIMULI.md
  § Air), N4 is not. The source's strength and the arena's size (item 11 (f)) set the gradient's steepness, which the
  reference spans by its own landscape; matching that range without a calibrated concentration is open.
- **Q6, the body length and the box filter.** As salt's Q2: 1123 µm is the lab's own worms, an assumption for every
  length here. The 3.2 mm box filter on the reference's landscape is the script's; on the Grid it is scaled by that
  length, which keeps the comparison one instrument only if the Grid's field is as smooth at that scale.
