# Salt Chemotaxis: the Measures, a Draft

A written definition of the candidate measures for the salt assay. This is STATE.md item 11 (b), from
[the stimulus assays' plan](STIMULI.md); the journal of 2026-11-04, § The Salt Measures, Drafted, records how it was
written. It is a **draft**: nothing here is frozen, no data were downloaded for it, and nothing is computed. It is
frozen only through the cycle in [STIMULI.md](STIMULI.md)'s opening paragraph, and before any brain is scored.

What the reference tracks hold, and the sources for them, is in
[STIMULI.md § Taste: Salt Chemotaxis](STIMULI.md#taste-salt-chemotaxis). That covers the preprint
([arXiv:2508.00191](https://arxiv.org/abs/2508.00191)), Luo, Wen, Ren et al. 2014
([doi:10.1016/j.neuron.2014.05.010](https://doi.org/10.1016/j.neuron.2014.05.010)) and the archive's fields. This note
does not repeat it. The lab's common units and the one-instrument rule are in
[FEATURES.md § Common Units](FEATURES.md#common-units) and
[§ Open Questions Before Freezing](FEATURES.md#open-questions-before-freezing). The rule is that both bodies are
measured by the lab's one pipeline, so that a difference of instrument is never scored as a difference of behaviour.

## The Shared Steps

Every measure is computed from the same few series, made the same way for both bodies. Each is either in the reference
tracks or can be rebuilt for a Grid life without anything reaching the brain (the rule *No cheating*).

| Step | Reference tracks | A Grid life | Open question |
|---|---|---|---|
| P1. Position *r*, the centroid | The worm's centre of mass from the image, as tracked (`xy`, pixels) | The mean of the chain's segment midpoints, from the poses on the Disk | Q4 (the image centroid of a curled body differs from a midline mean) |
| P2. Time step Δt | 5/14 s, after the authors' smoothing of 14 Hz tracks | The 32 Hz centroid interpolated linearly to *t* = *j*/14 s, smoothed as the authors smoothed, every fifth sample kept | Q3 (the authors' smoothing) |
| P3. Concentration *C* at the centroid | `dc`, read as mM (*inferred*; STIMULI.md) | The floor's solute field at the centroid's cell, rebuilt from the input log by an evaluation tool | Q5 (the world's solute unit) |
| P4. Gradient direction *û* | Fitted (below) | Fitted the same way, not taken from the world's setting | none |
| P5. Heading θ and heading change Δθ | Recomputed from *r* | Recomputed from *r* | none |
| P6. Goal sign *g* | +1 for 0-50 mM (setpoint 50 mM above), -1 for 100-50 mM | Set by the condition the life is raised in | Q6 (raising a Program) |
| P7. Turns and runs | From Δθ | From Δθ | none |

- **P4, the gradient's direction, recovered from the data.** For each file (each life on the Grid side), fit
  *C* = *a* + *b x* + *c y* by least squares over every step, and take *û* = (*b*, *c*) / |(*b*, *c*)|. The fit's R² is
  reported and |∇*C*| = |(*b*, *c*)| converted to mM/cm with the pixel size read as 32 µm (STIMULI.md, *inferred*). The
  preprint does not state the slope. If the gradient spans the 9 cm lid it is about 50 / 9 = 5.6 mM/cm, but this is
  not stated, and the fit is what decides it. A file whose R² is below 0.9 is not used until the shortfall is
  understood. The 0.9 is this draft's choice, not a sourced value. The authors' own `dcp`, read as the perpendicular
  difference, is compared with the fit as a check, not used.
- **P5.** θ*ₖ* = atan2 of *r*ₖ - *r*ₖ₋₁, and Δθₖ = θₖ - θₖ₋₁ wrapped to (-180°, 180°]. Both are recomputed from *r*
  for both bodies; the file's `theta` and `dth` are compared with them as a check, not used.
- **The bearing** βₖ is the angle from *g û* to the heading θₖ, counter-clockwise, in (-180°, 180°]. 0° is heading
  towards the setpoint. This is Luo et al.'s "preferred direction", the side the worm prefers, which is up the gradient
  for 0-50 and down it for 100-50.
- **P7.** A turn is a step with |Δθₖ| > 50°, the preprint's threshold ("The threshold for a turn (±50 degrees)", its
  Figure 1's caption). A run is the stretch of steps between two turns. A run that touches a track's start or end is
  *censored*. A run's heading is the circular mean of its steps' θ. The threshold is defined only together with Δt:
  50° per 5/14 s is not 50° per tick, which is why P2 samples both bodies alike.
- **Tracks** are kept by the authors' rule, applied to both bodies: removed if shorter than 3 minutes or if they "move
  less than 1cm2" (§ 4.2, as extracted from the PDF; read as 1 cm²). On the Grid side, 1 cm² is scaled to body lengths squared, with the reference body length
  of Q2. The phrase is read as the area the track covers, which is the lab's reading.

## The Candidate Measures

Each measure is a single number per unit, where the unit is the open question Q1. The benchmark's per-measure
comparison, `benchmark::compare` ([STATE.md](../../STATE.md) item 2 (e2)), then compares the two samples of units.
Measures M2 to M6 use only signs and directions of *C*, never its size in mM, so they need no calibration of the world's
solute unit (Q5). M1 uses no concentration at all.

### M1. Centroid Speed

*s* = the median over the unit's steps of |*r*ₖ - *r*ₖ₋₁| / Δt, in body lengths per second.

- *Inputs*: P1, P2.
- *The reference's own*: the preprint's step speed *dr* (§ 2.1, its Figure 1d), in mm/s.
- *Comparable because*: the same centroid, step and smoothing on both sides, and the units of FEATURES.md § Common Units.
- *Depends on*: Q1, Q2 (the files give no body length), Q3, Q4.

### M2. Turn Rate by the Sign of dC/dt

*ρ* = log(*n*₋ / *T*₋) - log(*n*₊ / *T*₊), where:

- *n*₊ and *T*₊ are the unit's turns and its time over steps with *g* · (*C*ₖ - *C*ₖ₋₁) > 0, moving towards the setpoint;
- *n*₋ and *T*₋ are the same over steps where that product is < 0;
- steps where it is 0 are left out.

A positive *ρ* means the worm turns more often when moving away from the setpoint.

- *Inputs*: P1-P3, P6, P7.
- *The reference's own*: "The initiation of pirouettes was correlated with the rate of change of concentration (dC/dt) but
  not with absolute concentration" ([Pierce-Shimomura et al. 1999](https://doi.org/10.1523/JNEUROSCI.19-21-09557.1999),
  abstract). The preprint fits a kernel of turning on the concentration's history instead (§ 4.3). This measure is the
  simplest summary of both, with no model.
- *Comparable because*: only the sign of the concentration's change enters, so the solute unit does not.
- *Depends on*: Q1, Q3, Q6. A version in fixed bins of dC/dt, which would show the curve and not only its sign, needs Q5.

### M3. Curvature Against Bearing (Weathervaning)

*w* = the mean of -sign(βₖ) · κₖ over the unit's run steps with 45° < |βₖ| < 135°, where κₖ = Δθₖ / (|*r*ₖ - *r*ₖ₋₁| / *L*)
is in degrees per body length and is positive counter-clockwise. A positive *w* means the worm veers towards the setpoint
when it is heading across the gradient.

- *Inputs*: P1, P2, P4-P7.
- *The reference's own*:
    - Chen et al. 2023, for odour, give the curvature's distribution "conditioned on bearing" by quadrant
      ([doi:10.7554/eLife.85910](https://doi.org/10.7554/eLife.85910), its Figure 7f).
    - [Iino and Yoshida 2009](https://doi.org/10.1523/JNEUROSCI.3633-08.2009) describe salt weathervaning as a gradual
      curve "toward higher concentration".
    - Luo et al. 2014 found no such bias in their linear gradients: "During runs orthogonal to the gradient, we found that
      worms gradually veer towards the preferred and non-preferred directions by similar amounts".
    - The reference is expected to settle which holds in its own tracks, and the Grid's worm to match it either way.
- *Comparable because*: the same bearing and the same steps, with curvature per body length.
- *Depends on*: Q1, Q2, Q3, Q4, Q6.

### M4. Run Duration by Heading

*d* = log(*D*_towards / *D*_away). Here *D* is the mean duration of the unit's uncensored runs whose heading is within
22.5° of the setpoint (towards) or more than 157.5° from it (away). These are Luo et al. 2014's bins ("Runs up: runs with
mean headings between −22.5° and +22.5°. Runs down: runs with mean headings <−157.5° or >+157.50°", their Figure 2's
legend), with the preferred direction in place of up. The share of runs left out as censored is reported beside *d*.

- *Inputs*: P1, P2, P4-P7.
- *The reference's own*: Luo et al. 2014's runs end at "reorientation movements (sharp turns or reversal-turns)". These are
  detected with shape analysis, by a method in their supplement, which has not been read. From a centroid at 5/14 s the
  lab's runs end at turns of P7 instead. So the reference statistic is the lab's measure on the authors' tracks, not
  Luo et al.'s published values, which are kept only as context.
- *Comparable because*: the same turn rule and the same censoring on both sides.
- *Depends on*: Q1, Q3, Q6. Excluding censored runs shortens both bodies' long runs alike, but it biases *d* if runs
  towards the setpoint are the long ones, which is what is measured. A censored-data estimate (Kaplan and Meier's) is the
  alternative, chosen or rejected before 11 (c) computes anything.

### M5. Chemotactic Index

CI = ⟨*v*_g⟩ / ⟨*s*⟩ over the unit's steps, where *v*_g,k = (*r*ₖ - *r*ₖ₋₁) · *g û* / Δt and *s*ₖ = |*r*ₖ - *r*ₖ₋₁| / Δt.
This is Luo et al. 2014's index, "the mean velocity of the trajectories along the gradient direction, 〈vg〉, divided by
the mean crawling speed along the trajectory, 〈s〉". Here *g* signs it towards the setpoint rather than up the gradient,
and it lies in [-1, 1].

- *Inputs*: P1, P2, P4, P6.
- *Comparable because*: it is dimensionless, and only *û* enters, never the size of *C*.
- *Depends on*: Q1, Q3, Q6, and the start positions. Luo et al. note that worms at their setpoint "move randomly, without
  drifting up or down the gradient", so the index of a track depends on where it lies relative to the setpoint. A Grid
  life must start where the reference's worms start, which the files do not say: a question for the authors (Q1's
  letter).

### M6. Reorientation After Orthogonal Runs

*p* = the share of the unit's uncensored runs with 67.5° < |β| < 112.5° (Luo et al.'s orthogonal bin) whose next run
heads within 90° of the setpoint, minus 0.5.

- *Inputs*: P1, P2, P4-P7.
- *The reference's own*: "sharp turns after these orthogonal runs are more likely to point towards the preferred direction"
  (Luo et al. 2014, their Figure 2C and 2E, "Probabilities of reorientation up or down the gradient after orthogonal runs").
  How they split up from down is in the supplement, not read. The 90° split is this draft's choice.
    - They also found that "[r]eversal-turns after orthogonal runs are not more likely to reorient the worm towards preferred
      directions".
    - A centroid cannot tell a sharp turn from a reversal-turn, so *p* pools the two, and is expected to be weaker than
      Luo et al.'s sharp-turn figure.
- *Depends on*: Q1, Q3, Q6.

## The Open Questions of Item 11 (b)

Answered where the record allows; the rest are for the maintainer.

- **Q1, the unit of the test. Not answerable from the record; for the maintainer.**
    - The preprint reports 7-11 plates per condition (§ 2.1), so the reference has about 7-11 independent units per
      condition, not 277-297 tracks (STIMULI.md).
    - The files keep neither plate nor worm. A test on tracks would treat dependent rows as independent and claim a
      precision the data do not have.
    - The routes:
        - (a) ask the authors for each track's plate, and with it their smoothing (Q3), the start positions (M5),
          `Data_salt0_0.mat` and `mask`, in one letter. Recommended.
        - (b) take the file as one unit per condition and compare each Grid assay's summary with it. This has no
          reference spread to test against, and is not recommended.
    - On the Grid side, a plate would be one world holding several worms, or one life per world. Several worms per world
      match the plate, and their encounters too. One per world is independent but less like the assay. This is for the
      maintainer to choose, with route (a).
    - Until Q1 is settled, the salt assay cannot be frozen. Item 11 (c) can still compute every measure per track, as
      description.
- **Q2, the body length. Partly answered.** The files give none, and M1 and M3 are in body lengths.
    - The lab's own reference worms have a median length of 1123 µm (interquartile range 86), over block 1 (journal of
      2026-11-03, § The Reference Statistics of Block 1).
    - Those worms come from another laboratory's protocol, and their age is not checked against the salt worms'
      "day-1 adult". Using 1123 µm is an assumption, to be said so beside every M1 and M3 figure, unless the authors
      give a length.
- **Q3, how a Grid life is sampled to match. Answered except for one step.** P2 resamples the Grid's 32 Hz centroid to the
  reference's 14 Hz, applies the authors' smoothing, and keeps every fifth sample, so both bodies reach 5/14 s by one
  path.
    - The smoothing is not stated in the preprint ("The trajectories were smoothed and down-sampled", § 4.2). The authors'
      code (Kevin-Sean-Chen/Chemotaxis_function on GitHub) could not be opened from the lab's sessions so far.
    - Until it is read, the reference is smoothed once by an unknown filter and the Grid's track is not, which breaks the
      one-instrument rule for every measure that uses Δθ.
    - Next: the maintainer attaches that repository to a session, or asks for it in the letter of Q1 (a).
- **Q4, centroid against head. Answered: the centroid, for both bodies.** The reference has only the centroid (P1), so the
  one-instrument rule (FEATURES.md, decided 2026-10-30 for the locomotion states) puts both bodies' measures at the
  centroid, and the concentration is read there (P3).
    - This does not touch the sense. The Grid's worm still tastes at its nose (tron-grid-lite `docs/ORGANS.md` § 4);
      where it senses and where it is measured are different things, as in the real worm.
    - What stays open is the centroid's definition. The image's centre of mass of a body that is curled or coiled lies
      off its midline, while a mean of segment midpoints lies on it. The difference is largest in turns. It is measured
      when the Disk reader exists, on lives with known poses, before anything is frozen.
- **The gradient's direction. Answered** by P4: fitted from the data, never assumed, for both bodies, with its acceptance
  rule written above before any data are fetched.
- **Q5, the world's solute unit.** STIMULI.md's calibration need is unchanged, but this draft does not depend on it: no
  measure uses the size of *C*.
- **Q6, raising a Program at a setpoint.** Every measure is signed by the setpoint (P6). The reference worms were raised
  at 50 mM (the preprint, § 2.3), and a Grid life would need the same: a stretch at one concentration before the
  gradient. This is the same need the heat assay has (STATE.md *Needs From Other Repositories*, item (4)), and it is
  added there for salt.
