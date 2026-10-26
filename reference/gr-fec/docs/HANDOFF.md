# HANDOFF — read this first (written 26 Oct 2026, ~09:45 UTC)

This file lets a new Claude Code session resume the project without the chat history. Read in this order:
1. **this file** (state, map, commands, gotchas, open threads);
2. `docs/principles.md` — everything learned, 17 principles + fruit-fly transfer + comparison with Soar/ACT-R/OpenCog/Psi;
3. `docs/ai_branch.md` §0 (start-here summary) and §8 (prioritised next steps); §6.x = every experiment in detail;
4. `results/LEDGER.md` — one row per result (L1–L52 FEC/physics phase, L53–L93 AI/worm phase);
5. `results/PREREG*.md` — every pre-registration with its recorded outcome;
6. `docs/brief.md` — the original FEC/physics brief, with a dated running log at the end.

## 1. Who and how
- **User:** Matej - intuition-first. *(A personal detail removed when the archive was published.)*
  Communicates warmly and playfully; **explicitly wants brutal, honest pushback and "mirage or real?" flagging.** British English.
- **Working rules he set (keep them):**
  - Pre-register predictions before running; record outcomes honestly, including failures; replicate surprising wins.
  - **Write everything down** after each step (notes, ledger, pre-registration outcome, brief log, zip).
  - **No cheating in the worm:** the brain may receive information only through simulated senses (World → Body → Brain);
    hunger etc. must come from simulated biology. Designing the genome/body/world with knowledge of the world is fine.
  - **Less hard-coding, more organic growth/evolution.** But **stay with an interpretable thought hypergraph** — no LLM or
    opaque network as the brain; every concept, edge, hyperedge and gene must be printable and auditable.
  - Long-term aims: find "the grand AGI pattern" (motifs that recur across independent evolutions), fully demystify the worm
    brain, then move to the fruit fly; eventually design minds directly once principles are understood.
  - Also wants statistics on everything, including on evolution itself, to optimise the optimiser.

## 2. Project arc (one paragraph)
Started as "GR/physics geometry on FEC code spaces" for CCSDS TC LDPC (128,64) (results L1–L52, brief §0–§18: decoder
temperature/chaos phase picture, gravity gate, multiverse OSD). On 24 Oct it pivoted to the **AI branch**: the gravity gate as a
hallucination detector (works on dense answer spaces like an LLM, not on sparse memories), then Matej's AGI hunch
(substrate-independent concept hypergraph, hyperwalls, complex edges, randomness) tested on a **simulated C. elegans**: lab
assays → embodied worm (no cheats) → lives in worlds → learned trust in instinct → evolution of numbers → organic evolution of
architecture → steering/imagination → optimiser work. Current focus: **the worm only** (Matej: no more FEC/EDAC for now).

## 3. Current state of the art (numbers to beat)
| Brain | file | never-seen worlds (energy h24–48): cold / warm / rich / worst case |
|---|---|---|
| **e45: designed v6 architecture + evolved numbers** (best) | `src/evolve.py`, genomes `results/e45_evolve.json` | 0.836 / 0.828 / 0.892 / **0.828** |
| organic v7c (architecture in genome, min-fitness) | `src/evolve2.py`, `results/e51_v7c.json` | 0.690 / 0.703 / 0.780 / 0.690 |
| organic v7b (mean fitness, 12 h lives) | `results/e49_general.json` | 0.576 / 0.853 / 0.858 / 0.576 |
| organic v8 (steering genes) | `src/evolve3.py`, `results/e59_v8.json` | 0.551 / 0.826 / 0.779 / 0.551 |
| hand-built v6 trust | `src/embodied.py` (`separate=True, trust_k=2`) | 0.431 / 0.810 / 0.890 / 0.431 |
| random walker | — | 0.379 / 0.345 / 0.427 |
Never-seen layouts = `RandomHardWorld(side=−1, seed 23000)`, `RandomHardWorld(side=+1, seed 23001)`, `RandomRichWorld(seed 23002)`;
all comparisons in `results/e62_eval_all.json` (latest; includes v7, v7b, v7c, v6, e45, random, v8, GA, OPT2).
**Steering:** hand-built greedy value-steering (`src/steer.py`, mode `greedy`) = robust +0.17…+0.24 over non-steering in the
never-seen cold world (5 replications, L87–L90). Multi-step imagination never beat it (6 variants).
**Motif tally (grand pattern):** learned trust in smell positive in 3 of 4 evolutions where learning was on (e45 +2.9, v7 +3.7,
v7c +3.3, v8 −1.8); neutral where learning was off (v7b).

