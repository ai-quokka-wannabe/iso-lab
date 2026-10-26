"""Evolvable brain (PREREG34): every number is a per-worm gene; architecture as embodied v6."""
import numpy as np
from embodied import Brain, NCH, MX, NT

GENES = {  # name: (low, high, log10?)
    'g_odour': (-3, 3, 0), 'g_salt': (-3, 3, 0), 'g_gut': (-2, 2, 0), 'r_eat': (-5, 5, 0), 'r_hunger': (-5, 5, 0),
    'trust_k': (-5, 5, 0), 'roam': (0, 5, 0), 'curiosity': (0, 2, 0), 'eta': (-5, -2, 1), 's0': (-3, -1, 1),
    'tau': (-1.5, 1, 1), 'integ': (1, 60, 0)}
NAMES = list(GENES); LO = np.array([GENES[k][0] for k in NAMES], float); HI = np.array([GENES[k][1] for k in NAMES], float)


def random_genomes(n, rng): return LO + (HI - LO) * rng.random((n, len(NAMES)))


def mutate(G, rng, sigma=0.1): return np.clip(G + sigma * (HI - LO) * rng.standard_normal(G.shape), LO, HI)


def decode(G):
    d = {k: G[:, i] for i, k in enumerate(NAMES)}
    for k in NAMES:
        if GENES[k][2]: d[k] = 10 ** d[k]
    return d


class EvoBrain(Brain):
    def __init__(self, n, rng, genome):
        super().__init__(n, rng, s0=0.01, tau=0.3, eta=5e-4, hyper=True, integ=10, separate=True)
        self.g = decode(genome); self.W[:] = 0.0; self.Dbar = None; self.nov = np.zeros(n); self.hist = []

    def step(self, y):
        g = self.g; self.births = np.zeros(self.n); F = self.features(y)
        h = np.clip(y[:, 7], 0, 1); stretch = np.clip(y[:, 6], 0, None); x = np.clip(y[:, :2], 0, None)
        r = g['r_eat'] * np.clip(y[:, 5], 0, None) - g['r_hunger'] * h
        pred = (self.W * F).sum(1)
        self.W += (g['eta'] * (r - pred))[:, None] * F
        c0 = NCH * MX; wl = self.W[:, c0:c0 + 2] + h[:, None] * self.W[:, NT + c0:NT + c0 + 2]
        t = np.clip(1 + g['trust_k'][:, None] * wl, 0, 1.5)
        D = pred + g['g_gut'] * stretch + (x * t * np.c_[g['g_odour'], g['g_salt']]).sum(1)
        self.nov = self.nov * np.exp(-1 / 300) + self.births; D = D + g['curiosity'] * self.nov
        self.Dbar = D.copy() if self.Dbar is None else self.Dbar + (D - self.Dbar) / g['integ']
        self.hist.append(self.Dbar)
        dD = self.Dbar - self.hist[-11] if len(self.hist) >= 11 else np.zeros(self.n)
        if len(self.hist) > 11: self.hist.pop(0)
        P = 0.3 / (1 + np.exp(np.clip((dD / g['s0'] + 1) / g['tau'], -50, 50)))
        P = P / (1 + g['roam'] * h * (1 - np.clip(stretch, 0, 1)))
        pir = self.rng.random(self.n) < P
        self.touch_w += (0.9 - self.touch_w) / 300; tapped = y[:, 4] > 0.5
        rev = tapped & (self.rng.random(self.n) < self.touch_w); self.touch_w = np.where(tapped, self.touch_w * 0.88, self.touch_w)
        return pir, rev
