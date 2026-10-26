# Pre-registration 2 — written 2026-10-24 04:30:00 UTC, BEFORE analysing confirmation data

Code: CCSDS TC LDPC (128,64), sum-product BP, 50 iterations, BPSK/AWGN, all-zero codeword.
Signal: lam = ||J u|| for the BP map Jacobian J (finite-difference JVP, 2 warm-started power-iteration
probes per iteration, eps=1e-4), plus lam_trend (log change over 3 iterations).
Baseline: syndrome weight, its 3-iteration trend, log mean |posterior LLR| and its trend, number of hard-decision
flips, log message-change norm, log message-change ratio, PLUS the full per-iteration history of syndrome weight,
log mean |LLR| and log message-change norm up to the checkpoint ("hist").
Models: standardised logistic regression (C=1), FITTED ON EXPLORATORY DATA ONLY (3.0 dB seed 100; 3.5 dB seed 101),
then FROZEN and evaluated on confirmation data (3.0 dB seed 200, 3.5 dB seed 201, 24 000 frames each).
Task: among frames still unconverged at checkpoint t0, predict "will not converge by iteration 50".

PRIMARY (P1): at 3.0 dB, t0 = 10: AUC(hist+curv) − AUC(hist) > 0 with bootstrap 95% CI lower bound > 0.
SECONDARY (S1): same at 3.5 dB, t0 = 10.
SECONDARY (S2): at 3.0 dB, t0 = 10, single-feature AUC of lam >= the best single baseline feature AUC.
Reported but not claimed: other checkpoints, stopping-rule iteration savings (threshold too noisy at a 1% FER budget).
Known limitation stated in advance: lam probing as implemented triples decoder compute; practical value requires a
cheaper estimator (checkpoint-only probing) — not tested here.
