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
  wall to the air and to the solutes (`src/air.rs`, `src/solutes.rs`). An assay plate is flat agar.
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
  isotherm. Fairly sure of the measures; their exact definitions wait on the full texts, which
  [pace.md § Temperature in the Physiological Range](../research/pace.md#temperature-in-the-physiological-range) found
  closed to the lab's services.
- *The pace.* AFD's current answers a step within about 96 ms (Ramot et al. 2008, in pace.md); a behavioural latency to a
  temperature step is still a gap there (pace.md § Gaps, to Be Read), and so are the slow time constants of the
  remembered temperature.

**Reference data.** No public dataset of *C. elegans* thermotaxis tracks with a licence was found (one web search,
2026-11-04, journal). The nearest hit was a parasitic nematode's heat-seeking data, another species. Missing, then: any
trajectories at all. Next leads: the data statements of Luo et al. 2014 and Clark et al. 2007, and asking an author for
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

**Reference data.** One candidate, not adopted: Chen, Pillow and Leifer's salt chemotaxis tracks, worms cultivated at
50 mM tested in linear gradients of 0 to 50 or 50 to 100 mM, centroids at 14 Hz
([arXiv:2508.00191](https://arxiv.org/abs/2508.00191), a preprint, its methods and data statement read in the PDF on
2026-11-04). The data are on Figshare at [doi:10.6084/m9.figshare.25996498](https://doi.org/10.6084/m9.figshare.25996498),
CC BY 4.0, one archive of 5.48 GB (Figshare's API, 2026-11-04). CC BY 4.0 allows redistribution with attribution;
whether it can sit beside GPL-3.0 code or must stay an external download with a checksum is decided when it is adopted,
by the rule *Licences*. Centroids, not skeletons: posture measures cannot be compared on it. Not recorded in
[data/README.md](../../data/README.md), since nothing is adopted.

**What the benchmark would need.**

- *Measures*: pirouette rate against dC/dt at the head, weathervaning as heading change against bearing, and a
  chemotaxis index if the reference uses one; the concentration at the head rebuilt from the input log by an
  evaluation tool. A pirouette needs a definition from the turning rate (Pierce-Shimomura et al. 1999) that the lab's
  states can carry; to be read in full first.
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
and Leifer measured the butanone concentration each worm met with an array of gas sensors and report, beside the
chemotaxis index, turn rate against bearing and heading change before turns
([Chen et al. 2023](https://doi.org/10.7554/eLife.85910), PMC10425172, full text read on Europe PMC on 2026-11-04).
Sure.

**Odour: reference data.** One candidate, not adopted: Chen et al. 2023's recordings of the odour flow control, the
concentration measurements and the behavioural tracks, Figshare
[doi:10.6084/m9.figshare.21737303](https://doi.org/10.6084/m9.figshare.21737303), CC BY 4.0, a raw archive of 1.29 GB
(Figshare's API, 2026-11-04). Images at 14 frames per second and 32 µm a pixel (their methods). Two caveats: the
landscape is made by patterned airflow in a flow chamber, not still air, and agar "greatly disrupts an applied butanone
gradient" (their words); measures conditioned on the concentration the worm met may carry over to still air, an
end-point index less so.

**Odour: what the benchmark would need.** The same measures as for salt, against the bound fraction's source
concentration rebuilt for evaluation; upstream, **an odour source apart from food**, a placed source with a declared
pattern over the receptor types, so the assay smells without feeding; and wind, only if the flow chamber's data are
taken as they are. The pace note has one weak latency for odour avoidance (4 s, Sanders et al. 2013, pace.md § Smell).

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

1. **Salt chemotaxis**, first, because it has the one public dataset of a gradient of a single stimulus with tracks and
   a licence. One run each: (a) read the preprint's methods and those of its reference 22 (Luo, Wen, Ren et al.,
   whose salt gradients it follows) and inventory the Figshare archive's file list without downloading it; (b) draft the measures and their
   rules (pirouette, weathervaning, the bearing, the index) before any data; (c) fetch, checksum and register the data,
   and compute the reference statistics in `features`. Blocked for any Grid life by the taste sense (ABI 13), a salt
   source apart from food, and a calibration of the solute unit.
2. **Odour chemotaxis**, second: the sense exists today, but the reference is made in moving air and the world has no
   odour without food. One run to read Chen et al. 2023's methods and inventory its archive; then as for salt. Blocked
   by an odour source apart from food.
3. **Thermotaxis and isothermal tracking**, third: the world's field is the most complete, but no public tracks were
   found. One run to search the data statements and, if nothing is public, to write the measures from the papers so the
   benchmark is ready when data are. Blocked by the sense (ABI 13), a way to raise a Program at one temperature and then
   test it, and the arena's size.
4. **Aerotaxis**, last: blocked by the gases in the world.

Shared by all four: the arena (a flat floor, and its size against a plate, already an open question in
[FEATURES.md § Open Questions Before Freezing](FEATURES.md#open-questions-before-freezing)); an evaluation tool that
rebuilds a field along a track from the input log's settings, which waits on reading a Disk (STATE.md *Blocked*); and the
pace of each response, measured like any other behaviour.
