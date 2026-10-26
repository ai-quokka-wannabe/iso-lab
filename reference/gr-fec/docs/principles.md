# What the worm has demystified so far — principles with their evidence

Written 26 Oct 2026 as a living synthesis of the AI branch (`docs/ai_branch.md`, ledger rows L53–L90+). Each principle is
stated plainly, with the experiments that support it, the experiments that limit it, and its status. "Pre-reg" = a prediction
written before the run. Toy-world caveat applies to everything: a few dozen concepts, simulated agar plates, 100 worms.

## A. How a minimal mind learns

**1. Think by predicting value over a grown concept graph; act when the prediction falls.**
One mechanism — learned edges from sensed concepts to a valence node, turn when predicted value drops — reproduces
chemotaxis, thermotaxis memory, salt-avoidance learning, foraging search and a starvation effect, two of them with no
task-specific code. Evidence: L57, L65, L69, L77 (four versions, all pre-registered, ablations fail where predicted).
Status: **solid** in this toy world; it is classic klinokinesis + Rescorla–Wagner learning, integrated.

**2. Walls come from events, not silence.**
Every time a signal that is "always on" entered the learner, it swallowed the learning: absence-firing sensor nodes (L56),
gut fullness as a predictor (L64), clock-based imagination steps (L86). Fix each time: only events create concepts,
transitions and predictions. Evidence: three independent failures, three fixes. Status: **solid design rule**.

**3. Instinct and learning are separate pathways; learning must not "correct" an instinct.**
With the instinct inside the learner, a smell that over-promised taught the worm that the food zone was disappointing and it
fled the food (conditioned inhibition; L72). Separate pathways removed the damage (L73). Status: **solid**.

**4. Learned trust in instinct is the key hyperedge** (instinct strength × learned value of the cue). *Update 26 Oct:* the fourth
evolution with learning on (v8, steering genes) evolved negative trust in smell, so the tally is **3 of 4**.
It turned a worm trapped by lying smells into one that finds hidden food (+50 % energy, replicated; L75, L76), and
**evolution rediscovered it from random starts in all three runs where learning was on** (+2.9, +3.7, +3.3; L78, L79, L85).
Biological precedent: learned avoidance of attractive pathogen smells in C. elegans. Status: **strongest convergent motif —
first grand-pattern candidate**.

**5. Inner senses gate; they do not predict.**
Hunger and gut fullness as additive predictors overshadowed the outside world and reversed thermotaxis (L64); as gates
everything recovered and the starvation effect sharpened (L65). Status: **solid**; hunger-gating via hyperedges itself is
only suggestive (L67, L70).

**6. Honest senses are noisy, so integrate before deciding.**
A noise-free "food memory" had been a hidden cheat; with real noisy gut sensing foraging failed until the brain integrated
over ~10 s (L68, L69). Limit: evolution (e45) preferred no integration with a sharper turning rule — the trade-off depends on
the rest of the brain. Status: **solid as a constraint, open as an optimum**.

## B. Randomness, exploration and generality

**7. Noise helps only above the decision threshold, and only on a rugged internal landscape.**
Sensor noise smaller than the turn threshold is blur; comparable noise becomes search (L61); with smooth concept codes a
fully deterministic worm needs no noise at all (L66). Evolution drove decision temperature towards determinism while keeping
structured exploration (roaming, curiosity) (L78). Status: **solid**; Matej's "randomness through the senses" holds conditionally.

**8. Learning pays only when the world is unpredictable within a lifetime and instinct is not enough.**
Rich worlds: nothing to learn (L73). Predictable world schedules: evolution swaps learning for hard-wired instincts and loses
generality (L79, L80, L83 — the Baldwin trade-off, seen live). Status: **solid**.

**9. Generality must be selected for: judge a design by its worst life, not its average.**
Mean-over-lives fitness still bred side-specific instincts (L83); minimum-over-lives produced the best organic generalist
(worst case 0.44 → 0.58 → 0.69; L80, L83, L85). Status: **solid**; still short of designed learning + evolved numbers (0.83).

**10. Designed learning structure + evolved numbers currently beats evolving everything.**
e45 (v6 architecture, evolved numbers) generalised best in every never-seen world (L80, L83, L85). Status: **current state of
the art in this project**; the organic route needs more generations, better world schedules, or a smaller genome.

## C. Acting, planning, imagination

**11. A body that can steer by local comparison beats multi-step imagination (at worm level).**
"Steer towards the neighbouring concept with higher learned value" (value-guided weathervaning, using a head-swing lateral
sense) gives +0.17…+0.24 energy in never-seen worlds, replicated five times (L87, L88, L89, L90). Six pre-registered imagination
variants (random walks on the concept graph) never beat it (L86–L90). Status: **solid for gradient-like worlds**; imagination
should be re-tested only where one-step greed must fail (detours, barriers).

**12. Planning pays only when the world model is accurate.**
Each imagination failure was a model failure visible in the printed walks: walks that did not move, labels flickering with
noise, actions whose effects were pooled over headings (L86–L90). Status: **solid lesson**.

## D. Hallucination and detection (from the FEC bridge)