## 4. Code map (AI/worm phase)
| file | what |
|---|---|
| `src/worm.py` | v0/v1 worm: plate, `Worms` agent (fixed sensor bins), `chemotaxis_index` |
| `src/worm3.py` | v3/v3b (inner senses, hyperedges; *pre-embodiment, still has cheats*) |
| `src/embodied.py` | **embodied worm**: `World`, `Body` (metabolism, sensory neurons → signal vector y[0..7]: odour, salt, N, temp, touch, pharynx, gut stretch, hunger hormone), `Brain` (sees only y; flags: `integ`, `separate`, `trust_k`, `curiosity`, `roam`, `hyper`, `random_walk_p`), `Worm` wrapper with `run(world, secs, taps)` |
| `src/lifeworld.py` | living worlds: `LifeWorld` (Agarland, rich), `HardWorld` (Hardland: odourless cold food, decoys), `RandomHardWorld(n, rng, side)`, `RandomRichWorld(n, rng)` (randomised layouts) |
| `src/evolve.py` | `EvoBrain`: v6 architecture, 12 evolvable numbers (e45) |
| `src/evolve2.py` | `OrganicBrain` v7: architecture in the genome (9 channels × 7 genes + globals; normalised LMS). Env `LETA_LO/LETA_HI` set the learning-rate gene range (v7c used −3..0) |
| `src/evolve3.py` | `OrganicSteerBrain` v8: + steering genes (k_value, k_innate, steering noise) |
| `src/fastbrain.py` | numba-compiled `FastOrganicBrain` / `FastOrganicSteerBrain` (2.3× faster; equal to 1e-16 then chaotic divergence) |
| `src/steer.py` | `DirBody` (head-swing lateral temperature sense y[8], steering), `SteeringBrain` (modes plain/greedy/imagine; `stable`, `relational` flags) |
| `src/imagine.py` | `ImaginingBrain` (random-walk imagination over temperature concepts; flags `events`, `nodir`) |
| `src/opt2.py` | optimiser v2: `subset` (slice per-worm state), `sa_mutate` (self-adaptive mutation), `init_sigma` |
| FEC phase | `src/ldpc.py`, `bp_track.py`, `layered.py`, `osd.py`, etc. (see brief) |

