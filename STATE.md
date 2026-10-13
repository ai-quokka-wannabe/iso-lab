# State of the Lab

**Last updated:** 2026-10-13, the opening (by the maintainer, before the first night shift).

## Where We Are

Phase 0, the benchmark, has not started. Nothing is built yet: no dataset is chosen, no feature is defined,
no creature has been scored. [docs/PLAN.md](docs/PLAN.md) says what the phases are and why the benchmark
comes before any brain.

## Next

Top first. Each item is sized for one run; split it here before starting if it is bigger.

1. **Survey the public datasets of wild-type *C. elegans* behaviour.** Start from the Open Worm Movement
   Database ([Zenodo](https://zenodo.org/communities/open-worm-movement-database/); Javer et al. 2018,
   [doi:10.1038/s41592-018-0112-1](https://doi.org/10.1038/s41592-018-0112-1)). For each candidate record what
   it contains, its file format, its size, its licence and how to download it reproducibly, in
   [data/README.md](data/README.md). Recommend the first dataset to use. Download nothing above 2 GB yet.
2. **Write the literature note on how worm behaviour is measured**, as `docs/research/behaviour-measures.md`
   (the conventions are in [docs/research/README.md](docs/research/README.md)):
   posture as a few eigen-shapes (Stephens et al. 2008), the feature database of Yemini et al. 2013, the
   behavioural motifs of Brown et al. 2013, locomotion states (forward runs, reversals, omega turns, pauses),
   roaming and dwelling. Cite each claim.
3. **Draft the comparison space** between the real worm and the Grid's worm, as
   `docs/benchmark/FEATURES.md`: which measures carry over to a body of eight rigid segments and seven joint
   angles, in which units (body lengths, undulation periods), and which cannot be compared yet and why.
4. **Build the feature-extraction tool** for the chosen dataset (Rust, with tests and a CI job joined to
   `CI Success`), and compute the wild-type reference statistics. Explore the data in a notebook first if it
   helps; the tool itself is Rust.
5. **Prove the arena**: in the cloud machine, build Master Control and the Grid (software Vulkan through
   Mesa's lavapipe) from `external/`, write the lab's own minimal Program - a placeholder brain that only
   undulates gently, shaped after [rc-worm](https://github.com/ai-quokka-wannabe/rc-worm) as the example of
   a Program - live one short headless life on a flat arena, keep its Disk, and extract the same features
   from it.
6. **Survey associative memory for a worm-sized brain**, as `docs/research/associative-memory.md`: Hebbian
   learning and Oja's rule, Hopfield networks classical and modern, Boltzmann machines, dense associative
   memory - what each is, what it would cost a brain on the worm's scale to run, which of the worm's
   behaviours it could carry, and how each would be ablated and tested ([H5, H6 and H9](docs/hypotheses/README.md)).
   A notebook that tries them on toy problems is welcome.

## Blocked

Nothing yet.

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

- 2026-10-13: the lab opened - repository, rules ([.claude/CLAUDE.md](.claude/CLAUDE.md)), plan, hypotheses,
  journal and dataset registry.
