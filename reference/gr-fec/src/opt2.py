"""Optimiser v2 (PREREG43): self-adaptive per-gene mutation (evolution-strategy style) + successive halving of lives.
Works on evolve3 genomes. Helper `subset` slices any per-worm simulation object (body, brain, world) to surviving worms."""
import numpy as np

def subset(obj, idx, n):
    for k, v in list(vars(obj).items()):
        if isinstance(v, np.ndarray):
            if v.ndim >= 1 and v.shape[0] == n: setattr(obj, k, v[idx].copy())
            elif v.ndim == 2 and v.shape[1] == n: setattr(obj, k, v[:, idx].copy())
    if hasattr(obj, 'n'): obj.n = len(idx)
    return obj

def init_sigma(G, LO, HI, frac=0.1): return np.full(G.shape, frac) * (HI - LO)

def sa_mutate(G, S, LO, HI, rng, discrete, C):
    """Schwefel self-adaptation: σ' = σ·exp(τ'·N(0,1) + τ·N_i(0,1)); x' = x + σ'·N_i(0,1)."""
    m = G.shape[1]; tau_g = 1 / np.sqrt(2 * m); tau_l = 1 / np.sqrt(2 * np.sqrt(m))
    S2 = S * np.exp(tau_g * rng.standard_normal((len(G), 1)) + tau_l * rng.standard_normal(G.shape))
    S2 = np.clip(S2, 1e-3 * (HI - LO), 0.5 * (HI - LO))
    G2 = np.clip(G + S2 * rng.standard_normal(G.shape), LO, HI)
    G2[:, discrete] = G[:, discrete]
    flip = rng.random((len(G), len(discrete))) < 0.1
    G2[:, discrete] = np.where(flip, rng.integers(0, C, (len(G), len(discrete))), G2[:, discrete])
    return G2, S2
