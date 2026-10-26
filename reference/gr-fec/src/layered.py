"""Layered (serial, check-by-check) sum-product BP with a chosen check order — BP is not 'causally invariant':
the update order changes the result. Optional damping on check-to-variable messages."""
import numpy as np
import scipy.sparse as sp


class LayeredBP:
    def __init__(self, H):
        self.H = H.astype(np.uint8); self.m, self.n = H.shape
        self.rows = [np.nonzero(H[c])[0] for c in range(self.m)]
        self.Hs = sp.csr_matrix(self.H.astype(float))

    def decode(self, llr, order, iters=50, damping=0.0, reshuffle_rng=None, track_flips=0, avg_window=0, minsum=0.0, qclip=0.0):
        B = llr.shape[0]
        tot = llr.astype(float).copy()
        c2v = [np.zeros((B, len(r))) for r in self.rows]
        conv = np.full(B, iters + 1); active = np.ones(B, bool); hard_out = (llr < 0).astype(np.uint8)
        flips = np.zeros((B, self.n)); prev = hard_out.copy(); acc = np.zeros((B, self.n))
        for it in range(iters):
            a = np.nonzero(active)[0]
            if reshuffle_rng is not None:
                order = reshuffle_rng.permutation(self.m)
            for c in order:
                r = self.rows[c]
                v2c = tot[np.ix_(a, r)] - c2v[c][a]
                if minsum > 0:   # normalised min-sum: sign product x (minsum * min |.| over the other edges)
                    sg = np.where(v2c < 0, -1.0, 1.0); mg = np.abs(v2c)
                    sprod = np.prod(sg, axis=1, keepdims=True) * sg
                    o = np.argsort(mg, axis=1); m1 = np.take_along_axis(mg, o[:, :1], 1); m2 = np.take_along_axis(mg, o[:, 1:2], 1)
                    excl = np.where(np.arange(mg.shape[1])[None, :] == o[:, :1], m2, m1)
                    new = minsum * sprod * excl
                else:
                    t = np.tanh(np.clip(v2c, -40, 40) / 2)
                    t = np.where(np.abs(t) < 1e-15, 1e-15, t)
                    p = np.prod(t, axis=1, keepdims=True)
                    new = 2 * np.arctanh(np.clip(p / t, -1 + 1e-12, 1 - 1e-12))
                if qclip > 0:
                    new = np.clip(np.round(new * 2) / 2, -qclip, qclip)   # 0.5-step quantised messages
                if damping > 0:
                    new = (1 - damping) * new + damping * c2v[c][a]
                tot[np.ix_(a, r)] = v2c + new
                c2v[c][a] = new
            hard = (tot[a] < 0).astype(np.uint8)
            ok = (np.asarray((self.Hs @ hard.T.astype(float)).T) % 2).sum(1) == 0
            if avg_window and it >= iters - avg_window:
                acc[a] += tot[a]
            if track_flips and it >= iters - track_flips:
                flips[a] += (hard != prev[a])
            prev[a] = hard
            hard_out[a] = hard; conv[a[ok]] = it + 1; active[a[ok]] = False
            if not active.any():
                break
        if avg_window:   # time-crystal step: failed frames decided from the cycle-averaged posterior
            f = np.nonzero(conv > iters)[0]
            if len(f):
                hard = (acc[f] < 0).astype(np.uint8)
                ok = (np.asarray((self.Hs @ hard.T.astype(float)).T) % 2).sum(1) == 0
                hard_out[f[ok]] = hard[ok]; conv[f[ok]] = iters     # rescued by averaging (reported at the last iteration)
        if track_flips:
            return conv, hard_out, flips, tot
        return conv, hard_out
