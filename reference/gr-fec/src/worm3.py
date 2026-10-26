"""Worm v3 (PREREG27): grown concept nodes (tuned + intensity), interoceptive nodes (hunger, gut fullness),
hunger-gated hyperedges, hunger-dependent food reward. Same step() interface as worm.Worms so the e33 assays run."""
import numpy as np
import worm
from worm import R_PLATE, SPEED, HEAD_SD

MX = 30; WID = np.array([0.15, 0.15, 1.0]); NOISE = np.array([0.01, 0.01, 0.05])
NT = 3 * MX + 2            # tuned nodes + 2 intensity nodes (external senses)
NF = NT + 2 + NT           # + hunger, gut nodes + hunger-gated copies (hyperedges)


class Worm3(worm.Worms):
    def __init__(self, n, rng, s0=0.03, tau=0.3, eta=5e-4, random_walk_p=None, hyper=True, k=1.0, det=False, gates_only=False):
        self.n, self.rng, self.s0, self.tau, self.eta, self.rw = n, rng, s0, tau, eta, random_walk_p
        self.hyper, self.k, self.det, self.gates_only = hyper, k, det, gates_only
        self.W = np.zeros((n, NF)); self.W[:, 3 * MX] = 2.354; self.W[:, 3 * MX + 1] = 0.706
        if not gates_only: self.W[:, NT + 1] = 1.0                                  # gut fullness -> drive (innate)
        self.proto = np.zeros((n, 3, MX)); self.cnt = np.zeros((n, 3), int)
        self.e = np.ones(n); self.g = np.zeros(n); self.touch = np.full(n, 0.9)
        self.place(np.zeros((n, 2)))

    def place(self, xy):
        self.xy = xy.astype(float).copy(); self.hist = []
        self.head = np.linspace(0, 2 * np.pi, self.n, endpoint=False) if self.det else self.rng.uniform(0, 2 * np.pi, self.n)

    def set_state(self, hunger=None, gut=None):
        if hunger is not None: self.e = np.full(self.n, 1 - hunger)
        if gut is not None: self.g = np.full(self.n, gut)

    def sense(self, plate):
        o, s, t = plate.fields(self.xy); k = self.k
        X = np.c_[o + k * NOISE[0] * self.rng.standard_normal(self.n), s + k * NOISE[1] * self.rng.standard_normal(self.n),
                  t + k * NOISE[2] * self.rng.standard_normal(self.n)]
        S = np.zeros((self.n, NT)); ar = np.arange(self.n)
        for m in range(3):
            x = X[:, m]; exist = np.arange(MX)[None] < self.cnt[:, m][:, None]
            d = np.where(exist, np.abs(self.proto[:, m] - x[:, None]), np.inf); dmin = d.min(1); j = d.argmin(1)
            event = (x > 3 * NOISE[m]) if m < 2 else np.ones(self.n, bool)
            new = event & (dmin > WID[m]) & (self.cnt[:, m] < MX)
            kk = self.cnt[new, m]; self.proto[ar[new], m, kk] = x[new]; self.cnt[new, m] += 1
            upd = event & ~new & np.isfinite(dmin)
            self.proto[ar[upd], m, j[upd]] += 0.005 * (x[upd] - self.proto[ar[upd], m, j[upd]])
            exist = np.arange(MX)[None] < self.cnt[:, m][:, None]
            act = np.exp(-(self.proto[:, m] - x[:, None]) ** 2 / (2 * WID[m] ** 2)) * exist
            if m < 2: act = act * event[:, None]
            S[:, m * MX:(m + 1) * MX] = act
        S[:, 3 * MX:3 * MX + 2] = np.clip(X[:, :2], 0, None)
        h = np.clip(1 - self.e + 0.02 * self.rng.standard_normal(self.n), 0, 1)
        g = np.clip(self.g + 0.02 * self.rng.standard_normal(self.n), 0, 1)
        if self.gates_only: h_, g_ = np.zeros_like(h), np.zeros_like(g)          # v3b: inner senses gate, do not predict
        else: h_, g_ = h, g
        F = np.hstack([S, h_[:, None], g_[:, None], S * h[:, None] if self.hyper else np.zeros_like(S)])
        return F

    def step(self, plate, on_food=False, move=True, tap=False):
        F = self.sense(plate)
        if on_food:
            self.e += (1 - self.e) / 600; self.g += (1 - self.g) / 120
        else:
            self.e *= np.exp(-1 / 1800); self.g *= np.exp(-1 / 300)
        h = 1 - self.e
        v = np.where(on_food, 0.6 + 0.8 * h, -h)
        pred = (self.W * F).sum(1)
        self.W += self.eta * (v - pred)[:, None] * F
        D = pred + (self.g if self.gates_only else 0.0)
        self.hist.append(D)
        dD = D - self.hist[-5] if len(self.hist) >= 5 else np.zeros(self.n)
        if len(self.hist) > 5: self.hist.pop(0)
        if self.det: pir = (-dD / self.s0 - 1) > 0
        else:
            P = np.full(self.n, self.rw) if self.rw is not None else 0.3 / (1 + np.exp(np.clip((dD / self.s0 + 1) / self.tau, -50, 50)))
            pir = self.rng.random(self.n) < P
        rev = np.zeros(self.n, bool); self.touch += (0.9 - self.touch) / 300
        if tap: rev = self.rng.random(self.n) < self.touch; self.touch *= 0.88
        if move:
            if self.det: self.head = self.head + 2.4 * pir
            else: self.head = np.where(pir, self.rng.uniform(0, 2 * np.pi, self.n), self.head) + HEAD_SD * self.rng.standard_normal(self.n)
            self.xy += SPEED * np.c_[np.cos(self.head), np.sin(self.head)]
            r = np.linalg.norm(self.xy, axis=1); out = r > R_PLATE
            if out.any(): self.xy[out] *= (R_PLATE / r[out])[:, None]; self.head[out] += np.pi
        return pir, rev
