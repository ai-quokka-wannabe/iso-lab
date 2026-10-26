"""Numba-compiled organic brain step (optimiser speed-up, PREREG43). Must reproduce evolve2.OrganicBrain.step exactly
(same random-number order: all draws stay outside the kernel). Use FastOrganicBrain / FastOrganicSteerBrain as drop-in
replacements."""
import numpy as np
from numba import njit
from evolve2 import OrganicBrain, C, MXC
from evolve3 import OrganicSteerBrain


@njit(cache=True)
def _kernel(z, grow, width, thr, proto, cnt, W, eta, gA, gB, gAon, gBon, rew, trk, inst, lam,
            Dbar, has_dbar, integ, hist, lag, t, s0, tau, bias, pmax):
    n = z.shape[0]; NS = C * MXC + C
    P = np.empty(n); births = np.zeros(n)
    S = np.zeros(NS); F = np.zeros(3 * NS)
    for i in range(n):
        S[:] = 0.0
        for c in range(C):
            x = z[i, c]; w = width[i, c]; k0 = cnt[i, c]
            dmin = np.inf; j = 0
            for k in range(k0):
                d = abs(proto[i, c, k] - x)
                if d < dmin: dmin = d; j = k
            event = grow[i, c] and abs(x) > thr[i, c]
            new = event and dmin > w and k0 < MXC
            if new:
                proto[i, c, k0] = x; cnt[i, c] = k0 + 1; births[i] += 1.0
            elif event and k0 > 0:
                proto[i, c, j] += 0.005 * (x - proto[i, c, j])
            if event:
                for k in range(cnt[i, c]):
                    dd = proto[i, c, k] - x
                    S[c * MXC + k] = np.exp(-dd * dd / (2.0 * w * w))
        for c in range(C): S[C * MXC + c] = z[i, c]
        a = z[i, gA[i]] if gAon[i] else 0.0
        b = z[i, gB[i]] if gBon[i] else 0.0
        ff = 0.0
        for q in range(NS):
            F[q] = S[q]; F[NS + q] = S[q] * a; F[2 * NS + q] = S[q] * b
        r = 0.0
        for c in range(C): r += rew[i, c] * z[i, c]
        pred = 0.0
        for q in range(3 * NS):
            pred += W[i, q] * F[q]; ff += F[q] * F[q]
        g = eta[i] * (r - pred) / (1.0 + ff)
        for q in range(3 * NS): W[i, q] += g * F[q]
        D = lam[i] * pred
        for c in range(C):
            tt = 1.0 + trk[i, c] * W[i, C * MXC + c]
            if tt < 0.0: tt = 0.0
            if tt > 1.5: tt = 1.5
            D += inst[i, c] * tt * z[i, c]
        if has_dbar: Dbar[i] = Dbar[i] + (D - Dbar[i]) / integ[i]
        else: Dbar[i] = D
        for h in range(20, 0, -1): hist[h, i] = hist[h - 1, i]
        hist[0, i] = Dbar[i]
        L = lag[i] if lag[i] < 20 else 20
        dD = Dbar[i] - hist[L, i] if t > lag[i] else 0.0
        logit = (-dD / s0[i] - 1.0) / tau[i]
        for c in range(C): logit += bias[i, c] * z[i, c]
        if logit > 50.0: logit = 50.0
        if logit < -50.0: logit = -50.0
        P[i] = pmax[i] / (1.0 + np.exp(-logit))
    return P, births


class FastOrganicBrain(OrganicBrain):
    def step(self, y):
        n = self.n
        z = np.ascontiguousarray(np.c_[y[:, :3], (y[:, 3] - 20) / 5, y[:, 4:8], self.nov])
        if self.Dbar is None: self.Dbar = np.zeros(n); has = False
        else: has = True
        self.t += 1
        P, births = _kernel(z, self.grow, self.width, self.thr, self.proto, self.cnt, self.W, self.eta, self.gA, self.gB,
                            self.gAon, self.gBon, self.rew, self.trk, self.inst, self.lam, self.Dbar, has, self.integ,
                            self.hist, self.lag, self.t, self.s0, self.tau, self.bias, self.pmax)
        pir = self.rng.random(n) < P
        self.nov = self.nov * np.exp(-1 / self.nov_tau) + births
        self.touch_w += (0.9 - self.touch_w) / 300; tapped = y[:, 4] > 0.5
        rev = tapped & (self.rng.random(n) < self.touch_w); self.touch_w = np.where(tapped, self.touch_w * 0.88, self.touch_w)
        return pir, rev


class FastOrganicSteerBrain(OrganicSteerBrain, FastOrganicBrain):
    """MRO: OrganicSteerBrain.step → super().step → FastOrganicBrain.step."""
    pass
