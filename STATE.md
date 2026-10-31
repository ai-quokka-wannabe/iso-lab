# State of the Lab

**Last updated:** 2026-10-31 ([the journal of 2026-10-31](docs/journal/2026-10-31.md)): C18 was unparked - its lead read in
full, its cliff defined, its test piloted on four controls, which it read as it must; section D was planned whole, and
the toolbox's event statistics (done 2026-10-30) were merged. Before it, 2026-10-30
([the journal of 2026-10-30](docs/journal/2026-10-30.md)): `external/` moved to latest
`main` twice, the first time with the world unchanged, the second to Link protocol 12 with food eaten in the tick.
The global rule without the margin was tested on the ten fresh banded layouts: the cold-band hunch is refuted for it
too. H14 was added, and its toy-worm prediction held: a graded hunger gain beat the on/off gate, and ablations trace
that lead to smaller turns, with neither the missing latch nor grading detected to contribute; the series rests
there. H15 and a standing watch for a recurring core were added, and the benchmark's choice of speed for the
locomotion states was decided: one instrument for both bodies. Before it,
2026-10-29 ([the journal](docs/journal/2026-10-29.md)): mostly housekeeping - the
linter's and the fetch tool's dependencies patched, the night's audits, this file cut to pointers, and `external/`
moved twice, the second time to the food world, where Master Control's new physics shifts the placeholder's life in
the sixth decimal - and one toy-worm test: the cold-band hunch, refuted narrowly on ten fresh layouts. The night
before is [2026-10-28](docs/journal/2026-10-28.md), summarised in
[gr-fec.md § 10.2](docs/research/gr-fec.md#102-2026-10-28). The next step for Phase 0 is the locomotion
states' durations and reversal rates (item 2(c)).

## Where We Are

Phase 0, the benchmark, is under way. The reference data is chosen - the Open Worm Movement Database's N2
recordings ([data/README.md § The Phase 0 Survey](data/README.md#the-phase-0-survey)) - and the first
literature note says how worm behaviour is measured. The comparison space is drafted
([docs/benchmark/FEATURES.md](docs/benchmark/FEATURES.md)): which measures carry over to the Grid's body, and what
must be settled before the benchmark is frozen, food first. The Rust workspace reads the reference skeletons and computes
the first measures - eight-arc posture and a midbody speed, the latter chosen as the benchmark's instrument but its durations and
reversals not yet checked - and the arena is proven to a
first life: a placeholder Program, which undulates only, lived fifteen minutes on the Grid and its world replays
exactly. No creature has been scored.
[docs/PLAN.md](docs/PLAN.md) says what the phases are and why the benchmark comes before any brain. Beside the benchmark, the lab takes apart the
maintainer's toy worm ([docs/research/gr-fec.md](docs/research/gr-fec.md) §§ 9-11) to learn how to design a brain
rather than search for one; what carries up the ladder is kept in [docs/research/findings.md](docs/research/findings.md).

## Standing Watch

- **The Pattern** ([H15](docs/hypotheses/README.md#h15-the-pattern-a-small-core-that-recurs)), for as long as the
  lab runs. Once a month, the first run of the month reads the journal's registered ablations since the last review,
  adds each mechanism that survived or failed to H15's ledger, and writes in the journal whether any candidate is
  recurring. A watch, not a task: it never leaves this list, and it claims nothing the ledger does not show.

## Next

Top first. Each item is sized for one run; split it here before starting if it is bigger. What each finished step
found lives in the journal section it names; this list keeps only what is open.

1. **Fill the gaps in the pace note** ([docs/research/pace.md § Gaps, to Be
   Read](docs/research/pace.md#gaps-to-be-read)): a touch-to-reversal latency with frame-level timing (none of the 45
   open papers scanned states one in text, and Liu et al. 2018's figure shows reversals clustering within about a
   second without resolving their onset); what Mohammadi et al. 2013's heat reaction times count from; a thermotaxis
   latency; fitted time constants for tap habituation from primary papers; and the primary sources for lethargus and
   sleep-like quiescence (for satiety, You et al. 2008 is identified, its full text not yet reached). Figures are
   within reach: Europe PMC's supplementary-files service serves a paper's figure images (pace.md § Noxious Heat).
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
     Next: reproduce the stored durations and frequencies from the published speed, then check the lab's speed against them;
   - (d) the reference statistics, split into runs (the maintainer's request, 2026-10-31, to build the benchmark
     next; journal of 2026-10-31, § The Benchmark, Split Into Runs):
       - (d1) one row of measures per recording: `features::profile` and a binary that writes one tab-separated row
         per recording of a block, in the units of FEATURES.md § Common Units. It first decides, with its reason in
         the journal, how recordings of unequal length and frame rate enter (FEATURES.md § Open Questions Before
         Freezing asks for that before any statistic), and states a rule for a recording too gappy to measure, with a
         reason column rather than silent dropping. Its columns are the measures already built: the spread of each
         of the eight mean-removed arc angles (the eigen-shapes are fitted in (d2), across all the real worms), the
         midbody speed, the three locomotion states' fractions, durations and rates, and each recording's share of
         frames without a skeleton. The durations and rates stand only once item 2(c)'s check passes, and are marked
         provisional until then;
       - (d2) the reference statistics over block 1: the eigen-shapes fitted on its real worms' eight angles, with the
         share of variance they carry; each measure's median and spread across recordings, with a seeded bootstrap
         interval; and how stable they are between the block's two halves;
       - (d3) the gaps by cause, as FEATURES.md § Open Questions Before Freezing asks: how a recording's missing
         frames split between dropped frames, stage movements and failed extraction, and per measure how much it
         leans on frames likely to be lost;
   - (e) the benchmark's test (PLAN.md Phase 0, step 5), split into runs:
       - (e1) done (journal of 2026-10-31, § The Benchmark's Test, Built Red-First): crate `benchmark`, a 21-nearest-
         neighbour classifier two-sample test with five folds and a 999-shuffle permutation p-value, its settings fixed
         on synthetic samples; with 199 shuffles it held its level (4 of 100 identical pairs told apart, 95% interval
         1-10%) but caught a change of spread only 17 times in 100 (10-26%), which (e2) must cover;
       - (e2) done (journal of 2026-10-31, § The Per-Measure Comparisons): `benchmark::compare`, a two-sample
         Cramér-von Mises statistic per measure with a 999-shuffle permutation p-value, Holm's correction across the
         measures, and each sample's median and interquartile range, the statistic computed exactly in integers; with 199
         shuffles it held its level (49 of 1000, 3.6-6.4%) and caught a doubled spread 73 times in 100 (63-81%);
       - (e3) the test on real data alone: block 1's real worms against block 2's, recordings neither has seen, which
         must come out indistinguishable; and a positive control - the same recordings deliberately altered by a
         stated amount - which must be told apart;
       - (e4) how large an alteration of each measure the test can see, from the positive control at several sizes;
       - (e5) freezing the definition, which waits on
         [FEATURES.md § Open Questions Before Freezing](docs/benchmark/FEATURES.md#open-questions-before-freezing) -
         food first - and on reading a Disk (*Blocked*).
3. **Prove the arena**: build Master Control and the Grid from `external/` on Linux with lavapipe, write a placeholder
   Program, live one life and extract the same features from it. Done: (a) Master Control, (b) the Grid,
   (c) the placeholder (`crates/placeholder-worm`), (d1) one life, its Disk and log kept and replayed (journal of
   2026-10-14, § The Arena, Fourth Step: One Life). Open: (d2) the same features extracted from the Disk, which needs a Disk
   reader in `features`, and the share of refused intents measured; waits on a ruling (see *Blocked*).
4. **Survey associative memory for a worm-sized brain**, as `docs/research/associative-memory.md`: Hebbian
   learning and Oja's rule, Hopfield networks classical and modern, Boltzmann machines, dense associative
   memory - what each is, what it would cost a brain on the worm's scale to run, which of the worm's
   behaviours it could carry, and how each would be ablated and tested ([H5, H6 and H9](docs/hypotheses/README.md)).
   A notebook that tries them on toy problems is welcome. The maintainer points out that the 2024 Nobel Prize
   in Physics went to this line of work (Hopfield and Hinton); cite the Nobel committee's own text, read at the
   source.
5. **Read the quantum-like Program's sources, then test it again**: read Noest 1988, Jankowski, Lozowski and
   Zurada 1996 and Kobayashi 2017 at the source (H10 cites them unread), check the first test's attribution of its
   update rule, then run the next test under *To Revisit* with a finer grid, more cues, a symmetric success rule and
   a keep-the-state rule for ties.
6. **Plan an experiment for every one of Matej's 128 ideas and hunches** - the maintainer's request - in the format
   of [docs/hypotheses/experiments/README.md](docs/hypotheses/experiments/README.md), whose index shows which are
   planned. Next, one task each, in this order: **E11-E20** (E1-E3, E5 and E6-E10 are planned; E4 is parked, see *Blocked*), **F** (17), **G** (30, from the gr-fec sandbox;
   [docs/research/gr-fec.md](docs/research/gr-fec.md) has its numbers). The journal records the triage, and hunches
   that survive are promoted into numbered hypotheses or tasks here.
7. **Audit H10's first test by A12's plan**
   ([docs/hypotheses/experiments/A9-A15.md § A12](docs/hypotheses/experiments/A9-A15.md#a12-experiment-hygiene)):
   planted copies prepared by one agent and audited, blind, by another that has read neither that test's review nor
   the preparation.
8. **Audit the Program interface field by field**, by
   [docs/hypotheses/experiments/B.md § B1](docs/hypotheses/experiments/B.md#b1-a-nervous-system-not-a-game-client):
   every sense and action against what a worm can transduce and produce, with a blind planted-field control; the
   delay grades wait on item 1.
9. **gr-fec's worm by equation** - the maintainer's aim (2026-10-26): understand [gr-fec's worm](docs/research/gr-fec.md)
   well enough to design it for a chosen behaviour, with every decision auditable, instead of evolving it. What is
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
     far*. Next: the hunger-gain series rests here; reopening the floor would take a test over many more rich layouts, sized beforehand to resolve
     an effect of about +0.002; why gated steering of any kind still
     trails never steering in warm layouts; more body
     seeds per layout, to tell a small cold-band gain from none; what the gate does in 23001's first day, where its harm starts; and the timing of a worm's
     meals against the turning of its map;
   - (d) the composition law: a third lever that acts alone, and roaming's interaction as its own test.

   Once a calculation matters to anything the lab builds on, it moves into tested Rust.
10. **Make the repository easy to enter** - the maintainer's request (2026-10-31): tidy the repository and make it
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
      updated* paragraph; and the longest *Next* items (2 and 9) cut to their open steps, each removed line first
      checked, one by one, to be held by the journal it points to;
    - (g) the hypotheses' README: (c)'s diagram is in it; what is left is a short section pointing to the experiments'
      index for the rest;
    - (h) the audits (`check-coherence`, `check-style`, `check-spelling`) once more, to catch what is left.
11. **Run C18's plan** ([C13-C24.md § C18](docs/hypotheses/experiments/C13-C24.md#c18-spreading-activation-has-a-cliff)):
    the four controls first, then the pure network, the budget at both gains, the corrected fan-out penalty and
    inhibition, with fresh seeds; its expectations are pre-registered in the journal of 2026-10-31, § C18 Unparked: the
    Cliff Defined, the Test Piloted. Waits on nothing; minutes of compute.

## To Revisit

Experiments that failed or came out inconclusive, kept rather than discarded (the maintainer's ruling); each with
the idea for the next attempt.

- **For the maintainer**: whether each night's write-up (step 8 of *Each run, in order*) should also add a line to
  the journal index in `docs/journal/README.md` (item 10 (e)); without that, the index goes stale.
- **For the maintainer**: whether a decided item, such as the speed for the locomotion states, stays under
  [FEATURES.md § Open Questions Before Freezing](docs/benchmark/FEATURES.md#open-questions-before-freezing) or moves to
  a section of decisions (journal of 2026-10-30, § A Coherence Audit, and What It Mended).
- **H10, phasor against real memory** ([the notebook](notebooks/h10_phasor_memory.ipynb), 2026-10-13): no better at
  the same memory budget, at twice the compute. Next, after item 5's re-run of this test with fairer rules: test
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

- **The plan for E4, Hebb's rule with a stabiliser and a "used together" rule**
  ([#180](https://github.com/ai-quokka-wannabe/iso-lab/pull/180), merged with the plan marked as parked in
  [E1-E5.md](docs/hypotheses/experiments/E1-E5.md#e4-hebbian-learning-needs-a-stabiliser-and-a-used-together-rule)).
  A probe found that sign units ignore Oja's rescaling, so on the plain reading the stabiliser cannot act; Oja's update is not written as an
  equation; and an unstated zero-input rule decides the hub condition. Next: pilot the network in a notebook before
  writing the plan again - graded units, or a threshold, so a rescaling can matter; Oja's update as an equation with
  its output named; a stated rule at zero input; overlap taken as absolute, so a cue that settles in the mirror of a
  pattern counts; and a capacity check per seed. Write the plan only once the pilot shows Hebb's rule alone capturing
  and a stabiliser able to change it.
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
  file nothing). The Grid at the pinned commit carries a chemical sense to Programs: Program ABI 11's
  `TglSenses.odour[2][8]`, the bound fraction of eight receptor types at each of two amphids, smelt from the food orbs
  (journal of 2026-10-31, § The Grid Moves to Program ABI 11). Chemotaxis stays listed as not comparable until a
  Program reads it and the benchmark measures it - never quietly dropped. B2 of
  [the B plans](docs/hypotheses/experiments/B.md#b2-smell-as-an-unlabelled-fingerprint) lists the properties a
  hunch asks of it (intensity, its change over time, a left-right difference, blending mixtures, blurring traces).
  Smell is built with food (tron-grid-lite `docs/CHEMORECEPTION.md`); see the next line.
- **Food, and a sense of taking it in** ([master-control#63](https://github.com/ai-quokka-wannabe/master-control/issues/63)
  for food on the floor and the rule for intake; [tron-grid-lite#153](https://github.com/ai-quokka-wannabe/tron-grid-lite/issues/153)
  for the sense of the body's own intake). The reference worms are filmed on food, and the world has had food orbs since the pins of 2026-10-30, drawn by the
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
- **Terrace edges** (master-control; the maintainer takes it up once the lab is set up - file nothing). A
  chain can be thrown when a segment crosses a terrace edge; a fix is in progress. Until it lands, every life
  in this lab is lived on a flat arena.

## Done

The latest night only, one line a task; earlier nights are in [the journal](docs/journal/), and each toy-worm
experiment's pull request is in [gr-fec.md § 11](docs/research/gr-fec.md#11-where-the-labs-toy-worm-scripts-are).

- 2026-10-31: plans for E6-E10 (learning and memory) (journal of 2026-10-31, § Plans for E6-E10).
- 2026-10-31: C18 unparked - Shrager, Hogg and Huberman 1987 read in full, the hunch's cliff settled as a threshold
  between finite and extensive reach, and a two-stage test of it piloted on four controls, which it read as it must
  (journal of 2026-10-31, § C18 Unparked: the Cliff Defined, the Test Piloted).
- 2026-10-31: item 2 (e2), the per-measure comparisons - Cramér-von Mises per measure, permutation p-values and
  Holm's correction, red-first (journal of 2026-10-31, § The Per-Measure Comparisons).
- 2026-10-31: item 2 (e1), the benchmark's test - crate `benchmark`, a classifier two-sample test, red-first, with
  its settings and power measured on synthetic samples (journal of 2026-10-31, § The Benchmark's Test, Built
  Red-First).
- 2026-10-31: item 10 (d), a "start here" tour and a glossary of the lab's own words in the README (journal of
  2026-10-31, § A "Start Here" Tour in the README).
- 2026-10-31: item 10 (e), a journal index - one line a night, with the section to start at, in
  `docs/journal/README.md` (journal of 2026-10-31, § A Journal Index).
- 2026-10-31: item 10 (c), the ideas diagram and a forest plot of every contrast H14 records, both drawn by
  `notebooks/figures.ipynb` (journal of 2026-10-31, § An Idea's Path, and the Toy Worm's Graded Gain as a Plot).
- 2026-10-31: item 10 (b), two diagrams - a night's cycle, in the README and NIGHT_SHIFT.md, and the benchmark's
  pipeline, in PLAN.md and FEATURES.md (journal of 2026-10-31, § Two Diagrams: a Night's Cycle and the Benchmark's
  Pipeline).
- 2026-10-31: item 10 (a), the figure tooling and the first diagram, how the pieces fit, in the README (journal of
  2026-10-31, § The Figure Tooling, and the First Diagram).
- 2026-10-31: plans for E1-E3 and E5 (learning and memory), taken up again after parking and read whole by a fresh
  verifier; E4 parked again, its network needing a pilot (journal of 2026-10-31, § The E1-E5 Plans, Taken Up Again).
- 2026-10-31: four documents brought in step with the Grid's chemical sense at Program ABI 11 (journal of
  2026-10-31, § Four Documents in Step With the Grid's Smell).
- 2026-10-31: `external/` at the latest mains: tron-grid-lite to Program ABI 11, the placeholder rebuilt; build,
  render and the placeholder's life unchanged - a life with no refused intent is the same at both pins in everything
  but its wall-clock start (journal of 2026-10-31, § The Grid Moves to Program ABI 11).
- 2026-10-31: plans for D7-D13, and A10's question answered: no report of play in a nematode found (journal of
  2026-10-31, § Plans for D7-D13).
- 2026-10-31: plans for D1-D6 (chemistry and affect), each waiting on food and a sense of intake; the leads' bearing
  checked against their abstracts (journal of 2026-10-31, § Plans for D1-D6).
- 2026-10-30: the toolbox's event statistics in Rust (`motion_events`, `event_stats`), tested red-first (journal of
  2026-10-30, § The Toolbox's Event Statistics in Rust).
- 2026-10-30: the toolbox's definitions of state durations and rates, read at source for item 2(c) (journal of
  2026-10-30, § How the Toolbox Turns States Into Durations and Rates).
- 2026-10-30: the night's coherence audit: fourteen points, most mended (journal of 2026-10-30, § A Coherence Audit,
  and What It Mended).
- 2026-10-30: one instrument for both bodies' locomotion states: the published rule applied to the lab's own speed,
  decided in FEATURES.md, with its reason in the journal of 2026-10-30, § One Instrument for Both Bodies' Locomotion
  States.
- 2026-10-30: the constant floor on ten fresh rich layouts: H1 fails (+0.0021, −0.0014 to +0.0052); eight of ten lean its
  way and one is clearly worse (descriptive).
- 2026-10-30: a constant floor below 0.3, three seeds in the rich layouts: F1 and F2 fail; the graded gain's rich edge
  over the matched step is not detected pooled (+0.0014, −0.0039 to +0.0061), but varies by layout (descriptive).
- 2026-10-30: the graded gain without its steering below 0.3: Z1 fails (rich, +0.0025, −0.0012 to +0.0053); its edge
  goes in 82201 and 82204 (descriptive); rich worms spend about 99% of their time below 0.3.
- 2026-10-30: the matched step in the rich layouts: W1 holds in all five (lowest bound −0.0046); graded ends above it
  in three (descriptive; sub-0.3 steering, grading and turn size not separated).
- 2026-10-30: graded, matched and un-latched with three body seeds per layout: Q2 holds (+0.0471, +0.0281 to +0.0644;
  new seeds alone +0.0458); Q1 fails (+0.0015, −0.0202 to +0.0216).
- 2026-10-30: the same with the constant closer to matched (0.5572): P2 holds (+0.0496, +0.0198 to +0.0772); P1 fails (graded
  minus matched −0.0056, −0.0365 to +0.0218).
- 2026-10-30: grading or smaller turns, a scaled step: M2 holds (scaled minus un-latched +0.0423, +0.0116 to +0.0725);
  M1 fails (graded minus scaled +0.0018, −0.0298 to +0.0362): smaller turns carry the lead, grading adds nothing detected.
- 2026-10-30: graded or un-latched, an ablation: L1 holds (graded minus un-latched +0.0440, +0.0096 to +0.0768); L2
  fails (un-latched minus latched gate −0.0063, −0.0275 to +0.0140): the gain carries the lead; no latch effect detected.
- 2026-10-30: the graded hunger gain, a second body seed and the rich layouts: K3 holds (+0.0441, +0.0149 to +0.0737,
  seeds pooled; new seed alone +0.0506, +0.0161 to +0.0892); K4 holds in all five rich layouts; still not shown better
  than never steering (−0.0305, −0.0725 to +0.0111).
- 2026-10-30: H14's graded hunger gain on ten fresh layouts: beats the on/off gate (K1 holds, +0.0377, +0.0046 to
  +0.0678); not shown better than never steering (K2 −0.0279, −0.0723 to +0.0151).
- 2026-10-30: H14 added to the hypotheses (graded slow knobs, change-detecting senses, a set point, individuality),
  from a conversation the maintainer relayed; three new sources read at their abstracts.
- 2026-10-30: the global rule without the margin on ten fresh layouts: N1 fails (cold +0.0179, −0.0321 to +0.0630); N2
  holds only through harm in the warm band (−0.0623, wholly below zero); the margin limits that harm.
- 2026-10-30: `external/` moved to latest `main` again: Link protocol 12 and food eaten in the tick (master-control
  `53e5acb`, tron-grid-lite `ee5332d`); C1-C3 hold, the placeholder's life unchanged.
- 2026-10-30: `external/` moved to latest `main` (the upstream re2js linter override; link `9b1b771`, master-control
  `866b13c`, tron-grid-lite `91f38a0`); B1-B3 hold, the world unchanged.
