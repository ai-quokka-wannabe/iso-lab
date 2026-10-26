"""PREREG25: worm v2 with concept nodes grown by a novelty gate."""
import sys, warnings, numpy as np
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src'); sys.path.insert(0, 'experiments')
import worm
from worm import Worms
import e33_worm as E
MX = 30; WID = np.array([0.15, 0.15, 1.0]); NOISE = np.array([0.01, 0.01, 0.05]); INNATE = np.array([1.0, 0.3, 0.0])

import os
TWO = os.environ.get("TWO_KINDS") == "1"
if TWO: INNATE = np.array([0.0, 0.0, 0.0])

class GrownWorms(Worms):
    def __init__(self, n, rng, **kw):
        kw.setdefault('s0', 0.03); super().__init__(n, rng, **kw)
        self.W = np.zeros((n, 3 * MX + (2 if TWO else 0)));
        if TWO: self.W[:, -2:] = [float(os.environ.get("GO", 1.0)), float(os.environ.get("GS", 0.3))]
        self.proto = np.zeros((n, 3, MX)); self.cnt = np.zeros((n, 3), int)
    def sense(self, plate):
        o, s, t = plate.fields(self.xy)
        X = np.c_[o + NOISE[0] * self.rng.standard_normal(self.n), s + NOISE[1] * self.rng.standard_normal(self.n),
                  t + NOISE[2] * self.rng.standard_normal(self.n)]
        S = np.zeros((self.n, 3 * MX)); ar = np.arange(self.n)
        for m in range(3):
            x = X[:, m]; P = self.proto[:, m]; exist = np.arange(MX)[None] < self.cnt[:, m][:, None]
            d = np.where(exist, np.abs(P - x[:, None]), np.inf); dmin = d.min(1); j = d.argmin(1)
            event = (x > 3 * NOISE[m]) if m < 2 else np.ones(self.n, bool)
            new = event & (dmin > WID[m]) & (self.cnt[:, m] < MX)
            k = self.cnt[new, m]; self.proto[ar[new], m, k] = x[new]; self.W[ar[new], m * MX + k] = INNATE[m] * x[new]
            self.cnt[new, m] += 1
            upd = event & ~new & np.isfinite(dmin); self.proto[ar[upd], m, j[upd]] += 0.005 * (x[upd] - self.proto[ar[upd], m, j[upd]])
            exist = np.arange(MX)[None] < self.cnt[:, m][:, None]
            act = np.exp(-(self.proto[:, m] - x[:, None]) ** 2 / (2 * WID[m] ** 2)) * exist
            if m < 2: act = act * event[:, None]
            S[:, m * MX:(m + 1) * MX] = act
        if TWO:
            I = np.clip(X[:, :2], 0, None); S = np.hstack([S, I]); self._I = True
        return S

E.Worms = GrownWorms
s0 = 0.03; res = []; B = int(os.environ.get('SEEDBASE', '8100')) - 8100
ci, pr = E.a1(8101 + B, s0); res.append(ci >= .3); print(f'A1 odour chemotaxis: CI {ci:+.2f} (pass {ci >= .3}); pirouettes {pr*60:.2f}/min')
t = E.a2(8110 + B, s0=s0); d = t[25].mean() - t[15].mean(); res.append(d >= 3)
print(f'A2 thermotaxis memory: Tc=15 -> {t[15].mean():.2f}, Tc=25 -> {t[25].mean():.2f}, diff {d:+.2f} °C (pass {d >= 3})')
m = E.a3(8120 + B, s0=s0); p3 = m['mock'] >= .15 and m['salt'] <= m['mock'] - .3; res.append(p3)
print(f"A3 salt learning: mock {m['mock']:+.2f}, salt-starved {m['salt']:+.2f} (pass {p3})")
f, l, rec = E.a4(8130 + B, s0=s0); p4 = l <= .5 * f and rec >= .8 * f; res.append(p4); print(f'A4 habituation: {f:.2f} -> {l:.2f} -> {rec:.2f} (pass {p4})')
e, la = E.a5(8140 + B, s0=s0); res.append(e >= 2 * la); print(f'A5 foraging: {e:.2f} vs {la:.2f} /min, ratio {e/la:.2f} (pass {e >= 2*la})')
print(f'P25: {sum(res)}/5 pass -> {"PASS" if sum(res) >= 4 else "FAIL"};  S25 (A2 and A3): {"PASS" if res[1] and res[2] else "FAIL"}')
w = GrownWorms(100, np.random.default_rng(8150 + B)); E.cultivate(w, temp=25); E.thermotaxis(w)
print('nodes per worm after cultivation at 25 °C + thermal plate (odour, salt, temperature): mean', w.cnt.mean(0).round(1))
print('  temperature prototypes of worm 0:', np.sort(w.proto[0, 2, :w.cnt[0, 2]]).round(1))
