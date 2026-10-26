"""Post-processing of failed BP frames: bit-selection rules + 'guess and re-decode' (augmented BP style).

Selection rules (higher score = try first):
  lyap   : sensitivity of the bit's posterior LLR along the Lyapunov vector (dominant stretching direction) of the
           BP map at the final state (warm-started power iteration).
  unrel  : smallest |posterior LLR| at the final iteration (classic reliability).
  osc    : number of hard-decision sign flips over the last W iterations (oscillation, Gounai & Ohtsuki 2005).
  chan   : smallest |channel LLR|.
  random : random order (control).
"""
import numpy as np
from bp_track import TrackBP


def final_state(dec, llr, T=50, W=20, K=20, eps=1e-4, seed=0):
    rng = np.random.default_rng(seed)
    B = llr.shape[0]
    v2c = llr[:, dec.ev].copy()
    flips = np.zeros((B, dec.n)); prev = (llr < 0)
    for it in range(T):
        new, tot, c2v = dec.step(v2c, llr)
        hard = tot < 0
        if it >= T - W:
            flips += (hard != prev)
        prev = hard
        v2c = new
    # Lyapunov vector at the final state
    base, tot0, _ = dec.step(v2c, llr)
    u = rng.standard_normal((B, dec.E)); u /= np.linalg.norm(u, axis=1, keepdims=True)
    for _ in range(K):
        pert, _, _ = dec.step(v2c + eps * u, llr)
        Ju = (pert - base) / eps
        u = Ju / np.maximum(np.linalg.norm(Ju, axis=1), 1e-30)[:, None]
    _, tot_p, _ = dec.step(v2c + eps * u, llr)
    lyap = np.abs(tot_p - tot0) / eps
    return {'lyap': lyap, 'unrel': -np.abs(tot0), 'osc': flips + 1e-3 * (-np.abs(tot0)),   # tie-break by reliability
            'chan': -np.abs(llr), 'tot': tot0}


def guess_redecode(bp, llr, order, k, tot, big=50.0, iters=50):
    """For each frame try bits order[:, j] for j < k, each with value 0 then 1 (LLR = +big / -big), re-decoding from
    scratch. Returns (rescued_correct, rescued_wrong) masks: first successful attempt per frame decides."""
    B, n = llr.shape
    done = np.zeros(B, bool); correct = np.zeros(B, bool)
    for j in range(k):
        for attempt in (0, 1):   # attempt 0: flip the decoder's current belief about the bit; attempt 1: reinforce it
            act = np.nonzero(~done)[0]
            if len(act) == 0:
                break
            bits = order[act, j]
            cur = np.sign(tot[act, bits]); cur[cur == 0] = 1.0
            L = llr[act].copy(); L[np.arange(len(act)), bits] = (-cur if attempt == 0 else cur) * big
            hard, conv, _ = bp.decode(L, iters=iters)
            ok = conv
            done[act[ok]] = True
            correct[act[ok]] = hard[ok].sum(1) == 0          # all-zero codeword sent
    return correct, done & ~correct
