# AI branch — "physics of thought" (started 24 Oct 2026)

This file holds the ideas from the 24 Oct 2026 conversation about carrying the decoder physics over to AI, so that nothing is lost and a later Claude Code session can pick it up. Results of the experiments are in §6 and in `results/LEDGER.md` (rows L53+). Conventions as in the main brief: pre-register, fresh seeds, say "mirage" when it is one.

## 0. START HERE — state of play

**Resuming? `docs/HANDOFF.md` first.** Short version of everything learned: `docs/principles.md` (15 principles with evidence, plus what transfers to the fruit fly).
 (written 24 Oct 2026, ~11:30 UTC)

### 0.1 What happened in one paragraph
Matej's AGI hunch — a substrate-independent, complex-weighted concept hypergraph, shaped by "hyperwalls" from the senses,
running slightly noisy, with gates that tell creativity from hallucination — was mapped to established science (§1–2, §7)
and tested in two lines. (a) **The decoder's gravity gate as a hallucination detector:** fails on a sparse associative
memory, works on a dense answer space (an LLM). (b) **A worm-level agent** grown from a small hand-wired agent into an *embodied*
worm whose brain sees the world only through simulated senses and a simulated metabolism; the latest version (v4b)
passes all five classic C. elegans assays without cheats.

### 0.2 Findings, strongest first (ledger rows in brackets)
1. **Code/answer-space density decides which hallucination detector works** [L53–L55, L71]. Dense spaces (CCSDS, an LLM's
   answers) produce *confusions* → the margin to the runner-up (gap; gravity gate) detects them (LLM AUROC 0.881, best of
   all scores). Sparse spaces (a memory of a few items) produce *novelty* → distance-from-evidence (fit) detects them.
2. **One mechanism reproduces several worm behaviours** [L57, L65, L69]: predict valence over a learned concept graph,
   turn when the prediction falls. Thermotaxis memory and salt-avoidance learning emerge with no assay-specific code.
3. **Embodied, no-cheat worm v4b passes 5/5** [L69] once the brain integrates its noisy senses over time (leaky τ 10 s).
4. **Design rules the worm forced on us** (each found by a failure): walls from *events, not silence* [L56]; two kinds of
   concept node — *intensity* ("how much") and *tuned* ("which value") [L62]; *inner senses gate, they do not predict*
   [L64–L65]; *honest senses are noisy, so integrate over time* [L68–L69]; *calibrate on more than one behaviour* [L56].
5. **Complex edges = a compact Fourier code for timing** [L63]: one complex number per edge composes delays (6.97 vs 7)
   and carries confidence; efficiency, not new capability. Decision: keep them where relations have timing (§7.9).
6. **Noise and thought** [L58, L60, L61, L66]: noise helps only above the decision threshold (below it, it is blur), and
   only when the internal landscape is rugged; a smooth representation removes the need. No edge-of-chaos optimum in
   decision temperature at worm level.
7. **Hunger gating via hyperedges — suggestive, not shown** [L67, L70]: without hyperedges, hunger turns a learned food
   cue into a learned disappointment; with them attraction holds. Pre-registered threshold failed twice.
8. **Frozen vs chaotic hallucinations exist in an LLM** [L71]: 10 % of wrong answers are frozen (consistent, e.g. "Euclid"
   for Eu), 65 % chaotic; the gate still works on the frozen ones (AUROC 0.823) where sample agreement cannot.

9. **A worm's life — what living taught us (§6.9–6.12)** [L72–L75]: (a) with instinct inside the learner, learning made
   worms *worse* (they learned the food zone was "disappointing" — conditioned inhibition); (b) with instinct as a separate
   fixed pathway the damage goes, but a well-fed worm in a rich world has nothing to learn; (c) in a harsh world (odourless
   food, decoy smells) learning beats instinct, but only edges out a random walker, because a learned value cannot outvote a
   fixed instinct; (d) **learned trust in instinct** (instinct × learned value — a hyperedge) fixes that: energy 0.61 vs
   0.40 (v5) vs 0.37 (random), 41 % of worms living where the odourless food is, no harm in the rich world. Curiosity as a
   novelty drive did not help.

10. **Evolution beats hand design (§6.13–6.14)** [L76–L78]: v6 trust replicates (0.68 vs 0.43 learner vs 0.39 random) and
   passes all lab exams; then, with every brain number turned into a gene and selection by lifetime energy only, evolution
   rediscovered smell instinct, eat-is-good and **learned trust** from random starts, dropped salt instinct and smoothing,
   switched on roaming, curiosity and fast learning — and beat my design in both worlds (harsh 0.77 vs 0.68; rich 0.89 vs 0.85).

11. **Organic brain v7 (§6.15)** [L79–L82]: with the architecture in the genome and random worlds, evolution still improved
   fitness, but an unbalanced world schedule let it **memorise "go warm" and switch learning off**; in never-seen worlds the
   learning-heavy e45 brains won everywhere (cold band 0.84 vs v7 0.44). Generality must be selected for (several balanced
   lives per genome). The hypergraph made the failure readable (a drifted "hunger feels good" gene). Learned trust in smell is
   the one motif kept by both independent evolutions so far.

12. **Organic v7b, selecting for generality (§6.16)** [L83–L84]: three balanced lives per genome improved the cold band
   (0.44 → 0.58) and gave the best warm-band worm (0.85), but evolution again switched learning off; designed learning
   structure + evolved numbers (e45) still generalises best. Fitness must punish failure anywhere, and learning must pay
   within a life. First full-population gene–fitness statistics recorded.
13. **Matej's long-term aim (§7.11):** once the principles are demystified, skip evolution and design a worm's personality
   and experience directly.

14. **Organic v7c (§6.17)** [L85]: fitness = worst of three lives makes the best organic generalist (worst case 0.69); learning
   stays on; learned trust in smell positive in 3/3 evolutions where learning is on — the first recurring motif.
15. **Imagination as random walks on the concept graph (§6.18)** [L86]: imagination must step through concept events, not
   clock time (diagnosed from printed walks), but a three-seed replication shows no reliable benefit — the worm cannot steer
   towards what it imagines (only keep/random-turn; no heading concept). Imagination needs a body that can act on it.

16. **A body that can act (§6.19)** [L87–L88]: with a head-swing lateral sense and directed steering, steering towards the
   neighbouring concept with higher learned value is a robust win in never-seen worlds (+0.17, +0.17, +0.24 over three seed
   pairs, five replications); six multi-step imagination variants never beat it — at worm level, local value comparison wins;
   imagination should be re-tested only where greed must fail (detours, barriers).

17. **Evolvable steering and a faster optimiser (§6.20–6.21)** [L91–L93]: evolution kept value-steering but mixed it with an
   innate warm bias and maximal wobble (worst case 0.55, below v7c); the compiled brain is 2.3× faster; self-adaptive mutation +
   successive halving did not beat the plain GA at equal budget. Learned-trust motif now 3 of 4 runs with learning on.

### 0.3 Refuted or weakened (do not re-claim)
Gravity gate on a sparse memory [L53]; two-wall detector [L55]; edge-of-chaos in decision temperature [L58]; universal
benefit of sensor noise [L66]; inner senses as predictors [L64]; "quantum" mind (metaphor; §7.2); Einstein equations on
concept space (unproven; §2).

### 0.4 How to run the current best model
**Current best: the evolved genomes** — final population in `results/e45_evolve.json` (first 20 rows = elites); run them with
`EvoBrain(n, rng, genomes)` from `src/evolve.py` (see `experiments/e46_evo_eval.py`). Best hand-built: v6 with learned trust —
`Worm(n, seed, s0=0.01, integ=10, separate=True, trust_k=2.0)` from
`src/embodied.py`, living in `src/lifeworld.py` (`LifeWorld` rich, `HardWorld` harsh); lives run with
`experiments/e40_life.py` (rich, env SEP/SEED0/OUT) and `experiments/e42_harsh_life.py` (harsh; `V6=1` for v6).
Lab assays: `experiments/e39_embodied.py`. v4b: `INTEG=10 OFF=1000 python3
experiments/e39_embodied.py` (≈ 8 min on 2 CPUs; log `results/e39b.log`). Gating test only: add `gating` and `S0=0.01`.
Older models: `src/worm.py` (v0–v1, e33/e34), grown nodes `experiments/e36_grounding.py`, `src/worm3.py` (v3/v3b, e38).
LLM gate: `experiments/e31_llm_gate.py` + `a31_llm_gate.py` (needs `transformers`, `torch` CPU, HF cache).

### 0.5 Honesty notes for any write-up
Many worm passes came one diagnosed fix after a failure (forking paths) — each was pre-registered on fresh seeds, but
v2c, v3b and v4b all need replication. The worm's genome (sensor widths, reward gain 4, s0 by calibration) is chosen, not
evolved; turning rates are 3–5× a real worm's. The worm models are close to classic klinokinesis + Rescorla–Wagner
models; the contribution is the integration and the design rules, not a new learning mechanism.

## 1. Matej's picture (in his words, lightly tidied)
1. **AGI as a concept hypergraph.** Nodes are concepts, (hyper)edges are conceptual relations (relations may join more than two concepts). It "has a gravity and a quantum-mechanics vibe".
2. **The hypergraph is abstract.** Vectors in an LLM, like neurons in a brain, are the *physical layer*; the hypergraph is the layer where meaning lives. (OSI analogy: PHY vs application layer; a TC frame means the same over any modulation.)
3. **Hallucinations are creativity.** The same generative mechanism produces both.
4. **Hyperwalls.** AGI needs walls that contain and shape the hypergraph, and so shape thought. The walls come from external input: we see that cats can't fly, so we treat flying cats as impossible.

