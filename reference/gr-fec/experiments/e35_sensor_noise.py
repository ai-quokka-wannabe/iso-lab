"""PREREG24: sensor noise as the only source of randomness in a deterministic worm."""
import sys, numpy as np, warnings
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
import worm
from worm import Worms, Plate

class DetWorm(Worms):
    def __init__(self, n, rng, k, **kw):
        super().__init__(n, rng, rectify=True, s0=0.03, **kw); self.k = k
        self.head = np.linspace(0, 2 * np.pi, n, endpoint=False)
    def place(self, xy):
        super().place(xy); self.head = np.linspace(0, 2 * np.pi, self.n, endpoint=False)
    def sense(self, plate):
        o, s, t = plate.fields(self.xy); k = self.k
        o = o + 0.01 * k * self.rng.standard_normal(self.n); s = s + 0.01 * k * self.rng.standard_normal(self.n)
        t = t + 0.05 * k * self.rng.standard_normal(self.n)
        g = lambda x, c, w: np.exp(-(x[:, None] - c[None]) ** 2 / (2 * w ** 2))
        S = np.hstack([g(o, worm.OB, 0.15), g(s, worm.SB, 0.15), g(t, worm.TB, 1.0)]); S[:, 0] = 0; S[:, 8] = 0
        return S
    def step(self, plate):
        s = self.sense(plate); self.m *= np.exp(-1 / 300); self.toff += 1
        h = 1 - np.exp(-self.toff / 1800); pred = (self.W * s).sum(1)
        self.W += self.eta * (-h - pred)[:, None] * s
        D = pred + self.m; self.hist.append(D)
        dD = D - self.hist[-5] if len(self.hist) >= 5 else np.zeros(self.n)
        if len(self.hist) > 5: self.hist.pop(0)
        turn = (-dD / self.s0 - 1) > 0
        self.head = self.head + 2.4 * turn
        self.xy += worm.SPEED * np.c_[np.cos(self.head), np.sin(self.head)]
        r = np.linalg.norm(self.xy, axis=1); out = r > worm.R_PLATE
        if out.any(): self.xy[out] *= (worm.R_PLATE / r[out])[:, None]; self.head[out] += np.pi
        return turn

class TrapPlate(Plate):
    def fields(self, xy):
        o, s, t = super().fields(xy)
        o = o + 0.6 * np.exp(-((xy - np.array([-10.0, 0])) ** 2).sum(1) / (2 * 6.0 ** 2)); return o, s, t

KS = [0, 0.1, 0.3, 1, 3, 10, 30]; best = {}
for name, P in (('W1 single peak', Plate(odour=True)), ('W2 trap world', TrapPlate(odour=True))):
    res = {}
    for k in KS:
        cis, stk = [], []
        for sd in range(4):
            w = DetWorm(100, np.random.default_rng(6000 + 100 * KS.index(k) + sd + (50 if 'W2' in name else 0)), k)
            w.place(np.zeros((100, 2))); hist = []
            for t in range(1200):
                w.step(P)
                if t == 899: x900 = w.xy.copy()
            nt = (np.linalg.norm(w.xy - [15, 0], axis=1) < 8).sum(); nc = (np.linalg.norm(w.xy - [-10, 0], axis=1) < 8).sum()
            cis.append((nt - nc) / 100)
            stk.append(((np.linalg.norm(w.xy - x900, axis=1) < 2) | (np.linalg.norm(w.xy - [-10, 0], axis=1) < 6)).mean())
        res[k] = (np.mean(cis), np.std(cis) / 2, np.mean(stk))
    b = max(res, key=lambda k: res[k][0]); best[name] = b
    print(name + ': ' + '  '.join(f'k={k}: CI {v[0]:+.2f}±{v[1]:.2f} stuck {v[2]:.2f}' for k, v in res.items()))
    print(f'   best k = {b};  P24a (CI(0) < best - 0.1): {res[0][0] < res[b][0] - .1};  P24b interior: {b not in (0, 30)}')
print(f"P24c: best k trap {best['W2 trap world']} >= single {best['W1 single peak']}: {best['W2 trap world'] >= best['W1 single peak']}")
