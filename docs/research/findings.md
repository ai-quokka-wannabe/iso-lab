# What the Lab Has Found

A ledger of what the lab has *tested*, with failures kept beside successes, so that the brains further up the ladder
([docs/VISION.md](../VISION.md)) start from what was learned on the worm and not from scratch. Each entry says what was
found, how firmly, where the evidence lives, and what it might mean for a bigger brain. The numbers live where the runs
are recorded - the journal and the notebooks - and are not copied here; each entry points to them.

How firmly, in four words:

- **tested** - a pre-registered comparison with five seeds or more, verified by a second agent;
- **pilot** - a pre-registered comparison, verified, but with too few seeds or steps to decide;
- **observed** - seen in a verified run, but not the comparison that run pre-registered;
- **reading** - an interpretation, not itself tested; a hunch until a run isolates it.

Every result so far is from toy tasks - in notebooks, and in the gr-fec sandbox's toy worm - not from a creature on
the Grid. A toy result says what a module
can do in principle; only the benchmark says what a brain needs.

## Memory and Learning

### M1. A complex-valued memory holds no more than a real one of the same size - tested

Phase as well as magnitude in an associative memory (H10's first test) gave no more retrievable information per number
stored, at twice the multiply-adds. Evidence: [journal 2026-10-13 § H10's First Test](../journal/2026-10-13.md#h10s-first-test-phase-against-real-memory),
[notebooks/h10_phasor_memory.ipynb](../../notebooks/h10_phasor_memory.ipynb).
*For bigger brains*: phase is not a free gain in storage; if it helps, it will be where the hunch put it - in rhythm and
interference - not in raw capacity.

### M2. A frozen self-written memory keeps binding pairs in streams longer than it was trained on - observed

Every memory in H11's tests writes each sequence's new key-value bindings as the sequence arrives, with no gradient:
that is how all of them work, in training and at test alike. What the frozen test added is length: trained on 16
pairs and then frozen, a fast-weight memory written by a Hebbian outer-product rule still recalled most bindings in
streams three times as long, its recall falling as the stream grew; softmax attention, whose memory grows, lost nothing.
This was observed beside the pre-registered comparison (the hybrid against that memory), not pre-registered itself.
Evidence: [journal 2026-10-14 § H11: Both Tests Run](../journal/2026-10-14.md#h11-both-tests-run-both-refute-it),
[notebooks/h11_hebbian_hopfield_transformer.ipynb](../../notebooks/h11_hebbian_hopfield_transformer.ipynb).
*For bigger brains*: slow weights trained once and a fixed-size memory written by a local rule while living is a cheap
module any brain on the ladder could reuse for one-shot association; its limit is its fixed size.

### M3. Hopfield settling did not earn its place, in five versions - tested (v1, v3, v4), pilot (v2, v5)

Reading a Hebbian or delta-rule memory by several steps of noisy, temperature-controlled Hopfield settling - the hybrid
of H11 - never beat the same model reading in one step or none, and no version beat its matched control (linear
attention in v1, DeltaNet after) at the same memory:

- v1, Hebb's rule into one Hopfield matrix: the settling was harmful;
- v2, the delta rule in four heads: the hybrid recovered, but the settling added nothing detectable (pilot);
- v3, the same on noisy cues: the settling harmed again;
- v4, a DeltaNet whose cue settles in a Hopfield memory of its keys: no help at any noise level;
- v5, the same with binary keys: no help (pilot, one seed collapsed).

Evidence: the five result sections of [journal 2026-10-14](../journal/2026-10-14.md#h11-both-tests-run-both-refute-it)
(v2's [pilot](../journal/2026-10-14.md#h11-v2-the-pilots-result), [v3](../journal/2026-10-14.md#h11-v3-the-result),
[v4](../journal/2026-10-14.md#h11-v4-the-result), [v5's pilot](../journal/2026-10-14.md#h11-v5-the-pilots-result))
and their notebooks.
*For bigger brains*: an attractor read-out is not free accuracy. Add one only where a test shows it beating the same
brain without it.

### M4. Does the delta rule beat Hebb's rule? Unresolved - reading

v1's Hebbian Hopfield matrix collapsed where v2's delta-written one recovered most of its recall, but v2 changed three
things at once, under another protocol. The only like-for-like comparison of the two rules, v3's hybrid written each
way, was pre-registered but not computed, and its means showed no sign of a difference. DeltaNet, a delta-rule memory,
was the strongest fixed-size memory in every test it was in (v2 to v5), but that compares architectures, not write
rules. Evidence: the v2 and v3 result sections above.
*For bigger brains*: the question is open; settle it with the ablation before choosing a write rule.

### M5. Softmax attention coped better with noisy cues than DeltaNet - reading

Softmax attention, which keeps every key, beat DeltaNet more the noisier the cue. One reading is that a linear read
cannot remove the part of the noise inside the span of the stored keys; no run has isolated that cause, and the two
models differ in more than their read. Evidence: [journal 2026-10-14 § H11 v3: The Result](../journal/2026-10-14.md#h11-v3-the-result).
*For bigger brains*: senses are noisy; if the reading holds, a brain that must recall from degraded cues needs either
growing memory or a nonlinear clean-up that works - which M3 has not yet found.

### M6. The memories with narrow keys trained unreliably - observed

The models with keys of 24 dimensions lost seeds to collapses - in v4 one hybrid seed and one seed of its no-settling
ablation, in v5 one hybrid seed - while DeltaNet with keys of 64 trained cleanly in every seed. Whether the narrow keys
are the cause is a reading: those models also differ from DeltaNet in their `tanh` keys and their key memory. Evidence:
the v4 and v5 result sections above.
*For bigger brains*: before judging a small module, make its training reliable; one collapsed seed can hide or fake an
effect.

## Designing a Brain

From the toy worm of the maintainer's gr-fec sandbox ([gr-fec.md](gr-fec.md) §§ 9-11), on 2026-10-26 and 2026-10-27.
Toy worlds, few layouts and body seeds; none of it is evidence about *C. elegans*, and none of it has met the
benchmark.

### D1. A brain whose drive is a sum of terms can be audited exactly - tested

Every decision of gr-fec's e45 brain split into fourteen pathways that add back to it within about 1e-14, and into
signal and sensor noise; every learned weight split into three ledgers that sum to it within 1e-9, the bound
registered (five lives each, verified). Evidence: journal 2026-10-26, § An Audit Trail of Its Decisions, § How Much
of Each Decision Is Noise?, § Where Its Learned Values Come From.
*For bigger brains*: keep the drive additive where it can be, and the audit comes free; whether a bigger animal's
learned valuation can be built in that shape is *a hunch*, untested.

### D2. Two repairs of one failure act as alternatives, not partners - pilot

The weak smell instinct keeps the worm off decoys; slow learning steadies its temperature map, which fast learning
leaves at chance, and also cuts its time near decoys, less. Either alone recovers most of the loss and the gains are
sub-additive - consistent with, not shown to explain, removing both from the optimised genome costing far more than
either (journal 2026-10-26). Evidence: journal 2026-10-27, § Why Slow Learning and a Weak Nose Need Each Other.
*For bigger brains*: an interaction found by removal can be redundancy; check by adding each part alone.

### D3. Multiplying remaining failures predicts a pair of levers - pilot

The two levers' remaining failures, measured alone, multiply to predict a pair's score better than adding their
gains, and (in the two later worlds) better than ignoring either lever: three worlds (layouts 75000, 82003, 82104),
one pair, errors 0.011 to 0.019. Mild pairs detectably beat the rule in the first world only. Where one lever barely acts, the rule is no better than
ignoring it. Evidence: journal 2026-10-27, § A First Design Rule; journal 2026-10-28, § The Composition Law Beyond
Its First World and Pair and § The Composition Law Where Both Levers Work.
With a third lever (`roam`) that barely acts alone, the law held to 0.015 but could not be told from ignoring it;
post hoc, raising that lever cost energy in two of nine pairs, not surviving a correction for multiple comparisons.
If such an interaction is real, no rule built from edges can see it (journal 2026-10-28, § Three Levers at Once).
*For bigger brains*: a candidate rule for composing levers without searching every combination - to be tested on
other pairs of strong levers, and on three at once; a lever that seems to do nothing alone may still matter in company - to be shown.

### D4. A lever's sign can depend on the world, and a gate can make it worse - pilot

Steering by learned value gained in two cold layouts and cost in rich ones; a hunger gate removed the rich-world cost
but left the worm below one that never steers in two Hardland layouts, with or without hysteresis. Evidence: journal
2026-10-27, § A Positive Control for Steering, § Steering Only When Hungry, § Why the Hunger Gate Harms, § A Hunger
Gate with Hysteresis. Why is open. In ungated lives in two layouts about nine in ten hungry decisions came from
worms whose map peaks off the band and who live off it (post hoc); but with lives branched from one shared state
and the gate switched on at the branch, it cost the worms whose map peaks *on* the band more, in one layout (journal 2026-10-28, § Hungry Steering,
Scored Towards the Food, post hoc; § Who Loses Under the Hunger Gate, from One Shared Life). In that layout the
cost is in getting back: gated, well-mapped worms leave the band less often but return less than half as often, and
off the band their steering follows their map's peak no better than chance (§ Leaving the Band and Coming Back).
*For bigger brains*: test every world again after gating a module; a fix in one world is a new claim in all. A state
used as a gate may mark *which* individuals are in it, not only *when* - split by individual before reading a gate,
and test the split under the gate itself - with the same individuals in both arms (branch one life), never two
lives paired by index.

### D5. Search finds the same recipe again from one start - pilot

Two seeds of a sound CMA-ES, like the first defective run, chose slow learning and a weak nose. Evidence: journal
2026-10-27, § A Sound CMA-ES, from Two Seeds.
*For bigger brains*: search, then ablate, then explain: the levers a search keeps choosing are where to look first.

## How the Lab Works

Lessons about method, each learned at a cost, for every brain to come.

- **Pre-register, then let a fresh agent verify.** Every one of the night's research pull requests (#38 to #43) had
  something wrong that its verifier caught: claims stronger than the intervals, a number from the wrong comparison, a pattern asserted across
  tests that two of them contradicted, mathematics misdescribed, a citation used unread. None changed a verdict; all
  would have misled a reader.
- **One change at a time, with its ablation.** v2 changed three things and its gain over v1 could not be credited to
  any of them.
- **Check that the notebook computes every pre-registered comparison before it runs.** v3's delta-against-Hebb
  comparison was pre-registered but never computed.
- **Determinism pays.** The same code and seeds reproduced their per-seed accuracies to the last printed digit across
  notebooks (v3 and v4), and verifiers reproduced single runs exactly; a result that cannot be reproduced cannot be
  checked.
- **Wait on the process, not on its name.** Watchers found by matching a command line caught the wrong process: once
  their own shell (journal 2026-10-13, H10's first test), and on 2026-10-14 a wrapper or a second process, which
  cost about 35 minutes (journal 2026-10-14, *The Findings, Kept for the Ladder*). Wait on the process id of the
  program that does the work.
- **A cancelled job reads as a failure.** When a newer push supersedes a CI run, the gate reports the cancelled run as
  failed (journal 2026-10-14, *The Findings, Kept for the Ladder*); check the job's conclusion before treating it as a
  fault.
- **A quantity fitted to the worm is no evidence when tested against the worm.** Two of the six C19-C24 plans, as
  first drafted, set a window from the worm and then compared it with the same measurement, two left a fitted
  quantity's source unstated, and a fifth tested only what it had built in. Fit on one condition and test on another, or call the agreement a harness check (journal
  2026-10-14, *Experiment Plans for C19-C24*).
- **Run the model before writing its margin.** C18's margin refuted the hunch by construction, and C16's control failed
  in the plainest reading of its coupling, which a short pilot showed (journal 2026-10-14, *Experiment Plans for
  C13-C17, and C18 Parked* and *C16's Coupling, Piloted*).
- **A fix is a new claim.** Wording changed to answer a verifier introduced a new error of its own at least once (a
  dropped qualifier, in [#52](https://github.com/ai-quokka-wannabe/iso-lab/pull/52)), caught on the next look; re-read a
  fix as sceptically as the first draft.
- **Smoke-test a module against its switch.** A gate's threshold silently took the name of an attribute gr-fec's brain
  already had (`H`), and a first smoke test showed the gated worm identical to the plain one
  ([#128](https://github.com/ai-quokka-wannabe/iso-lab/pull/128)'s description; the clash itself is in journal
  2026-10-27, § Why the Hunger Gate Harms). A switch that does nothing looks exactly like a module that does nothing.
- **A rate is not a rhythm.** 88 crossings an hour were first read as one every 41 s; a slow signal under fast noise
  crosses in bursts (journal 2026-10-27, § Why the Hunger Gate Harms).
- **`git submodule update` puts the recorded pins back.** A submodule moved by hand and then updated runs the old
  commit; check out, record, then build (journal 2026-10-27, § The World Moved to Its Latest Main). And a `pkill -f`
  whose pattern appears in its own command kills the shell running it; match a bracketed pattern or a process id.
- **An event can go missing.** Once a push to a pull request started no CI run at all; a real change, merging the
  base, started one. Never an empty commit (journal 2026-10-14, *Closing the Night*).

## Open Questions Carried Up

- Does any nonlinear read of a fixed-size memory beat DeltaNet on noisy cues, once its training is reliable (M3, M5,
  M6)? Candidates: dense, higher-order energies; binary keys at full scale.
- Does the delta rule beat Hebb's rule, once the ablation is computed (M4)?
- How should a fixed-size memory hold many time scales at once (M2's limit; the hunch
  [F13](../hypotheses/matej-hunches.md#f13-a-memory-that-renormalises-time-shaped-like-mera))?
- Why do mild pairs of levers beat the multiplying rule (D3), and why does hunger-gated steering fall below no
  steering in some Hardland layouts (D4)?
- Which of these does the worm need at all? Only the benchmark can say, and it comes first.
