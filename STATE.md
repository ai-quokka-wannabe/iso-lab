# State of the Lab

**Last updated:** 2026-10-13, after the maintainer's rulings, the hunches (#12) and H10's first test.

## Where We Are

Phase 0, the benchmark, is under way. The reference data is chosen - the Open Worm Movement Database's N2
recordings ([data/README.md § The Phase 0 Survey](data/README.md#the-phase-0-survey)) - and the first
literature note says how worm behaviour is measured. No feature is defined and no creature has been scored.
[docs/PLAN.md](docs/PLAN.md) says what the phases are and why the benchmark comes before any brain.

## Next

Top first. Each item is sized for one run; split it here before starting if it is bigger.

1. **Draft the comparison space** between the real worm and the Grid's worm, as
   `docs/benchmark/FEATURES.md`: which measures carry over to a body of eight rigid segments and seven joint
   angles, in which units (body lengths, undulation periods), and which cannot be compared yet and why.
   Start from [docs/research/behaviour-measures.md](docs/research/behaviour-measures.md): the motion states
   are already in body lengths per second; check whether 15-minute recordings can measure roaming and dwelling,
   whose states last several minutes.
2. **Write the literature note on the worm's pace**, as `docs/research/pace.md`: the reaction latencies (touch,
   light, temperature) and the time constants of internal states that the rule *The animal's pace* asks for,
   each read from its source in PubMed Central.
3. **Build the feature-extraction tool** for the chosen dataset (Rust, with tests and a CI job joined to
   `CI Success`), and compute the wild-type reference statistics. Explore the data in a notebook first if it
   helps; the tool itself is Rust. It fetches its subset of the N2 records itself, by record id, and checks each
   file's MD5 ([data/README.md § The Phase 0 Survey](data/README.md#the-phase-0-survey)); fix the subset and how
   it is drawn before computing anything.
4. **Prove the arena**: in the cloud machine, on Linux alone (the maintainer's ruling), build Master Control and
   the Grid (software Vulkan through Mesa's lavapipe) from `external/`, write the lab's own minimal Program - a placeholder brain that only
   undulates gently, shaped after [rc-worm](https://github.com/ai-quokka-wannabe/rc-worm) as the example of
   a Program - live one short headless life on a flat arena, keep its Disk, and extract the same features
   from it.
5. **Survey associative memory for a worm-sized brain**, as `docs/research/associative-memory.md`: Hebbian
   learning and Oja's rule, Hopfield networks classical and modern, Boltzmann machines, dense associative
   memory - what each is, what it would cost a brain on the worm's scale to run, which of the worm's
   behaviours it could carry, and how each would be ablated and tested ([H5, H6 and H9](docs/hypotheses/README.md)).
   A notebook that tries them on toy problems is welcome. The maintainer points out that the 2024 Nobel Prize
   in Physics went to this line of work (Hopfield and Hinton); cite the Nobel committee's own text, read at the
   source.
6. **Try the maintainer's hunch of a quantum-like Program** (H10 is written; its first test is under *To
   Revisit*): weights and states that are complex numbers, or
   numbers further up the hypercomplex tower (quaternions and beyond). Write it into
   [the hypotheses](docs/hypotheses/README.md) as H10, marked *a hunch*, with the outcomes that would refute it
   fixed first - among them, a complex or hypercomplex module that scores no better than a real-valued one of
   the same compute and memory. The undulation phase of Stephens et al. 2008 is already one complex number;
   start there. Read the literature on complex- and quaternion-valued networks at the source before citing it.
7. **Plan an experiment for every one of Matej's 93 ideas and hunches**, in
   [docs/hypotheses/matej-hunches.md](docs/hypotheses/matej-hunches.md) - the maintainer's request. Split into
   eight tasks of a night's size, one group each, section A first:
   - **A** (15; split into A1-A8 and A9-A15 first if it does not fit a night), **B** (9), **C1-C12**, **C13-C24**, **D** (13), **E1-E10**, **E11-E20**, **F** (12).

   Each writes `docs/hypotheses/experiments/<group>.md`: for every hunch, the question, the method, baselines and
   controls, the outcomes that would refute it (fixed first), the statistics, seeds and repetitions, the cost in
   compute and memory, what it waits on (the benchmark, the arena, a sense the Grid lacks), and its leads checked
   at the source. A hunch that cannot be tested yet keeps its plan and says what it waits on. The first task also
   writes the format and an index of all 93 in `docs/hypotheses/experiments/README.md`, and this item then points
   there instead of repeating the format; the journal records the
   triage, and hunches that survive are promoted into numbered hypotheses or tasks here.

## To Revisit

Experiments that failed or came out inconclusive, kept rather than discarded (the maintainer's ruling); each with
the idea for the next attempt.

- **H10, phasor against real memory** ([the notebook](notebooks/h10_phasor_memory.ipynb), 2026-10-13): no better
  at the same memory budget, at twice the compute. Next: test phase where the hunch says it matters - the
  undulation phase of a rhythm generator, interference between two paths (F1), or more phase states.

## Blocked

Nothing.

## Needs From Other Repositories

Each need is an issue in the repository that would meet it, triaged there by its maintainers; this list
points to the open ones and says what waits on each.

- **A sense of smell** (tron-grid-lite, master-control; the maintainer takes it up once the lab is set up -
  file nothing). The Grid has no chemical sense, and chemotaxis is the worm's best-studied behaviour. Until
  the world provides one, chemotaxis is listed as not comparable - never quietly dropped.
- **Terrace edges** (master-control; the maintainer takes it up once the lab is set up - file nothing). A
  chain can be thrown when a segment crosses a terrace edge; a fix is in progress. Until it lands, every life
  in this lab is lived on a flat arena.

## Done

- 2026-10-13: the literature note on how worm behaviour is measured,
  [docs/research/behaviour-measures.md](docs/research/behaviour-measures.md).
- 2026-10-13: the dataset survey - the Open Worm Movement Database's N2 records chosen for Phase 0, selected by
  their metadata; the CeMEE data set aside ([data/README.md § The Phase 0 Survey](data/README.md#the-phase-0-survey)).
- 2026-10-13: the lab opened - repository, rules ([.claude/CLAUDE.md](.claude/CLAUDE.md)), plan, hypotheses,
  journal and dataset registry.
