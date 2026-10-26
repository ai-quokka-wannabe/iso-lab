# PREREG37 — organic v7c: fitness = worst of three lives, 24 h lives, learning cannot be switched off
Written 25 Oct 2026 before any run. Brain = organic v7 (`src/evolve2.py`) except the learning-rate gene range
log10 η ∈ [−3, 0] (was [−5, −1]): evolution can make learning slow but not absent. Fresh start, seed 26000, 100 genomes,
each lives three **24 h** lives per generation (harsh cold band, harsh warm band, rich; new random layouts);
**fitness = minimum** of the three lifetime mean energies. Top 20 + 4 mutated children each. 10 generations (time-boxed,
≈ 10 min/generation). Full populations logged. Test: never-seen worlds of PREREG35 (seeds 23000–23002), elites 10 × 5, 48 h.
P37a: never-seen cold band ≥ 0.70. P37b: final elite median |temperature instinct| < 1. P37c: learned trust in smell > 0
(grand-pattern tally). P37d: ≥ e45 − 0.05 in all three layouts (0.836, 0.828, 0.892).

## Outcome (recorded after running; evolution 20:41–21:32 UTC, test after)
Evolution (fitness = worst of three lives): elite min-fitness 0.570 → 0.715 (gen 6) / 0.677 (gen 9); lives balanced at the end
(cold 0.712, warm 0.692, rich 0.753). Genes: learning rate 10^−0.56 (near the top; learning stayed on; gene–fitness r +0.20);
temperature instinct +2.78 (**P37b FAIL**) but temperature trust −4.40 (experience can override it); **learned trust in smell
+3.30 (P37c PASS)**. Strongest gene–fitness correlations: turning sensitivity s0 r +0.40, novelty threshold +0.28, trend lag −0.26;
21 of 73 genes selection-neutral.
Never-seen worlds (energy h24–48): cold **0.690** (**P37a FAIL**, by 0.01), warm 0.703, rich 0.780 — worst case 0.690, best of all
organic brains (v7b 0.576, v7 0.442) but below e45 (0.828). **P37d FAIL.**
Grand-pattern tally: learned trust in smell positive in all three evolutions where learning was on (e45 +2.92, v7 +3.68,
v7c +3.30); neutral/drifting in the one where learning was switched off (v7b).
