"""Imagination as random walks on the concept graph (H-AI14, PREREG38).

The brain (embodied v6 with learned trust) additionally learns a transition model between *states* of its own concept
graph: state = (most active temperature concept, direction of temperature change over the last 10 s), separately for the
two actions it can take (keep going / turn). Every 10 s it imagines K random walks of H steps for each first action
(later steps: keep going), sampling transitions in proportion to experienced counts, and scores each walk by the learned
values of the concepts visited (discount 0.9). If "turn" is imagined better than "keep going" it turns more, and vice versa.
Everything is readable: states are named concepts, transitions are counts, imagined walks can be printed.
"""
import numpy as np
from embodied import Brain, MX, NT, WID

NMAX = MX; NSTATE = NMAX * 3                     # temperature concepts × direction (−, 0, +)


class ImaginingBrain(Brain):
    def __init__(self, n, rng, K=8, H=6, kappa=2.0, imagine=True, events=False, nodir=False, **kw):
        super().__init__(n, rng, **kw)
        self.K, self.H, self.kappa, self.imagine, self.events, self.nodir = K, (8 if events else H), kappa, imagine, events, nodir
        self.T = np.full((n, 2, NSTATE, NSTATE), 0.0)               # experienced transition counts per action
        self.prev_state = np.full(n, -1); self.turned = np.zeros(n, bool); self.temp_hist = []
        self.dQ = np.zeros(n); self.tick = 0; self.last_walks = None

    def _state(self, y):
        x = y[:, 3]; c0 = 3 * MX; exist = np.arange(MX)[None] < self.cnt[:, 3][:, None]
        act = np.exp(-(self.proto[:, 3] - x[:, None]) ** 2 / (2 * WID[3] ** 2)) * exist
        node = np.where(exist.any(1), act.argmax(1), 0)
        self.temp_hist.append(x)
        if len(self.temp_hist) > 11: self.temp_hist.pop(0)
        d = np.sign(np.round(x - self.temp_hist[0], 1)) if len(self.temp_hist) > 10 else np.zeros(self.n)
        return node * 3 + (1 if self.nodir else (d + 1).astype(int))

    def _values(self, h):
        c0 = 3 * MX; v = self.W[:, c0:c0 + MX] + h[:, None] * self.W[:, NT + c0:NT + c0 + MX]   # learned value per temp concept
        return np.repeat(v, 3, axis=1)                                                          # same value for each direction

    def _imagine(self, s, h):
        n, K, H = self.n, self.K, self.H; V = self._values(h); Q = np.zeros((n, 2)); ar = np.arange(n)[:, None]
        walks = {}
        for a in (0, 1):
            cur = np.repeat(s[:, None], K, 1); tot = np.zeros((n, K)); path = [cur.copy()]
            for step in range(H):
                act = a if step == 0 else 0
                row = self.T[ar, act, cur] + 1e-6                                  # (n, K, NSTATE)
                cdf = np.cumsum(row, -1); u = self.rng.random((n, K, 1)) * cdf[..., -1:]
                nxt = np.minimum((cdf < u).sum(-1), NSTATE - 1)
                known = self.T[ar, act, cur].sum(-1) > 0
                cur = np.where(known, nxt, cur); tot += (0.9 ** step) * V[ar, cur]; path.append(cur.copy())
            Q[:, a] = tot.mean(1); walks[a] = np.stack(path, -1)
        self.last_walks = walks
        return Q[:, 1] - Q[:, 0]

    def step(self, y):
        pir, rev = super().step(y)
        self.turned |= pir; self.tick += 1
        if self.tick % 10 == 0:
            s = self._state(y); h = np.clip(y[:, 7], 0, 1)
            ok = self.prev_state >= 0
            if self.events: ok = ok & (s != self.prev_state)                 # event-based: only concept changes count
            idx = np.nonzero(ok)[0]
            self.T[idx, self.turned[idx].astype(int), self.prev_state[idx], s[idx]] += 1
            if self.events:
                first = self.prev_state < 0; chg = ok | first
                self.prev_state = np.where(chg, s, self.prev_state); self.turned[chg] = False
            else:
                self.prev_state = s; self.turned[:] = False
            if self.imagine: self.dQ = self._imagine(s, h)
        else:
            self._state(y)
        if self.imagine:
            g = np.tanh(self.kappa * self.dQ)
            extra = self.rng.random(self.n) < 0.05 * np.clip(g, 0, None)          # imagined: turning is better
            keep = self.rng.random(self.n) < 0.5 * np.clip(-g, 0, None)          # imagined: keep going is better
            pir = (pir | extra) & ~keep
        return pir, rev