## 2. How each idea maps to what we already know
| Idea | Closest established notion | Decoder (FEC) counterpart | Verdict |
|---|---|---|---|
| Concept hypergraph | Semantic networks (Quillian 1960s), knowledge graphs; in LLMs: *features* (directions in activation space) and *attribution graphs* of features (Anthropic interpretability, 2025); Wolfram hypergraph rewriting (same mathematical object) | Tanner graph: variable nodes = bits, check nodes = relations among several bits (a hypergraph!) | **Real** as a description; concepts in nets are directions, not crisp nodes |
| Abstract vs physical layer | Marr's three levels (1982); multiple realisability; Fodor & Pylyshyn (1988) "connectionism is at best implementation" vs Smolensky (symbols approximate a subsymbolic process) | Codeword (abstract) vs waveform/LLRs (physical) | **Real and well supported**; open question is whether the abstraction is clean or leaky (polysemantic features, partial attribution graphs) |
| Different substrates, same graph | "Platonic representation hypothesis" (Huh et al. 2024): different models converge to similar representations; partial brain–LLM alignment | Same code decoded by sum-product, min-sum, OSD — same codewords | **Supported** |
| Gravity in concept space | (a) attractors / energy wells (Hopfield 1982; Nobel 2024 Hopfield & Hinton); (b) curvature: hierarchies embed in hyperbolic space (Nickel & Kiela 2017, Poincaré embeddings); (c) Fisher information metric on model space | Fixed points of BP; our "gravity gate" (gap/action) | **Half real**: attractors and curvature yes; Einstein equations unproven. Our own lesson: Tanner-graph curvature predicted nothing (brief §12) |
| Quantum vibe | "Superposition" in interpretability (more features than dimensions — sphere packing, *classical*); quantum cognition (Busemeyer & Bruza; contested); Penrose–Hameroff (very speculative) | Near-orthogonal codewords = packing | **Mostly metaphor** |
| Hallucination = creativity | Predictive processing: perception is "controlled hallucination" (Seth; Friston; Clark) | Decoder generates candidate codewords from the code's structure | **Real**: same mechanism |
| Hyperwalls from external input | Sensory prediction errors; Bayesian likelihood; in AI: retrieval/grounding (RAG) | Channel LLRs constrain which codeword | **Real**; but some walls are internal (logic, proofs: no largest prime) = parity checks |
| Wall stiffness | Bayesian precision weighting (predictive processing); sampling temperature | LLR scale α / min-sum β = decoder temperature | Too stiff → frozen (dogma, confident wrong fixed point); too soft → chaotic wandering (hallucination); FER optimum just on the soft side (brief §18) |
| Creativity vs hallucination | — | **Undetected error** = confidently wrong codeword. The gate does not forbid guessing; it labels answers that left the evidence | **Our claim:** *creativity is hallucination with a label on it* |
| Associative recall | Kanerva sparse distributed memory; hyperdimensional computing (concepts as points in high-dimensional Hamming space) | Recall = nearest-codeword decoding | **Literal**, not analogy |
| LLM sampling temperature | Softmax = Boltzmann distribution | α | Low T → repetition loops (frozen); high T → incoherence (chaotic); people use ≈ 0.7–1 |
| Feedback on temperature | Mirostat (Basu et al. 2021): adaptive sampling that holds surprise at a target — a servo loop; entropy-based adaptive temperature methods | Anti-freeze servo (brief §15) | **Known**: our servo would be a re-discovery |
| Hallucination detection | Semantic entropy (Farquhar et al., *Nature* 2024); token log-prob / P(True) self-evaluation (Kadavath et al. 2022) | Gravity gate (gap/action), Forney erasure rule | Gap/action for LLMs not found; to test |

## 3. Hypotheses (AI branch)
- **H-AI1 (gate transfers to associative memory).** In a Hamming-space associative memory, a cue from a *never-stored* pattern still recalls *some* stored pattern (a hallucination). The gap/action gate separates true recalls from hallucinations better than recall confidence alone.
- **H-AI2 (gate transfers to an LLM).** For short factual answers from a small LLM, gap (best vs runner-up answer log-probability) over action (answer surprise) detects wrong answers at least as well as plain log-probability; compare with sample agreement (a cheap semantic-entropy proxy).
- **H-AI3 (frozen and chaotic phases of generation).** As sampling temperature rises, LLM generations go from frozen (repetition loops; trajectories insensitive to tiny perturbations) to chaotic (trajectories diverge quickly); the loop-free onset and the divergence onset coincide.
- **H-AI4 (edge-of-chaos generation).** Text quality judged by a second model is best just past that onset.
- **H-AI5 (hallucination split).** Hallucinations split into "wrong edge" (the abstract graph lacks or has a wrong relation — consistent, confident) and "channel noise" (unstable across samples). Frozen ≈ wrong edge; chaotic ≈ noise. *Not yet tested.*

## 4. Prior art to check before claiming anything
Hopfield (1982); Amit, Gutfreund & Sompolinsky (1985, spin-glass analysis of Hopfield nets; spurious mixture states); Sompolinsky, Crisanti & Sommers (1988, chaos in random neural networks); Ramsauer et al. (2020, "Hopfield Networks is All You Need": attention = modern Hopfield update); Kanerva (1988, Sparse Distributed Memory); Schoenholz et al. (2017, deep information propagation / edge of chaos); Holtzman et al. (2020, neural text degeneration, nucleus sampling); Basu et al. (2021, Mirostat); Kadavath et al. (2022); Farquhar et al. (2024); Huh et al. (2024); Sourlas (1989, codes as spin glasses); Nickel & Kiela (2017).

## 5. Experiment plan
- E30 associative-memory gate (H-AI1), PREREG19.
- E31 LLM factual-QA gate (H-AI2), PREREG20. Model: Qwen2.5-0.5B-Instruct (CPU). Questions with verifiable answers built from packages (element symbols via `periodictable`, capitals via `countryinfo`).
- E32 generation phases (H-AI3/H-AI4), PREREG21. Generator GPT-2 small; judge Qwen2.5-0.5B.

## 6. Results
(filled in below as the experiments complete)

### 6.1 E30 — the gate on a Hamming-space associative memory (PREREG19, 19b, 19c)
- **19 (N = 256, σ ≤ 1): uninformative.** Every score separates perfectly (AUROC 1.000); the design was too easy.
- **19b (N = 64, σ 1.2–2.0, P = 16/64): FAIL.** Gap/action (r) does *not* beat plain fit. AUROC, e.g. P = 16, σ = 1.6:
  cos 0.959, action 0.960, r 0.945, gap 0.922.
- **Forensics f30 — the useful finding: hallucinations come in two kinds, and each needs a different wall.**
  - *Novelty* (the cue matches nothing stored: a "flying cat" 10–30 % away from a memory, or a random pattern):
    fit-type scores win (σ = 2.0 flying cat: cos 0.876, action 0.871 vs gap 0.825).
  - *Confusion* (a stored cue recalled as the wrong memory): gap wins (σ = 2.0: gap 0.960, r 0.960 vs cos 0.896, action 0.844).
  - **Why the gate worked on CCSDS:** a dense code (2^64 codewords) always has a codeword near any received word, so
    there is no "novelty" — every error is a confusion, and gap is the right wall. A sparse memory (16–64 items in
    2^64 points) is the opposite: most hallucinations are novelty, and fit is the right wall.
- **19c (two-wall logistic detector, cos + log gap, trained on one seed, tested on another): FAIL (0/6).** The learned
  weights put almost everything on cos (19.2 vs −0.17) because confusions are rare (≤ 3 % of stored cues) next to
  novelty cues. A second wall only pays when both kinds of hallucination are common.
- **FEC meaning:** code density decides which detector to use. Dense codebook → margin (Forney gap). Sparse codebook
  (memories, concept dictionaries) → distance from evidence (fit).

### 6.2 E31 — the gate on a language model (PREREG20): DONE (finished 24 Oct 2026 ~11:20 UTC)
Qwen2.5-0.5B-Instruct, 469 questions (element symbols both ways, capitals); accuracy 60.8 % (symbols 75 %, element names
44 %, capitals 62 %). Analysis `experiments/a31_llm_gate.py`, log `results/a31.log`.
| Score | AUROC all | symbols | element names | capitals | keeps correct @ 10 % wrong accepted |
|---|---|---|---|---|---|
| mean token log-prob | 0.839 | 0.858 | 0.777 | 0.866 | 58.9 % |
| total log-prob (= −action) | 0.857 | 0.924 | 0.726 | 0.881 | 61.1 % |
| sample agreement (semantic-entropy proxy) | 0.858 | 0.844 | **0.830** | 0.874 | 62.1 % |
| gap (Forney margin) | 0.875 | 0.933 | 0.760 | 0.889 | 66.3 % |
| **gravity gate r = gap/action** | **0.881** | **0.943** | 0.760 | **0.901** | **67.0 %** |
- **P20a PASS, P20b PASS, S20 PASS.** The gate transfers to an LLM: best overall, though the margin over plain
  log-probability and sample agreement is modest (+0.02–0.04 AUROC); sample agreement wins on element names.
- **Why it transfers here but not to the associative memory (§6.1):** an LLM's answer space is *dense* — every wrong
  answer has a plausible neighbour (Cobalt/Carbon, Indium/Iron, Brazzaville/Kinshasa, Abu Dhabi/Dubai) — so errors are
  mostly *confusions*, which the margin detects. Same density rule as CCSDS vs sparse memory.
- **H-AI5:** of 184 wrong answers, 10 % are "frozen" (≥ 6/8 samples agree on the wrong answer, e.g. "Euclid" for Eu,
  "Kinshasa" for the Republic of the Congo, "Dubai" for the UAE) and 65 % "chaotic" (≤ 2/8 agree). Within the frozen
  subset, where sample agreement is useless by construction, the gate still reaches AUROC 0.823.
- **Caveat:** the pre-registered symbol-scoring rule (first word) marked 4 correct sentence answers wrong
  ("Boron has the chemical symbol B."); kept as registered; small effect.
