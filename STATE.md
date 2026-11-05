# State of the Lab

**Last updated:** 2026-11-04 ([the journal of 2026-11-04](docs/journal/2026-11-04.md)): `external/` moved to the
latest mains - Link protocol 14, Program ABI 12 without the speed over the ground, the world's reflection attenuation,
its air, heat and floor solutes - with the placeholder rebuilt at ABI 12, the render moved by the attenuation alone, and
the placeholder's life unchanged; the toy worm's steering decisions, scored against the band, place its failure, by
elimination, in the map's choice as the greedy rule reads it (its own registered test undecided); and a post-mortem
of the toy-worm series, read from the record, with recommendations awaiting the maintainer
([gr-fec.md § 12](docs/research/gr-fec.md#12-why-the-demystification-stalled-a-post-mortem-2026-11-04)); and a plan for the stimulus assays the world's new fields open
([docs/benchmark/STIMULI.md](docs/benchmark/STIMULI.md)); and block 1's real worms and block 2's, unseen, were found
indistinguishable by the benchmark's two tests, with a positive control told apart. Earlier
nights are in [the journal's index](docs/journal/README.md). The next step for Phase 0 is the locomotion states'
durations and reversal rates (item 2(c)).

## Where We Are

Phase 0, the benchmark, is under way. The reference data is chosen - the Open Worm Movement Database's N2
recordings ([data/README.md § The Phase 0 Survey](data/README.md#the-phase-0-survey)) - and the first
literature note says how worm behaviour is measured. The comparison space is drafted
([docs/benchmark/FEATURES.md](docs/benchmark/FEATURES.md)): which measures carry over to the Grid's body, and what
must be settled before the benchmark is frozen, food first. The Rust workspace reads the reference recordings and computes
their measures - posture, speed, the locomotion states and the eigen-shape amplitudes, the durations and rates from
the lab's speed still provisional until item 2 (c)'s check - with the reference statistics
over block 1 and their gaps by cause; the benchmark's test is built (item 2 (e1), (e2)) and run on real data
alone (e3): two blocks of real worms are not told apart, and a stated alteration is; and the arena is proven to a
first life: a placeholder Program, which undulates only, lived fifteen minutes on the Grid and its world replays
exactly. No creature has been scored.
[docs/PLAN.md](docs/PLAN.md) says what the phases are and why the benchmark comes before any brain. Beside the benchmark, the lab takes apart the
maintainer's toy worm ([docs/research/gr-fec.md](docs/research/gr-fec.md) §§ 9-12) to learn how to design a brain
rather than search for one; what carries up the ladder is kept in [docs/research/findings.md](docs/research/findings.md).

## Standing Watch

- **The Pattern** ([H15](docs/hypotheses/README.md#h15-the-pattern-a-small-core-that-recurs)), for as long as the
  lab runs. Once a month, the first run of the month reads the journal's registered ablations since the last review,
  adds each mechanism that survived or failed to H15's ledger, and writes in the journal whether any candidate is
  recurring. A watch, not a task: it never leaves this list, and it claims nothing the ledger does not show.

## Next

Top first. Each item is sized for one run; split it here before starting if it is bigger. What each finished step
found lives in the journal section it names; this list keeps only what is open.

1. **Fill the gaps in the pace note** that are left after the passes of 2026-11-03, as listed with their next leads in
   [docs/research/pace.md § Gaps, to Be Read](docs/research/pace.md#gaps-to-be-read). Figures are within reach through
   Europe PMC's supplementary-files service (pace.md § Noxious Heat).
2. **Build the feature-extraction tool** for the chosen dataset, in the lab's Rust workspace (`crates/`), compute
   the wild-type reference statistics, and build the benchmark's test on them. Split into runs:
   - (a) the workspace and the MD5 check, done;
   - (b) the eligible manifest, the seeded draw in blocks of 200 and `fetch-block`, done
     ([data/README.md § The Eligible Manifest](data/README.md#the-eligible-manifest),
     [§ The Reference Draw](data/README.md#the-reference-draw), [§ Fetching a Block](data/README.md#fetching-a-block));
     block 1 is fetched afresh on each machine;
   - (c) the measures from the skeletons (crate `features`), in seconds and body lengths, with a rule for recordings
     of other lengths than 15 minutes (journal of 2026-10-14, § How Long the Reference Recordings Are). The published
     locomotion states are reproduced exactly (`features::states`). The lab's midbody speed leaves more frames in no
     state than the published one, and the chain of tests that traced why, from 2026-10-14 to 2026-10-15, ends at
     the stored skeletons' uneven spacing, which accounts for most of it (a median 92%) (journal of 2026-10-15, § Do the Undefined Frames Follow From the Spacing?,
     and the sections before it). Decided: both bodies' states come from the published rule applied to the lab's
     own speed, one instrument for both ([docs/benchmark/FEATURES.md § Open Questions Before Freezing](docs/benchmark/FEATURES.md#open-questions-before-freezing)).
     The durations and rates are written to the toolbox's definitions (`features::states::event_stats`;
     [behaviour-measures.md § How the Toolbox Turns States Into Durations and Rates](docs/research/behaviour-measures.md#how-the-toolbox-turns-states-into-durations-and-rates)).
     From the published speed they reproduce the stored durations and frequencies exactly, in 600 of 600
     recording-modes of block 1 (journal of 2026-11-04, § The Published Durations and Rates: Reproduced Exactly Over
     Block 1; `stored-events <block>`). The lab's speed gives a median 0.97 of the stored forward events and 0.91 of the
     paused, backward as many. Next: drop `provisional_` from `features::profile`'s columns; whether any recording-mode
     exercises the lone-edge-event exception; an interval on the lab-against-published ratios, and whether they matter
     for the benchmark;
   - (d) the reference statistics, split into runs (the maintainer's request, 2026-10-31, to build the benchmark
     next; journal of 2026-10-31, § The Benchmark, Split Into Runs):
       - (d1) one row of measures per recording, done (journal of 2026-10-31, § One Row of Measures per Recording:
         Built, and Run on Block 1): `features::profile` and `profiles <block>`, its durations and rates provisional
         until (c)'s check passes;
       - (d2) the reference statistics over block 1, done (journal of 2026-11-03, § The Reference Statistics of Block
         1): `features::eigenshapes`, `benchmark::reference` and `reference-stats <block>`; the 17 short recordings
         are recordings of the same protocol, 16 of them of five minutes from the summer of 2009, and the rule stands (§ The
         Short Recordings of Block 1, Read From Their Metadata);
       - (d3) the gaps by cause, done (journal of 2026-11-03, § The Gaps by Cause in Block 1): `features::gaps`,
         `features::read_frame_index` and `gap-causes <block>`. The files tell stage movements apart from frames with
         no good skeleton, but not dropped frames from failed or rejected skeletons. Stage movements follow a moving
         worm, so they remove moving frames. Next, from it: the edge test against a baseline matched for motion,
         since its control departed from 1; the skeletons files, if published, to split *no good skeleton*; and for
         (e5), whether the Grid's frames are masked to match, and the window counted from the video's first frame
         (recommended; it changes nothing in block 1);
       - (d4) the eigen-shape amplitudes as measures, done (journal of 2026-11-03, § The Eigen-Shape Amplitudes of
         Block 1): block 1's first four shapes frozen as `features::eigenshapes::REFERENCE_SHAPES`, and each
         recording's spread along them four new columns of `features::profile`; (e3) projects block 2 on the basis;
   - (e) the benchmark's test (PLAN.md Phase 0, step 5), split into runs:
       - (e1) done (journal of 2026-10-31, § The Benchmark's Test, Built Red-First): crate `benchmark`, a 21-nearest-
         neighbour classifier two-sample test with five folds and a 999-shuffle permutation p-value, its settings fixed
         on synthetic samples; with 199 shuffles it held its level (4 of 100 identical pairs told apart, 95% interval
         1-10%) but caught a change of spread only 17 times in 100 (10-26%), which (e2) must cover;
       - (e2) done (journal of 2026-10-31, § The Per-Measure Comparisons): `benchmark::compare`, a two-sample
         Cramér-von Mises statistic per measure with a 999-shuffle permutation p-value, Holm's correction across the
         measures, and each sample's median and interquartile range, the statistic computed exactly in integers; with 199
         shuffles it held its level (49 of 1000, 3.6-6.4%) and caught a doubled spread 73 times in 100 (63-81%);
       - (e3) done (journal of 2026-11-04, § Real Against Unseen Real: the Results): `benchmark::holdout` and
         `compare-blocks`; block 1's 179 against block 2's 189, 27 measures, seed 20261104: no measure differs and the
         classifier does not tell them apart (p = 0.181); the control, one block-1 sd added to block 2's first
         eigen-shape amplitude spread, is told apart by both, by that measure alone. Every posture median is slightly
         higher in block 2, none flagged; noted, not claimed, and the upward control sat on it, so was effectively
         about 1.2 sd;
       - (e4) how large an alteration of each measure the test can see, from the positive control at several sizes,
         in both directions, and at the sample sizes a Grid comparison will have (journal of 2026-11-04, § Real
         Against Unseen Real: the Results, *The control*);
       - (e5) freezing the definition, which waits on
         [FEATURES.md § Open Questions Before Freezing](docs/benchmark/FEATURES.md#open-questions-before-freezing) -
         food first - and on reading a Disk (*Blocked*).
3. **Prove the arena**: build Master Control and the Grid from `external/` on Linux with lavapipe, write a placeholder
   Program, live one life and extract the same features from it. Done: (a) Master Control, (b) the Grid,
   (c) the placeholder (`crates/placeholder-worm`), (d1) one life, its Disk and log kept and replayed (journal of
   2026-10-14, § The Arena, Fourth Step: One Life). Open: (d2) the same features extracted from the Disk, which needs a Disk
   reader in `features`, and the share of refused intents measured; waits on a ruling (see *Blocked*).
4. **Read the quantum-like Program's sources, then test it again**: read Noest 1988, Jankowski, Lozowski and
   Zurada 1996 and Kobayashi 2017 at the source (H10 cites them unread), check the first test's attribution of its
   update rule, then run the next test under *To Revisit* with a finer grid, more cues, a symmetric success rule and
   a keep-the-state rule for ties.
5. **Plan an experiment for every one of Matej's 128 ideas and hunches** - the maintainer's request - in the format
   of [docs/hypotheses/experiments/README.md](docs/hypotheses/experiments/README.md), whose index shows which are
   planned. Next, one task each, in this order: **F** (17; A to E are planned), then **G** (30, from the gr-fec sandbox;
   [docs/research/gr-fec.md](docs/research/gr-fec.md) has its numbers). The journal records the triage, and hunches
   that survive are promoted into numbered hypotheses or tasks here; E15, E11, E16 and E20 are the candidates from E11-E20
   (journal of 2026-11-05, § E11-E20 Planned, and a Triage).
6. **Audit H10's first test by A12's plan**
   ([docs/hypotheses/experiments/A9-A15.md § A12](docs/hypotheses/experiments/A9-A15.md#a12-experiment-hygiene)):
   planted copies prepared by one agent and audited, blind, by another that has read neither that test's review nor
   the preparation.
7. **Audit the Program interface field by field**, by
   [docs/hypotheses/experiments/B.md § B1](docs/hypotheses/experiments/B.md#b1-a-nervous-system-not-a-game-client):
   every sense and action against what a worm can transduce and produce, with a blind planted-field control; the
   delay grades wait on item 1. B1 names the interface at an older pin; the audit lists the fields at the pin it runs
   on (Program ABI 12 since 2026-11-04, which removed the speed over the ground, one field B1 would have had to grade).
8. **gr-fec's worm by equation** - the maintainer's aim (2026-10-26): understand [gr-fec's worm](docs/research/gr-fec.md)
   well enough to design it for a chosen behaviour, with every decision auditable, instead of evolving it. Why the
   series stalled, and what to do instead, is in
   [gr-fec.md § 12](docs/research/gr-fec.md#12-why-the-demystification-stalled-a-post-mortem-2026-11-04). What is
   known is in [gr-fec.md §§ 9-10](docs/research/gr-fec.md#9-what-the-lab-found-on-taking-it-up-2026-10-26) and
   [findings.md § Designing a Brain](docs/research/findings.md#designing-a-brain); each experiment's script and pull
   request are in [gr-fec.md § 11](docs/research/gr-fec.md#11-where-the-labs-toy-worm-scripts-are). Open, one run each:
   - (a) the weak nose in the warm world, where its effect is small and mixed;
   - (b) the hunger gate and its fix. The global-contrast fix does not generalise: the cold-band hunch was refuted on
     the ten fresh banded layouts, narrowly with the margin and also without it, and there gated steering mostly ends
     below never steering; the margin rule keeps the rich-world fix in the five fresh rich layouts (journal of
     2026-10-29 and 2026-10-30).
     H14's graded hunger gain and its ablations, tested on 2026-10-30: the results are in
     [H14](docs/hypotheses/README.md#h14-graded-slow-knobs-change-detecting-senses-and-individuality) § *Tested so
     far*. Why steering trails never steering in warm layouts, tested on 2026-11-03, and each decision scored against
     the band on 2026-11-04: the journals of those nights, read together in gr-fec.md § 12. **Held, awaiting the
     maintainer's decision** on [gr-fec.md § 12.4](docs/research/gr-fec.md#124-recommendations-for-the-maintainer-to-decide),
     which recommends pausing this series and returning to the equations. Held with it, the series' earlier open
     steps: reopening the floor (a test over many more rich layouts, sized beforehand to resolve about +0.002); more
     body seeds per layout, to tell a small cold-band gain from none; what the gate does in 23001's first day; the
     timing of a worm's meals against the turning of its map; and what makes the map point away in layouts such as
     82606. Nothing in (b) is started until the maintainer decides;
   - (d) the composition law: a third lever that acts alone, and roaming's interaction as its own test.

   Once a calculation matters to anything the lab builds on, it moves into tested Rust.
9. **Make the repository easy to enter** - the maintainer's request (2026-10-31): tidy the repository and make it
    presentable so that new people are not lost, with more plots, diagrams and flowcharts, as SVG and never as
    binaries. The survey and the reasons are in the journal of 2026-10-31, § A Plan to Make the Lab Easy to Enter.
    One run each, in this order:
    - (a) done (journal of 2026-10-31, § The Figure Tooling, and the First Diagram): the figure tooling,
      `notebooks/figures.ipynb`, which writes every new diagram as plain SVG from Python and
      every new plot with matplotlib by the rules of *Figures are SVG*, and checks in its last cell, run locally, that
      drawing again changes no byte (a local check, not a CI job; H11's figures stay with their own notebooks); proved
      on the first diagram, how the pieces fit;
    - (b) done (journal of 2026-10-31, § Two Diagrams: a Night's Cycle and the Benchmark's Pipeline): two diagrams,
      a night's cycle and the benchmark's pipeline;
    - (c) done (journal of 2026-10-31, § An Idea's Path, and the Toy Worm's Graded Gain as a Plot): the ideas diagram,
      in the README and the hypotheses' README, and the graded-gain series as a forest plot in two panels, under H14;
    - (d) done (journal of 2026-10-31, § A "Start Here" Tour in the README): a six-stop tour at the README's head,
      built around the diagrams and pointing to *Where Things Are*, and a short glossary of the lab's own words;
    - (e) done (journal of 2026-10-31, § A Journal Index): a table in `docs/journal/README.md`, one line a night with
      the section to start reading at; whether each night's write-up step should add its line is the maintainer's call
      (see *To Revisit*);
    - (f) STATE.md cut back to what is current: *Done* to the latest night only, as its own header says; the *Last
      updated* paragraph (done in [#214](https://github.com/ai-quokka-wannabe/iso-lab/pull/214), cut to the latest
      night with a pointer to the journal index); and the longest *Next* items (2 and 8) cut to their open steps, each
      removed line first checked, one by one, to be held by the journal it points to;
    - (g) the hypotheses' README: (c)'s diagram is in it; what is left is a short section pointing to the experiments'
      index for the rest;
    - (h) the audits (`check-coherence`, `check-style`, `check-spelling`) once more, to catch what is left.

10. **Try the associative memories on the worm's scale**, from
    [docs/research/associative-memory.md](docs/research/associative-memory.md) § For the Lab: first read its gaps
    (§ Gaps, to Be Read - Hopfield 1982 and Oja 1982 in full, Amit, Gutfreund and Sompolinsky's capacity), then, in a
    notebook, a 302-unit classical network, a dense one and a Boltzmann machine at a load of two or three states,
    each with its switch, measuring the sweeps a settling takes against the decision times in
    [pace.md](docs/research/pace.md). Exploration only; a module that earns its place moves into Rust and waits, as
    E4 does, on a Program and the frozen benchmark.
11. **The stimulus assays**, from [docs/benchmark/STIMULI.md](docs/benchmark/STIMULI.md) § Order, Effort and What
    Blocks Each (journal of 2026-11-04, § What the World's New Fields Open for the Benchmark: a Plan). They are a second
    part of the benchmark, frozen on its own and never delaying item 2 (e5). One run each, in this order; no Grid life
    before the senses and sources below exist:
    - (a) done (journal of 2026-11-04, § The Salt Dataset: Its Methods and What the Archive Holds): the methods read,
      and the archive listed by range requests. Its salt tracks are centroids at 5/14 s, with no posture and no
      worm or plate identity (STIMULI.md § Taste);
    - (b) partly done (journal of 2026-11-04, § The Salt Measures, Drafted): six measures drafted in
      [docs/benchmark/SALT_MEASURES.md](docs/benchmark/SALT_MEASURES.md), with the gradient's direction fitted from
      the data and the centroid chosen for both bodies. Open there: Q1, the unit of the test, which has 7–11 plates per
      condition and no plate in the files; Q3, the authors' smoothing, both for the maintainer (a lead: Chen et al. 2023's
      0.5 s polynomial, unconfirmed). Q7, the heading's noise at low speed, is decided (journal of 2026-11-04, § Q7:
      Headings Over a Quarter of a Body Length): a heading only over a displacement of at least 0.25 L;
    - (b2) for the maintainer: a letter to the preprint's authors (SALT_MEASURES.md § The Open Questions, Q1 (a)) for
      each track's plate, their smoothing, the start positions, `Data_salt0_0.mat` and `mask`; or the authors' code
      repository attached to a session, for the smoothing. Then Q1's choice of route and of the Grid's plate;
    - (c) salt, which may start now, with Q1 and Q3 open: fetch the archive, checksum and register it in data/README.md,
      fit the gradient's direction
      by SALT_MEASURES.md's rule (P4), and compute M1-M6 per track as description in `features`, in tested Rust, each
      test seen failing first, with Q7's registered check at ℓ = 0.125 L and 0.5 L reported beside the primary 0.25 L. No
      Grid life is needed, so it is not blocked upstream. The measures that use heading
      change (M2-M4, M6) stay provisional until Q3 is answered, and the reference statistics wait on Q1;
    - (d) odour: read Chen et al. 2023's methods ([doi:10.7554/eLife.85910](https://doi.org/10.7554/eLife.85910)) and
      list its archive's files (STIMULI.md § Air), and say whether its measures against bearing, made in a flow
      chamber, can stand for still air;
    - (e) heat: search the thermotaxis papers' data statements for public tracks, and read the assays' plates, gradients
      and measures in full. Ryu and Samuel 2002 and Luo et al. 2014 were found closed (pace.md); try Luo et al. 2006 and
      Ito et al. 2006 first, then an author's manuscript in Europe PMC, then data or a reprint asked of an author through
      an issue the maintainer files; park with what was tried if none opens;
    - (f) the arena: the floor's 128 m (28.6 body lengths, 2.86 cm at the world's scale) and its terraces against an
      assay plate, for every assay, folded into FEATURES.md § Open Questions Before Freezing; first measure the spawn
      pad's terrace (STIMULI.md § What a Program Senses Today).

## To Revisit

Experiments that failed or came out inconclusive, kept rather than discarded (the maintainer's ruling); each with
the idea for the next attempt.

- **C18's threshold hunch** (journal of 2026-11-03, § C18's Loose Ends, Run): the sweep of the activation threshold at
  the budget's gain of 1.2 came out void by its control, L0, missed by 0.001. Read post hoc, the miss is largely the 1%
  locator's dependence on the grid's step: re-located at the run's step it is 1.569, still 0.04 from 1.528, so the
  fresh draws account for part of it. Descriptively, D fell from 1.654 to 1.160 towards the graph's own 1.014 as the
  threshold fell. Next attempt: the same sweep with fresh seeds, with L0 set against the run's own grid of step 0.25.
- **For the maintainer**: whether each night's write-up (step 8 of *Each run, in order*) should also add a line to
  the journal index in `docs/journal/README.md` (item 9 (e)); without that, the index goes stale.
- **For the maintainer**: whether a decided item, such as the speed for the locomotion states, stays under
  [FEATURES.md § Open Questions Before Freezing](docs/benchmark/FEATURES.md#open-questions-before-freezing) or moves to
  a section of decisions (journal of 2026-10-30, § A Coherence Audit, and What It Mended).
- **H10, phasor against real memory** ([the notebook](notebooks/h10_phasor_memory.ipynb), 2026-10-13): no better at
  the same memory budget, at twice the compute. Next, after item 4's re-run of this test with fairer rules: test
  phase where the hunch says it matters - the undulation phase of a rhythm generator, interference between two paths
  (F1), or more phase states.
- **H11, a Hebbian Hopfield transformer** ([v1](notebooks/h11_hebbian_hopfield_transformer.ipynb),
  [v2](notebooks/h11v2_delta_pilot.ipynb), [v3](notebooks/h11v3_noisy_cue.ipynb),
  [v4](notebooks/h11v4_keyclean.ipynb) and the [v5 pilot](notebooks/h11v5_binary_keys.ipynb), 2026-10-14): none
  beating its matched control (linear attention in v1, DeltaNet after), and in none did the settling beat its own
  ablation; binary keys (v5) did not change that in a pilot of 3 seeds. Next: make the narrow-key models train reliably
  (v4 and v5 each lost a seed), then a full test of binary keys or Krotov and Hopfield 2016's dense energies against
  DeltaNet. Kanter and Sompolinsky 1987 was not found in open access (Crossref lists no abstract; Semantic Scholar's
  API marks it closed and the abstract elided by the publisher, checked 2026-10-14); it stays unread.
- **H13, the proof of concept on synthetic worms** ([round 1](notebooks/h13_modular_poc.ipynb),
  [rounds 2 to 4](notebooks/h13_rounds_2_to_4.ipynb); the journals of
  [2026-10-17](docs/journal/2026-10-17.md), [2026-10-18](docs/journal/2026-10-18.md),
  [2026-10-19](docs/journal/2026-10-19.md) and [2026-10-22](docs/journal/2026-10-22.md) hold every round).
  Where it stands: the grind's decisions work - ablation told real modules from a decoy in every run - and on five
  fresh populations, one optimiser seed each, the whole loop passed its first criterion for 5 of 5 (journal of
  2026-10-19, § H13: Does the Pass Hold beyond DEV?), with the rate of leaving roaming (`k_rd`) within 0.2 for 4;
  habituation's `dep` and `tau` were not both recovered in three of five, the behaviour statistics seeing one
  combination of them far more clearly than the two apart. A short-interval tap train separates them on a near noise-free target but did not carry to the whole
  loop on five fresh 30-worm populations (journal of 2026-10-22, § H13: The Whole Loop on Fresh Populations, under
  the Short-Interval Train). Next: the same loop under that protocol on fresh populations of 60 or more to fit, to
  see whether the gain arrives as sampling noise falls. For the maintainer: whether a 2 s tap interval is right for
  the benchmark's assay (not yet checked against the literature). For the benchmark: its comparison space should hold
  state-conditioned rates, bout lengths and whole distributions, not only means.

## Blocked

- **How the lab reads a Disk** (item 3(d2); waits on the maintainer's ruling). A Disk is Link's frames in a file,
  read back through Link's own library (`replay_open` and `poll` in `external/link/include/lnk/lnk_client.h`), which
  must lie beside the executable that loads it; the lab's rules make each route cost something. (1) Depend on
  `external/master-control` as a crate and reuse its `link_dll` mirror and `replay_open`, which its Clu module
  uses: one implementation, its `unsafe` kept inside Master Control's crate, and its `build.rs` builds Link and
  copies the library beside the lab's binaries and tests - but every lab build, CI's included, then needs the
  submodules checked out recursively. (2) Copy the parts of that mirror the reader needs into `features` (most of
  its structs, since the message view's union and the vtable hold every message type and call): a third copy of the
  layout, guarded against the library only by the vtable's size and ABI version; `unsafe` code in a workspace that
  denies it, allowed locally as in `placeholder-worm` - a second such place in the lab's Rust, which that crate calls
  the one; and the reader's tests still
  need Link built and placed beside them, from `external/master-control/external/link`, so CI is no lighter unless
  the reader goes untested, which the lab's rules do not allow. (3) Decode the frames in Rust: a second decoder,
  against the organisation's "one implementation" doctrine - not recommended. The lab's rule that `external/link` is
  not what gets built rules out depending on it directly. Recommended: (1).

## Needs From Other Repositories

Each need is an issue in the repository that would meet it, triaged there by its maintainers; this list
points to the open ones and says what waits on each.

- **A sense of smell** (tron-grid-lite, master-control; the maintainer takes it up once the lab is set up -
  file nothing). The Grid at the pinned commit carries a chemical sense to Programs: `TglSenses.odour[2][8]`, since
  Program ABI 11 and at 12, the bound fraction of eight receptor types at each of two amphids, smelt from the food orbs
  (journal of 2026-10-31, § The Grid Moves to Program ABI 11); since the pins of 2026-11-04 the odour spreads on Master
  Control's air grid and each type is a population over five decades of affinity (journal of 2026-11-04). The organs
  bump, ruled upstream and not yet built, replaces `odour` with declared organs (tron-grid-lite `docs/ORGANS.md`). Chemotaxis stays listed as not comparable until a
  Program reads it and the benchmark measures it - never quietly dropped. B2 of
  [the B plans](docs/hypotheses/experiments/B.md#b2-smell-as-an-unlabelled-fingerprint) lists the properties a
  hunch asks of it (intensity, its change over time, a left-right difference, blending mixtures, blurring traces).
  Smell is built with food (tron-grid-lite `docs/CHEMORECEPTION.md`); see the next line.
- **Food, and a sense of taking it in** ([master-control#63](https://github.com/ai-quokka-wannabe/master-control/issues/63)
  for food on the floor and the rule for intake; [tron-grid-lite#153](https://github.com/ai-quokka-wannabe/tron-grid-lite/issues/153)
  for the sense of the body's own intake). The reference worms are filmed on food, and the world has had food orbs since
  the pins of 2026-10-29, eaten in the tick since 2026-10-30, drawn by the
  Grid since Program ABI 11
  ([docs/benchmark/FEATURES.md § Open Questions Before Freezing](docs/benchmark/FEATURES.md#open-questions-before-freezing)).
  The benchmark is not frozen until food is settled, by these issues or by comparing with foodless worms instead.
  Under way upstream, by the maintainer's comment on both issues (2026-10-29): the design is merged (tron-grid-lite
  #164, #166), then master-control#70 (the world's half, no wire) and link#49 (protocol v12), both merged, then the
  wire and food in the tick (master-control#72 and #73, pinned since the journal of 2026-10-30); then the Grid's
  Program ABI 11 (tron-grid-lite #172 and #173), which carries `intake` and `odour[2][8]` to Programs, pinned with the
  placeholder rebuilt against it (journal of 2026-10-31, § The Grid Moves to Program ABI 11); next rc-worm.
- **The worm's bends against the Grid's joint limit**
  ([tron-grid-lite#165](https://github.com/ai-quokka-wannabe/tron-grid-lite/issues/165), filed 2026-10-29). At the eight-arc
  resolution 9.8 to 68.6% (median 35.8%, 34 recordings) of the reference worms' skeletonised frames bend beyond the
  chain's 0.9 rad at some joint, against 0.15% beyond Master Control's pi/2. The issue asks to raise the limit to
  pi/2, with the caveats (journal of 2026-10-15, *The Joint-Limit Issue, Drafted*).
- **Senses and sources for the stimulus assays** (tron-grid-lite, master-control; written here, not filed, per this
  task's brief; [docs/benchmark/STIMULI.md](docs/benchmark/STIMULI.md) has the evidence). At the pins of 2026-11-04 no
  Program senses heat or taste, and the gases are not in the world. Needed, for the benchmark's stimulus assays and
  for nothing a brain is tuned by: (1) the organs bump (tron-grid-lite `docs/ORGANS.md`, ruled), for heat and salt;
  (2) a salt source apart from food, a placed solute gradient or spot, since every salt assay is salt alone and the
  world's only salt leaches from orbs that also smell and feed; (3) an odour source apart from food, with a declared
  pattern over the receptor types (deferred upstream "when a world has a second kind of source"); (4) a way to raise a
  Program at one temperature and test it on a gradient - a scheduled change of the thermal map, or a radial gradient -
  which would also give temperature steps for a reaction latency, and likewise a Program raised at one salt
  concentration before a salt gradient (SALT_MEASURES.md, Q6); (5) the world's solute unit tied to a concentration,
  so a gradient in millimolar has a meaning; (6) oxygen and carbon dioxide as tenants of the air, for aerotaxis, last.
  The arena's size and flatness against a plate is the lab's open question first (item 11 (f)).
- **Terrace edges** (master-control; the maintainer takes it up once the lab is set up - file nothing). A
  chain can be thrown when a segment crosses a terrace edge. At the pins of 2026-11-04 master-control's CHANGELOG
  records a fix for one such throw ("A riser is solid only up to its top"); whether it closes this need is the
  maintainer's call. The floor is terraced (`GRID_FLOOR_CONFIG` in master-control's `src/ground.rs`) and has no flat
  setting, so the lab's lives are not on a flat arena, as this line used to say: they start on the spawn pad, a cell
  proven flat for a metre (`src/roster.rs`), and the placeholder's 320-tick lives of 2026-11-04 ended 0.7 m from its
  centre (journal of 2026-11-04, § What the World's New Fields Open for the Benchmark: a Plan).

## Done

The latest night only, one line a task; earlier nights are in [the journal](docs/journal/), and each toy-worm
experiment's pull request is in [gr-fec.md § 11](docs/research/gr-fec.md#11-where-the-labs-toy-worm-scripts-are).

- 2026-11-05: item 5, hunches E11-E20 planned ([E11-E15.md](docs/hypotheses/experiments/E11-E15.md),
  [E16-E20.md](docs/hypotheses/experiments/E16-E20.md)); E15, E11, E16 and E20 ready to become tasks, the rest wait on a
  Program, senses the Grid lacks or worm sources (journal of 2026-11-05, § E11-E20 Planned, and a Triage).
- 2026-11-04: item 2 (c), the stored durations and frequencies recomputed from the published speed: 600 of 600
  recording-modes of block 1 equal to the last bit; the lab's speed against them in a second table (journal of
  2026-11-04, § The Published Durations and Rates: Reproduced Exactly Over Block 1).
- 2026-11-04: item 2 (e3), real against unseen real: block 1 (179) and block 2 (189) are indistinguishable by both
  tests at seed 20261104, and the positive control (a 1 sd shift of one measure) is told apart by both; block 1's
  frozen eigen-shapes fit block 2 to a cosine of 0.997 or better (journal of 2026-11-04, § Real Against Unseen Real:
  the Results).
- 2026-11-04: `external/` at the latest mains (tron-grid-lite `067fb7e`, master-control `7c527d9`, link `0304788`),
  the placeholder at Program ABI 12: D1-D3 hold - Master Control's suite 124 then 158 passed, `ctest` 22 of 22, the
  three device checks; the digest moved to `6A94D9A1…464F` and a build at an attenuation of 1 gives the old one - and
  the placeholder's life is the same at both pins once its join tick is taken out (journal of 2026-11-04, § The World
  Moves to Protocol 14 and Program ABI 12, Measured Before and After).
- 2026-11-04: item 8 (b), each steering decision of the graded gain scored against the band on ten fresh warm layouts:
  the lateral sense is right in 0.986 of decisions and temperature leads to the band in 0.945, so by elimination the
  direction's failure lies in the map's choice as the greedy rule reads it, which points the band's way in 0.513 (0.424 to 0.619), undecided by its rule
  (journal of 2026-11-04, § Does the Greedy Rule's Direction Fail? Each Steering Decision Scored).
- 2026-11-04: item 11 (b), Q7 decided before any data: a heading is taken only once the centroid has moved 0.25 L,
  so that one pixel of tracking noise cannot alone fire a 50° turn; a sensitivity check at 0.125 L and 0.5 L registered
  ([docs/benchmark/SALT_MEASURES.md](docs/benchmark/SALT_MEASURES.md) § The Open Questions).
- 2026-11-04: item 11 (b), partly: six salt measures drafted, signed by the setpoint and using no salt concentration
  in mM; the centroid for both bodies, and the gradient fitted from the data; the unit of the test and the authors'
  smoothing left for the maintainer ([docs/benchmark/SALT_MEASURES.md](docs/benchmark/SALT_MEASURES.md)).
- 2026-11-04: item 11 (a), the salt dataset's methods and its archive listed without downloading it: centroids every
  5/14 s, 297 and 277 tracks in the 0-50 and 100-50 mM gradients, no posture, and neither worm nor plate kept; not adopted
  (journal of 2026-11-04, § The Salt Dataset: Its Methods and What the Archive Holds).
- 2026-11-04: item 11, a plan for the stimulus assays the world's new fields open
  ([docs/benchmark/STIMULI.md](docs/benchmark/STIMULI.md)): at Program ABI 12 only odour is sensed, heat and salt exist
  unsensed until ABI 13, the gases not at all; two CC BY 4.0 datasets of tracks found (salt and odour, both from the
  Leifer lab), none for thermotaxis in one search; order salt, odour, heat, aerotaxis.
- 2026-11-04: item 8, a post-mortem of the toy-worm series from the record, why it stalled and what stands, with
  recommendations awaiting the maintainer
  ([gr-fec.md § 12](docs/research/gr-fec.md#12-why-the-demystification-stalled-a-post-mortem-2026-11-04)).
