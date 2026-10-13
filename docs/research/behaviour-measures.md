# How Worm Behaviour Is Measured

> **Draft, unfinished.** Started on 2026-10-13 in a short shift. Every section was written from the papers' own
> texts; the sections marked *partly read* say what is still missing. Item 1 of [STATE.md](../../STATE.md)'s *Next*
> list finishes it.

The measures the benchmark will be built from, and where each comes from. Every claim below was read from the
paper's text in PubMed Central on 2026-10-13, through NCBI's E-utilities or the article's PMC page; where a
section has not been read in full, it says so.

## Posture: a Few Eigen-Shapes

[Stephens et al. 2008](https://doi.org/10.1371/journal.pcbi.1000028) describe the worm's shape by the
tangent angle along its body and find that the shapes of crawling worms live in a space of few dimensions:
"over 95% of the total variance in angle along the body is accounted for by just four eigenvalues". The four
shapes are the *eigenworms*.

- **The data**: tracking microscopy of single worms; the equations of motion were built from 12 worms, 5
  trials each, 4000 frames a trial, and the phase-space densities from worms sampled at 32 Hz.
- **Undulation as a phase**: the first two eigenworm amplitudes trace a rotation, so their angle is a phase
  φ of the body wave. Its rate, the phase velocity ω = dφ/dt, predicts the worm's crawling speed; the phase
  grows steadily during forward crawling, with "occasional, abrupt reversals".
- **Robust to condition**: the same four modes still capture about 95% of the shape variance during a thermal
  response, where the distribution of shapes is very different.
- **For the lab**: posture and the rhythm of undulation become comparable between a real worm and a body of
  seven joint angles by fitting eigen-shapes to each and comparing in phase and in periods of the wave rather
  than in millimetres. How many modes seven joints can carry is for the comparison space (item 2) to work out.

## The Feature Database

[Yemini et al. 2013](https://doi.org/10.1038/nmeth.2560) is the database the Open Worm Movement Database grew
from ([data/README.md § The Phase 0 Survey](../../data/README.md#the-phase-0-survey)).

- **What was recorded**: 1218 N2 young-adult hermaphrodites over three years, 2009 to 2012, and at least 20
  worms of each mutant strain, each "spontaneously behaving on food", given 30 minutes to habituate and then
  tracked for 15 minutes. Videos entered the comparisons only if they ran at least 20 frames per second for 14
  to 15 minutes with at least 20% of frames segmented.
- **What was measured**: "measures of morphology, posture, locomotion, and path dynamics", with behavioural
  events such as reversals and omega turns generating secondary measures such as event frequency, time spent
  and distance covered. "Altogether, the permutation of these measures yielded 702 distinct feature
  measurements."
- **How strains were compared**: each feature against wild type by a Wilcoxon rank-sum test, with Fisher's
  exact test for measurements found only in one group.
- **Event definitions are thresholds**: an omega turn, for example, needs a body angle that "must initially
  exceed 30° at the first but not the last third of the body". The benchmark has to fix such thresholds in
  units both bodies share before any comparison.
- **A difference worth noting**: the paper counts 1218 N2 worms recorded 2009 to 2012; the survey counted
  1792 N2 records on Zenodo, recorded 2009 to 2016. The two are not the same set, and which records the
  paper used is not yet traced.

## Behavioural Motifs

[Brown et al. 2013](https://doi.org/10.1073/pnas.1211447110), read on its PMC page (PMC3545781), carry the
eigenworms over to mutants: "four basic shapes, or eigenworms, previously described for wild-type worms, also
capture mutant shapes", and the representation is used "to build a dictionary of repetitive behavioral motifs
in an unbiased way". Measuring the distance between an individual's behaviour and each motif in the dictionary
gives a fingerprint that compares mutants with wild type and with each other.

- **For the lab**: a motif dictionary is a candidate measure that needs no hand-set event thresholds; whether
  motifs fitted to real worms make sense on seven joint angles is for the comparison space to judge.

## Locomotion States

*Partly read.* Forward runs, reversals, omega turns and pauses are the feature database's events (above).
[Pierce-Shimomura et al. 1999](https://doi.org/10.1523/JNEUROSCI.19-21-09557.1999), read on its PMC page
(PMC6782915), divide each track "into periods of smooth swimming (runs) and periods of frequent turning
(pirouettes)"; in a chemical gradient, "the initiation of pirouettes was correlated with the rate of change of
concentration (dC/dt) but not with absolute concentration", and the average heading after a pirouette was up
the gradient. Chemotaxis itself is not comparable on the Grid yet, which has no chemical sense
([STATE.md](../../STATE.md)); the run and pirouette structure of the track is, and still has to be read in
detail.

## Roaming and Dwelling

[Flavell et al. 2013](https://doi.org/10.1016/j.cell.2013.08.001): worms on food "spontaneously switch between
two discrete foraging states called roaming and dwelling". Both include forward locomotion, reversals and
turns, but in combinations specific to each state that "last several minutes and change through abrupt
transitions". The states were scored from the speed and angular speed of each animal, filmed for 90 minutes on
large (600 cm²) homogeneous bacterial lawns.

- **For the lab**: the 15-minute recordings of the feature database are short against states lasting several
  minutes, so roaming and dwelling may need longer recordings than the chosen dataset holds. Still to check.

## Still to Do

- Read Brown et al. 2013 and Pierce-Shimomura et al. 1999 for their methods and numbers, not only their claims.
- Add the reaction latencies and state time constants the pace rule asks for, each from a cited source.
- Say, for each measure, how sure the lab is that it carries over to the Grid's body.