**13. Answer-space density decides which hallucination detector works.**
Sparse memories hallucinate by *novelty* (fit-to-evidence detects it); dense spaces (CCSDS codes, an LLM's answers) by
*confusion* (margin to the runner-up — the gravity gate — detects it) (L53–L55, L71). Status: **solid**.

## E. Method

**14. Interpretability is the debugging tool.** Every major failure was read directly off the hypergraph: a drifted "hunger feels
good" gene in a starved worm (L82), imagined walks that stood still (L86), flickering labels (L87). An opaque model would have
said only "worse". Status: **demonstrated repeatedly** — keep the mind a printable graph (Matej's rule, §7.11).

**15. Pre-register, replicate, and expect to be wrong.** About 40 % of pre-registered predictions pass; two apparent wins were
seed luck caught by replication (L58, L86). Status: **the reason the principles above can be trusted at all**.

**16. Every new tool given to evolution is first used to cheat.** Steering genes became an innate warm bias plus random wobble
(L91), as the free architecture became "go warm" and "learning off" (L79, L83). Status: **recurring observation** — the world
schedule and fitness must make the honest strategy the only winning one.

**17. Optimise the optimiser with measurements, not intuition.** A compiled brain gave a free 2.3×; textbook upgrades
(self-adaptive mutation, successive halving) did not help at this scale (L92, L93). Status: **measured**.

## Open questions (where the "grand pattern" search goes next)
- Does evolution rediscover **value-steering** (principle 11)? → partly: kept it, but mixed with innate bias and wobble (L91).
- Where does imagination first pay? → detour/barrier worlds.
- Can the organic brain match designed learning with better world schedules (minimum fitness, longer lives, more generations)?
- What changes when concepts can be built from other concepts (concepts of concepts) — the step beyond worm level?

## F. What transfers to the fruit fly (Matej, 26 Oct 2026: will this help with the fly brain?)
| Worm principle | Fly brain structure where it can be tested | Note |
|---|---|---|
| 1. Prediction-error learning over grown concepts | **Mushroom body**: odours → sparse Kenyon-cell codes ("concepts"); dopamine neurons signal reward/punishment and reshape Kenyon-cell → output-neuron links | closest biological analogue of our learner |
| 3–4. Separate instinct pathway + learned trust | **Lateral horn** (innate odour valence) alongside the mushroom body (learned); learning can override innate attraction | the separate-pathway + trust structure we arrived at |
| 5. Inner senses gate | Hunger/satiety neuropeptides and modulators gate odour responses (same smell attractive when starved, ignored when fed) | gating, not prediction |
| 11. Value-guided steering; imagination needs heading | **Central complex**: a heading compass (ring attractor), steering circuits | the missing piece for imagination/planning in the worm |
| 14. Interpretability | **FlyWire connectome** (~140,000 neurons, 2024) as ground truth to compare evolved hypergraphs | |
Caveat: the fly adds vision, flight and a richer body; the toy world must grow first. Use the worm to find what matters cheaply.

## G. Did we rediscover what cognitive architectures postulate? (Matej, 26 Oct 2026)
| Our principle (evidence) | Closest established idea | Verdict |
|---|---|---|
| Concept hypergraph with weighted, gated relations; confidence on edges | **OpenCog** AtomSpace (weighted labelled hypergraph; truth values = strength + confidence); MicroPsi node nets | rediscovered (by design intent) |
| Inner senses (hunger, gut) gate behaviour; reward from sensed body state | **Dörner's Psi theory / MicroPsi (Bach)**: needs (energy, integrity…) produce urge signals; pleasure = urge reduction; *modulators* change processing globally; **OpenPsi** in OpenCog | strongly rediscovered |
| Prediction-error learning of value over concepts | **Soar-RL**, **ACT-R** utility learning (reward prediction error on production utilities); TD learning | rediscovered (standard) |
| Choose among options by comparing learned values (greedy value-steering) | Soar's propose → evaluate → select operator cycle with numeric preferences | rediscovered |
| Noise in decisions helps only above threshold / on rugged landscapes | ACT-R activation/utility noise parameters; exploration–exploitation in RL | partly known; our threshold/ruggedness conditions are explicit tests |
| **Learned trust in instinct** (instinct × learned reliability) | **Uncertainty-based arbitration** between controllers (Daw, Niv & Dayan 2005: model-based vs model-free; Pavlovian vs instrumental arbitration) | rediscovered in comp. neuroscience form; as an evolved motif in an interpretable hypergraph, our angle |
| Imagination = random walks on a learned transition model; pays only if the model is accurate | **Dyna** (Sutton 1990) model-based planning; "model bias" in model-based RL | rediscovered, including its known failure mode |
| Evolution swaps learning for instinct when the world is predictable (Baldwin trade-off) | **Hinton & Nowlan 1987**, "How learning can guide evolution"; Baldwin effect | rediscovered (classic) |
| Evolve programs/architectures | **MOSES** (OpenCog's evolutionary program learning), NEAT neuroevolution | same approach |
| Judge by worst life (min fitness) for generality | robust / minimax optimisation | known |
| Walls from events, not silence (three independent failures) | event-driven processing in Soar/LIDA; sparse coding; predictive coding's emphasis on surprise | **not stated as a unifying rule in the architectures we know — our clearest candidate contribution** |
| Interpretability as the debugging tool | symbolic architectures are inspectable by design (Soar traces) | shared value |
**Reading:** most of what the worm forced on us is what Soar, ACT-R, OpenCog/OpenPsi and Psi theory postulate. That is good
news for the grand-pattern idea: independent routes (decades of cognitive-architecture design vs. our pre-registered,
evolved, embodied worm) converge on the same motifs. What we add is method: each motif earned or failed in pre-registered
tests, several were *re-discovered by evolution*, and the failure modes (overshadowing, conditioned inhibition, cheating
evolution) are measured, not assumed. Citations from memory — verify before any publication.
