# State of the Lab

**Last updated:** 2026-10-28 (journal of 2026-10-28): gr-fec's toy worm - the composition law tested in more worlds
and with three levers; the hunger gate's harm traced with branched lives, in one layout (75000), a chain to well-mapped
worms not finding their way back to the band over a bumpy map, each link measured and their causal join a reading; and
a designed fix, steering by the map's global contrast, that helps in the two cold layouts tested but is not yet shown
safe in the two warm ones. Before it, on 2026-10-27, eight verified experiments, a tidy and a world bump.
The benchmark's choice of rule for the lab's speed is still the next step for Phase 0.

## Where We Are

Phase 0, the benchmark, is under way. The reference data is chosen - the Open Worm Movement Database's N2
recordings ([data/README.md § The Phase 0 Survey](data/README.md#the-phase-0-survey)) - and the first
literature note says how worm behaviour is measured. The comparison space is drafted
([docs/benchmark/FEATURES.md](docs/benchmark/FEATURES.md)): which measures carry over to the Grid's body, and what
must be settled before the benchmark is frozen, food first. The Rust workspace reads the reference skeletons and computes
the first measures - eight-arc posture and a midbody speed, the latter not yet trusted - and the arena is proven to a
first life: a placeholder Program, which undulates only, lived fifteen minutes on the Grid and its world replays
exactly. No creature has been scored.
[docs/PLAN.md](docs/PLAN.md) says what the phases are and why the benchmark comes before any brain. Beside the benchmark, the lab takes apart the
maintainer's toy worm ([docs/research/gr-fec.md](docs/research/gr-fec.md) §§ 9-11) to learn how to design a brain
rather than search for one; what carries up the ladder is kept in [docs/research/findings.md](docs/research/findings.md).

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
   block and checks every MD5, done; block 1 itself was fetched in full on 2026-10-15 ([data/README.md § Fetching a
   Block](data/README.md#fetching-a-block) gives the rate), and deleted at the session's end, a fresh machine starting empty; (c) the skeletons are read
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
   between worms and the first measure fails, but the lab's states are biased one way - about 6% more frames in no
   state, so lower shares in every record (journal of 2026-10-15, *The Midbody Speed, Judged by the States It
   Gives*), and those frames are lost not to missing skeletons but to the speed's values - often a little above the
   paused band where the worm is nearly still, its runs breaking where the published ones bridge (journal of
   2026-10-15, *Why the Lab's States Leave More Frames Undefined*); a midbody of the three middle ends, to test a
   one-point jitter as the cause, refutes it - the excess rises (journal of 2026-10-15, *A Midbody of Three Ends*);
   the lab's length equals the published and its speed is lower at the median on nearly-still frames, so it is not an
   upward shift of the speed; inside the published paused runs the lab's speed leaves the paused band for a
   quarter-second or more in about one gap in seven, fewer but longer excursions than the published speed's (journal
   of 2026-10-15, *How the In-Band Frames Fall in Time*); the edges and the middle each hold
   about half its undefined paused frames (journal of 2026-10-15, *Where in the Paused Runs the Lab's Undefined Frames
   Lie*); on nearly-still frames the lab's speed varies about a third as much as the published from frame to frame
   (journal of 2026-10-15, *How Much the Two Speeds Vary From Frame to Frame on Nearly-Still Frames*); the published speed,
   smoothed to at least the lab's smoothness by a five-frame moving mean, leaves a median 83% as many paused frames undefined as
   the lab's states, so smoothness alone is enough to leave most of them undefined, though not all the same frames (journal of
   2026-10-15, *Does Smoothing the Published Speed Leave as Many Frames Undefined?*); of the eight arcs' two
   changes the midbody alone carries the smoothness, the point half-way by length in place of the centroid of points 21
   to 29 (journal of 2026-10-15, *Which of the Eight Arcs' Two Changes Smooths the Speed*); and that centroid slides
   along the body because the stored points' spacing varies from frame to frame - resampled to equal spacing, the
   toolbox's speed is as smooth as the lab's (journal of 2026-10-15, *Does the Centroid of Points 21 to 29 Slide Along
   the Body?*), all three replicated on block 1's further 178 records (journal of 2026-10-15, *The Smoothness Findings on
   the Rest of Block 1*); and, counted on those records, the toolbox's speed on equally spaced skeletons leaves a median
   92% as many published-paused frames undefined as the lab's states (journal of 2026-10-15, *Do the Undefined Frames
   Follow From the Spacing?*); next, write down, with its reason, whether the benchmark applies the published rule unchanged to the
   lab's speed, before the reference statistics are frozen - then the
   states' durations and the rate of reversals; (d) the reference statistics.
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
6. **Plan an experiment for every one of Matej's 128 ideas and hunches** - the maintainer's request - in the format
   of [docs/hypotheses/experiments/README.md](docs/hypotheses/experiments/README.md), whose index shows which are
   planned. A, B, C1-C17 and C19-C24 are done (C18 is under *Blocked*); the rest, one task each, in this order: **D** (13, and answer
   A10's question whether nematodes show anything like play), **E1-E10**, **E11-E20**, **F** (17), **G** (30, from
   the gr-fec sandbox; [docs/research/gr-fec.md](docs/research/gr-fec.md) has its numbers). The journal
   records the triage, and hunches that survive are promoted into numbered hypotheses or tasks here.
7. **Audit H10's first test by A12's plan**
   ([docs/hypotheses/experiments/A9-A15.md § A12](docs/hypotheses/experiments/A9-A15.md#a12-experiment-hygiene)):
   planted copies prepared by one agent and audited, blind, by another that has read neither that test's review nor
   the preparation.
8. **Audit the Program interface field by field**, by
   [docs/hypotheses/experiments/B.md § B1](docs/hypotheses/experiments/B.md#b1-a-nervous-system-not-a-game-client):
   every sense and action against what a worm can transduce and produce, with a blind planted-field control; the
   delay grades wait on item 1.
9. **gr-fec's worm by equation** - the maintainer's aim (2026-10-26): understand [gr-fec's worm](docs/research/gr-fec.md)
   well enough to design it for a chosen behaviour, with every decision auditable, instead of evolving it. Done on
   2026-10-26 (that night's journal): the turning rule by equation; exact audits of e45's decisions, their noise and
   its learned values; a smoothed hunger reading; CMA-ES; the optimised worm's genes swapped back; and two of its gene
   values moved onto e45, recovering 95% of its worst-case lead. Next, one run each:
   - (a) done on 2026-10-27: they do not need each other - the weak nose keeps e45 off decoys, slow learning steadies
     its map (and keeps it off decoys too, less), and either alone recovers most of the gap; next, the warm world,
     where the weak nose's effect is small and mixed;
   - (b) done on 2026-10-27: gr-fec's steering gain is real and survives the corrected body (+0.19 to +0.21 in two cold
     layouts), but the rule costs in both rich layouts and one warm one, and the port fell short in cold layouts
     where the rule gains (lost in one, flat in the other); a hunger gate removes the rich-world cost but does worse
     than never steering in two Hardland layouts; in one, 75000, its gate opens and shuts a median 88 times an hour,
     but a gate with hysteresis, switching under once an hour, leaves the harm (flicker refuted); steering only while
     fed gains in all four Hardland layouts but keeps a rich-world cost the hunger gates remove; on 2026-10-28, in
     75000 hungry steering points towards cold (the food's side) less often than fed steering (0.48 against 0.57),
     while hunger barely moves the map's peak (the moved-peak hunch refuted); next, score each decision towards the food
     rather than towards cold, and look for a local rise in the hungry map; on 2026-10-28 scored towards the food
     the gap grew (0.48 against 0.71; position refuted), the local view carries at most a small part, and post hoc 87%
     of hungry decisions come from the quarter of worms whose map peaks off the band, who live off it (a post hoc
     reading, not a cause); on 2026-10-28 a test of
     the gate's cost by map was uninformative: it paired worms by index across lives that had diverged (post hoc,
     regression to the mean); with lives branched from one
     shared state at h24 and the gate switched on there, the reading was refuted in 75000 - on-band worms lose more
     (−0.170 against −0.093) - and, weakly, in 23001, where a gate from h24 barely costs anyone; for an on-band worm a gate from
     h24 halves its time on the band and cuts its intake by about 28%, but its steering inside the band is not detectably
     biased away (G3 fails); outside the band its steering leans slightly
     away from the band (0.481; O1 holds), as off-band worms' does, and is not shown to follow its map's peak (O2 fails,
     a scoring convention possibly all of it); it leaves the band less often under the
     gate but returns less than half as often (X2 holds), and off the band its steering is not shown to point to
     the band or its map's peak (0.481, 0.495); the learned map, read as the greedy rule reads it,
     does not point home from far concepts, beyond a weak net lean - it is bumpy, not flat (S1, S2 hold: 0.556 and 0.547 point to the peak);
     gated steering by the map's global
     contrast lifts the gated worm by 0.17 in 75000, to near never-steering (Q1 holds, Q2 narrowly fails), but not in
     23001 (Q1 fails), where the harm depends on the first day; the rule also drops the greedy 0.02 margin, a confound;
     elsewhere it keeps the rich-world fix and does better
     in cold 23000 (+0.074, descriptive; the margin confound applies), but may cost in warm 75001 (W1 fails there); with the greedy margin kept the global
     rule still beats greedy gating in 75000 (+0.213, M1 holds) and ends above never steering (+0.040, descriptive, 75000 only); in the other five layouts
     the margin rule keeps the rich fix and helps in cold 23000 (+0.091, descriptive) but fails V1 in both warm layouts; next, the
     fifteen fresh layouts to test the cold-band hunch, and what the gate does in 23001's first day; where off-band maps
     come from: on 2026-10-28 the first meal is not a switch that sets the map (imprinting refuted; a weaker lean not
     excluded), but worms whose map ends off the band ate 88-89% of their first day's food at the smelly patches,
     against 42-48% (an association; which comes first is open); next, the timing of meals against the map's turning;
   - (c) done on 2026-10-27: a sound CMA-ES from two seeds reaches last night's level and again chooses slow learning
     and a weak nose (K1, K2 hold); neither run converged, and all start from e45's median on one panel;
   - (d) a first rule done on 2026-10-27: in the cold world, multiplying the two levers' remaining failures predicts a
     pair's score from each lever alone (error 0.019, against 0.059 for adding; on the eight fresh cells only just
     better), while mild settings do better than the law; on 2026-10-28 it held in the warm world and for `eta` with
     the turning rule's `tau` (errors 0.012, 0.013), but there one lever barely moves the score and ignoring it does as
     well (post hoc), and the hunch that a weak nose also cleans the map held on its registered point estimates but
     not post hoc (both intervals include zero); next, a
     second pair of two strong levers, and why mild pairs beat the law; on 2026-10-28 the law held for `eta` and
     `g_odour` in two fresh worlds where both levers work (errors 0.018 and 0.011, beating adding and either lever
     alone), and the mild-pair synergy was not detected there (though cold 82003 cannot exclude it); next, other pairs of strong
     levers, and three at once; on 2026-10-28 with `roam` as a third lever the law held (error 0.015) but the test
     was uninformative, `roam` alone moving the score by 0.011; post hoc, raising it cost energy in two of nine pairs (up to
     0.06), not surviving a correction for nine comparisons; next, a third lever that acts alone, and roaming's interaction as its own test.

   Once a calculation matters to anything the lab builds on, it moves into tested Rust.

10. **Move `external/` to the food world**: link protocol v12 (food, smell in the letter), master-control's food and
    smell and its risers with a top, and tron-grid-lite's documents, all merged upstream on 2026-10-29 after that
    night's pin. This changes the world: measure the arena before and after, check the placeholder against protocol v12,
    and say what moved (journal of 2026-10-29, § The World Moved to Its Latest Main Again, Measured Before and After).

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
- **H13, the proof of concept on synthetic worms** (2026-10-17; [round 1](notebooks/h13_modular_poc.ipynb),
  [rounds 2 to 4](notebooks/h13_rounds_2_to_4.ipynb); the journal of that date has the numbers). The grind's decisions
  work: ablation told real modules from a decoy in every run (round 1's under the chosen reading of the noise), and the
  fit after pruning the decoy was better each time (one comparison per run, an observation); the held-out check caught
  a memorising model once, in round 1. Recovering the
  hidden brain by training it as a whole does not yet: with 12 behaviour statistics it is refuted, and the fault points to
  the statistics more than the search (one parameter barely seen, two trading); five more statistics recover every
  real parameter in one fit, and the whole loop of fit, ablate, prune and refit then recovers the brain for one
  optimiser seed of three - refuted by the rule set beforehand. Round 5 targeted the two obstacles left (bout
  lengths from complete pre-tap bouts; ten finer tap bins), each alone and together, five seeds each: refuted - both
  together pass for 3 of 5, neither change earns its credit, and the rate of leaving roaming is too high in all 20
  refits, not (or not only) from the bouts' edges. A diagnosis found the fitter's noise a real part: with 128
  worms per candidate the noise-free fit recovers `k_rd` (1.02 times), and on DEV the loop passes for 4 of 5, but
  `k_rd` is within 0.2 for only 2. A precision test on twelve fresh populations then found the fitter's noise the larger
  part (by its rule; the data's share is not resolved): the fit follows the optimiser's seed (seed 8 above seed 7 in
  all 12), and every fit shares one simulation noise stream; the bias, 1.15 times, holds for that stream and those
  two seeds (seed 7 alone 1.03). Eight seeds on a noise-free target, with the shared stream and with
  streams of their own, then showed the fitter biased on average (1.15 times, interval excluding zero, either
  stream); averaging removes the scatter, not the bias, and is not shown to suffice. A polish test then weakened the smoothing
  reading, by its pre-registered rule: the search's spread on `k_rd` had already collapsed to its floor (0.02) by
  generation 120 in all 16 fits. Post hoc, 40 more generations moved the estimate towards the truth in 7 of 8 seeds
  (mean -0.040, interval including zero). A long run to 480 generations
  then failed its rule (L1); post hoc, it brought the outlying seeds in (8 of 8 within 0.2 on `k_rd`, sd 0.124 to
  0.043), but the fall in the mean (1.15 to 1.05 times) is not shown by a paired interval, and a residual excluding
  zero remains. Then round 4's
  whole loop on DEV at 128 worms and 360 generations **passed its rule**: criterion 1 for 5 of 5 seeds, `k_rd` within
  0.2 for 5 of 5, though still biased high (about 1.17 times) and with habituation's two parameters trading in two.
  On five fresh
  populations the loop then passed criterion 1 for 5 of 5 (`k_rd` within 0.2 for 4), and no bias in `k_rd` was
  shown (+0.064 in log, interval -0.056 to +0.184, which also covers DEV's +0.153); habituation's `dep` and `tau` were
  not both within 0.2 in three of five, trading in two. A tap protocol
  with a rest and three probe taps then failed to separate them: both within 0.2 for 3 of 8 seeds on the near
  noise-free target, against 5 of 8 before. A profile of the distance then showed it nearly flat along
  the dep-tau trade under both protocols (one to two hundred times flatter than across it): the statistics see one
  combination far more clearly than the two apart; the rest protocol sharpens the trade about 1.4 times, not 2 (P1
  fails).
  On 2026-10-22, three protocols with taps at two intervals
  did no better (the trade cost stayed at 0.0004 to 0.0007 against a bar of 0.0022), and a deterministic calculation
  showed why for the five designs tried, all at intervals of 10 s or more: a yes-or-no tap assay then needs 170 or
  more worms, against DEV's 30. A short-interval train - 60 taps every 2 s, then three probes - then lifted the trade
  cost about thirteenfold, and on the near noise-free target the fit recovered `dep` and `tau` within 0.2 in 8 of 8
  seeds (largest error 0.105). But in the whole loop on five fresh 30-worm populations it did not carry: both within
  0.2 for 1 of 5 (old protocol 2 of 5), criterion 1 for 3 of 5 (G1 and G3 fail). Next: the same loop under that
  protocol on fresh populations of 60 or more to fit, to see whether the gain arrives as sampling noise falls. For the maintainer:
  whether a 2 s tap interval is right for the benchmark's assay (not yet checked against the literature). For the
  benchmark: its comparison space should hold state-conditioned rates, bout lengths and whole distributions, not only
  means.

## Blocked

- **Three Dependabot alerts on `main`**, triaged on 2026-10-14
  ([the journal](docs/journal/2026-10-14.md#dependabot-alerts-triaged)). All three sit in markdownlint-cli2's
  dependencies, and none can be fixed cleanly yet. Only `smol-toml` is reachable, through any Markdown file's
  configuration comment, and its worst case is a slow CI job. They wait on the maintainer to dismiss them: `braces`
  and `smol-toml` as a tolerable risk, `katex` as not used. This session's GitHub access cannot write to the
  alerts. Dependabot will propose the fix when upstream releases one.
- **The plan for C18, spreading activation's cliff**
  ([#45](https://github.com/ai-quokka-wannabe/iso-lab/pull/45), merged on 2026-10-22 at the maintainer's request,
  with the plan marked as parked in [C13-C24.md](docs/hypotheses/experiments/C13-C24.md#c18-spreading-activation-has-a-cliff)). Its model
  as written - 20 steps of spreading with fixed weights - would refute the hunch by construction: the verifier
  simulated it and found a clear cliff whose sharpness the 20-step horizon sets, not the network's size, so the
  size-scaling margin fires anyway. Of the cures as written, the "conserved budget" grows with degree and so is not
  conserved, lateral inhibition only shifts the cliff, and the fan-out penalty pushes it beyond the range of degree
  swept. The pilot is done (journal of 2026-10-14, *C18's Model, Piloted*;
  `notebooks/c18_spreading_pilot.ipynb`): run to a steady state, the pure network's mean curve does not narrow
  steadily with size, likely because draws that die out early mix with draws that fill the network, and no cure
  removed the transition. Tried again on 2026-10-15 (journal, *C18's Plan, Attempted Again*): a width or
  steepest-slope test cannot tell a continuous transition from a smooth change - the giant component's 10-90% width
  converges to about 1.49, top taken at d = 6 - and the pilot's own model may jump near d = 2.5 rather than rise.
  Next: settle what the hunch's "cliff" means (Shrager, Hogg and Huberman 1987, not yet read), then plan a test of the
  reached fraction below and above the threshold (falling as 1/N below, holding above), with the giant component and a
  jump-shaped model as controls, the pilot's conditioned statistic and more draws. C13-C17 went up on another path,
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

- 2026-10-29: four `rustls` advisories in the fetch tool's TLS (1 moderate, 3 low), and three on its root certificates, patched by `cargo update`; OSV clean.
- 2026-10-29: `external/` moved to each repository's `main` as fetched at about 20:20 UTC (link `c8412b8`, master-control `3ba732e`,
  tron-grid-lite `c5b2815`); B1-B3 hold, render digest unchanged; the food world since merged upstream is item 10.
- 2026-10-29: the night's audits: spelling and style clean; seven coherence findings fixed (VISION today, CI today, gr-fec § 10, counts); STATE.md's slimming next.
- 2026-10-29: the linter's vulnerable `katex` and `re2js` overridden to patched releases; `npm audit` clean, lint unchanged.
- 2026-10-28: the margin rule elsewhere: keeps the rich-world fix (V2 holds), helps in cold 23000 (+0.091, descriptive), but new harm is not
  excluded in either warm layout (V1 fails in 23001 and 75001) - a cold-band hunch, two layouts a side.
- 2026-10-28: the global rule with the greedy margin in 75000: +0.213 over greedy gating (M1 holds) and +0.040 over never
  steering (descriptive) - the dropped margin does not explain the lift; the global reading carries at least part of it.
- 2026-10-28: the global rule elsewhere: keeps the rich-world fix (W2 holds), helps in cold 23000 (+0.074 over greedy gated),
  but new harm in warm 75001 is not excluded (−0.043, W1 fails there).
- 2026-10-28: hunger-gated steering by the map's global contrast (towards the peak's side): +0.167 over the greedy gated
  brain in 75000, near never steering (Q1 holds, Q2 fails narrowly); no help in 23001 (Q1, Q2 fail).
- 2026-10-28: does the hungry map point home? Read locally, a well-mapped worm's map points towards its peak from only 0.556
  of far concepts (S1, S2 hold); it is bumpy (39% tilt away), not flat (5%), and other worms' maps are alike.
- 2026-10-28: leaving the band and coming back in 75000, gate from h24: well-mapped worms leave it less often (X1 fails the
  other way) but return less than half as often (X2 holds, 2.23 against 5.87 per hour out); why is untested.
- 2026-10-28: where a gated well-mapped worm steers off the band in 75000: slightly away from the band (0.481, O1 holds),
  as every worm does; not shown to follow its map's peak (0.480, O2 fails, partly or wholly a scoring convention).
- 2026-10-28: what a hunger gate from h24 changes for a well-mapped worm in 75000: half the time on the band (G1), about
  28% less food (G2), but its steering inside the band is not detectably biased away (G3 fails).
- 2026-10-28: who loses under a hunger gate switched on at h24, from one shared life: in 75000 on-band worms lose more
  (+0.077 off minus on; B2 fails the other way), refuting the reading; in 23001 (B4 fails) weakly, the late gate costing little.
- 2026-10-28: where a worm's map comes from: the first meal is not a switch that sets it (I1 fails, +0.09 and +0.12); worms
  with off-band maps ate most of their first day's food at the smelly patches (I2 holds, +0.47 and +0.41), cause unordered.
- 2026-10-28: who loses under the hunger gate, by map: W1 held and W2-W4 failed as registered, but post hoc the design
  could not answer - worms paired by index had diverged, so the class gap is regression to the mean. Uninformative.
- 2026-10-28: three levers at once (`eta`, `g_odour`, `roam`) in cold 82003: the OR law's error is 0.015 (R1 holds), but
  `roam` alone barely acts (0.011), so R2-R3 are uninformative; post hoc, raising `roam` cost energy in two of nine pairs, not shown to differ from its effect alone.
- 2026-10-28: hungry steering scored towards the food in 75000: 0.483 against 0.707 fed (F1, position, fails); towards
  the map's peak 0.637 against 0.686 (F2 holds, narrowly). Post hoc, hungry decisions come mostly from worms whose maps peak off the band.
- 2026-10-28: the OR composition law for slow learning and a weak nose in two fresh worlds (cold 82003, warm 82104):
  S1-S3 hold in both (errors 0.018, 0.011; better than adding and than either lever alone); S4, the mild-pair synergy, fails.
- 2026-10-28: where hungry steering points in layout 75000: towards cold 0.483 of the time, fed 0.572 (D1 holds);
  the value peak is the same concept at hunger 1 and 0.2 in about 98% of samples (D2 fails): the peak does not explain the harm.
- 2026-10-28: the OR composition law in the warm world and for `eta` with `tau` (W1, T1 hold: 0.012, 0.013; W2
  uninformative, T2 fails); post hoc, ignoring the weak lever does as well there, so only the cold `eta`-nose pair tests it.
- 2026-10-27: value-steering only while fed: +0.14 over never steering in layout 75000 (F1, F2 hold), and above it in all four
  Hardland layouts, in two of which steering only while hungry harms; but it costs up to −0.057 in rich layouts, which hunger
  gates avoid.
- 2026-10-27: `external/` moved to each repository's latest main (link `5a1c6b0`, master-control `2e809e2`, tron-grid-lite
  `081503c`); the arena's reference measurements are unchanged at old and new pins (B1-B3), render digest as on 2026-10-14.
- 2026-10-27: a hunger gate with hysteresis switches under once an hour (Y1) and keeps the rich-world fix (Y3), but the
  harm in 75000 and 23001 stays (Y2, Y4 fail): the flicker hunch is refuted.
- 2026-10-27: why the hunger gate harms in layout 75000: not a cycle (C1 fails), not a worse map (C3 fails); of three
  candidates only flicker survives (C2) - the gated worm's gate opens and shuts a median 88 times an hour. A lead, not a cause.
- 2026-10-27: a hunger gate on value-steering removes its rich-world cost (G2 holds, to within 0.005) but fails in the cold (G1):
  in one cold layout and one warm the gated worm does worse than never steering (−0.17, −0.10). Why is open.
- 2026-10-27: a sound CMA-ES (no injected mean) from two seeds beats e45's elites by +0.06 to +0.07 on gr-fec's panel and
  +0.23 to +0.24 on the fresh one (K1), and both runs end with slow learning and a weak nose (K2): two optimiser seeds agree, from one start.
- 2026-10-27: a positive control for value-steering: gr-fec's result reproduces exactly (P0) and survives the corrected body
  (P1) and fresh cold layouts (+0.19, +0.21; P2), but its sign depends on the world: it costs in both rich layouts and one warm.
- 2026-10-27: a first design rule for gr-fec's worm: multiplying slow learning's and a weak nose's remaining failures predicts a
  4 x 4 sweep's interior from its edges (mean error 0.0185, D1 holds; additive 0.0591, D2 holds), one world; mild pairs beat it.
- 2026-10-27: why slow learning and a weak nose looked as if they needed each other: they are alternatives, not partners.
  Each alone lifts e45's worst case by about +0.19 to +0.21 (A1 fails): the weak nose cuts decoy time 0.59 to 0.17, slow
  learning turns the map towards the food, 0.50 to 0.75 (A2-A4 hold), and cuts decoy time too, less. Sub-additive, an OR.
- 2026-10-26: the two-gene gr-fec worm on 15 new layouts beat e45 in all 15 (worst case 0.858 against 0.622; L1, L2 hold),
  and its worst case is above the optimised worm's (0.858 against 0.824; L3 fails by 0.004, that advantage not shown).
- 2026-10-26: two of the optimised gr-fec worm's genes (slow learning, a weak smell instinct) moved onto e45 recover 95% of
  its worst-case lead (0.860 against 0.873, no difference shown; warm world -0.041); the pair acts together; the lead (+0.25)
  holds over five random streams (sd 0.003 and 0.011). H0-H3 hold.
- 2026-10-26: why the optimised gr-fec worm wins, gene by gene: slow learning (-0.050 if undone) and a weak smell instinct
  (-0.024) are shown; the single swaps add to a third of the gap, the rest in combinations or saturation; G1 and G2 fail.
- 2026-10-26: on gr-fec's worm, the designed gain is not carried by any single route (A1 fails by 0.0001); CMA-ES on the
  designed brain did not beat CMA-ES alone (C1, C2 fail); gr-fec's value-steering hurt the optimised brain (S1 fails). On a fresh
  panel the first CMA-ES genome beat e45 by +0.25 in worst case. Two pairs suggest one random stream can move a worst case by about 0.01 (not measured).
- 2026-10-26: CMA-ES on gr-fec's e45 genome raised its worst case in gr-fec's never-seen worlds from 0.818 to 0.891 (+0.073,
  interval +0.053 to +0.100; over genomes +0.047 to +0.115; O1, O2 hold); three genes ended at range limits; why it wins is not yet attributed.
- 2026-10-26: where e45's learned values come from: each weight is an exact ledger of eating credit, hunger blame and
  the prediction term, which cancels about 99% of the other two; the peak sits over the odourless food (86-93% of worms); P1-P3 hold per band.
- 2026-10-26: a designed change to gr-fec's e45, from the noise audit - the brain smooths its own hunger reading (60 s) -
  raised its worst case in gr-fec's never-seen worlds from 0.818 to 0.853 (+0.036; D0-D2 hold over worms, but over genomes
  the interval, -0.013 to +0.080, includes zero); which of its four routes carries it is not yet shown.
- 2026-10-26: e45's decisions split into signal and sensor noise: overall mostly signal (N3 fails), gut stretch half noise
  (N2 fails narrowly); the hunger-gated temperature map is 80% noise, the hunger gate reading a slow hormone through a noisy sensor.
- 2026-10-26: an exact audit trail of gr-fec's e45 decisions: fourteen pathways sum to the brain's own decision quantity to
  1e-14; in Hardland the smell instinct's share halves, a learned temperature map's grows largest, curiosity fades (P1-P4 hold).
- 2026-10-26: gr-fec's turning rule by equation: a closed-form drift theory agreed with gr-fec's own code in 36 of
  36 conditions (median error 3%, biased slightly high), and four worms designed by inverting it hit their target drifts
  within 10%.
- 2026-10-26: Rust toolchain moved to 1.99.1 ([issue #86](https://github.com/ai-quokka-wannabe/iso-lab/issues/86)):
  fmt, doc, clippy and both test profiles clean with no code change; the registry's two manifests re-run to the same bytes.

Earlier work, from 2026-10-13 to 2026-10-22, is recorded night by night in [the journal](docs/journal/); each night's
entry says what was done and what came of it.
