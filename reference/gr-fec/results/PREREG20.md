# PREREG20 — gravity gate on a language model's factual answers (H-AI2, H-AI5)
Written 24 Oct 2026 before any run of e31.

Model: Qwen2.5-0.5B-Instruct (CPU, float32), chat template, system prompt default. Questions (≈ 460): chemical symbol
of each element; element named by each symbol; capital of each country/territory in `results/e31_facts.json` after
removing contested/ambiguous entries, with an alias list for accepted alternatives (both fixed in e31 before running).
Answer = greedy decoding, ≤ 12 new tokens. Correctness by normalised string match (lower-case, accents and
punctuation removed; symbols matched exactly on the first word).
Candidates for the runner-up: greedy continuations from the 2nd–5th most likely first tokens, plus 8 samples at T = 1
(seed 2000). All log-probabilities computed by a forward pass, including the end-of-turn token.
Scores (higher = trust): meanlp = mean token log-prob of the answer; lp = total log-prob; gap = lp(answer) − max lp of
distinct other candidates; action = −lp(answer) (surprise: how far from certainty the answer is); r = gap/max(action,
1e-3); agree = share of the 8 samples equal to the greedy answer (cheap semantic-entropy proxy).

P20a: AUROC(r) ≥ AUROC(meanlp) − 0.01.
P20b: AUROC(r) ≥ AUROC(agree) − 0.01.
S20: AUROC(gap) > AUROC(meanlp) (margin beats confidence).
H-AI5 (descriptive): share of wrong answers that are "frozen" (agree ≥ 0.75) vs "chaotic" (agree ≤ 0.25); report each
score's AUROC within the frozen-wrong vs correct-and-frozen subset.