## 5. Experiment index (AI/worm phase) — exact commands (run from `gr-fec/`)
| exp | pre-reg | command | output |
|---|---|---|---|
| E30 memory gate | 19/19b/19c | `python3 experiments/e30_memory_gate.py 1901 64 1.2,1.6,2.0 0.10,0.30`; `e30c_twowall.py`; forensics `f30_memory_forensics.py` | log in notes §6.1 |
| E31 LLM gate | 20 | `HF_HOME=/home/claude/hfcache python3 experiments/e31_llm_gate.py` (~2 h on 2 CPUs), then `a31_llm_gate.py` | `results/e31_rows.json`, `a31.log` |
| E32 generation phases | 21 | `e32_gen_phases.py` — **ready, never run** | — |
| E33/E34 worm v0/v1 assays | 22/23 | `e33_worm.py calib`, `e33_worm.py main 0.003`; `e34_worm_v1.py`; forensics `f33_*.py` | `e33_main.log`, `e34_v1.log` |
| E35 senses-only noise | 24 | `e35_sensor_noise.py`; `f35_noise_dip.py` | `e35.log`, `f35.log` |
| E36 grown grounding | 25/25b/25c | `e36_grounding.py`; `TWO_KINDS=1 SEEDBASE=8200 …`; `TWO_KINDS=1 GO=2.354 GS=0.706 SEEDBASE=8300 …` | `e36*.log` |
| E37 complex edges | 26 | `e37_complex_edges.py` | `e37.log` |
| E38 worm v3/v3b | 27/27b | `e38_worm3.py`; `GATES=1 SB=1000 …`; `f38_gating.py` | `e38*.log` |
| E39 embodied v4/v4b | 28/28b | `e39_embodied.py`; `INTEG=10 OFF=1000 …` (`gating` arg + `S0` for gating only) | `e39*.log` |
| E40/41 lives (rich) | 29/30 | `e40_life.py`; `SEP=1 SEED0=16000 OUT=results/e41_life_v5.json …` | `e40_life.json`, `e41_life_v5.json` |
| E42/43 harsh lives | 31/32 | `e42_harsh_life.py`; `V6=1 …` | `e42_harsh.json`, `e43_v6.json` |
| E44 day-2 | 33 | `e44_day2.py A` / `C` (B was stopped) | `e44_A.json`, `e44_C.json` |
| E45/46 evolve numbers | 34 | `e45_evolve.py 20`; `e46_evo_eval.py` | `e45_evolve.json`, `e46_evo_eval.json` |
| E47/48 organic v7 | 35 | `e47_organic_evolve.py 20`; `e48_organic_eval.py` | `e47_organic.json`, `e48_organic_eval.json` |
| E49/50 organic v7b | 36 | `e49_organic_general.py 14`; `e50_eval_v7b.py` | `e49_general.json`, `e50_eval_all.json` |
| E51/53 organic v7c | 37 | `LETA_LO=-3 LETA_HI=0 LIFE_H=24 AGG=min SEED=26000 OUTF=results/e51_v7c.json python3 experiments/e49_organic_general.py 10`; eval `SRC=results/e51_v7c.json TAG=v7c BASE=results/e50_eval_all.json OUT=results/e53_eval_all.json python3 experiments/e50_eval_v7b.py` | `e51_v7c.json`, `e53_eval_all.json` |
| E52/54 imagination | 38/38b/38c/38d | `e52_imagination.py`; `EVENTS=1 OFFS=100 OUTJ=results/e52b_imagination.json …`; `EVENTS=1 NODIR=1 OFFS=200 OUTJ=results/e52c_imagination.json …`; `e54_imagination_rep.py` | `e52*.json`, `e54.log` |
| E55–58 steering/imagination | 39/39b/40/41 | `e55_steer_imagine.py`; `e56_greedy_rep.py`; `STABLE=1 SEED0=29000 MODES=greedy,imagine OUTS=results/e57_stable.json …`; `STABLE=1 REL=1 SEED0=30000 MODES=greedy,imagine OUTS=results/e58_relational.json PREV=results/e57_stable.json …` | `e55_steer.json`, `e56*.json`, `e57*.json`, `e58*.json` |
| E59 organic v8 | 42 | `e59_organic_steer.py 10` (~9 min/gen uncompiled) | `e59_v8.json` |
| E60 optimiser race | 43 | `e60_opt_race.py GA`, `e60_opt_race.py OPT2` | `e60_GA.json`, `e60_OPT2.json` |
| E61 never-seen eval (steering genomes) | 42/43 | `e61_eval_steer.py TAG SRC` | `e62_eval_all.json` |
| analyses | — | `a47_organic_genes.py`; `a49_mind_printout.py [elite] [hard|rich] [side] [hours]`; `a50_evolution_stats.py`; `SRC=… a52_v7b_genes.py`; `a51_project_stats.py` (verdict counts, pass rate) | `results/a*.log` |
| figures | — | `a34`, `a37`, `a38`, `a43`, `a45` → `results/worm_*.png` | |

