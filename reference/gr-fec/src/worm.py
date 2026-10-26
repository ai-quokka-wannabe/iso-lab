"""Worm-level hypergraph agent (v0) and agar-plate simulator. See results/PREREG22.md for the specification.

Hypergraph: tuned sensory nodes (odour, salt, temperature) + internal nodes (food-memory trace m, hunger h)
-> valence node V via edges W (innate prior + learned by prediction error); motor nodes forward / reverse / turn.
Thinking = drive D = W.s + m; acting = pirouette when D is falling (klinokinesis), with exploration temperature tau.
"""
import numpy as np

R_PLATE, SPEED, HEAD_SD = 25.0, 0.2, 0.05
OB = np.linspace(0, 1, 8); SB = np.linspace(0, 1, 8); TB = np.linspace(15, 25, 11)
NS = len(OB) + len(SB) + len(TB)
SL_O, SL_S, SL_T = slice(0, 8), slice(8, 16), slice(16, 27)


class Plate:
    def __init__(self, odour=False, salt=False, thermal=False, temp=20.0, salt_uniform=None, peak=(15.0, 0.0), sigma=12.0):
        self.odour, self.salt, self.thermal, self.temp, self.salt_uniform = odour, salt, thermal, temp, salt_uniform
        self.peak, self.sigma = np.array(peak), sigma

    def _bump(self, xy):
        return np.exp(-((xy - self.peak) ** 2).sum(1) / (2 * self.sigma ** 2))

    def fields(self, xy):
        n = len(xy)
        o = self._bump(xy) if self.odour else np.zeros(n)
        if self.salt_uniform is not None: s = np.full(n, self.salt_uniform)
        else: s = self._bump(xy) if self.salt else np.zeros(n)
        t = 20 + 5 * xy[:, 0] / R_PLATE if self.thermal else np.full(n, float(self.temp))
        return o, s, t


class Worms:
    def __init__(self, n, rng, s0=0.01, tau=0.3, eta=5e-4, random_walk_p=None, rectify=False):
        self.n, self.rng, self.s0, self.tau, self.eta, self.rw, self.rectify = n, rng, s0, tau, eta, random_walk_p, rectify
        self.W = np.zeros((n, NS)); self.W[:, SL_O] = OB; self.W[:, SL_S] = 0.3 * SB   # innate edges
        self.m = np.zeros(n); self.toff = np.full(n, 0.0); self.touch = np.full(n, 0.9)
        self.place(np.zeros((n, 2)))

    def place(self, xy):
        self.xy = xy.astype(float).copy(); self.head = self.rng.uniform(0, 2 * np.pi, self.n); self.hist = []

    def sense(self, plate):
        o, s, t = plate.fields(self.xy)
        o = o + 0.01 * self.rng.standard_normal(self.n); s = s + 0.01 * self.rng.standard_normal(self.n)
        t = t + 0.05 * self.rng.standard_normal(self.n)
        g = lambda x, c, w: np.exp(-(x[:, None] - c[None]) ** 2 / (2 * w ** 2))
        S = np.hstack([g(o, OB, 0.15), g(s, SB, 0.15), g(t, TB, 1.0)])
        if self.rectify: S[:, 0] = 0.0; S[:, 8] = 0.0          # v1: no node fires for absence
        return S

    def step(self, plate, on_food=False, move=True, tap=False):
        s = self.sense(plate)
        if on_food: self.m[:] = 1.0; self.toff[:] = 0.0
        else: self.m *= np.exp(-1 / 300); self.toff += 1.0
        h = 1 - np.exp(-self.toff / 1800)
        v = np.where(on_food, 1.0, -h)
        pred = (self.W * s).sum(1)
        self.W += self.eta * (v - pred)[:, None] * s                       # learned edges (prediction error)
        D = pred + self.m                                                  # drive = path sum to valence
        self.hist.append(D)
        dD = D - self.hist[-5] if len(self.hist) >= 5 else np.zeros(self.n)
        if len(self.hist) > 5: self.hist.pop(0)
        if self.rw is not None: P = np.full(self.n, self.rw)
        else: P = 0.3 / (1 + np.exp(np.clip((dD / self.s0 + 1) / self.tau, -50, 50)))
        pir = self.rng.random(self.n) < P
        rev = np.zeros(self.n, bool)
        self.touch += (0.9 - self.touch) / 300
        if tap:
            rev = self.rng.random(self.n) < self.touch; self.touch *= 0.88
        if move:
            self.head = np.where(pir, self.rng.uniform(0, 2 * np.pi, self.n), self.head) + HEAD_SD * self.rng.standard_normal(self.n)
            self.xy += SPEED * np.c_[np.cos(self.head), np.sin(self.head)]
            r = np.linalg.norm(self.xy, axis=1); out = r > R_PLATE
            if out.any():
                self.xy[out] *= (R_PLATE / r[out])[:, None]; self.head[out] += np.pi
        return pir, rev


def chemotaxis_index(xy, target=(15.0, 0.0), control=(-15.0, 0.0), radius=8.0):
    nt = (np.linalg.norm(xy - np.array(target), axis=1) < radius).sum()
    nc = (np.linalg.norm(xy - np.array(control), axis=1) < radius).sum()
    return (nt - nc) / len(xy)
