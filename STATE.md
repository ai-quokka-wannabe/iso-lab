# State of the Lab

**Last updated:** 2026-10-14: the comparison space drafted, the pace note filled in part, and the Rust workspace begun.

## Where We Are

Phase 0, the benchmark, is under way. The reference data is chosen - the Open Worm Movement Database's N2
recordings ([data/README.md § The Phase 0 Survey](data/README.md#the-phase-0-survey)) - and the first
literature note says how worm behaviour is measured. The comparison space is drafted
([docs/benchmark/FEATURES.md](docs/benchmark/FEATURES.md)): which measures carry over to the Grid's body, and what
must be settled before the benchmark is frozen, food first. The Rust workspace exists, with one crate that checks downloads; no feature is computed yet and no
creature has been scored.
[docs/PLAN.md](docs/PLAN.md) says what the phases are and why the benchmark comes before any brain.

## Next

Top first. Each item is sized for one run; split it here before starting if it is bigger.

1. **Fill the gaps in the pace note** ([docs/research/pace.md § Gaps, to Be
   Read](docs/research/pace.md#gaps-to-be-read)): a touch-to-reversal latency with frame-level timing (none of the 45
   open papers scanned states one in text, and Liu et al. 2018's figure shows reversals clustering within about a
   second without resolving their onset); what Mohammadi et al. 2013's heat reaction times count from; a thermotaxis
   latency; fitted time constants for tap habituation from primary papers; and the primary sources for lethargus and
   sleep-like quiescence. Figures are within reach: Europe PMC's supplementary-files service serves a paper's figure
   images (pace.md § Noxious Heat).
2. **Build the feature-extraction tool** for the chosen dataset, in the lab's Rust workspace (`crates/`), and compute
   the wild-type reference statistics. Explore the data in a notebook first if it helps; the tool itself is Rust.
   Split into runs: (a) the workspace and the MD5 check, done; (b) the eligible manifest, done ([data/README.md § The
   Eligible Manifest](data/README.md#the-eligible-manifest)); and the seeded draw in blocks of 200, done
   ([data/README.md § The Reference Draw](data/README.md#the-reference-draw)); then fetch block 1 by record id with
   retries, checking every file with `fetch::verify_md5`; (c) read the features file, by a pure-Rust HDF5 reader if
   one reads it (`hdf5-pure`, `hdf5-reader` and `rust-hdf5` exist; none tried yet), or else from the WCON JSON, and
   compute the comparison space's measures; (d) the reference statistics.
3. **Prove the arena**: in the cloud machine, on Linux alone (the maintainer's ruling), build Master Control and the
   Grid (software Vulkan through Mesa's lavapipe) from `external/`, write the lab's own minimal Program - a
   placeholder brain that only undulates gently, shaped after
   [rc-worm](https://github.com/ai-quokka-wannabe/rc-worm) as the example of a Program - live one short headless
   life on a flat arena, keep its Disk, and extract the same features from it.
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
6. **Plan an experiment for every one of Matej's 93 ideas and hunches** - the maintainer's request - in the format
   of [docs/hypotheses/experiments/README.md](docs/hypotheses/experiments/README.md), whose index shows which are
   planned. A, B and C1-C12 are done; the rest, one task each, in this order: **C13-C24**, **D** (13, and answer
   A10's question whether nematodes show anything like play), **E1-E10**, **E11-E20**, **F** (12). The journal
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

## Blocked

- **Three Dependabot alerts on `main`**, triaged on 2026-10-14
  ([the journal](docs/journal/2026-10-14.md#dependabot-alerts-triaged)). All three sit in markdownlint-cli2's
  dependencies, and none can be fixed cleanly yet. Only `smol-toml` is reachable, through any Markdown file's
  configuration comment, and its worst case is a slow CI job. They wait on the maintainer to dismiss them: `braces`
  and `smol-toml` as a tolerable risk, `katex` as not used. This session's GitHub access cannot write to the
  alerts. Dependabot will propose the fix when upstream releases one.

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
- **Terrace edges** (master-control; the maintainer takes it up once the lab is set up - file nothing). A
  chain can be thrown when a segment crosses a terrace edge; a fix is in progress. Until it lands, every life
  in this lab is lived on a flat arena.

## Done

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
