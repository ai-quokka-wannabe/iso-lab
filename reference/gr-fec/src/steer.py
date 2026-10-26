"""PREREG39: a body that can steer (head-swing lateral temperature sense, directed turns) and brains that use it."""
import numpy as np
from embodied import Body, Brain, MX, NT, WID

class DirBody(Body):
    def __init__(self, n, rng):
        super().__init__(n, rng); self.steer = np.zeros(n)
    def sense(self, world, tap=False):
        y, b = super().sense(world, tap)
        hx, hy = np.cos(self.head), np.sin(self.head); lx, ly = -hy, hx             # unit vector to the worm's left
        L = self.xy + np.c_[lx, ly]; R = self.xy - np.c_[lx, ly]
        tl = world.at(L)[3]; tr = world.at(R)[3]
        return np.c_[y, tl - tr + 0.02 * self.rng.standard_normal(self.n)], b
    def act(self, pirouette, move=True):
        self.head = self.head + 0.15 * self.steer
        super().act(pirouette, move)

NS = MX * 3

class SteeringBrain(Brain):
    def __init__(self, n, rng, mode='plain', K=8, H=8, stable=False, relational=False, **kw):
        super().__init__(n, rng, **kw); self.mode, self.K, self.H, self.stable, self.relational = mode, K, H, stable, relational
        self.rel = np.ones(n, int)
        self.lat_s = np.zeros(n); self.lat_lab = np.ones(n, int)
        self.T = np.zeros((n, 3, NS, NS)); self.prev = np.full(n, -1); self.act_count = np.zeros((n, 3))
        self.steer = np.zeros(n); self.tick = 0; self.last_walks = None
    def _node_and_lat(self, y):
        x = y[:, 3]; exist = np.arange(MX)[None] < self.cnt[:, 3][:, None]
        act = np.exp(-(self.proto[:, 3] - x[:, None]) ** 2 / (2 * WID[3] ** 2)) * exist
        node = np.where(exist.any(1), act.argmax(1), 0)
        if self.stable:                                                                # PREREG40: smoothed + hysteresis
            self.lat_s += (y[:, 8] - self.lat_s) / 10
            self.lat_lab = np.where(self.lat_s > 0.02, 2, np.where(self.lat_s < -0.02, 0, np.where(np.abs(self.lat_s) < 0.005, 1, self.lat_lab)))
            return node, self.lat_lab
        lat = np.where(y[:, 8] > 0.01, 2, np.where(y[:, 8] < -0.01, 0, 1))            # 0: right warmer, 2: left warmer
        return node, lat
    def _values(self, h):
        c0 = 3 * MX; return self.W[:, c0:c0 + MX] + h[:, None] * self.W[:, NT + c0:NT + c0 + MX]
    def step(self, y):
        pir, rev = super().step(y); self.tick += 1
        node, lat = self._node_and_lat(y); s = node * 3 + lat; h = np.clip(y[:, 7], 0, 1)
        self.act_count[np.arange(self.n), (self.rel if self.relational else (self.steer + 1).astype(int))] += 1
        if self.mode == 'imagine':
            chg = (s != self.prev) & (self.prev >= 0); idx = np.nonzero(chg)[0]
            a = self.act_count[idx].argmax(1)
            self.T[idx, a, self.prev[idx], s[idx]] += 1
            upd = chg | (self.prev < 0); self.prev = np.where(upd, s, self.prev); self.act_count[upd] = 0
        if self.tick % 10 == 0:
            V = self._values(h)
            if self.mode == 'greedy':
                ar = np.arange(self.n); exist = np.arange(MX)[None] < self.cnt[:, 3][:, None]
                # neighbouring concepts: nearest prototype colder / warmer than the current one
                p = np.where(exist, self.proto[:, 3], np.nan); cur = self.proto[ar, 3, node]
                colder = np.where(p < cur[:, None], p, -np.inf); warmer = np.where(p > cur[:, None], p, np.inf)
                ic = np.nanargmax(np.where(np.isfinite(colder), colder, -1e9), 1); iw = np.nanargmin(np.where(np.isfinite(warmer), warmer, 1e9), 1)
                vc = np.where(np.isfinite(colder).any(1), V[ar, ic], V[ar, node]); vw = np.where(np.isfinite(warmer).any(1), V[ar, iw], V[ar, node])
                want_warm = vw > vc + 0.02; want_cold = vc > vw + 0.02
                left_warm = lat == 2; right_warm = lat == 0
                self.steer = np.where(want_warm & left_warm, 1, np.where(want_warm & right_warm, -1,
                             np.where(want_cold & left_warm, -1, np.where(want_cold & right_warm, 1, 0)))).astype(float)
            elif self.mode == 'imagine':
                Vs = np.repeat(V, 3, axis=1); Q = np.zeros((self.n, 3)); ar = np.arange(self.n)[:, None]; walks = {}
                for a in range(3):
                    cur = np.repeat(s[:, None], self.K, 1); tot = np.zeros((self.n, self.K)); path = [cur.copy()]
                    for k in range(self.H):
                        act = a if k == 0 else 1
                        row = self.T[ar, act, cur]; known = row.sum(-1) > 0
                        cdf = np.cumsum(row + 1e-9, -1); u = self.rng.random((self.n, self.K, 1)) * cdf[..., -1:]
                        nxt = np.minimum((cdf < u).sum(-1), NS - 1); cur = np.where(known, nxt, cur)
                        tot += (0.9 ** k) * Vs[ar, cur]; path.append(cur.copy())
                    Q[:, a] = tot.mean(1); walks[a] = np.stack(path, -1)
                self.last_walks = walks
                best = Q.argmax(1); spread = Q.max(1) - Q.min(1)
                if self.relational:                                                        # 0: away, 1: straight, 2: towards warmer
                    self.rel = np.where(spread > 0.02, best, 1)
                    toward = np.where(lat == 2, 1.0, np.where(lat == 0, -1.0, 0.0))
                    self.steer = np.where(self.rel == 2, toward, np.where(self.rel == 0, -toward, 0.0))
                else:
                    self.steer = np.where(spread > 0.02, best - 1, 0).astype(float)            # 0: right, 1: straight, 2: left
        return pir, rev
