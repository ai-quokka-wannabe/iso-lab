# State of the Lab

**Last updated:** 2026-10-30 ([the journal of 2026-10-30](docs/journal/2026-10-30.md)): `external/` moved to latest
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
2. **Build the feature-extraction tool** for the chosen dataset, in the lab's Rust workspace (`crates/`), and compute
   the wild-type reference statistics. Split into runs:
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
   - (d) the reference statistics.
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
   planned. Next, one task each, in this order: **D7-D13** (D1-D6 are planned; answer A10's question whether nematodes
   show anything like play), **E1-E10**, **E11-E20**, **F** (17), **G** (30, from the gr-fec sandbox;
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

## To Revisit

Experiments that failed or came out inconclusive, kept rather than discarded (the maintainer's ruling); each with
the idea for the next attempt.

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

- **The plan for C18, spreading activation's cliff**
  ([#45](https://github.com/ai-quokka-wannabe/iso-lab/pull/45), merged as a record, with the plan marked as parked in
  [C13-C24.md](docs/hypotheses/experiments/C13-C24.md#c18-spreading-activation-has-a-cliff)). Its model as written
  would refute the hunch by construction; the pilot, run to a steady state, found no steady narrowing with size, and a
  width test cannot tell a continuous transition from a smooth change (journal of 2026-10-14, § C18's Model,
  Piloted; journal of 2026-10-15, § C18's Plan, Attempted Again: the Width Test Cannot See a Transition).
  Next: settle what the hunch's "cliff" means (Shrager, Hogg and Huberman 1987, not yet read), then plan a test of
  the reached fraction below and above the threshold (falling as 1/N below, holding above), with the giant component
  and a jump-shaped model as controls, the pilot's conditioned statistic and more draws.
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
  file nothing). The Grid has no chemical sense, and chemotaxis is the worm's best-studied behaviour. Until
  the world provides one, chemotaxis is listed as not comparable - never quietly dropped. B2 of
  [the B plans](docs/hypotheses/experiments/B.md#b2-smell-as-an-unlabelled-fingerprint) lists the properties a
  hunch asks of it (intensity, its change over time, a left-right difference, blending mixtures, blurring traces).
  Smell is now designed with food (tron-grid-lite `docs/CHEMORECEPTION.md`: two amphids, eight receptor types) and
  being built with it; see the next line.
- **Food, and a sense of taking it in** ([master-control#63](https://github.com/ai-quokka-wannabe/master-control/issues/63)
  for food on the floor and the rule for intake; [tron-grid-lite#153](https://github.com/ai-quokka-wannabe/tron-grid-lite/issues/153)
  for the sense of the body's own intake). The reference worms are filmed on food and the Grid has none
  ([docs/benchmark/FEATURES.md § Open Questions Before Freezing](docs/benchmark/FEATURES.md#open-questions-before-freezing)).
  The benchmark is not frozen until food is settled, by these issues or by comparing with foodless worms instead.
  Under way upstream, by the maintainer's comment on both issues (2026-10-29): the design is merged (tron-grid-lite
  #164, #166), then master-control#70 (the world's half, no wire) and link#49 (protocol v12), both merged, then the
  wire and food in the tick (master-control#72 and #73, pinned since the journal of 2026-10-30); next the Grid
  (Program ABI 11) and rc-worm. When Program ABI 11 lands, the lab's placeholder Program must be
  rebuilt against it before the next bump merges.
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

- 2026-10-31: plans for D1-D6 (chemistry and affect), each waiting on food and a sense of intake; two leads' claims
  corrected from their abstracts (journal of 2026-10-31, § Plans for D1-D6).
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
