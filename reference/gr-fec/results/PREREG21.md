# PREREG21 — frozen and chaotic phases of language-model generation (H-AI3, H-AI4)
Written 24 Oct 2026 before any run of e32.

Generator: GPT-2 small, pure sampling at temperature T ∈ {0.3, 0.5, 0.7, 0.85, 1.0, 1.15, 1.3, 1.6}, 60 fixed prompts
(listed in e32), 130 new tokens. Sampling by inverse CDF with a per-prompt stream of uniforms u_t (seed 2100 + prompt).
- Frozen measure: loop flag = ≥ 50 % of the 4-grams in the last 60 generated tokens already occurred earlier in the
  generation. Loop share L(T).
- Chaos measure (common random numbers, the LLM analogue of the FTLE): twin trajectory B identical to A except that at
  step 10 it takes A's runner-up token; afterwards both use the same u_t. Similarity J = Jaccard overlap of the 4-gram
  sets of the last 60 tokens of A and B. Frozen = twins re-merge (J high); chaotic = twins separate (J low).
- Onsets (interpolated in log T): T_L where L crosses 0.5; T_c where median J crosses 0.5.
- Quality (Matej's "creativity inside hyperwalls"): judge = Qwen2.5-0.5B-Instruct as a plain LM. Re-tokenise prompt +
  generation; Q = share of generated judge-tokens that are both novel (the 4-gram ending at the token has not occurred earlier in the
  generation) and plausible (judge probability ≥ 1 %). T* = argmax of mean Q.

P21a: Spearman(T, L) < 0 and Spearman(T, median J) < 0.
P21b (phases coincide): |ln(T_L / T_c)| ≤ ln 1.25.
P21c (edge of chaos): T_c ≤ T* ≤ 1.5 T_c.
If an onset is not crossed inside the grid the corresponding prediction is declared failed (not re-gridded).
