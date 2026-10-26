"""Wick-rotated ('complex-temperature') sum-product BP for check-regular codes.
Channel evidence alpha * LLR with complex alpha = |alpha| e^{i theta}. Messages are complex log-ratios; the usual
tanh rule is applied with complex arithmetic (exact analytic continuation of BP). Hard decision: Re(total) < 0.
theta = 0 reproduces ordinary BP exactly."""
import numpy as np


class ComplexBP:
    def __init__(self, H):
        self.H = H.astype(np.uint8); self.m, self.n = H.shape
        dc = H.sum(1); assert len(set(dc)) == 1, 'check-regular codes only'
        self.dc = int(dc[0])
        self.ec, self.ev = np.nonzero(H)                       # row-major: edges grouped by check, dc per check
        self.var_edges = [np.nonzero(self.ev == v)[0] for v in range(self.n)]
        import scipy.sparse as sp
        self.Sv = sp.csr_matrix((np.ones(len(self.ev)), (np.arange(len(self.ev)), self.ev)), shape=(len(self.ev), self.n))
        self.Hs = sp.csr_matrix(self.H.astype(float))

    def decode(self, llr, alpha=1.0 + 0j, iters=50, clipre=40.0):
        B = llr.shape[0]; ev = self.ev; dc = self.dc
        L = alpha * llr.astype(complex)
        v2c = L[:, ev].copy()
        conv = np.full(B, iters + 1); idx = np.arange(B); hard_out = (llr < 0).astype(np.uint8)
        for it in range(iters):
            z = np.clip(v2c.real, -clipre, clipre) + 1j * v2c.imag
            t = np.tanh(z / 2).reshape(len(idx), self.m, dc)
            # exclusive products via prefix/suffix
            pre = np.ones_like(t); suf = np.ones_like(t)
            for k in range(1, dc):
                pre[:, :, k] = pre[:, :, k - 1] * t[:, :, k - 1]
                suf[:, :, dc - 1 - k] = suf[:, :, dc - k] * t[:, :, dc - k]
            ex = (pre * suf).reshape(len(idx), -1)
            ex = np.where(np.abs(ex) > 1 - 1e-12, ex / np.abs(ex) * (1 - 1e-12), ex)
            c2v = 2 * np.arctanh(ex)
            tot = L + np.asarray((self.Sv.T @ c2v.T).T)
            v2c = tot[:, ev] - c2v
            hard = (tot.real < 0).astype(np.uint8)
            ok = (np.asarray((self.Hs @ hard.T.astype(float)).T) % 2).sum(1) == 0
            hard_out[idx] = hard; conv[idx[ok]] = it + 1
            keep = ~ok
            if not keep.any():
                break
            idx, L, v2c = idx[keep], L[keep], v2c[keep]
        return conv, hard_out
