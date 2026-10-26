"""BP with a per-iteration temperature schedule alpha_t (channel-LLR scaling) and optional message damping."""
import numpy as np
from bp_track import TrackBP


def decode_sched(dec, llr, alphas, damping=0.0, return_hard=False):
    B = llr.shape[0]; T = len(alphas)
    conv = np.full(B, T + 1); idx = np.arange(B); hard_out = np.zeros((B, dec.n), np.uint8)
    L = llr; v2c = alphas[0] * L[:, dec.ev]
    for it in range(T):
        new, tot, c2v = dec.step(v2c, alphas[it] * L)
        if damping > 0 and it > 0:
            new = (1 - damping) * new + damping * v2c
        hard = (tot < 0).astype(np.uint8)
        ok = (np.asarray((dec.Hs @ hard.T.astype(np.float64)).T) % 2).sum(1) == 0
        conv[idx[ok]] = it + 1; hard_out[idx] = hard
        keep = ~ok
        if not keep.any():
            break
        idx, L, v2c = idx[keep], L[keep], new[keep]
    return (conv, hard_out) if return_hard else conv


def schedules(T=50, rng=None):
    t = np.arange(T)
    lin = lambda a, b, n: np.concatenate([np.geomspace(a, b, n), np.full(T - n, b)])
    rng = rng or np.random.default_rng(0)
    return {
        'const 1.0':            (np.ones(T), 0.0),
        'damping 0.3':          (np.ones(T), 0.3),
        'damping 0.5':          (np.ones(T), 0.5),
        'anneal hot->1 (20)':   (lin(0.5, 1.0, 20), 0.0),
        'quench cold->1 (20)':  (lin(2.0, 1.0, 20), 0.0),
        'cool 1->2 (50)':       (np.geomspace(1.0, 2.0, T), 0.0),
        'heat 1->0.5 (50)':     (np.geomspace(1.0, 0.5, T), 0.0),
        'temper cycle p10':     (np.exp(0.4 * np.sin(2 * np.pi * t / 10)), 0.0),
        'temper cycle p4':      (np.exp(0.4 * np.sin(2 * np.pi * t / 4)), 0.0),
        'jitter logU(0.6,1.6)': (np.exp(rng.uniform(np.log(0.6), np.log(1.6), T)), 0.0),
    }
