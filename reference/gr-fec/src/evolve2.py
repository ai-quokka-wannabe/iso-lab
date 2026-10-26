"""Organic brain v7 (PREREG35): the architecture lives in the genome.

All 9 input channels are treated alike: 8 senses + the brain's own novelty trace. Temperature is rescaled to (T-20)/5.
Per channel genes: grow nodes?, node width, event threshold, instinct gain, trust strength, reward weight, turning bias.
Global genes: learning rate, learned-drive gain, integration time, trend lag, turning sensitivity s0, exploration tau,
max turn rate, novelty decay, two gate channels (+ on/off).
Designed substrate kept: novelty-gate node growth, prediction-error learning (normalised LMS), multiplicative trust, drive -> leaky
integration -> trend -> sigmoid turning, touch reflex with depression.
"""
import os
import numpy as np

C = 9; MXC = 12                                   # channels; max grown nodes per channel
CH = ['odour', 'salt', 'N', 'temp', 'touch', 'pharynx', 'stretch', 'hunger', 'novelty']
PER = {'grow': (-1, 1), 'lwidth': (-2, 0), 'thr': (-0.1, 0.5), 'inst': (-3, 3), 'trust': (-5, 5), 'rew': (-5, 5), 'bias': (-3, 3)}
GLOB = {'leta': (float(os.environ.get('LETA_LO', -5)), float(os.environ.get('LETA_HI', -1))), 'lam': (-3, 3), 'integ': (1, 120), 'lag': (1, 20), 'ls0': (-3, 0), 'ltau': (-2, 1),
        'pmax': (0.05, 0.5), 'lnov': (1, 4), 'gA_on': (-1, 1), 'gB_on': (-1, 1)}
NAMES = [f'{k}_{c}' for k in PER for c in CH] + list(GLOB) + ['gA', 'gB']
LO = np.array([PER[k][0] for k in PER for _ in CH] + [GLOB[k][0] for k in GLOB] + [0, 0], float)
HI = np.array([PER[k][1] for k in PER for _ in CH] + [GLOB[k][1] for k in GLOB] + [C - 1, C - 1], float)
DISCRETE = [NAMES.index('gA'), NAMES.index('gB')]


def random_genomes(n, rng):
    G = LO + (HI - LO) * rng.random((n, len(NAMES))); G[:, DISCRETE] = rng.integers(0, C, (n, 2)); return G


def mutate(G, rng, sigma=0.1, p_disc=0.1):
    M = np.clip(G + sigma * (HI - LO) * rng.standard_normal(G.shape), LO, HI)
    M[:, DISCRETE] = G[:, DISCRETE]
    flip = rng.random((len(G), 2)) < p_disc
    M[:, DISCRETE] = np.where(flip, rng.integers(0, C, (len(G), 2)), M[:, DISCRETE]); return M


def per(G, k): return G[:, [NAMES.index(f'{k}_{c}') for c in CH]]
def glob(G, k): return G[:, NAMES.index(k)]


class OrganicBrain:
    def __init__(self, n, rng, G):
        self.n, self.rng, self.G = n, rng, G
        self.grow = per(G, 'grow') > 0; self.width = 10 ** per(G, 'lwidth'); self.thr = per(G, 'thr')
        self.inst = per(G, 'inst'); self.trk = per(G, 'trust'); self.rew = per(G, 'rew'); self.bias = per(G, 'bias')
        self.eta = 10 ** glob(G, 'leta'); self.lam = glob(G, 'lam'); self.integ = glob(G, 'integ'); self.lag = np.round(glob(G, 'lag')).astype(int)
        self.s0 = 10 ** glob(G, 'ls0'); self.tau = 10 ** glob(G, 'ltau'); self.pmax = glob(G, 'pmax'); self.nov_tau = 10 ** glob(G, 'lnov')
        self.gA = G[:, NAMES.index('gA')].astype(int); self.gB = G[:, NAMES.index('gB')].astype(int)
        self.gAon = glob(G, 'gA_on') > 0; self.gBon = glob(G, 'gB_on') > 0
        self.NS = C * MXC + C
        self.W = np.zeros((n, 3 * self.NS)); self.proto = np.zeros((n, C, MXC)); self.cnt = np.zeros((n, C), int)
        self.nov = np.zeros(n); self.Dbar = None; self.hist = np.zeros((21, n)); self.t = 0
        self.touch_w = np.full(n, 0.9)

    def step(self, y):
        n = self.n; ar = np.arange(n)
        z = np.c_[y[:, :3], (y[:, 3] - 20) / 5, y[:, 4:8], self.nov]
        S = np.zeros((n, self.NS)); births = np.zeros(n)
        for c in range(C):
            x = z[:, c]; w = self.width[:, c]; exist = np.arange(MXC)[None] < self.cnt[:, c][:, None]
            d = np.where(exist, np.abs(self.proto[:, c] - x[:, None]), np.inf); dmin = d.min(1); j = d.argmin(1)
            event = self.grow[:, c] & (np.abs(x) > self.thr[:, c])
            new = event & (dmin > w) & (self.cnt[:, c] < MXC)
            k = self.cnt[new, c]; self.proto[ar[new], c, k] = x[new]; self.cnt[new, c] += 1; births += new
            upd = event & ~new & np.isfinite(dmin)
            self.proto[ar[upd], c, j[upd]] += 0.005 * (x[upd] - self.proto[ar[upd], c, j[upd]])
            exist = np.arange(MXC)[None] < self.cnt[:, c][:, None]
            act = np.exp(-(self.proto[:, c] - x[:, None]) ** 2 / (2 * w[:, None] ** 2)) * exist * event[:, None]
            S[:, c * MXC:(c + 1) * MXC] = act
        S[:, C * MXC:] = z
        gA = np.where(self.gAon, z[ar, self.gA], 0.0); gB = np.where(self.gBon, z[ar, self.gB], 0.0)
        F = np.hstack([S, S * gA[:, None], S * gB[:, None]])
        r = (self.rew * z).sum(1)                                            # evolved: what feels good / bad
        pred = (self.W * F).sum(1)
        self.W += (self.eta * (r - pred) / (1 + (F * F).sum(1)))[:, None] * F      # normalised LMS (stable)
        wl = self.W[:, C * MXC:C * MXC + C]                                  # learned value of each channel's intensity
        t = np.clip(1 + self.trk * wl, 0, 1.5)
        D = self.lam * pred + (self.inst * t * z).sum(1)
        self.Dbar = D.copy() if self.Dbar is None else self.Dbar + (D - self.Dbar) / self.integ
        self.hist = np.roll(self.hist, 1, axis=0); self.hist[0] = self.Dbar; self.t += 1
        past = self.hist[np.minimum(self.lag, 20), ar]
        dD = np.where(self.t > self.lag, self.Dbar - past, 0.0)
        logit = (-dD / self.s0 - 1) / self.tau + (self.bias * z).sum(1)
        P = self.pmax / (1 + np.exp(-np.clip(logit, -50, 50)))
        pir = self.rng.random(n) < P
        self.nov = self.nov * np.exp(-1 / self.nov_tau) + births
        self.touch_w += (0.9 - self.touch_w) / 300; tapped = y[:, 4] > 0.5
        rev = tapped & (self.rng.random(n) < self.touch_w); self.touch_w = np.where(tapped, self.touch_w * 0.88, self.touch_w)
        return pir, rev