## 6. Environment (cloud container, 2 CPUs, ~8 GB RAM)
`pip install --break-system-packages numpy scipy scikit-learn matplotlib numba` (numba 0.68); LLM work also needs
`torch` (CPU wheel from download.pytorch.org) and `transformers`; models cached in `/home/claude/hfcache` (GPT-2,
Qwen2.5-0.5B-Instruct; ~1.5 GB) — use `HF_HOME=/home/claude/hfcache`. FEC phase also used POT and jax. Python 3.13.
Timings: 100 embodied worms ≈ 0.4 s per 1,000 steps (1 step = 1 s); organic brain ≈ 2.1 s per 1,000 steps for 50 worms,
2.3× faster with `fastbrain`; one evolution generation with three 24 h lives ≈ 225 s compiled / ≈ 545 s uncompiled.

## 7. Conventions
- Seeds: every experiment has its own seed block (see pre-registrations); never reuse a seed block for a new claim.
- Pre-registration file first (`results/PREREG<n>.md`), then run, then append "## Outcome" with PASS/FAIL per prediction.
- After each result: ledger row(s) (`results/LEDGER.md`, insert after the last row), `docs/ai_branch.md` §6.x section,
  §0 findings list, §8 next steps, `docs/principles.md` if a principle changes, a dated line at the end of `docs/brief.md`
  (master copy at `/home/claude/GR_FEC_research_brief.md`, copied to `docs/brief.md`), rebuild `/home/claude/gr-fec.zip` with
  `cd /home/claude && rm -f gr-fec.zip && zip -qr gr-fec.zip gr-fec -x "*/__pycache__/*" -x "*.npz"`.
- Energy is the body's energy store E ∈ [0, 1]; "late energy" = mean over the last day of a life (h24–48 for 48 h; h48–72 for 72 h).

## 8. Gotchas (all learned the hard way)
- `pkill -f <pattern>` kills the calling shell if the pattern appears in its own command line (exit 144). Kill by PID.
- `pgrep -f` / `ps | grep` inside an `until` loop can match the loop itself and never exit — use `ps aux | grep "[p]ython3 experiments/eNN"`.
- Foreground `sleep` > a few seconds is blocked; wait with `until …; do sleep 30; done` (≤ 590 s per call) or `run_in_background`.
- Processes started with `nohup … &` may be stopped a few minutes after a reply ends; long runs must be watched within a turn
  or started with `run_in_background`.
- `multiprocessing.Pool(2)` workers must set `LETA_LO/LETA_HI` in their own env before importing `evolve2/3` (the gene range is
  read at import).
- `opt2.subset` slices arrays whose first dimension equals n (or second dimension for `hist` (21, n)); it would mis-slice if n
  equals another dimension (e.g. n = 21 or n = 8) — avoid those population sizes.
- Random layouts: the harsh-world side generator was biased in practice (8/10 warm in PREREG35); force sides explicitly.
- Fitness comparisons across generations are confounded by per-generation layouts; use a fixed panel of layouts for clean
  optimiser comparisons (next step).
- Numba brain is not bit-identical to the numpy brain (floating-point order + chaos); compare statistically.

## 9. Open threads, prioritised (also in `docs/ai_branch.md` §8)
1. **Fixed evaluation panel** (several fixed layouts per world type, common random numbers) for all future evolution and
   optimiser comparisons.
2. **Evolve on top of the best structure:** e45 (designed learning) + greedy value-steering + evolvable numbers, judged by
   worst-of-several lives — expected new state of the art; then progressively release designed parts into the genome.
3. **Optimiser:** CMA-ES on the e45-style genome; freeze neutral genes (~25 %); cull on several short lives, not one.
4. **Imagination** only in worlds where one-step greed must fail (detours, barriers, valleys to cross).
5. **Grand-pattern tally:** repeat evolutions (≥ 3 seeds) and count recurring motifs (trust, value-steering, gating, wobble).
6. **Realism:** turning rates 3–5× too high; slowing on food; reversal→omega turns; sensor adaptation; quiescence.
7. Compare internal state flow with real worm whole-brain states (Kato et al. 2015); then fruit fly (`principles.md` §F).
8. Unrun: E32 (LLM generation phases, PREREG21); hunger-gating hyperedge test needs more power (L70); H-AI15 noise genes per stage.
