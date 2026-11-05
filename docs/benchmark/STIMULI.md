# The World's New Fields: A Plan for Stimulus Assays

What the world's heat, taste and air fields could let the benchmark measure, what reference data exists for each, what
the benchmark and the world would still need, and in what order. This is **planning only** (STATE.md item 11, journal of
2026-11-04, § What the World's New Fields Open for the Benchmark: a Plan). It fixes nothing a score depends on and
changes nothing in [the comparison space](FEATURES.md), whose *Responses* row stays *not yet*. Each assay below enters
the benchmark only through the same cycle as every other measure: its definition written down and frozen before any brain
is scored against it (the rule *Realism over speed*).

The world's facts are read from the commits `external/` pins since the journal of 2026-11-04: tron-grid-lite `067fb7e`,
master-control `7c527d9` and link `0304788`. The biology is read from the papers' abstracts on Europe PMC (2026-11-04)
unless a line says otherwise; an abstract is a weak reading of a method, and each assay's protocol is read in full
before its definition is drafted (the follow-on items in STATE.md).

## What a Program Senses Today, and What Only Exists

| Field | In the world (master-control `7c527d9`) | Sensed at Program ABI 12 (tron-grid-lite `067fb7e`) | Ruled, not built (Program ABI 13) |
|---|---|---|---|
| Odour | Eight channels in the air grid, `src/odour.rs` and `src/air.rs` (#87, #88: `f4222b8`, `87d981b`); sources are the food orbs only | **Yes**: `TglSenses.odour[2][8]`, bound fractions at two amphids the world places itself (`libs/program-abi/include/tgl/tgl_program_abi.h`; the letter's `odour`, link `docs/PROTOCOL.md` § Inside a PROPRIOCEPTION) | Declared noses replace `odour` (`docs/ORGANS.md` § 3, *Noses*) |
| Heat | One channel in the same air, `src/heat.rs` (#90, `1fe85f1`): an ambient (20 °C), an optional linear floor gradient, the six pillars held 10 K warm | **No.** `TglSenses.irradiance` is radiance over the sphere, a pit organ's sense, not a temperature (its comment in the header; `docs/THERMORECEPTION.md` § Scope) | Thermoreceptors outside (AFD, 0.01 K) and inside (the body, staged as the air at the head) (`docs/ORGANS.md` § 3, *Thermoreceptors*) |
| Taste | Four floor solutes, `src/solutes.rs` (#92, `4b1248e`): sugar, salt, acid, bitter, leached from orbs only | **No** | Taste organs that read only what they touch (`docs/ORGANS.md` § 3, *Taste organs: the contact rule*); the worm's ASE pair, salt only (§ 4) |
| Oxygen, carbon dioxide | **Not in the world**: not yet tenants of the air (`docs/CHEMORECEPTION.md` § Oxygen and carbon dioxide) | **No** | Gas sensors staged at the open air's 0.21 and 0.0004, "the exact field of a world in which nothing breathes yet" (`docs/ORGANS.md` § 3, *Gas sensors*) |

Three facts about the world that every assay below depends on, from the same pins:

- **The floor is 128 m across** (64 cells of 2 m, `GRID_FLOOR_CONFIG` in master-control's `src/ground.rs`), which is
  128 / 4.48 = 28.6 body lengths, or 2.86 cm of the real worm's world at the world's own scale of 44.8 m to the
  centimetre (`CENTIMETRE` in `src/heat.rs`). The fields' cells are 2 m, 0.45 of a body length.
- **The floor is terraced** (`relief_terraces: 6`, `relief_amplitude: 5.0` in the same config), and every riser is a
  wall to the air and to the solutes (`src/air.rs`, `src/solutes.rs`). An assay plate is flat agar. No launch setting
  makes the floor flat (the flags in `src/main.rs`); every body is rezzed on the spawn pad, "a cell centre the tests
  prove flat for a metre of opening walk" (`SPAWN_PAD_X`, `SPAWN_PAD_Z` in `src/roster.rs`), on one terrace. How large
  that terrace is has not been measured.
- **The fields are outside the state hash and not on the wire.** They are pure functions of the launch settings, the
  food and the ticks (the module comments of `src/heat.rs`, `src/odour.rs` and `src/solutes.rs`), and the input log's
  header records every launch setting (#93, `7c527d9`). The Disk carries no field. So an evaluation tool can rebuild
  the field a creature crawled through, from the log, without the field ever reaching the brain - ground truth for
  evaluation only, as *No cheating* allows.

## Heat: Thermotaxis and Isothermal Tracking

**What exists.** The field is in the world; the sense is not, until ABI 13. `--floor-gradient` sets a linear gradient in
°C per centimetre of the worm's world, `--floor-gradient-direction` its direction, `--ambient` the temperature at the
floor's centre, and `--pillar-heat` the pillars' excess, which can be set to 0 (master-control `src/main.rs`,
`src/heat.rs`). The assay's 0.5 °C/cm (`ASSAY_GRADIENT`, after Hedgecock and Russell 1975 as described by Raiders et al.
2022) gives 0.5 × 2.86 = 1.43 °C across the whole floor. No radial gradient and no change of temperature in time are
settable.

**The behaviour it would let the benchmark measure.**

- *Migration to the cultivation temperature, then isothermal movement.* Worms grown at 16 to 25 °C and placed on a
  gradient migrate to their growth temperature and move isothermally; "[b]ehavioral adaptation to a new temperature
  takes several hours" ([Hedgecock and Russell 1975](https://doi.org/10.1073/pnas.72.10.4061), abstract). Sure: a
  founding observation, reproduced since.
- *A population thermotaxis index* on a gentle linear gradient, with a time course in which the response at 23 °C was
  "initially suppressed for at least 15 min" ([Ito et al. 2006](https://doi.org/10.1016/j.jneumeth.2005.11.011),
  abstract). The index's definition and the plate's range are not read; moderately sure that it is the assay's measure.
- *Strategies, from single worms' tracks.* Above the cultivation temperature migration down the gradient "works by
  shortening (lengthening) the duration of forward movement in response to positive (negative) temperature changes",
  and isothermal tracking "by orienting persistent forward movement to offset temperature changes"
  ([Ryu and Samuel 2002](https://doi.org/10.1523/jneurosci.22-13-05727.2002), abstract). Negative thermotaxis biases
  the frequency and direction of reorientations, positive thermotaxis only their direction
  ([Luo et al. 2014](https://doi.org/10.1073/pnas.1315205111), abstract). During isothermal tracking the worm changes
  "the curvature of its propulsive undulations in response to temperature changes measured at its head"
  ([Luo et al. 2006](https://doi.org/10.1242/jeb.02590), abstract). These are the measures that suit the benchmark: run
  durations and turn rates conditioned on the temperature change the head met, and the track's alignment to the
  isotherm. Fairly sure of the measures; their exact definitions wait on the full texts. Of these, [pace.md § Temperature
  in the Physiological Range](../research/pace.md#temperature-in-the-physiological-range) found Ryu and Samuel 2002 and
  Luo et al. 2014 closed to the lab's services (with Zariwala et al. 2003 and Clark et al. 2007); Luo et al. 2006 and Ito
  et al. 2006 have not been tried.
- *The pace.* AFD's current answers a step within about 96 ms (Ramot et al. 2008, in pace.md); a behavioural latency to a
  temperature step is still a gap there (pace.md § Gaps, to Be Read), and so are the slow time constants of the
  remembered temperature.

**Reference data.** One web search did not find a public dataset of *C. elegans* thermotaxis tracks with a licence (its
query and service are in the journal of 2026-11-04, § What the World's New Fields Open for the Benchmark: a Plan). One
search is not a survey: the absence is unconfirmed. Its nearest hit was a parasitic nematode's heat-seeking data,
another species. Missing, so far: any trajectories at all. Next leads: the data statements of Luo et al. 2014 and Clark et al. 2007, and asking an author for
data.

**What the benchmark would need.**

- *Measures* in `features`, computed from tracks and the temperature at the head: the share of time in each locomotion
  state against the temperature's rate of change at the head, run lengths against the same, reversal and turn rates
  against bearing to the gradient, and the angle between heading and the isotherm. The lab's existing states
  (`features::states`) are the base; the temperature at the head is rebuilt from the input log's `heat` row by an
  evaluation tool and never reaches a brain.
- *Which temperature the evaluation reads*: the floor's thermal map (the agar's temperature, as the assay sets it) or the
  air at the head (what the thermoreceptor will read). Decided before freezing.
- *Upstream* (STATE.md *Needs From Other Repositories*): the sense itself (ABI 13); and a way to raise a Program at one
  temperature for hours and then test it on a gradient, since the world's thermal map is fixed for a life - a
  scheduled change of the ambient or the gradient, or a radial gradient whose centre is the cultivation temperature.
  The same schedule would give the temperature steps a reaction latency is measured against.
- *The arena*: 1.43 °C across the floor at the assay's gradient is likely narrower than the range the cited assays span
  (not read; to be checked), and 28.6 body lengths is less than a plate. Measures local to the track - conditioned on
  what the head met - survive a small arena; population indices that count where worms end up do not, unless the floor
  is matched to the plate. A steeper gradient is a world setting chosen from the assay before any brain is scored,
  never to make a brain look better.

## Taste: Salt Chemotaxis

**What exists.** The field is in the world; the sense is not, until ABI 13, which declares the worm's ASE pair tasting
salt only, by contact (`docs/ORGANS.md` § 4). The only source of salt is food: each orb leaches its taste profile
(`--leach-rate`, 1 by default; `--solute-fade`, 5e-4 per second), and salt spreads "about 2.8 m in a minute"
(`src/solutes.rs`), rings around food a cell or two wide. No salt gradient can be set apart from food.

**The behaviour it would let the benchmark measure.**

- *Chemotaxis to salts as a klinotaxis*: Na+ and Cl- among the attractants, and "orientation and movement up the
  gradient, accumulation, and then habitutation" (their spelling) ([Ward 1973](https://doi.org/10.1073/pnas.70.3.817),
  abstract). ASE is "uniquely important for chemotaxis" to Cl- and Na+
  ([Bargmann and Horvitz 1991](https://doi.org/10.1016/0896-6273(91)90276-6), abstract). Sure.
- *Pirouettes and weathervaning.* Pirouette onset "was correlated with the rate of change of concentration (dC/dt) but
  not with absolute concentration" ([Pierce-Shimomura et al. 1999](https://doi.org/10.1523/JNEUROSCI.19-21-09557.1999),
  abstract, ammonium chloride and biotin); in the weathervane mechanism animals "gradually curve toward higher
  concentration", and both act in parallel in chemotaxis to NaCl
  ([Iino and Yoshida 2009](https://doi.org/10.1523/JNEUROSCI.3633-08.2009), abstract). Sure; these are the field's
  standard decomposition.
- *A remembered concentration*: worms prefer the salt concentration they were cultivated at
  ([Kunitomo et al. 2013](https://doi.org/10.1038/ncomms3210), abstract). Sure of the effect; its time course not read.

**Reference data.** One candidate, not adopted: Chen, Leifer and Pillow's salt chemotaxis tracks (that order is arXiv's
metadata; the PDF's byline and Figshare give Chen, Pillow and Leifer), worms cultivated at
50 mM tested in linear gradients of 0 to 50 or 50 to 100 mM, centroids at 14 Hz
([arXiv:2508.00191](https://arxiv.org/abs/2508.00191), a preprint, its methods and data statement read in the PDF on
2026-11-04). The data are on Figshare at [doi:10.6084/m9.figshare.25996498](https://doi.org/10.6084/m9.figshare.25996498),
CC BY 4.0, one archive of 5.48 GB (Figshare's API, 2026-11-04). CC BY 4.0 allows redistribution with attribution;
whether it can sit beside GPL-3.0 code or must stay an external download with a checksum is decided when it is adopted,
by the rule *Licences*. Centroids, not skeletons: posture measures cannot be compared on it. Not recorded in
[data/README.md](../../data/README.md), since nothing is adopted.

**The salt archive, read (STATE.md item 11 (a); journal of 2026-11-04, § The Salt Dataset: Its Methods and What the
Archive Holds).** Nothing here adopts it.

- *Its methods*, read in the preprint's PDF: worms grown on NGM with OP50 at 20 °C, synchronised and tested as day-1
  adults; linear gradients made by the method of its reference 22, in a 9 cm square lid, in the odour work's chamber
  without airflow; the centre of mass tracked at 14 Hz, "smoothed and down-sampled to a data point every 5/14 second";
  tracks moving less than 1 cm² or shorter than 3 minutes removed. The strain for the salt runs is not named in the
  methods. The only "N2" read is Supplementary Figure 6's labels "N2 naive" and "N2 aversive", which refer to butanone
  training, not to the salt runs; so it is weak evidence. *Probably N2*, not confirmed for the salt files.
- *Its turns*: a turn is a heading change beyond ±50° ("The threshold for a turn (±50 degrees)", Figure 1's caption).
- *Its reference 22*, [Luo, Wen, Ren et al. 2014](https://doi.org/10.1016/j.neuron.2014.05.010) (PMC4082684, the author
  manuscript read in full through NCBI's BioC service): N2 grown at 50 mM NaCl crawl up a 2 mM/cm gradient from 25 mM
  and down it from 75 mM, on 25 cm × 25 cm plates, filmed for 15 min at 2 frames per second. Its measures are a
  chemotactic index, "the mean velocity of the trajectories along the gradient direction, 〈vg〉, divided by the mean
  crawling speed along the trajectory, 〈s〉", and run durations by heading (runs up, down and orthogonal, in 45° bins).
  Sure: the full text.
- *The archive*, listed without downloading it: its central directory read by HTTP Range requests through Python's
  `zipfile` over a ranged reader, 5,972 bytes fetched. It holds 50 entries under `data4plot/`: seven directory records, 42 MATLAB files in six
  folders (salt and odour chemotaxis, mutants, optogenetics, bearing data and simulations) and a note mapping figures
  to files. No raw video and no skeletons. The salt folder has twelve files. Five are the model's cross-validation
  results (`*_cv_staPWA.mat`) and two its fitted variables (`*_staPAW*.mat`, 0.39-0.42 GB). The other five are tracks:
  `Data_salt0_50.mat` and `Data_salt100_50.mat` (the two gradients, the first used for Figures 1, 3 and 4),
  `Data_salt0_0.mat`, and two dated variants (`Data_salt100_50_0511.mat`, `Data_salt0_50_0513.mat`). The figure note names no figure for `Data_salt0_0`,
  and the text read does not describe it.
- *What the tracks hold*, from three files fetched by range from the same archive (36 MB of files, 67 MB fetched, the reader's
  buffer reading ahead), CRC-32 checked and deleted
  after reading. Each file is a MATLAB v5 array of track records with fields `xy`, `time`, `dc`, `dcp`, `theta`, `dth`,
  `dis`, `mask`, `opto`, `Basis` and `lambda`.
    - The time step is 5/14 s (0.357 s, 2.8 Hz) throughout.
    - `xy` runs over about 0-3,000 by 0-2,500. Read as pixels at the imaging system's 32 µm (Chen et al. 2023), that
      is about 9.6 cm by 8 cm, consistent with a 9 cm lid. The unit is not stated in the file.
    - `dc` spans 0.7-49.7 in the 0-50 file and 50.2-99.8 in the 100-50 file, so it reads as the concentration at the
      worm in mM (*inferred*). `dcp` is the perpendicular difference and `dth` the heading change in degrees, named as
      in the preprint's model (*inferred*).
    - `opto` is zero throughout; `mask` is 1 or NaN, NaN in 5-7% of frames, its meaning not stated.

  | File | Tracks | Track length, s (min, median, max) | Sum of track durations | Last time, s | Median step, px |
  |---|---:|---|---:|---:|---:|
  | `Data_salt0_50.mat` | 297 | 179, 289, 1,799 | 29.6 h | 1,800 | 3.74 |
  | `Data_salt100_50.mat` | 277 | 180, 280, 1,194 | 27.2 h | 1,800 | 3.64 |
  | `Data_salt0_0.mat` | 265 | 150, 226, 992 | 19.1 h | 1,200 | 5.11 |

  The preprint states the amount of data three ways that do not agree with each other: "∼15-20 animal hour of
  recordings per navigation condition" (its methods), "7–11 plates per condition, resulting in 127–226 animal-hours
  across both new and previously published measurements" (§ 2.1), and "∼ 20 animal-hours" (Figure 1's caption). The
  files' sums, 29.6 h and 27.2 h, match none of them; why is not known. A record
  is a track, not a worm: the files keep neither the worm's identity across lost tracks nor the plate.

**How it would map onto the lab's measures.** It maps onto the stimulus measures, and onto none of the spontaneous ones.

- *Not computable*: posture of any kind; forward, backward and paused states, since a centroid has no head and the
  states' thresholds are in body lengths per second, with no body length in the files; reversals; and any measure at
  the reference recordings' 25-33 frames per second.
- *Computable*: centroid speed (about 0.33 mm/s median, if a pixel is 32 µm); turns as heading changes above a threshold
  at 2.8 Hz, as the preprint defines them; the bearing to the gradient; the rate of turns against dC/dt; curvature against
  bearing; run durations by heading, and the probability of reorienting up or down the gradient after orthogonal runs
  (their Figure 2C and 2E), as Luo et al. 2014; and their chemotactic index 〈vg〉/〈s〉. The gradient's direction
  must be recovered from `dc` against `xy`, not assumed.
- *The unit of the test*: the lab's tests compare one row per recording. Here the rows would be tracks, several per worm
  and many per plate, with neither recorded, so they are not independent. With 7–11 plates per condition (§ 2.1), the
  independent units are about 7–11 per condition, not 277–297. Either rows are pooled per file and the test's level is
  checked against that dependence, or the plates are asked of the authors. Decided in item 11 (b), before
  any statistic.
- *The Grid's half*: a life would be sampled at the same 5/14 s and its centroid smoothed alike, by a rule written in
  (b), so that both bodies are measured by one instrument, as the locomotion states already are.

**What the benchmark would need.** The measures are drafted in [SALT_MEASURES.md](SALT_MEASURES.md) (STATE.md item
11 (b)), which supersedes the first bullet below where the two differ.

- *Measures*: turn or pirouette rate against dC/dt, weathervaning as heading change against bearing, and a chemotaxis
  index if the reference uses one; the concentration rebuilt from the input log by an evaluation tool. The reference
  reads the concentration at the centroid, not at the head, so by one instrument the Grid's would be read at the chain's
  centroid too; whether to do so, or to keep the head and name the difference, is a question for item 11 (b).
  A pirouette needs a definition from the turning rate (Pierce-Shimomura et al. 1999) that the lab's states can carry;
  to be read in full first.
- *Upstream*: the taste sense (ABI 13); and **a salt source apart from food** - a placed solute gradient or spot as a
  world setting - since every reference assay is a gradient of salt alone, and the Grid's only salt sits around orbs
  that also smell and feed. The plate is 9 cm square in the preprint (its methods), about 400 m at the world's scale,
  against a 128 m floor: the arena question again.
- *Calibration*: the world's solute unit to millimolar. The affinities are assumed (`docs/ORGANS.md` § 4), so a gradient
  of 0 to 50 mM has no meaning on the Grid until the unit is tied to the concentration ASE reads; for the maintainers.

## Air: Odour Chemotaxis and Aerotaxis

**Odour: what exists.** Sensed today. The odour reaches the amphids from the air grid, each receptor type a population
over five decades (`docs/CHEMORECEPTION.md` § The receptors), but its only sources are the orbs, each with a random
pattern over the eight types. Odours that are not food are deferred upstream "when a world has a second kind of source"
(`docs/CHEMORECEPTION.md` § Deferred, with triggers), and the air is still: wind is deferred too.

**Odour: the behaviour.** Chemotaxis to volatile odorants sensed by AWA and AWC
([Bargmann, Hartwieg and Horvitz 1993](https://doi.org/10.1016/0092-8674(93)80053-h), as tron-grid-lite's
CHEMORECEPTION.md cites it; its assay not read here), by the same pirouette and weathervane strategies. Chen, Wu, Gershow
and Leifer measured the butanone concentration each worm met with an array of gas sensors
([Chen et al. 2023](https://doi.org/10.7554/eLife.85910), PMC10425172, full text read on Europe PMC on 2026-11-04). For
*C. elegans* (their Figure 7) they report three measures against the bearing to the local gradient: the bearing's
distribution, the "[p]robability of observing a sharp turn per time ... as a function of the absolute value of the
bearing" (the biased random walk), and the trajectory's curvature "conditioned on bearing" (weathervaning). They compute
no chemotaxis index: their analysis "provides information relevant to a chemotaxis index", which they call a "coarser
readout". Turn rate and heading change against bearing (their Figure 10) are the fly larvae's, not the worm's. Sure.

**Odour: reference data.** One candidate, not adopted: Chen et al. 2023's recordings of the odour flow control, the
concentration measurements and the behavioural tracks, Figshare
[doi:10.6084/m9.figshare.21737303](https://doi.org/10.6084/m9.figshare.21737303) (version 9), CC BY 4.0 like the paper
itself. Its methods were read and its archive listed by range requests on 2026-11-05, without downloading it (the
journal of 2026-11-05, § The Odour Dataset: Its Methods and What the Archive Holds, has the commands, the folders and
the sizes). In short:

- The worm's tracks are one folder of the 1.29 GB archive, eight MATLAB files of 1.04 GB, named by the published
  figures (`track_fig7a`, `track_fig7b`, `track_fig7def_9`, `track_fig7SI`, `track_fig_8AIB`) beside two landscapes and a
  980-byte `OdorFx_low.mat`.
  Their insides were not read; that `track_fig7SI` is the no-odour control and `Landscape_low` and
  `Landscape_cone_110mM` the two landscapes are inferred from the names and the captions, not confirmed.
- The tracks are centroids: the posture was estimated, "but in this work only the position and velocity was used"
  (their methods), at 14 frames per second, smoothed "with a third-order polynomial in a 0.5 s time window", tracks under
  1 min or 3 mm left out, and those starting above 70% of the peak concentration removed. A turn is a heading change of
  over 60° in 1 s; curvature is the angle between displacements 1 mm apart. Strain N2, young adults, on 1.6% agar.
- Figure 7 rests on "six experiments, ∼100 worms per experiment" in two landscapes. The unit of a test is the
  experiment, about six, not the tracks, as for salt (SALT_MEASURES.md § The Open Questions, Q1).

**Odour: can a flow chamber stand for still air?** For the three measures against the bearing, probably yes; for
anything carrying the concentration's size or the chamber's shape, no. Moderately sure, and only by argument: no
experiment compares the two airs.

- *For*: the measures are conditioned on the local gradient the worm met, not on how the gradient was made; the
  landscape is held stationary - "stable over the timescale of 30 min behavioral experiments" when measured without agar,
  and with agar brought to quasi-equilibrium first and watched at the boundary - so what a worm samples is set by the
  field's shape and its own movement, as in a settled still-air field; and at the chamber's "∼5 mm/s" the
  authors "observed no evidence of anemotaxis", the biased random walk and weathervaning both "absent in control
  experiments with flow but not odor" (two experiments, Figure 7, figure supplement 1).
- *Against*: the flow is laminar convection along one axis, so the cone's gradient keeps one orientation to the wind
  that a radial still-air field does not have; the agar absorbs butanone and is held at quasi-equilibrium, and how much
  the worm senses through the agar rather than the air "is unclear" (their discussion); a cone and a radial field differ in
  shape, so the bearing's distribution reflects each landscape's geometry, and only the rates and curvatures
  conditioned on bearing can compare; and the drift velocity against the gradient's steepness (their Figure 9) is in
  the chamber's butanone units, to which the Grid's bound fractions have no calibration.
- *What the Grid has*: Master Control's air is diffusion with a loss and no wind, the floor a wall rather than a
  reservoir (`src/air.rs`, `src/odour.rs` at the pin), settling in about ten seconds; a still-air test must therefore
  use a source held steady, never one fading after a meal.

**Odour: what the benchmark would need.** Drafted, not frozen, in [ODOUR_MEASURES.md](ODOUR_MEASURES.md), with the
analysis script's rules beside the paper's and where they differ. In short: Chen et al.'s three measures against bearing - its distribution, the sharp
turns' rate and the curvature - against the local gradient of the source concentration, rebuilt for evaluation by
their rules above, with the experiment as the unit; the bearing's distribution compared only between like landscapes.
Upstream, **an odour source apart from food**, a placed source with a declared pattern over the receptor types, held
steady, so the assay smells without feeding. Wind is not needed if the case above holds. Adopting the tracks would cost
about 1.04 GB of download (eight members by range requests, not the 1.29 GB archive) and 1.04 GB on disk while read. The pace
note has one weak latency for odour avoidance (4 s, Sanders et al. 2013, pace.md § Smell).

**Aerotaxis: what exists.** Nothing measurable: oxygen and carbon dioxide are not in the world, and ABI 13 stages their
sensors at the open air's values. The behaviour is clear in the literature: a "strong behavioural preference for 5-12%
oxygen" ([Gray et al. 2004](https://doi.org/10.1038/nature02714), abstract), avoidance of carbon dioxide above 0.5% in
well-fed worms, with responses "within seconds" ([Bretscher et al. 2008](https://doi.org/10.1073/pnas.0707607105),
abstract), and neurons that "detect increases and decreases in oxygen levels using distinct guanylate cyclases"
([Zimmer et al. 2009](https://doi.org/10.1016/j.neuron.2009.02.013), its title only read; tron-grid-lite's
CHEMORECEPTION.md names them URX and BAG). No public dataset was searched for. It waits on the gases becoming tenants of the air, upstream, and is
last in the order.

## Order, Effort and What Blocks Each

The benchmark's spontaneous-behaviour definition (STATE.md item 2 (e5)) comes first and is not delayed by any of this.
Stimulus assays are a second frozen part, added to it by a justification in the journal, with nothing scored across the
change. Within them, in order:

1. **Salt chemotaxis**, first, because it has the one public dataset found (one search) of a gradient of a single stimulus with tracks and
   a licence. One run each: (a) read the preprint's methods and those of its reference 22 (Luo, Wen, Ren et al.,
   whose salt gradients it follows) and inventory the Figshare archive's file list without downloading it; (b) draft the
   measures and their rules (pirouette, weathervaning, the bearing, an index only if the reference uses one) before any
   data; (c), after (b), fetch, checksum and register the data, and compute the reference statistics in `features`. None
   of (a) to (c) needs a Grid life, so none is blocked upstream. Any Grid life is blocked by the taste sense (ABI 13), a
   salt source apart from food, and a calibration of the solute unit.
2. **Odour chemotaxis**, second: the sense exists today, but the reference is made in moving air and the world has no
   odour without food. One run to read Chen et al. 2023's methods and inventory its archive; then measures, data and
   reference statistics as for salt, none of which needs a Grid life. A Grid life is blocked by an odour source apart
   from food.
3. **Thermotaxis and isothermal tracking**, third: the world's field is the most complete, but one search found no public
   tracks. One run to search the data statements and, if nothing is public, to write the measures from the papers so the
   benchmark is ready when data are. The papers that pace.md found closed may stay closed; the fallbacks are Luo et al.
   2006 and Ito et al. 2006 (not yet tried), an author's manuscript in Europe PMC, and the data or a reprint asked of an
   author through an issue the maintainer files; failing those, the step parks with what was tried.
   A Grid life is blocked by the sense (ABI 13), a way to raise a Program at one temperature and then test it, and the
   arena's size.
4. **Aerotaxis**, last: blocked by the gases in the world.

Shared by all four: the arena (a flat floor, and its size against a plate, already an open question in
[FEATURES.md § Open Questions Before Freezing](FEATURES.md#open-questions-before-freezing)); an evaluation tool that
rebuilds a field along a track from the input log's settings, which waits on reading a Disk (STATE.md item 3 (d2)); and the
pace of each response, measured like any other behaviour.