### 6.3 E32 — frozen/chaotic phases of generation (PREREG21): READY, NOT RUN
Script `experiments/e32_gen_phases.py`, smoke-tested on 3 prompts: T = 0.3 gave loops ("he was a man of good will.
He was a man of good will…"), mean Q 0.38; T = 1.3 gave word salad ("challenged planning planning. COMMOS…"), Q 0.26.
**Known design risk, written down before the run:** at low T the twin trajectories may fall into *different* loops,
so the pre-registered Jaccard similarity may read "separated" for frozen text; the chaos onset T_c may then be
mis-measured. Report as pre-registered; any repaired measure is exploratory.

### 6.4 E33/E34 — worm-level hypergraph agent (PREREG22, PREREG23), 24 Oct 2026 10:25–11:00 UTC
Code: `src/worm.py` (agent + plate), `experiments/e33_worm.py` (v0 assays), `experiments/e34_worm_v1.py` (v1),
`experiments/f33_worm_forensics.py`, figure `results/worm_v1.png` (`experiments/a34_worm_figure.py`). Logs:
`results/e33_main.log`, `results/f33.log`, `results/e34_v1.log`. Runs in ~10 s; re-run reproduced v1 exactly.

**Agent:** tuned sensory nodes → valence node via innate + learned edges (prediction-error / Rescorla–Wagner learning);
internal food-memory and hunger nodes; drive = W·s + m; pirouette when the drive falls (klinokinesis) with
exploration temperature τ; built-in touch-edge depression. Complex phases not used (no order-dependent task).

| Assay | v0 (s0 = 0.003) | v1 (rectified senses, s0 = 0.03) | Real worm analogue |
|---|---|---|---|
| A1 odour chemotaxis | PASS CI +0.67 (random walk +0.05) | PASS CI +0.61 (random walk −0.04) | chemotaxis to attractants |
| A2 thermotaxis memory (emergent) | FAIL diff +2.03 °C | **PASS diff +4.34 °C** (17.59 vs 21.94 °C) | Hedgecock & Russell 1975 |
| A3 salt associative learning (emergent) | PASS mock +0.77, salt-starved +0.01 | **PASS mock +0.33, salt-starved −0.08** | Saeki et al. 2001 |
| A4 habituation (built in) | PASS 0.90 → 0.20 → 0.74 | PASS 0.92 → 0.23 → 0.80 | tap habituation |
| A5 foraging | FAIL 12.4 vs 8.8 /min | **PASS 5.64 vs 2.10 /min (×2.7)** | area-restricted search |
| A6 no-learning ablation | fails A2, A3 as predicted | fails A2 (−0.10), A3 (+0.46/+0.46) | — |
| A7 random walker | fails all | fails all | — |
| A8 exploration temperature (edge of chaos) | nominal pass, **refuted on 8-seed replication** | **FAIL**: plateau 0.54–0.57 for τ ≤ 0.3, then 0.52, 0.41, 0.14 | — |
| Exploratory: starved at 20 °C | disperse (|T−20| 3.99 vs 2.18 fed) | disperse (3.46 vs 1.79) | Mohri et al. 2005: starvation turns cultivation-temperature attraction into dispersal |

**Lessons**
- **One mechanism, several behaviours.** "Predict valence over a learned concept graph; turn when the prediction
  falls" reproduces chemotaxis, thermotaxis memory, salt-avoidance learning, foraging search and a Mohri-like
  starvation effect. Two of these (thermotaxis memory, salt learning) have no assay-specific code. The ablation shows
  the learned edges do the work.
- **Grounding detail matters (v0 → v1): no node may fire for absence.** Always-on "zero odour / zero salt" nodes
  soaked up the valence during cultivation (Rescorla–Wagner overshadowing) and starved the temperature edges. Real
  sensory neurons do not fire for absence; once fixed, thermal edges doubled (0.19 → 0.36) and thermotaxis passed.
  Hyperwall lesson: walls must be built from *events*, not from silence.
- **Calibrating on one behaviour overfits it.** Tuning for chemotaxis alone made turning noise-driven (10/min, 5–10×
  a real worm) and broke foraging; two-assay calibration gave a realistic 2–4/min.
- **No edge-of-chaos optimum at worm level.** Unlike the decoders, more randomness never helped chemotaxis: sensor
  noise already supplies enough exploration, and a gradient climber has no frozen trap to escape. Consistent with the
  decoder lesson "randomness helps search-limited problems" — chemotaxis on a single smooth peak is not
  search-limited. Test again in a world with traps (multiple food patches, a decoy peak).
- **Honesty:** sensory tuning, valence node, motor programme and learning rule are designed in (a "genome"); this is
  a classic reinforcement/klinokinesis model (cf. Pierce-Shimomura et al. 1999 pirouettes; Rescorla–Wagner 1972), so
  the contribution is the integration under the hypergraph framing, not a new mechanism. Not compared with Kato's
  brain states yet.

**Next rungs for the worm:** (1) a world with traps to re-test the edge-of-chaos hunch; (2) let nodes form by
clustering raw sensations instead of fixed bins (real grounding); (3) complex edge phases with an order-dependent
task (e.g. odour A then B predicts food, B then A does not); (4) compare internal state flow with Kato et al. 2015.

### 6.5 E35 — randomness entering thought through the senses (PREREG24; Matej's idea, 24 Oct 2026)
Idea: the worm explored without a random decision rule because its *senses* were noisy — maybe randomness enters
thought through sensorics. Test: worm v1 made deterministic inside (no heading noise, hard turn threshold, fixed
2.4 rad turn, evenly spaced start headings); sensor noise ×k is the only randomness. 4 seeds × 100 worms per k.

| k (× v1 sensor noise) | 0 | 0.1 | 0.3 | 1 | 3 | 10 | 30 |
|---|---|---|---|---|---|---|---|
| W1 single peak CI | +0.61 | +0.48 | +0.42 | +0.80 | **+0.87** | +0.32 | −0.06 |
| W2 trap world CI (target vs decoy) | −0.06 | −0.22 | −0.16 | **+0.29** | +0.24 | −0.12 | −0.21 |

- **P24a PASS, P24b PASS:** a noiseless mind does worse than one with noisy senses; the optimum is interior in both
  worlds. With no noise, 16 % of worms in W1 are stuck (frozen orbits) and in the trap world they score below zero.
- **This revises L58:** the worm *does* have a noise optimum — in its senses, not in its decision temperature. Once the
  sensors are noisy, extra decision randomness only hurts. Randomness has to enter somewhere, and the senses suffice.
- **P24c FAIL:** traps did not raise the optimal noise (k = 1 vs 3).
- **Unexplained:** a little noise (k = 0.1–0.3) is *worse* than none — a valley before the peak. Possible cause: weak
  noise breaks the deterministic orbits that happened to work without being strong enough to explore. Worth forensics.
- **Precedent:** stochastic resonance in sensory neurons (Douglass et al., *Nature* 1993, crayfish mechanoreceptors;
  Priplata et al. 2003, vibrating insoles improve balance); noise in the nervous system (Faisal, Selen & Wolpert 2008);
  in FEC, noisy gradient-descent bit-flipping decoders (Sundararajan, Winstead & Boutillon 2014) inject noise to escape
  traps — and for any decoder the channel noise *is* its sensorics.
- **FEC question this raises (new hypothesis H-AI10):** a decoder sees the channel only once; would re-sampling the
  "senses" (dithering the LLRs slightly per iteration, as the worm re-reads noisy sensors every step) rescue frozen
  CCSDS failures better than adding randomness to the decoder itself (random check order, temperature)?

### 6.6 Worm sprint 2 (24 Oct 2026, from 10:37 UTC) — figure `results/worm_sprint2.png`
**(a) Why a little sensor noise is worse than none (f35, 12 randomised starts per level).** The dip is real, not a lucky
deterministic run (W1: k = 0 CI +0.598 ± 0.006; k = 0.1 +0.458 ± 0.010; k = 1 +0.783 ± 0.010). Mechanism: with no noise
worms circle the peak in fixed wide orbits (wobble 2.8 mm; 61 % end inside 8 mm). Weak noise (k ≤ 0.3) does not change
decisions (turns stay ≈ 1.5–1.9 /min) — it only blurs orbit control, so orbits widen (median distance 8.5 vs 7.8 mm) and
more worms are captured by the decoy (47 % vs 39 %). Noise comparable to the turn threshold (k = 1: ≈ 5 turns/min)
changes decisions: tight hovering (wobble 1.1 mm), decoy escape (61 % at target vs 28 % at decoy).
**Lesson: below the decision threshold noise is blur; above it, noise becomes search** (the stochastic-resonance
condition). FEC counterpart: dithering smaller than the basin barrier can only hurt.

**(b) Grown grounding — concepts created by a novelty gate (PREREG25 / 25b / 25c).**
| Version | A1 odour | A2 thermo memory | A3 salt learning | A4 | A5 foraging | Result |
|---|---|---|---|---|---|---|
| 25: grown tuned nodes only | FAIL +0.05 | PASS +6.75 °C | FAIL +0.16 / −0.08 | PASS | PASS ×2.43 | 3/5 |
| 25b: + innate intensity node (gain 1.0/0.3) | PASS +0.31 | PASS +6.74 °C | FAIL +0.10 / −0.12 | PASS | FAIL ×1.97 | 3/5 |
| 25c: intensity gain matched to v1 (2.354/0.706) | PASS +0.50 | PASS +4.94 °C | **PASS +0.32 / −0.11** | PASS | FAIL ×1.50 | **4/5 — P25 and S25 pass** |
- **Lesson 1 — two kinds of concept node.** Tuned nodes ("this value") are right for *where/which* (temperature
  memory improved to 6.8 °C with nodes grown where the worm lived). Intensity nodes ("how much") are needed for
  *more-is-better* drives; a grown tuned odour node born at the starting concentration made the worm stay put.
  Biology agrees: AWC (odour) codes intensity, AFD (temperature) is tuned near the remembered value.
- **Lesson 2 — "walls from events, not silence"** was written in as a rule (nodes are only born for readings above 3×
  noise) — the v0 overshadowing problem did not return.
- Nodes grown per worm: ≈ 1 odour, 1 salt, 6 temperature (prototypes spaced ≈ 1.1–1.3 °C).
- **Caveat (forking paths):** three pre-registered rounds on fresh seeds, each fix diagnosed from the previous failure;
  25c's pass should be replicated on new seeds and a new task before it is trusted. Foraging failed in 25b and 25c
  (s0 was calibrated for v1, not re-calibrated).

**(c) Complex vs real edges — the mathematical answer, demonstrated (PREREG26, all predictions pass).**
- **A complex edge is a Fourier coefficient of the relation's timing**: z = E[e^{iω·lag}]. Composing relations multiplies
  the coefficients, which is convolution of the lag distributions (convolution theorem). So phase addition = delay
  addition, and |z| = how sharp/reliable the timing is.
- Task: learn A→B (3 ± 1) and B→food, predict when food follows A alone (never experienced).
  - Unimodal B→food (4 ± 1): one complex number per edge predicts **6.97** (truth 7), |z| 0.90 — with 2 numbers per edge
    vs 16 for the real lag vector (which also finds 7); scalar real edges know "food" but not "when".
  - Bimodal B→food (2 or 8): one complex number points at the mean (8.08) and **flags it** with low confidence (|z| 0.35);
    the real lag vector finds both peaks (5, 11); three harmonics (6 numbers) recover a true peak (11).
- **Verdict on Matej's reading ("complex edges are a storage optimisation"): essentially right, sharpened.** They add no
  new capability — a real lag vector does everything — but they are an *efficient code*: 8× less storage, O(1) instead of
  O(L²) composition, plus a free confidence signal. More harmonics trade storage for resolution, like truncating a
  Fourier series. FEC counterpart: working in the frequency domain (OFDM) instead of time-domain convolution.

### 6.7 Worm v3 / v3b — re-running every worm experiment on the refined model (PREREG27, 27b), 24 Oct 2026 from 10:52 UTC
Code `src/worm3.py`, `experiments/e38_worm3.py` (env `GATES=1 SB=1000` gives v3b), forensics `experiments/f38_gating.py`,
logs `results/e38.log`, `results/e38b.log`, `results/f38.log`, figure `results/worm_versions.png`.
v3 = grown nodes (intensity + tuned) + inner senses (hunger = energy deficit, gut fullness) as graph nodes + hunger-gated
hyperedges + eating worth more when hungry (v = 0.6 + 0.8 h).

| | v3 (inner senses as predictors) | v3b (inner senses as gates) |
|---|---|---|
| A1 odour chemotaxis | PASS +0.62 | PASS +0.63 |
| A2 thermotaxis memory | **FAIL −1.11 °C (reversed)** | PASS +3.10 °C |
| A3 salt learning | PASS +0.42 / −0.18 | PASS +0.49 / −0.16 |
| A4 habituation | PASS | PASS |
| A5 foraging | FAIL ×1.38 | FAIL ×1.53 |
| P27a (≥ 4/5) | FAIL 3/5 | **PASS 4/5** |
| A6 / A7 controls | as predicted | as predicted |
| A8 τ sweep | plateau to τ = 1, then falls | flat 0.60–0.63 to τ = 3, 0.49 at 10 |
| Mohri (|T−20| starved vs fed) | 3.27 vs 2.70 | **3.31 vs 0.74** (sharpest yet) |
| Senses-only noise (P27b) | FAIL: deterministic worm already perfect (W1 +1.00 at k = 0) | not re-run |
| Hunger gating (P27c) | FAIL (+0.01 / +0.00) | FAIL (hyper −0.08, pairwise −0.08) |

**Lessons**
1. **Inner senses must gate, not predict.** As additive predictors, hunger and gut fullness are "always on" while
   eating, so they explain away the reward (the v0 absence-node problem again). Since a sated meal is worth 0.6 but the
   gut predicted ≈ 1, the home temperature even got a *negative* link (thermotaxis reversed). As gates (v3b) everything
   recovers and the starvation effect becomes much sharper. Biology agrees: neuromodulators modulate.
2. **The value of sensor noise depends on the internal landscape.** With v1's overlapping tuned bins the drive
   landscape is rugged (small local traps) and noise was needed (L60). With smooth intensity coding a fully
   deterministic worm reaches CI +1.00 with no noise. Revised reading of L60: *noise is needed when the internal
   representation is rugged*; a better representation removes the need.
3. **Hunger gating via hyperedges learns the right sign but weakly.** After 20 five-minute sessions the gated salt
   intensity weight is +0.146 vs +0.947 additive: the value gradient is only ≈ 16 % steeper when hungry; the test sat at
   ceiling (CI 0.8–0.9). Not detectable. Next: neutral cue with no innate attraction, longer training, non-saturated test.
4. **Foraging is the persistent failure since grown nodes (v2–v3b, ×1.4–1.5).** Calibration keeps picking a twitchy s0
   (8 turns/min at s0 = 0.01). Needs its own forensics (likely the 3-value s0 grid and smoother drive).
5. **Forking-paths caveat:** v3 → v3b is a fix diagnosed from data; v3b's pass needs replication on fresh seeds.

### 6.8 Embodied worm v4 / v4b — no cheating: the brain sees the world only through senses (PREREG28, 28b), 24 Oct 2026 from 11:05 UTC
Prompted by Matej: realistic embodied AI; hunger must come from simulated biology, not be a node handed to the brain.
**Cheats found in v0–v3b:** brain told "on food"; reward computed from simulator truth; hunger set directly as a formula;
worms immobilised while "cultivated"; learning frozen by the experimenter in the v3b gating test.
**New architecture `src/embodied.py`:** World (plate physics/chemistry, bacterial lawn) → Body (hidden position/heading;
metabolism: gut G fills when eating bacteria and digests into energy E, which burns; sensory neurons → noisy signal
vector y: odour, salt, neutral odour N, temperature, touch, pharynx/eating, gut stretch, hunger hormone low-passed from
1 − E) → Brain (hypergraph; sees only y; outputs only turn/reverse; reward wired by the genome to *sensed* eating and
hunger, r = 4·pharynx − hunger; inner senses gate, never predict). Lab procedures realistic: lawns, starvation by
schedule, learning always on. Experiments `experiments/e39_embodied.py` (env INTEG/OFF for v4b), logs `results/e39.log`,
`results/e39b.log`; figure `results/worm_versions.png`.

| | v4 embodied | v4b + brain-side temporal integration (leaky τ 10 s, trend over 10 s) |
|---|---|---|
| A1 odour chemotaxis | PASS +0.57 | PASS +0.74 |
| A2 thermotaxis memory | FAIL +2.85 °C | **PASS +4.07 °C** |
| A3 salt learning | PASS +0.24 / −0.12 | PASS +0.37 / −0.13 |
| A4 habituation | PASS | PASS |
| A5 foraging | FAIL ×1.42 | **PASS ×5.82** |
| P28a | FAIL 3/5 | **PASS 5/5 — first version to pass everything, and without cheats** |
| Controls (no learning, random walk) | as predicted | as predicted |
| Mohri |T−20| fed vs starved | 0.92 vs 3.40 | 0.82 vs 3.35 |
| Hunger gating, neutral odour (P28b) | hyper +0.40→+0.52 (+0.12); pairwise +0.49→+0.28 (−0.21) | hyper +0.36→+0.43 (+0.07); pairwise +0.50→+0.40 (−0.10) |

**Lessons**
1. **Honest senses are noisy, so a brain must integrate over time.** The v1–v3 "food memory" was noise-free — a hidden
   cheat. With a real (noisy) gut-stretch sensor, a 4-second trend turns noise into false turns (baseline 3.6/min, foraging
   fails). A leaky integrator (a slow neuromodulator-like timescale) fixes it: foraging ×5.8. FEC counterpart: averaging
   soft information before a hard decision.
2. **Without hyperedges, hunger turns a learned food cue into a learned disappointment.** In both v4 and v4b the
   pairwise brain's starved worms were *less* attracted to the food-predicting odour N (−0.21, −0.10): a hungry worm
   smelling N without food learns "N disappoints" fast. Hunger-gated hyperedges keep or raise attraction (+0.12, +0.07).
   The model contrast is consistent (0.33, 0.17) but the pre-registered threshold fails twice — **suggestive, not shown.**
3. **Sated meals are worth less when value comes from senses** (pharynx signal falls as the gut fills) — thermotaxis
   memory is weaker for well-fed worms unless the brain integrates well.
4. **Realism flags still open:** turning rates (10–15 /min in tests) are 3–5× a real worm's; no slowing on food
   (dopamine), no reversal–omega turn sequence, sensors without adaptation, no quiescence. The genome constants
   (reward gain 4, s0 by calibration) are chosen, not evolved.
5. **Forking paths:** v4b is one diagnosed fix after v4; replicate on fresh seeds before relying on 5/5.

### 6.9 A worm's life (PREREG29), 24 Oct 2026 from 12:02 UTC — let it live, no lab, no calibration
World `src/lifeworld.py` ("Agarland": 100 mm arena, 15–25 °C gradient, 8 depleting/regrowing bacterial patches, 6 on the
warm side, 3 barren salt pans); experiment `experiments/e40_life.py`; data `results/e40_life.json`, log `results/e40.log`.
100 worms × 48 h per condition, born with the embodied v4b brain; exams only on clones.
| | learning worm L | no-learning twin F | random walker R |
|---|---|---|---|
| energy h24–48 | 0.757 | **0.858** | 0.499 |
| time on food | ≈ 0.27–0.33 | 0.24 | 0.11 |
| where it lives (mean x) | drifts **cool** (−20 mm by h24) | warm (+23 mm) | centre |
| thermal exam h0 → h48 | 20.1 → **17.5 °C** | 19.9 → 19.8 °C | — |
| salt exam h0 → h48 | 0.40 → 0.27 | 0.70 → 0.73 | — |
| odour exam h0 → h48 | 0.64 → 0.53 | 0.76 → 0.78 | — |
P29a FAIL, P29b FAIL (reversed), P29c FAIL (right direction), P29d PASS (nodes +22.9 then +2.0: the mind's vocabulary
saturates after a day).
**The surprise: learning made the worm worse at living.** The learned value of temperature is most negative at 22–23 °C
— the food zone. The innate odour drive sits inside the learned prediction; near food the smell promises far more
(≈ 2) than a sated meal delivers (≈ 0.6); the negative error is absorbed by whatever else is active — the warm
temperature — so the worm learns "the warm zone is disappointing" (conditioned inhibition) and moves to the cool side,
where food is scarce. **Lesson: instinct and learning must be separate pathways; prediction errors must not correct
the instincts.** (Same family as overshadowing by absence nodes and by gut fullness.)

### 6.10 A worm's life v5: instinct and learning in separate pathways (PREREG30), 24 Oct 2026 from 12:16 UTC
Brain change only: instinct (2.354·odour + 0.706·salt + gut stretch) drives behaviour directly and is never corrected;
learning predicts sensed reward from scratch (error excludes the instinct). Note: the "residual" version, error =
r − (P + I), is exactly what failed in §6.9. Data `results/e41_life_v5.json`, log `results/e41.log`.
| | learning worm L | no-learning twin F | random walker R |
|---|---|---|---|
| energy h24–48 | 0.854 | 0.860 | 0.490 |
| where it lives (mean x) | warm, +22 mm | warm, +22 mm | centre |
| thermal exam h0 → h48 | 20.1 → 18.8 °C | 20.0 → 19.8 °C | — |
| salt exam h0 → h48 | 0.80 → 0.70 | 0.76 → 0.74 | — |
P30a–c FAIL, P30d PASS (nodes +17.7 then +1.5).
**Reading:** the fix removed the damage — learners no longer flee the food zone (learned temperature values now within
±0.15 instead of down to −1.0) — but learning adds nothing, because **a well-fed worm in a rich world has nothing to
learn**: energy sits at ≈ 0.85 all life, so sensed reward ≈ 0 everywhere and prediction errors are tiny. Instinct alone
already keeps it fed. Lesson: *learning only pays when instinct is not enough* — the world must be hard (scarce food,
cues the instinct does not know, or misleading smells).
**Next:** a harder world in which instinct is insufficient — e.g. food that does not smell but grows at a particular
temperature, smell that sometimes comes from empty (decoy) patches, scarcer food so the worm is often hungry.

### 6.11 A worm's life in a harsh world (PREREG31), 24 Oct 2026 from 12:27 UTC
World `HardWorld` in `src/lifeworld.py`: 4 odourless lawns in the cool band (≈ 16.5 °C), 2 smelly warm lawns that deplete
fast and regrow slowly, 3 decoy smells from empty ground, 2 barren salt pans; life 72 h; brain = v5. Experiment
`experiments/e42_harsh_life.py`, data `results/e42_harsh.json`, log `results/e42.log`.
| | learning worm L | instinct-only twin F | random walker R |
|---|---|---|---|
| energy h48–72 | **0.404** | 0.277 | 0.369 |
| time on odourless (cold) food h48–72 | 0.027 | 0.000 | 0.058 |
| share of worms in the cold band | 0.02 → 0.10 | 0.00 | ≈ 0.2 |
| thermal exam h0 → h72 | 19.5 → 18.3 °C | 20.0 → 19.8 °C | — |
| odour exam h72 | +0.80 | +0.83 | — |
P31a FAIL (only because instinct < random), P31b FAIL (−1.27 °C, right direction), P31c PASS, P31d FAIL.
**What happened:**
- **For the first time learning clearly beats instinct** (+0.13 energy; it finds food the instinct cannot smell).
- **Instinct has become a trap:** the instinct-only worm sits at decoy smells and does *worse than a random walker*.
- **The learner learns the right facts but cannot act on them fully.** Read from its weights: smell is disappointing
  (−0.45 at full odour) and cold is slightly good (+0.06). But (1) a fixed instinct (+2.35 × odour) outweighs any learned
  value of order 1 — learning cannot override instinct; (2) it rarely visits the cold band, so it learns cold slowly —
  an exploration problem (random walkers reach the cold band twice as often).
**Next ideas (from the worm's two bottlenecks):**
- **Learned trust in instinct (a hyperedge):** instinct gain × learned value of the cue, so experience can turn an instinct
  down or off. Biological precedent: C. elegans learns to avoid the smell of pathogenic bacteria it is innately attracted to
  (Zhang, Lu & Bargmann, *Nature* 2005).
- **Curiosity from the novelty gate:** novel sensations (new nodes being born) give intrinsic reward, and/or hunger raises
  exploration when the drive has stopped paying — explore when the instinct fails.

### 6.12 Worm v6: learned trust in instinct + curiosity (PREREG32), 24 Oct 2026 from 12:50 UTC
Brain additions (constants fixed in advance): **trust** — each instinct (odour, salt) is multiplied by
t = clip(1 + 2·learned value of that cue, 0, 1.5), a hyperedge instinct × learned value; **curiosity** — node births feed a
novelty trace (τ 5 min) that adds to the drive. Code `src/embodied.py` (`trust_k`, `curiosity`), run
`V6=1 python3 experiments/e42_harsh_life.py`, data `results/e43_v6.json`, log `results/e43.log`, figure `results/worm_lives.png`.
| Hardland, h48–72 | energy | odourless (cold) food share | share living in cold band (h72) |
|---|---|---|---|
| instinct only (PREREG31) | 0.277 | 0.000 | 0.00 |
| random walker (PREREG31) | 0.369 | 0.058 | ≈ 0.2 |
| learning v5 (PREREG31) | 0.404 | 0.027 | 0.10 |
| v6 curiosity only | 0.406 | 0.021 | 0.09 |
| **v6 trust only** | **0.607** | **0.115** | **0.41** |
| v6 trust + curiosity | 0.572 | 0.095 | 0.37 |
Rich world (Agarland) safety check, h24–48: trust + curiosity 0.854 vs instinct-only 0.858 — no harm.
**Findings**
- **Learned trust in instinct is the breakthrough:** +50 % energy over the v5 learner, beats random by 0.24, four times the
  cold-food share. Experience switches the misleading smell instinct down (odour exam 0.85 → 0.64) and the worm settles
  where the odourless food is. This is the first clear win for a *hyperedge* in the worm (instinct × learned value) and
  matches biology (learned avoidance of attractive pathogen smells, Zhang, Lu & Bargmann 2005).
- **Curiosity as implemented does not help** (alone ≈ v5; combined slightly worse than trust alone). A novelty trace in the
  drive pulls worms towards whatever is new, not towards what pays.
- **Open puzzle:** trust worms live in the cold band, yet their clones show no cold preference in the 20-minute thermal lab
  exam — what they learned may be "leave the smell" more than "seek cold", or the exam plate is too short to show it.
  Not yet read from their weights (value curves were logged only for the v5 learner).
- Caveat: references from PREREG31 used different seeds; single run per condition.

### 6.13 Day-2 steps (PREREG33), 24 Oct 2026 13:58–14:20 UTC
- **Replication passes (P33a):** in one batch on fresh seeds, harsh world, energy h48–72: trust v6 **0.682**, v5 learner 0.426,
  random 0.387, instinct-only 0.292; odourless food share 0.158 vs 0.034. Stronger than the first run (0.607).
- **Lab exams on v6 pass 5/5 (P33c)** without calibration: chemotaxis +0.84, thermotaxis memory +3.88 °C, salt learning
  +0.69 → −0.18, habituation, foraging ×5.6.
- **The thermal-exam puzzle, solved by reading minds:** trust worms halve their trust in smell (0.70 → 0.55) and learn a
  temperature *map of places*: a peak at 16–17 °C (cold food band), a valley at 18–21 °C (the barren middle). The lab exam
  starts them at 20 °C, the bottom of the valley, where the slope is shallow both ways — so no direction shows in 20 min.
- Roaming and the changing world were **not run**: the plan changed to evolution (§6.14).
Data `results/e44_A.json`, `results/e44_C.json`; logs `results/e44_A.log`, `results/e44_C.log`; script `experiments/e44_day2.py`.

### 6.14 Evolving the brain instead of hand-picking it (PREREG34), 24 Oct 2026 14:02–14:32 UTC
Matej: "make sure you don't hardcode things in the brain — make it evolve organically." The *architecture* stays designed
(senses, body, novelty-gate nodes, learning rule, hyperedges, separate instinct pathway, trust/roam/curiosity mechanisms,
turning rule). **Every number became a gene** (12 genes, `src/evolve.py`), starting uniform-random over wide ranges that
include "off" and the wrong sign. 100 genomes × 20 generations, each a 24 h life from birth, worlds alternating harsh/rich,
selection by lifetime energy only — no lab assay used anywhere. Script `experiments/e45_evolve.py`, data
`results/e45_evolve.json`, evaluation `experiments/e46_evo_eval.py` → `results/e46_evo_eval.json`, figure `results/worm_evolution.png`.
**What evolution found (elite medians; my hand-picked values in brackets):**
| gene | evolved | hand | reading |
|---|---|---|---|
| smell instinct | 1.7 (2.2–2.8 most generations) | 2.35 | rediscovered: food smells good |
| salt instinct | ≈ 0 | 0.71 | dropped: salt is useless in these worlds |
| gut-stretch drive | −1.5 (late) | +1 | unexpected; to investigate |
| reward for eating | 4.4 | 4 | rediscovered |
| punishment for hunger | 4.0 | 1 | stronger than I chose |
| **trust in instinct** | **2.9** (positive from gen 8) | 2 | **evolution independently switched trust on** |
| roaming | 4.5 for gens 3–18, 1.5 at gen 19 | 0 / 3 | evolved strong hunger-driven roaming |
| curiosity | 1.3 | 0 | kept on, moderate |
| learning rate η | 0.01 (range limit) | 5e-4 | wants to learn 20× faster |
| integration time | 1 s (range limit) | 10 s | prefers no smoothing, with a sharper turning rule (τ 0.04) |
**Fair test, same protocol as §6.13:** harsh world energy h48–72 **0.773 vs 0.682** hand-built; odourless food **35.9 % vs 15.8 %**;
68 % of worms living in the cold band; rich world **0.892 vs 0.854**. **Evolution beats my design in both worlds.**
**Reading:** the hand-designed pieces that mattered (smell instinct, eat-is-good, learned trust) were rediscovered from random
starts; pieces I thought mattered (salt instinct, temporal integration) were thrown away; pieces I left off (roaming,
curiosity, faster learning) were switched on.
**Caveats:** one evolutionary run; fixed world layouts each generation (in-distribution test only); η and integration time
hit their range limits (ranges should be widened); the architecture is still designed.

### 6.15 Organic brain v7 — the architecture in the genome (PREREG35), 25 Oct 2026 from 10:26 UTC
Brain `src/evolve2.py`: all 9 input channels (8 senses + own novelty trace) treated alike; per channel genes for node
growth, width, event threshold, instinct, trust, **reward (what feels good is evolved)**, turning bias; global genes for
learning rate, learned-drive gain, integration, trend lag, turning, novelty decay, two evolvable gate channels
(hyperedges). Designed substrate left: novelty-gate node growth, normalised-LMS prediction-error learning, multiplicative
trust, drive → integration → trend → sigmoid turning, touch reflex. Worlds randomised every generation
(`RandomHardWorld`, `RandomRichWorld`). Evolution `experiments/e47_organic_evolve.py` (2 workers, ≈ 3.5 min/generation,
20 generations), data `results/e47_organic.json`, log `results/e47.log`.
**Evolution result:** elite fitness rose (harsh 0.762 → 0.851, rich 0.799 → 0.893; P35a PASS). But the random generator gave
the harsh food band the warm side in 8 of 10 harsh generations, and evolution **memorised the bias instead of learning**:
temperature instinct −2.0 → +2.3 ("go warm"), temperature reward +2.2, learning rate down 50× (10^−2.5 → 10^−4.2). The one
cold-band generation after adaptation (gen 10) crashed to 0.60. It kept learned trust in smell (+3.7) and rarely used gates.
**Statistics of evolution (Matej: measure the optimiser too; `experiments/a50_evolution_stats.py`):** selection
differential +0.06…+0.39 per generation; 90 % of the harsh gain by generation 4; 84 % of genes still drifting at the end
(neutral → wasted search); harsh vs rich worlds pull hardest on eating reward, touch instinct and gate switches.
**Lessons for optimising evolution:** (1) balance the world statistics (equal cold/warm); (2) evaluate each genome on
several different lives per generation so instincts cannot memorise a side and learning pays (Baldwin-effect trade-off:
learning survives only when the world is unpredictable *within* a genome's evaluation); (3) shrink or freeze neutral genes;
(4) log full populations every generation (now enabled) for gene–fitness correlations.
**Never-seen worlds (`experiments/e48_organic_eval.py`, `results/e48_organic_eval.json`, figure `results/worm_novel_worlds.png`):**
| energy h24–48 | organic v7 | hand-built v6 | e45 evolved numbers | random |
|---|---|---|---|---|
| new harsh, food band cold | 0.44 | 0.43 | **0.84** | 0.38 |
| new harsh, food band warm | 0.82 | 0.81 | **0.83** | 0.35 |
| new rich | 0.84 | 0.89 | **0.89** | 0.43 |
P35b passes only nominally; P35c fails. **The learning-heavy evolved brain (e45) generalises to worlds it never saw, in either
band; the more organic v7 memorised "go warm" and starves when the food is cold.**
**Reading the failure in the hypergraph** (`experiments/a49_mind_printout.py`, `results/a49_mind_v7_cold.log`): a v7 elite's
genome says instinct "go warm" +3.0 and reward "hunger feels good" +5.0 (a drifted, unselected gene because learning was nearly
off); after 24 h in the cold world it starved (energy 0.00). With an opaque model we would only see "it died".
**Lessons:**
1. **More freedom is not automatically better: what evolution builds depends on what the world schedule rewards.** With a
   predictable schedule (mostly warm), evolution chose cheap instincts over learning (Baldwin-effect trade-off) and lost
   generality. With e45's schedule, learning was pushed to its maximum and generalised.
2. **Generality must be selected for:** each genome must face several different worlds within one evaluation, balanced
   across the possibilities.
3. **Interpretability pays:** the failure (wrong instinct + drifted reward) was read directly off the graph.
4. **Candidate "grand pattern" evidence:** across both evolutions, *learned trust in smell* survived (e45 2.9; v7 3.7) —
   the one motif kept by independent runs so far.

### 6.16 Organic v7b: selecting for generality (PREREG36), 25 Oct 2026 12:21–~13:50 UTC
Same organic brain, fresh independent evolution (seed 25000); every genome lived **three 12 h lives per generation** (harsh
cold-band, harsh warm-band, rich; new random layouts), fitness = mean. Script `experiments/e49_organic_general.py`, data
`results/e49_general.json` (full populations logged), analysis `experiments/a52_v7b_genes.py` → `results/a52_v7b_genes.log`,
test `experiments/e50_eval_v7b.py` → `results/e50.log`, `results/e50_eval_all.json`.
| never-seen worlds, energy h24–48 | v7 | **v7b** | e45 | v6 | random |
|---|---|---|---|---|---|
| food band cold | 0.44 | **0.58** | 0.84 | 0.43 | 0.38 |
| food band warm | 0.82 | **0.85** | 0.83 | 0.81 | 0.35 |
| rich | 0.84 | **0.86** | 0.89 | 0.89 | 0.43 |
All four predictions fail (P36a–d). Balanced selection improved the cold band (+0.13) and produced the best warm-band worm,
but **evolution again switched learning off** (rate at the minimum, 10^−5) and kept a "go warm" instinct (+2.8) — a better
instinct compromise, not a learner.
**Why (diagnosis):** (1) fitness = *mean* over lives rewards being good in 2 of 3 world types; generality needs a fitness that
punishes failure anywhere (minimum or geometric mean across lives); (2) learning must pay *within* a life — 12 h lives and
slow normalised learning over ~350 features make its payoff come too late; (3) 73 genes × 14 generations is under-explored
(16 genes selection-neutral). **Contrast:** e45 kept the designed v6 learning machinery and only tuned numbers, and evolution
pushed its learning rate to the maximum — it generalises best. So far, *designed learning structure + evolved numbers*
beats *evolved everything*.
**Grand-pattern check:** learned trust in smell −2.5 in v7b — but with learning off this gene is neutral and drifts; so the
check is inconclusive, not a contradiction. Motif tally across three independent evolutions: trust in smell positive where
learning is on (2 of 2).
**Gene–fitness statistics (first full-population run):** strongest correlations |r| ≈ 0.2–0.3 — exploration temperature
(−0.32: more decisive turning pays), touch bias (−0.26), eating reward (+0.24: "eating feels good" is selected), neutral-odour
genes (±0.22); 16 of 73 genes selection-neutral.
**Next (to make learning pay in the organic brain):** fitness = minimum (or geometric mean) across lives; longer lives (24–48 h);
a learning-rate floor or per-feature normalisation so learning can act within a life; more generations or a smaller genome
(freeze neutral genes); then repeat for the grand-pattern tally.

### 6.17 Organic v7c: fitness = worst of three 24 h lives, learning cannot be switched off (PREREG37), 25 Oct 2026 20:41–21:45 UTC
Same organic brain; learning-rate gene range 10^−3…10^0; fresh evolution (seed 26000), 10 generations; data
`results/e51_v7c.json`, log `results/e51.log`, genes `results/a52_v7c_genes.log`, test `results/e53.log`, `results/e53_eval_all.json`.
| never-seen worlds, energy h24–48 | v7 | v7b | **v7c** | e45 |
|---|---|---|---|---|
| cold band | 0.44 | 0.58 | **0.69** | 0.84 |
| warm band | 0.82 | 0.85 | 0.70 | 0.83 |
| rich | 0.84 | 0.86 | 0.78 | 0.89 |
| **worst case** | 0.44 | 0.58 | **0.69** | 0.83 |
- **Minimum-fitness selection makes a generalist:** best worst-case of all organic brains, no collapse anywhere, lower peaks.
- **Learning stayed on and paid** (rate near the top; gene–fitness r +0.20).
- **Evolution found a new trick:** a "go warm" instinct (+2.8) paired with strongly *negative* temperature trust (−4.4), so
  experience about temperature can override the instinct.
- **Grand-pattern tally:** learned trust in smell positive in all three evolutions where learning was on (+2.9, +3.7, +3.3);
  neutral where learning was off. The first motif that recurs across independent runs.
- Still below designed-learning e45 (worst case 0.83): evolving everything remains harder than evolving numbers in a good
  architecture. P37a fails by 0.01; P37b, P37d fail; P37c passes.

### 6.18 Imagination as random walks on the concept graph (H-AI14, PREREG38 / 38b / 38c / 38d), 25 Oct 2026 21:35– UTC
Brain = embodied v6 with learned trust + `src/imagine.py`: learned transition counts between concept states for "keep going"
vs "turn"; every 10 s it imagines K = 8 walks per first action, scores them by the learned values of the concepts visited,
and turns more or less according to which choice it imagined better. Script `experiments/e52_imagination.py`
(env EVENTS, NODIR, OFFS), logs `results/e52*.log`; replication `experiments/e54_imagination_rep.py` → `results/e54.log`.
| variant | what a step of imagination is | Hardland (Δ energy h48–72) | new cold band (Δ h24–48) |
|---|---|---|---|
| 38 | 10 s of clock time; state = temp concept × direction | −0.025 | −0.009 |
| 38b | a change of state (event-based); state = temp concept × direction | −0.009 | **+0.062** |
| 38c | a change of temperature concept only | **+0.072** | +0.020 |
**Interpretability did the debugging:** the printed imagined walks showed exactly what was wrong each time — 38: walks
barely moved ("22.4 °C → 22.4 °C → …": a 1-minute horizon cannot see a food band 5 °C away); 38b: most "events" were
sensor-noise flickers of the direction label; 38c: walks finally travel across concepts (19.7 → 20.7 → 21.5 → 22.3 → 23.0 °C).
**Lesson:** imagination must step through *events in the concept graph*, not clock ticks — the same rule as "walls from
events, not silence". Single-seed differences are within seed-to-seed spread (plain baselines vary 0.58–0.63), hence the
three-seed replication (38d).
**Replication (38d, three seed pairs per world): FAILS.** Hardland mean −0.013 (−0.019, +0.009, −0.030); new cold band mean
+0.021 (+0.018, −0.002, +0.046). The 38c gain was seed luck. **Why imagination cannot pay yet:** the worm's only choices are
"keep going" or "turn to a random heading", and none of its concepts encodes its own heading — so it cannot steer towards
what it imagines. **Lesson: imagination needs a body that can act on it** — directed turns (left/right) and a heading sense
(proprioception) as concepts in the graph. Next: give the worm head-swing steering and a heading channel, then re-test.

### 6.19 A body that can act on imagination: steering + lateral temperature sense (PREREG39), 25 Oct 2026 ~22:55– UTC
`src/steer.py`: DirBody adds a head-swing lateral temperature sense (left − right of head, σ 0.02 °C) and directed steering
(±0.15 rad/s); SteeringBrain = v6 trust + *plain* (never steers), *greedy* (steers towards the side whose neighbouring
temperature concept has the higher learned value — one-step lookahead) or *imagine* (multi-step random walks through
event-based concept transitions per steering action). Script `experiments/e55_steer_imagine.py`, data `results/e55_steer.json`.
| energy | plain | greedy | imagine |
|---|---|---|---|
| Hardland h48–72 | **0.672** | 0.629 | 0.556 |
| never-seen cold band h24–48 | 0.388 | **0.625** | 0.475 |
P39a, P39b FAIL. **Findings:** (1) in a never-seen world, *steering by the learned value of the neighbouring concept* is a large
win (+0.24, replication in §6.19b) — weathervaning on learned value; (2) multi-step imagination helps less than one-step greed;
printed walks show why: the lateral label flickers (L → R → L) with head swings and noise, so the transition model learns label
noise — **concept states need stability (hysteresis) before imagination can work**; (3) in familiar Hardland steering hurts.
**§6.19b Replication (PREREG39b, two fresh seed pairs):** greedy − plain in the never-seen cold world +0.170 and +0.173
(greedy ≈ 0.62 vs plain ≈ 0.45) — **P39c PASS**. Robust finding: *learn which concept is good, then steer towards it with the
body's own head-swing sense* — value-guided weathervaning generalises to unseen worlds. Real C. elegans weathervanes on
chemical gradients by head swings; here the same motor trick is driven by *learned* value. Grand-pattern candidate:
**learned value + a body that can steer by local comparison** beats both instinct and (so far) multi-step imagination.
**§6.19c Stable labels (PREREG40):** smoothing + hysteresis on the left/right label made imagined walks coherent, but
imagination still lost to greedy (cold 0.501 vs 0.613; Hardland 0.546 vs 0.623) and did not improve on flickering labels
(P40a, P40b FAIL). Reason: the transition model pools each steering action's effect over all headings, so it cannot represent
the local truth "steer towards the warmer side when warm is good". **Lesson: a noisy multi-step world model loses to a correct
one-step rule — planning pays only when the model is accurate.** Greedy steering replicated a fourth time (0.61–0.62).
**§6.19d Relational actions (PREREG41):** imagination choosing "towards / straight / away from the warmer side" — still no gain
(Hardland 0.565 vs greedy 0.625; cold 0.493 vs 0.606). **Summary of the imagination line (§6.18–6.19d):** six pre-registered
variants, none beats one-step greedy value-steering, which replicated five times (0.61–0.63 in the never-seen cold world vs
≈ 0.39–0.45 plain). At worm level, *local value comparison* is the winning computation; multi-step imagination as random
walks over a learned transition model does not pay in these worlds — consistent with biology (worms climb gradients, they do
not plan). Imagination is worth revisiting where one-step greed fails: worlds with detours or traps that require going
"downhill" first (e.g. a barrier between the worm and the food band).

### 6.20 Organic v8: evolvable steering (PREREG42), 26 Oct 2026 06:44–08:23 UTC
Organic v7 + DirBody (head-swing lateral sense, steering) + genes for value-steering (k_value), innate lateral steering
(k_innate) and steering noise (`src/evolve3.py`, `experiments/e59_organic_steer.py`, `results/e59_v8.json`). Fitness = worst of
three 24 h lives. Gen 0 was catastrophic (elite 0.087): random steering makes worms spin in millimetre circles. Final elites:
**value-steering kept (+1.8)** but combined with **innate warm-steering (+2.5)**, an opposing cold-seeking temperature instinct
(−2.6) and **maximal steering noise** (≈ 0.9 rad/s, the strongest fitness correlate, r +0.30). Learned trust in smell −1.8 (grand-
pattern tally now **3 of 4** runs with learning on). Never-seen worst case **0.551** (v7c 0.690). P42a, P42b, P42c all FAIL.
**Reading:** giving evolution a new tool (steering) gave it a new way to cheat with instincts; the hand-built greedy value-steering
(§6.19) remains far better. Exploration-by-wobble emerged as evolution's favourite randomness.

### 6.21 Optimising the optimiser (PREREG43), 26 Oct 2026 07:30–09:30 UTC (Matej: "optimise the hell out of the optimiser")
- **Implementation speed-up:** compiled brain step (`src/fastbrain.py`, numba) — **2.3× faster** in practice (225 s vs 545 s per
  generation); equals the original to 1e-16, then diverges chaotically through floating-point summation order (the same
  sensitivity as the FEC-phase Lyapunov work) — statistically equivalent, not bit-identical. Profile: brain step 57 %, world
  look-ups 20 % (three per step with head-swing sensing) — next targets.
- **Algorithm race** (`src/opt2.py`, `experiments/e60_opt_race.py`): GA (fixed 10 % mutation) vs OPT2 (self-adaptive per-gene
  mutation + successive halving at hour 6), equal budget 43,200 worm-hours. Best elite fitness GA 0.557 vs OPT2 0.531 (P43a FAIL);
  never-seen worst case GA 0.498 vs OPT2 0.513 (P43b marginal PASS). Self-adaptation barely moved in 10 generations; halving saved
  37.5 % compute but a 6 h single-life score is too noisy a predictor to cull on.
- **Lessons:** keep the speed-up; evaluate on a fixed panel of layouts (common random numbers) so optimisers and generations are
  comparable; cull on several short lives rather than one; consider CMA-ES (covariance learning) and a smaller genome (freeze the
  ~25 % neutral genes) rather than per-gene self-adaptation.

## 7. Second conversation round (24 Oct 2026): complex edges, Feynman paths, quantum creativity

### 7.1 Matej's picture
5. **Complex-weighted, Feynman-like concept hypergraph.** Edges have different strengths and are complex numbers;
   the graph "behaves in a quantum way, almost like Feynman diagrams".
6. **Quantumness explains creativity.** There is randomness in the mind, there are hills and valleys, and there are
   interference patterns.

### 7.2 Mapping and verdicts
| Idea | Established notion | Decoder (FEC) counterpart | Verdict |
|---|---|---|---|
| Path sum over the concept graph | Amplitude = Σ over paths of products of complex edge weights; probability = \|sum\|²; on a graph this is matrix powers / resolvent of a complex adjacency matrix; continuous-time quantum walks | BP = sum over paths of the unrolled computation tree; loops cause double counting. **Loop calculus** (Chertkov & Chernyak 2006) corrects BP exactly by a sum over generalised loops — a genuine diagrammatic expansion | **Real mathematics**; the FEC bridge is literal |
| Complex edge weights | ComplEx (Trouillon et al. 2016): complex embeddings capture asymmetric relations (inverse = conjugate); RotatE (Sun et al. 2019): relation = rotation in the complex plane, composition = phase addition; Plate's Fourier holographic reduced representations (concepts as phase vectors, binding = phase addition); rotary position embeddings (RoPE) in LLMs | Complex-temperature BP (our L-row on complex temperature: **refuted** — phase never decays, 4–16 % of messages near tanh poles) | **Real and useful** in knowledge-graph AI; phases bring new failure modes too |
| Interference | Classical waves interfere too (optics, brain oscillations; phase/synchrony as a binding mechanism); quantum cognition (Busemeyer & Bruza) uses interference terms for order effects and the conjunction fallacy | — | Interference ≠ quantum. "Wave-like" is earned; "quantum" needs normalised (unitary) evolution, entanglement and measurement |
| **Hyperwall = destructive interference** (new idea, 24 Oct) | — | — | **Untested, original as far as we know.** Impossible ideas are not forbidden; their paths cancel. E.g. "cat → animal → some animals fly" cancels against "cat → mammal, no wings" |
| Randomness drives creativity | Stochastic resonance; unreliable synaptic release, thermal channel noise, chaos amplifying tiny differences | "Oscillation is search": best decoders slightly chaotic (brief §18); random check order helps search-limited short codes | **Real**, but classical noise suffices |
| Hills and valleys | Energy landscape (Hopfield; Boltzmann machines; simulated annealing). Hopfield **spurious mixture states** = valleys nobody stored, blends of 3 memories → a machine "inventing" a flying cat | Frozen BP failures = stuck in a wrong valley; anti-freeze servo / remelt kick = thermal escape | **Real** |
| Quantum brain | Tegmark (2000): decoherence of neural superpositions ~10⁻¹³–10⁻²⁰ s vs ms neural timescales; Penrose–Hameroff dispute it; recent microtubule/anaesthesia experiments intriguing, not established | — | **Unproven**; quantum is not *needed* to explain randomness, landscape or interference. May still be useful as mathematics |
| **Insight as tunnelling** | Thermal hopping (simulated annealing) climbs *over* wide low barriers; quantum tunnelling (quantum annealing) passes *through* tall thin barriers. A "tall thin barrier" in a mind ≈ one strong assumption blocking a nearby idea; dropping it = the "aha" | Quantum annealing has been tried for LDPC decoding (Kasi & Jamieson, ~2020, wireless networks; verify citation). ML decoding is an Ising energy problem | **Metaphor worth testing** where barrier shapes can be measured |

### 7.3 New hypotheses
- **H-AI6 (interference walls).** On a small concept graph with conflicting evidence paths, complex path-sum (or
  quantum-walk) inference rejects impossible combinations ("flying cat") better than additive random-walk inference,
  *without* rejecting legitimate exceptions ("flying squirrel", "penguin can't fly").
- **H-AI7 (tunnelling vs thermal escape).** For frozen CCSDS failures, measure the barrier shape between the wrong
  fixed point and the true codeword (height vs width in Hamming/energy terms). Tall-thin barriers favour
  tunnelling-style moves (simulated quantum annealing / path-integral Monte Carlo with replicas coupled along an
  imaginary-time axis); wide-low ones favour thermal kicks. Prediction: the escape method that wins per frame is
  predictable from the barrier shape.
- **H-AI8 (spurious states = creativity).** In a Hopfield memory, mixture states are "creative blends". A two-wall
  gate (novelty + confusion, §6.1) can label them: novel yet plausible = creativity; novel and implausible =
  hallucination.

### 7.4 Extra prior art to check
Chertkov & Chernyak (2006) loop calculus; Trouillon et al. (2016) ComplEx; Sun et al. (2019) RotatE; Plate (1995/2003)
holographic reduced representations; Su et al. (2021) RoPE; Busemeyer & Bruza (2012) *Quantum Models of Cognition and
Decision*; Tegmark (2000) *Phys. Rev. E* 61, 4194; Hameroff & Penrose (2014) "Consciousness in the universe";
Kadowaki & Nishimori (1998) quantum annealing; Santoro et al. (2002) quantum vs thermal annealing; Kasi & Jamieson
(2020) quantum annealing for LDPC decoding; Childs et al. quantum walks.

## 7.5 Third round (24 Oct 2026): substrate independence
7. **AGI is not tied to its substrate** (Matej): it can run on valves, transistors, pure software emulation, a quantum
   computer or fleshy neurons — an abstract layer on top of the substrate.

| Point | Notion | Our evidence / FEC counterpart | Verdict |
|---|---|---|---|
| Any universal computer can run it | Church–Turing thesis; multiple realisability; functionalism | Same code decoded by float sum-product, min-sum and 6-bit quantised min-sum | **Strong** (almost a theorem if minds are computations) |
| Quantum computer as substrate | Quantum computers compute nothing classically uncomputable, only some things faster | — | Allowed, not required |
| The abstract layer survives a substrate change | — | Frozen/chaotic phase picture held across 3 codes × float SP, min-sum, quantised hardware decoder (brief §18, L47–L52) | **Supported** in our small world |
| The abstraction leaks | Embodied cognition: timing and body are part of the function | Random check order helped float SP, null on the quantised decoder (L46): behaviour that lived in physical-layer detail | **Real caveat** |
| Speed and energy | A valve AGI thinks the same thoughts far slower; for an agent acting in the world, speed is part of the function | Iteration-limited long codes (AR4JA) vs search-limited short codes | Real caveat |
| Noise as a hidden dependency | Substrates supply randomness (neurons, thermal) or emulate it (PRNG) | Slightly chaotic decoders are best | Emulated noise suffices, but the abstract layer relies on it |
| Consciousness | Searle (Chinese Room); Penrose (non-computable insight, widely doubted); Integrated Information Theory (Tononi: experience depends on physical causal structure, so an identical-behaving simulation could have ~zero Φ) | — | **Capability: substrate-independent (broad consensus). Experience: open question** |

- **H-AI9 (portability test).** Any phenomenon we attribute to the abstract layer must survive a change of
  implementation (float vs quantised, sum-product vs min-sum, CPU vs another model family). Phenomena that do not
  survive belong to the physical layer. Already passed by the phase picture; failed by random check order.

## 7.6 Fourth round (24 Oct 2026): building a demo
8. **Demo plan** (Matej): build an animal-level demo AGI as a pure software emulation of the hypergraph and its
   dynamics — "zero abstractions, straight to the thought, no BS".

Assessment written at the time:
- **The hypergraph is itself an abstraction;** the hard question is where its nodes and edges come from
  (*symbol grounding problem*, Harnad 1990). Hand-built concept graphs have a long record of brittleness: Cyc
  (since 1984, millions of hand-entered assertions), semantic networks, expert systems. Sutton's "bitter lesson"
  (2019): built-in structure loses to learning from data at scale.
- **Matej's own hyperwall idea already says the fix:** walls come from external input, so the graph must be *grown
  from experience* — the demo needs a world and senses, not a hand-written graph.
- **Animal-level intelligence is mostly sensorimotor:** perception, navigation, prediction, food/danger — so the demo
  needs a (simulated) body in a (simulated) world.
- **Direct prior art to study first:** OpenCog / Hyperon AtomSpace (Goertzel: a weighted, probabilistic hypergraph of
  concepts — literally this idea, decades of work); Spaun (Eliasmith 2012: 2.5 M-neuron model using *semantic
  pointers* = holographic reduced representations, i.e. circular-convolution / phase binding); NARS (Pei Wang);
  Hawkins' HTM / Thousand Brains; Friston-style active-inference agents; cognitive architectures Soar and ACT-R;
  curiosity-driven RL (Pathak et al. 2017: novelty as intrinsic reward). Benchmark: Animal-AI Olympics / Animal-AI
  testbed (Crosby et al. 2019) — cognition tests taken from animal studies.
- **Minimal demo sketched ("mouse in a maze"):** 2D grid world with food, danger and objects; the agent senses local
  features → nodes formed online by clustering sensations (grounding) → edges learned from co-occurrence and
  succession, complex phase encoding relation type / temporal order → thinking = path sum with a temperature →
  walls = current sensory evidence → novelty gate = curiosity (go and look), confusion gate = look closer →
  action = the path with the largest amplitude to "food" minus "danger".
- **No-BS criteria to pre-register:** learns to find food and avoid danger; plans through never-visited routes
  ("imagination"); generalises to novel object combinations; beats a baseline of equal size (tabular Q-learning,
  small RL agent) on Animal-AI-style tasks; ablations show each part (phases, temperature, gates) earns its place.

## 7.7 Fifth round (24 Oct 2026): start with C. elegans instead of a mouse
9. **Matej:** start at *C. elegans*, the simplest animal with a brain — much easier to explore than a mouse.

Assessment:
- **Why it is a great first rung:** 302 neurons (hermaphrodite), ~7,000 synapses, complete wiring diagram (White et al.
  1986; both sexes, Cook et al. 2019, *Nature*); open data; behaviours with lab-standard tests; and it *learns*.
- **Behaviours = the no-BS benchmark** (each with a classic assay): chemotaxis up food/salt gradients; thermotaxis to the
  temperature it was raised at (memory; Hedgecock & Russell 1975); touch withdrawal; habituation to repeated taps;
  associative learning (salt paired with starvation → avoids salt; Saeki et al. 2001); foraging (local search after
  food loss, then roaming).
- **Warning from prior art:** OpenWorm (since 2011) has tried to simulate the worm from the connectome up and still
  does not fully reproduce its behaviour. The wiring diagram is not enough: synapse strengths and signs, gap
  junctions, neuromodulators (dopamine, serotonin, neuropeptides), graded (non-spiking) neurons and body mechanics all
  matter. A 2024 integrated brain–body–environment model (BAAIWorm, Beijing; verify citation) went further but is
  still partial. Bottom-up emulation is the *physical layer* — exactly the "no abstraction" route that is hard.
- **Evidence for Matej's abstract layer:** whole-brain imaging (Kato et al. 2015, *Cell*) found the worm's brain-wide
  activity lies on a low-dimensional manifold whose few states map onto behaviour (forward, reverse, turn). The worm
  has a small abstract layer above its 302 neurons — a handful of "concept" states and transitions between them.
- **Proposed route (top-down):** build the hypergraph agent at worm level (nodes such as food-gradient-up, touch,
  too-warm/too-cold, hungry, forward/reverse/turn; complex edges for order; temperature for exploration; walls from
  sensing) in a simulated agar plate, and require it to reproduce the classic assays. Optionally compare its state
  dynamics with Kato's manifold as ground truth for the abstract layer.
- **Limits:** worms do not plan through unseen routes, so imagination, interference walls and tunnelling (H-AI6,
  H-AI7) are barely exercised. Path: worm (grounding, temperature, gates, learning) → fly (~140 k neurons, connectome
  FlyWire 2024) → mouse.

## 7.8 Sixth round (24 Oct 2026): are complex edges needed?
10. **Matej:** complex edges may be a *storage optimisation* — if a worm-level AGI works fine without them, they are not
    strictly needed, just a memory-capacity trick.

Assessment:
- **In expressive power, Matej is right by construction:** a complex weight is two real numbers with a tied
  (rotation) structure, so any complex network equals a constrained real network with about 2× the parameters.
  "Strictly needed" can never be shown; the question is what the constraint *buys*.
- **Three things it might buy beyond storage:**
  1. *Composition*: chaining relations = adding phases, so A→C is predicted from A→B and B→C without being seen
     (RotatE infers composed relations this way). Real lag-links must learn each composition separately.
  2. *Interference*: cancellation of contradicting paths comes for free (the "walls as destructive interference" idea).
  3. *Inductive bias*: the rotation constraint is a prior; priors make learning faster and generalise further.
- **FEC analogy:** structure in codes is not needed for capacity (random codes reach it), but it buys efficient
  decoding. Complex edges may be to thought what QC-LDPC structure is to a code.
- **Biology hint:** the hippocampus encodes the order of places by firing *phase* relative to the theta rhythm (phase
  precession; O'Keefe & Recce 1993) — evidence that brains use phase for sequence.

**H-AI11 (decisive test, to pre-register):** order task "A then B → food, B then A → nothing", with longer chains.
Contestants with matched budgets: (1) real co-occurrence edges; (2) real edges + explicit lag links; (3) complex
edges with phase. Measures: accuracy, learning speed, links used, and **zero-shot composition** (train A→B and B→C,
test A→C). Reading: if (2) matches (3) everywhere, complex = storage/efficiency trick (Matej's reading); if (3) wins
on zero-shot composition or learning speed, complex edges are an inductive bias that does real work.

## 7.9 Design decision (24 Oct 2026)
**Matej: keep complex edges, because they make memory storage more efficient.** Agreed with a scope rule:
- complex edges where a relation has timing or order (one complex number composes delays and carries confidence);
- plain real edges where only strength matters (a complex edge would waste half its storage there);
- **adaptive harmonics (new idea, H-AI12):** start every timed edge with one harmonic; when its confidence |z| stays low
  (ambiguous timing), the confusion gate adds a harmonic — the same way the novelty gate adds nodes. Storage grows only
  where the world is ambiguous.

## 7.10 Interoception — "inside the body" senses (Matej's question, 24 Oct 2026)
**Honest status of the worm so far:** only partly. Hunger h = 1 − exp(−t_off/1800 s) and a food-memory trace m existed,
but hunger was wired straight into the *reward* (valence v = −h off food), not *sensed* as a graph node; m fed the drive
directly. No other inner senses: no gut fullness/satiety, energy reserves, fatigue, body posture (proprioception),
damage/pain, osmotic or oxygen state, and no neuromodulator states.
**Real worm facts to model:** feeding state reshapes behaviour — dwelling vs roaming states (serotonin from food-sensing
NSM pharyngeal neurons; PDF neuropeptide), satiety quiescence after a good meal, starvation changing sensory
preferences (Mohri-type thermotaxis switch; CO2 and odour preferences shift with hunger), dopamine release on
mechanically sensing bacteria (basal slowing). Interoception is central to predictive-processing accounts of feeling
(Seth; Damasio's somatic markers).
**Why this matters for the hypergraph:** hunger does not just add to valence, it *gates* it (food odour is great when
hungry, irrelevant when full). Gating is a three-way relation (hunger AND odour → valence) — a genuine **hyperedge**.
Pairwise additive edges cannot express it without a conjunction node.
**H-AI13 (to pre-register):** add interoceptive nodes (hunger, gut fullness) to the graph; compare pairwise-only edges vs
hyperedges (multiplicative hunger × sense) on state-dependent assays: (i) fed worms dwell on a food lawn, hungry worms
roam to find a better patch; (ii) satiety quiescence; (iii) hunger-dependent odour attraction. Prediction: hyperedges
needed for (i) and (iii); pairwise edges fail when the sign of the effect must flip with state.

### 7.10.1 Embedded-systems mapping (Matej: "brain is the CPU, senses are peripherals")
| Embedded | Worm / mind | Note |
|---|---|---|
| CPU | concept hypergraph + dynamics | — |
| Peripherals (ADC, sensors) | senses | but biological peripherals are *smart*: sensory neurons adapt and compute derivatives (AFD, AWC); in the worm sensors *are* neurons |
| On-chip sensors (die temperature, supply monitor) | interoception (hunger, gut fullness) | |
| Brown-out / low-battery interrupt | hunger | |
| Sleep / low-power mode | satiety quiescence | |
| Performance modes / DVFS | dwelling vs roaming states | set by serotonin / PDF |
| Global config registers, broadcast bus | neuromodulators, hormones | change how the whole graph behaves at once |
| Interrupt mask / mode register | hunger *gating* smell → valence | the hyperedge of H-AI13 |
| Interrupt-driven vs polling zeros | "walls from events, not silence" | absence-firing nodes = polling a dead sensor |
| Watchdog | pain / damage | not modelled yet |
Pushback: the CPU/peripheral split is cleaner in silicon than in biology — bodies compute too (morphological
computation), and the boundary is blurry.

## 7.11 Seventh round (25 Oct 2026): less design, the grand AGI pattern, interpretability
11. **Matej:** priority is that less is designed/hard-coded and more is organically grown (→ organic brain v7, PREREG35,
    where the architecture moves into the genome).
12. **"The grand AGI pattern"** (Matej): a pattern that demystifies what learning and thought are; once seen, it could allow
    drastic optimisation and performance indistinguishable from real animal thought.
    *Assessment:* the practical way to look for it is **convergence across independent evolutions** — if separate runs in
    different worlds keep landing on the same small set of graph motifs, that set is a candidate. First hint: evolution
    (PREREG34) switched on learned trust in instinct by itself, the motif found by hand. Current candidate motifs: predict
    and learn from the error; inner state gates the senses; trust instincts only as far as experience allows; grow
    concepts only from events. *Mirage check:* "indistinguishable from animal thought" is far away; collect convergence
    evidence first.
13. **Interpretability constraint** (Matej): stay with the thought hypergraph — it is interpretable, inspectable and
    auditable; do not devolve into an LLM or other opaque abstraction. *Design rule adopted:* everything evolved must stay
    readable as named concepts (prototypes on named senses), weighted relations and named hyperedges (which sense gates
    which); evolution only tunes and switches parts of a printable graph. Tool: `experiments/a49_mind_printout.py`.
14. **Matej:** even in this toy model we can observe the emergence of actual intelligence, at a very primitive stage.
    *Assessment:* partly agreed, with an operational definition — behaviour learned from experience that generalises to
    situations neither the designer nor evolution saw (in randomised worlds no instinct can know where the food band is,
    yet learners find odourless food, distrust lying smells and build a map of places). Comparative cognition treats
    associative learning beyond fixed reflexes as a hallmark of minimal cognition (e.g. Ginsburg & Jablonka's "unlimited
    associative learning" as a marker of minimal minds). Limits: no planning, imagination or concepts-of-concepts yet; the
    never-seen-world test of PREREG35 is what earns or refutes the claim.
16b. **Matej (25 Oct):** eventually, when all is demystified, skip evolution and directly design the desired worm
    personality and experience. *Note:* today's runs show we are not there — evolution still surprises us (memorised
    instincts, switched-off learning), and each surprise is a principle still to understand.
15. **Matej:** planning, imagination and choice need some kind of randomness/probability behind the scenes, otherwise it
    is fixed programming and stagnation; the issue is where and how that randomness comes from.
    *Assessment, from our own results:* (a) randomness worked best entering through the **senses**, not a decision dice
    (L60, L66); evolution drove the decision temperature almost to determinism (τ 0.044, PREREG34) while switching on
    roaming and curiosity — structured exploration rather than coin flips; (b) noise helps only **above the decision
    threshold** (L61) and only where the internal landscape is **rugged** (L66). *Neuroscience version:* the **sampling
    hypothesis** (brain represents uncertainty by drawing samples; Fiser, Berkes, Orbán & Lengyel 2010) and hippocampal
    **replay/preplay** of possible paths.
    *Hypergraph proposal (H-AI14, interpretable):* imagination = **random walks on the concept graph** with edge strengths
    as probabilities (the sampled cousin of the Feynman path-sum idea, §7); planning = imagine several walks from "here",
    score their end points, act on the best; every imagined path is printable.
    *Organic proposal (H-AI15):* give every stage its own noise gene (senses, node growth, learning, decision, imagination
    walks) and let evolution decide where randomness pays and how much.

## 8. The bone pile — prioritised (24 Oct 2026, ~11:30 UTC)
**Worm / embodied AI**
00. Imagination: test only in worlds where one-step greed must fail (detours, barriers, valleys to cross) — §6.19d;
    adopt greedy value-steering as the default steering of the worm; evolve the steering mode and its constants; test greedy steering in Hardland and rich worlds with more seeds. Evolvable noise genes at every stage (H-AI15).
    Organic evolution: continue v7c (minimum fitness) for more generations; grand-pattern tally across runs.
0. **Next (after §6.16):** organic evolution with fitness = minimum across lives, 24–48 h lives, learning that can act within a
   life; then (a) balanced worlds and several lives per genome per generation (done in §6.16 with mean fitness), (b) full-population logging, (c) per-stage noise genes (H-AI15), (d) imagination as random walks on
   the concept graph (H-AI14); compare convergent motifs across ≥ 3 independent runs (grand-pattern search).
   Earlier list: Evolution worked (§6.14). Next, in order: (i) test evolved genomes in **novel worlds** (new layouts, moved food bands)
   to check they learned to learn rather than memorised a map; (ii) evolve with randomised layouts each generation and
   wider gene ranges (η and integration time hit limits); (iii) lab exams on evolved worms; (iv) the changing world (trust
   must recover when an instinct becomes reliable again) — not run on day 2; (v) evolve parts of the architecture too
   (which hyperedges exist, node widths), so less is designed; (vi) investigate the evolved negative gut-stretch drive.
1. Replicate embodied v4b (5/5) on fresh seeds; then fix realism: turning rates, slowing on food (dopamine), reversal → omega
   sequence, sensory adaptation, quiescence after meals.
2. Hunger gating (H-AI13): more power (more worms, longer training, graded hunger levels) to settle L70.
3. Order task with complex edges *inside* the embodied worm (odour A then B → food); adaptive harmonics grown by the
   confusion gate (H-AI12, §7.9).
4. A world with traps and several food patches: re-test noise, interference walls (H-AI6), tunnelling vs thermal
   escape (H-AI7).
5. Compare the worm's internal state flow with real whole-brain states (Kato et al. 2015). Then fly, then mouse (§7.6–7.7).
16. **Statistics of everything** (Matej): evolution statistics (`a50`), project-wide verdict/pass-rate statistics (`a51`):
   78 ledger rows; pre-registered predictions with recorded outcomes pass 47 % (27/57).
**Hallucination gate**
6. LLM gate on a larger model and open-ended questions; check frozen vs chaotic split (H-AI5) at scale.
7. E32 generation phases (ready, not run; twin-loop design risk noted in §6.3).
**Back to FEC**
8. H-AI10: dithering the channel LLRs per iteration ("re-reading the senses") vs internal randomness on frozen CCSDS failures.
9. H-AI7 tunnelling (simulated quantum annealing) vs thermal kicks on frozen CCSDS failures, by barrier shape.
10. H-AI8 creativity labels for Hopfield mixture states.
