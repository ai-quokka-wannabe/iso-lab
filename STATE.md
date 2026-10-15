# State of the Lab

**Last updated:** 2026-10-14, night's end: the comparison space drafted, the pace note filled in part, H11 tried and refuted,
the reference worms' bends measured against the Grid's joint limit, a midbody speed built, and the arena proven to a
first life that replays exactly.

## Where We Are

Phase 0, the benchmark, is under way. The reference data is chosen - the Open Worm Movement Database's N2
recordings ([data/README.md § The Phase 0 Survey](data/README.md#the-phase-0-survey)) - and the first
literature note says how worm behaviour is measured. The comparison space is drafted
([docs/benchmark/FEATURES.md](docs/benchmark/FEATURES.md)): which measures carry over to the Grid's body, and what
must be settled before the benchmark is frozen, food first. The Rust workspace reads the reference skeletons and computes
the first measures - eight-arc posture and a midbody speed, the latter not yet trusted - and the arena is proven to a
first life: a placeholder Program, which undulates only, lived fifteen minutes on the Grid and its world replays
exactly. No creature has been scored.
[docs/PLAN.md](docs/PLAN.md) says what the phases are and why the benchmark comes before any brain.

## Next

Top first. Each item is sized for one run; split it here before starting if it is bigger.

1. **Fill the gaps in the pace note** ([docs/research/pace.md § Gaps, to Be
   Read](docs/research/pace.md#gaps-to-be-read)): a touch-to-reversal latency with frame-level timing (none of the 45
   open papers scanned states one in text, and Liu et al. 2018's figure shows reversals clustering within about a
   second without resolving their onset); what Mohammadi et al. 2013's heat reaction times count from; a thermotaxis
   latency; fitted time constants for tap habituation from primary papers; and the primary sources for lethargus and
   sleep-like quiescence (for satiety, You et al. 2008 is identified, its full text not yet reached). Figures are
   within reach: Europe PMC's supplementary-files service serves a paper's figure images (pace.md § Noxious Heat).
2. **Build the feature-extraction tool** for the chosen dataset, in the lab's Rust workspace (`crates/`), and compute
   the wild-type reference statistics. Explore the data in a notebook first if it helps; the tool itself is Rust.
   Split into runs: (a) the workspace and the MD5 check, done; (b) the eligible manifest, done ([data/README.md § The
   Eligible Manifest](data/README.md#the-eligible-manifest)); and the seeded draw in blocks of 200, done
   ([data/README.md § The Reference Draw](data/README.md#the-reference-draw)); and `fetch-block`, which fetches a
   block and checks every MD5, done; block 1 itself is not yet fetched (see *Blocked*); (c) the skeletons are read
   (crate `features`, by `hdf5-pure`; the published feature table cannot be read by the pure-Rust readers tried, and
   the lab computes its own measures anyway), next the comparison space's measures from them, in seconds and body
   lengths, as the frame rate varies between recordings (25.4 to 32.6 fps seen), and with a rule for recordings of
   other lengths than 15 minutes (5 and 120 minutes seen; journal of 2026-10-14, *How Long the Reference Recordings
   Are*) - a midbody speed is built but fails its check in one record (journal of 2026-10-14, *A Midbody Speed*), and
   the published speed's definition is read and reproduced exactly, and its choices copied at eight arcs fail the
   same check in the same record by 0.0001 (journal of 2026-10-15, *The Midbody Speed With the Published Choices, at
   Eight Arcs*), so the departure lies in the eight arcs themselves; the published locomotion states are reproduced
   exactly from the published speed and length, read at their source and then by tested Rust (`features::states`;
   journal of 2026-10-15, *The Published Locomotion States, Read at Their Source* and *The Locomotion States in
   Rust*); and judged by the states it gives, the speed with the published choices passes a bar set from the spread
   between worms, the first measure fails (journal of 2026-10-15, *The Midbody Speed, Judged by the States It
   Gives*); next, the states' durations and the rate of reversals checked the same way; (d) the reference statistics.
3. **Prove the arena**: in the cloud machine, on Linux alone (the maintainer's ruling), build Master Control and the
   Grid (software Vulkan through Mesa's lavapipe) from `external/`, write the lab's own minimal Program - a
   placeholder brain that only undulates gently, shaped after
   [rc-worm](https://github.com/ai-quokka-wannabe/rc-worm) as the example of a Program - live one short headless
   life on a flat arena, keep its Disk, and extract the same features from it. Split into runs: (a) Master Control
   builds from `external/` and passes its own cheap suite, done; (b) the Grid built with lavapipe, done; (c) the
   placeholder Program, done (`crates/placeholder-worm`); (d) split in two: (d1) one life, its Disk and input log
   kept and replayed, done; (d2) the same features extracted from the Disk, which needs a Disk reader in `features`,
   and the share of refused intents measured (journal of 2026-10-14, *The Arena, Fourth Step*); waits on a ruling (see
   *Blocked*).
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
6. **Plan an experiment for every one of Matej's 98 ideas and hunches** - the maintainer's request - in the format
   of [docs/hypotheses/experiments/README.md](docs/hypotheses/experiments/README.md), whose index shows which are
   planned. A, B, C1-C17 and C19-C24 are done (C18 is under *Blocked*); the rest, one task each, in this order: **D** (13, and answer
   A10's question whether nematodes show anything like play), **E1-E10**, **E11-E20**, **F** (17). The journal
   records the triage, and hunches that survive are promoted into numbered hypotheses or tasks here.
7. **Audit H10's first test by A12's plan**
   ([docs/hypotheses/experiments/A9-A15.md § A12](docs/hypotheses/experiments/A9-A15.md#a12-experiment-hygiene)):
   planted copies prepared by one agent and audited, blind, by another that has read neither that test's review nor
   the preparation.
8. **Audit the Program interface field by field**, by
   [docs/hypotheses/experiments/B.md § B1](docs/hypotheses/experiments/B.md#b1-a-nervous-system-not-a-game-client):
   every sense and action against what a worm can transduce and produce, with a blind planted-field control; the
   delay grades wait on item 1.

## To Revisit

Experiments that failed or came out inconclusive, kept rather than discarded (the maintainer's ruling); each with
the idea for the next attempt.

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

## Blocked

- **Fetching block 1 of the reference draw** (4.87 GB, item 2). Zenodo served the features files at 50 to 350 KB/s on
  2026-10-14 (the journal has the commands), so the block needs from about four hours to more than a day. `fetch-block
  1` resumes by byte, but only on a machine that keeps `data/raw/`; a fresh cloud machine starts empty, so the block
  has to come in one long run (started early in a night, in the background) or on a machine that persists; the night
  of 2026-10-14 fetched 34 of its 200 files, every one MD5-checked, before the fetch was stopped and `data/raw/`
  deleted at the night's end. Next: run it again early on a later night, in
  the background; item 2(c)'s work resumes on what it fetches.
- **Three Dependabot alerts on `main`**, triaged on 2026-10-14
  ([the journal](docs/journal/2026-10-14.md#dependabot-alerts-triaged)). All three sit in markdownlint-cli2's
  dependencies, and none can be fixed cleanly yet. Only `smol-toml` is reachable, through any Markdown file's
  configuration comment, and its worst case is a slow CI job. They wait on the maintainer to dismiss them: `braces`
  and `smol-toml` as a tolerable risk, `katex` as not used. This session's GitHub access cannot write to the
  alerts. Dependabot will propose the fix when upstream releases one.
- **The plan for C18, spreading activation's cliff**
  ([#45](https://github.com/ai-quokka-wannabe/iso-lab/pull/45), left open after three rounds of review). Its model
  as written - 20 steps of spreading with fixed weights - would refute the hunch by construction: the verifier
  simulated it and found a clear cliff whose sharpness the 20-step horizon sets, not the network's size, so the
  size-scaling margin fires anyway. Of the cures as written, the "conserved budget" grows with degree and so is not
  conserved, lateral inhibition only shifts the cliff, and the fan-out penalty pushes it beyond the range of degree
  swept. The pilot is done (journal of 2026-10-14, *C18's Model, Piloted*;
  `notebooks/c18_spreading_pilot.ipynb`): run to a steady state, the pure network's mean curve does not narrow
  steadily with size, likely because draws that die out early mix with draws that fill the network, and no cure
  removed the transition. Next: rewrite the plan with a statistic conditioned on the seed's out-component being
  non-empty, more draws, a degree range of 0.5 to 6 with a finer step near each transition, an operating point per
  variant, a smooth control and the plan's harness controls, and only then write the margins. C13-C17 went up on another path,
  [#47](https://github.com/ai-quokka-wannabe/iso-lab/pull/47), merged.

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
- **Food, and a sense of taking it in** ([master-control#63](https://github.com/ai-quokka-wannabe/master-control/issues/63)
  for food on the floor and the rule for intake; [tron-grid-lite#153](https://github.com/ai-quokka-wannabe/tron-grid-lite/issues/153)
  for the sense of the body's own intake). The reference worms are filmed on food and the Grid has none
  ([docs/benchmark/FEATURES.md § Open Questions Before Freezing](docs/benchmark/FEATURES.md#open-questions-before-freezing)).
  The benchmark is not frozen until food is settled, by these issues or by comparing with foodless worms instead.
- **The worm's bends against the Grid's joint limit** (tron-grid-lite; drafted, not yet filed). At the eight-arc
  resolution 9.8 to 68.6% (median 35.8%, 34 recordings) of the reference worms' skeletonised frames bend beyond the
  chain's 0.9 rad at some joint, against 0.15% beyond Master Control's pi/2. The issue - raise the limit to pi/2, with the caveats - is
  drafted in full in the journal of 2026-10-15, *The Joint-Limit Issue, Drafted*, for the maintainer to file, since
  this session cannot open issues there. Once filed, this line becomes its link.
- **Terrace edges** (master-control; the maintainer takes it up once the lab is set up - file nothing). A
  chain can be thrown when a segment crosses a terrace edge; a fix is in progress. Until it lands, every life
  in this lab is lived on a flat arena.

## Done

- 2026-10-15: the midbody speed judged by the states it gives: with the published choices it passes in all three
  states on seventeen records (paused narrowly), the measure of 2026-10-14 fails in all three.
- 2026-10-15: the locomotion states (forward, backward, paused) in tested Rust, `features::states`, reproducing the
  published states on all 602,314 frames of seventeen records.
- 2026-10-15: the published locomotion states (forward, backward, paused) read at the toolbox's source and reproduced
  exactly from the published speed and length, on all 602,314 frames of seventeen records.
- 2026-10-15: the joint-limit issue for tron-grid-lite drafted in full, from the evidence already measured, for the
  maintainer to file.
- 2026-10-15: which end is the head: Tierpsy names it mostly by movement, so the head's excess of bends is built in to a degree;
  no frame-to-frame swap in seventeen records (a planted-swap control catches all 474), 62 candidates across 18,105 gaps.
- 2026-10-15: the midbody speed with the published choices at eight arcs: 1007405 fails the check again (r 0.9499
  against 0.95), sixteen records pass; the hunch that the definition's choices cause the departure is refuted.
- 2026-10-15: the published midbody speed read at its source and reproduced exactly (correlation 1.00000 on two
  records); the stored skeletons were already smoothed along their length by Tierpsy, not in time.
- 2026-10-14: C18's model piloted: no cure removed the transition, and the plan's statistic would measure early
  die-out as much as sharpness; the plan is to be rewritten from it.
- 2026-10-14: satiety quiescence's source found (You et al. 2008), but its abstract gives no timing and its full text
  could not be reached (Europe PMC's HTTP 500, PMC's CAPTCHA, not got around); the gap stays, now with its source.
- 2026-10-14: averaged over neighbouring frames, the bends beyond 0.9 rad barely fall (0.6 to 1.8% per joint at a
  tenth of a second), on 34 recordings; the head's ratio within 0.002 of the body's, no evidence either way.
- 2026-10-14: the arena's first life: the placeholder lived all 28 800 ticks of a fifteen-minute life on lavapipe,
  and Clu replayed its log with all 902 hashes agreeing; 9 intents (0.03%) arrived a tick late and were refused.
- 2026-10-14: the midbody speed's review taken: no speed where the body has no direction, the notebook pinned to its
  records and numpy in step; the check's result unchanged.
- 2026-10-14: a midbody speed at the eight-arc resolution; its check against the dataset's own speed fails in one record
  of sixteen (correlation 0.947 against the 0.95 required), where the 49-point version agrees; not yet trusted.
- 2026-10-14: the placeholder Program, `crates/placeholder-worm`: an eight-segment chain undulating at 0.18 Hz and 0.3 rad,
  its C interface checked against the Grid's published layout; the Grid's loader lists it as usable.
- 2026-10-14: the Grid builds from `external/` with the Vulkan SDK and runs on lavapipe alone: its three verifications and
  22 tests pass, and its reference render is recorded as the lab's first arena reference (journal).
- 2026-10-14: Master Control builds in place from `external/` (the lab's Cargo workspace now excludes it) and passes its
  own cheap suite, 92 tests, at the pinned commit; the arena's first step.
- 2026-10-14: smoothing the skeletons along their length lowers the head's share beyond 0.9 rad no more than the body's,
  the tail's most; the tips stay unsmoothed; 31% of frames (median) still bend beyond it at some joint.
- 2026-10-14: experiment plans for hunches C19-C24 ([C13-C24.md](docs/hypotheses/experiments/C13-C24.md)); D next.
- 2026-10-14: experiment plans for hunches C13-C17 ([C13-C24.md](docs/hypotheses/experiments/C13-C24.md)), after four
  reviews of its pull request; C18 parked, C19-C24 next.
- 2026-10-14: per joint, the bends beyond the Grid's limit are greatest at the head (median 13% of frames) and fall along
  the body (medians 3.2 to 4.5% at joints 4 to 6); a smoothing check next.
- 2026-10-14: the eight-arc posture in Rust, and a first measurement: in 12 reference recordings, 31 to 46% of
  skeletonised frames bend beyond the Grid's 0.9 rad at some joint; a per-joint breakdown next.
- 2026-10-14: the pace note gains quiescence after stress (Hill et al. 2014): suppressed up to an hour after a heat shock,
  a peak 20 minutes after cold; satiety quiescence still a gap.
- 2026-10-14: the findings kept for the ladder: [docs/research/findings.md](docs/research/findings.md), a ledger of what
  the lab has tested (failures beside successes) and the lessons of method; H12, a plain worm brain, as a hunch.
- 2026-10-14: H11 v5, a pilot of binary keys for sharp basins: the settling still no help (-0.22 [-1.07, +0.64] against
  its ablation, 3 seeds, one collapsed).
- 2026-10-14: H11 v4, a DeltaNet that settles its cue, derived from v3: 0.75 against DeltaNet's 0.94 at the trained
  noise, its settling no help at any level; in none of four tests did the settling earn its place.
- 2026-10-14: H11 v3, recall from a noisy cue: v2's hybrid recalls 0.59 against DeltaNet's 0.94 and softmax's 0.98 at
  the trained noise, and its settling harms; v4 aims at softmax's lead over DeltaNet (0.98 against 0.94), whose cause is untested.
- 2026-10-14: H11 v2, a pilot: written by the delta rule in four heads, the hybrid recalls 0.89 (v1: 0.18, another
  protocol), but it does not beat a DeltaNet memory of the same size (-0.11 [-0.25, +0.03] at 3 seeds).
- 2026-10-14: H11, a Hebbian Hopfield transformer (the maintainer's hunch), refuted at the toy level: 18% recall
  against 99.8% for linear attention of the same memory, the settling itself the harm; frozen, the matched Hebbian memory
  still recalled 92% in streams three times the trained length.
- 2026-10-14: the lengths of 15 reference recordings: 13 last 15 minutes, one 5 and one 120; the benchmark needs a
  rule for unequal lengths.
- 2026-10-14: the reference skeletons read in Rust (crate `features`), identical to h5py's on 13 files; the published
  feature table is not readable by the pure-Rust readers tried, and the lab will compute its own measures.
- 2026-10-14: `fetch-block`, which downloads a block of the reference draw and accepts each file only by its MD5,
  resuming by byte; a run fetched part of block 1, every file checked.
- 2026-10-14: the reference draw: the 1646 eligible records in one order fixed by seed 20261014 (pre-registered and
  pushed first), in blocks of 200; block 1, 4.87 GB, is the development reference.
- 2026-10-14: the eligible manifest of the N2 reference records, written by a Rust tool (`list-n2`): 1792 records
  listed and 1646 eligible, the counts predicted from the survey.
- 2026-10-14: the lab's Rust workspace (toolchain 1.98.0 and its furniture taken from master-control and link, a Rust
  job in CI under `CI Success`) and its first crate, `fetch`, which checks a file against its record's MD5.
- 2026-10-14: Liu et al. 2018's touch responses read from a figure: classified reversals peak about 0.25 s after a
  tap, but their classifier's timing is uncertain by up to 2.7 s and onsets are not resolved, so the touch latency
  stays a gap.
- 2026-10-14: Ghosh et al. 2012, cited by Mohammadi et al. for the response time, read in full: it does not define a
  response latency either; the heat reaction times' reference moment stays a gap.
- 2026-10-14: heat reaction times read from Mohammadi et al. 2013, Figure 2c, by pixel: about 0.33 to 0.25 s for a
  head stimulus, falling with laser current; what they count from is not stated.
- 2026-10-14: Roman et al. 2023's fitted thermal-preference time scales added to the pace note (avoidance about 45-83
  min, habituation about 20 min and known only to a factor of about two, read as hours from a unitless table); Europe
  PMC's supplementary-files service found to serve figure images.
- 2026-10-14: the pace note's gaps, a third pass: thermotaxis memory and dispersal over hours without food, AFD's
  threshold following a hold of at least 15 min, tap habituation's protocol timescales (a review); no fitted τ yet.
- 2026-10-14: the pace note's gaps, a second pass: receptor latency to nose touch, a smell avoidance latency, the
  feeding and lethargus time scales; none of the 45 open papers scanned gives a touch-to-reversal latency in text.
- 2026-10-14: the pace note, a first pass ([docs/research/pace.md](docs/research/pace.md)): latencies to touch,
  heat and light, the body's rhythm and the time constants of four states, from text only; the gaps are item 1.
- 2026-10-14: the comparison space drafted ([docs/benchmark/FEATURES.md](docs/benchmark/FEATURES.md)): which
  measures carry over to the Grid's body and in which units; roaming and dwelling shares fit in 15 minutes, their
  durations do not; five open questions before freezing, food first.
- 2026-10-13: the five notes of #21's last verification written into C8 and C12 of
  [the C1-C12 plans](docs/hypotheses/experiments/C1-C12.md).
- 2026-10-13: the three notes of #20's last verification carried into
  [the C1-C12 plans](docs/hypotheses/experiments/C1-C12.md) (C8, C10, C12).
- 2026-10-13: experiment plans for hunches C1-C12
  ([docs/hypotheses/experiments/C1-C12.md](docs/hypotheses/experiments/C1-C12.md)).
- 2026-10-13: experiment plans for hunches B1-B9
  ([docs/hypotheses/experiments/B.md](docs/hypotheses/experiments/B.md)).
- 2026-10-13: experiment plans for hunches A9-A15, completing section A
  ([docs/hypotheses/experiments/A9-A15.md](docs/hypotheses/experiments/A9-A15.md)).
- 2026-10-13: experiment plans for hunches A1-A8, with the plan format and an index of all 93
  ([docs/hypotheses/experiments/](docs/hypotheses/experiments/README.md)).
- 2026-10-13: H10, the maintainer's quantum-like brain, written as a hunch; its first test (phasor against real
  memory) refuted it at that level, and the test is kept under *To Revisit*.
- 2026-10-13: the literature note on how worm behaviour is measured,
  [docs/research/behaviour-measures.md](docs/research/behaviour-measures.md).
- 2026-10-13: the dataset survey - the Open Worm Movement Database's N2 records chosen for Phase 0, selected by
  their metadata; the CeMEE data set aside ([data/README.md § The Phase 0 Survey](data/README.md#the-phase-0-survey)).
- 2026-10-13: the lab opened - repository, rules ([.claude/CLAUDE.md](.claude/CLAUDE.md)), plan, hypotheses,
  journal and dataset registry.
