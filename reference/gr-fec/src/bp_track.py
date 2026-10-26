"""Belief propagation with per-iteration state tracking and Jacobian (stability) probes.

The BP update is a map F: v2c -> v2c (given channel LLRs). Its Jacobian J at the current
messages governs local stability; by Heskes (2002) / Watanabe-Fukumizu, stable BP fixed points
are local minima of the Bethe free energy, so the spectral radius of J is a curvature-linked
quantity. We estimate ||J u|| for a warm-started power-iteration vector u using finite-difference
Jacobian-vector products (one extra BP step per probe).
"""
import numpy as np
import scipy.sparse as sp


class TrackBP:
    def __init__(self, H):
        self.H = H.astype(np.uint8)
        self.m, self.n = H.shape
        self.ec, self.ev = np.nonzero(H)
        self.E = len(self.ec)
        ones = np.ones(self.E)
        self.Sc = sp.csr_matrix((ones, (np.arange(self.E), self.ec)), shape=(self.E, self.m))
        self.Sv = sp.csr_matrix((ones, (np.arange(self.E), self.ev)), shape=(self.E, self.n))
        self.Hs = sp.csr_matrix(self.H.astype(np.float64))

    def step(self, v2c, llr):
        """One full BP iteration: v2c -> (new v2c, posterior LLRs, c2v)."""
        ec, ev = self.ec, self.ev
        t = np.tanh(np.clip(v2c, -40, 40) / 2)
        mag = np.maximum(np.abs(t), 1e-15)
        neg = (t < 0).astype(np.float64)
        logsum = (self.Sc.T @ np.log(mag).T).T
        negsum = (self.Sc.T @ neg.T).T
        lex = logsum[:, ec] - np.log(mag)
        sgn = np.where(((negsum[:, ec] - neg) % 2) > 0.5, -1.0, 1.0)
        ex = np.clip(sgn * np.exp(lex), -1 + 1e-12, 1 - 1e-12)
        c2v = 2 * np.arctanh(ex)
        tot = llr + (self.Sv.T @ c2v.T).T
        return tot[:, ev] - c2v, tot, c2v

    def run(self, llr, T=50, probes=2, eps=1e-4, seed=0, probe_at=(), kcold=(1, 2, 3, 5, 8)):
        """Run T iterations on all frames (no early exit). Returns dict of (B, T) feature arrays
        and per-frame convergence iteration (T+1 if never)."""
        rng = np.random.default_rng(seed)
        B = llr.shape[0]
        v2c = llr[:, self.ev].copy()
        u = rng.standard_normal((B, self.E)); u /= np.linalg.norm(u, axis=1, keepdims=True)
        prev_msg = None; prev_d = None
        F = {k: np.zeros((B, T)) for k in ['synd', 'mabs', 'dnorm', 'ratio', 'lam', 'nflip'] + ['ent'] + [f'lamc{k}' for k in kcold]}
        rng_c = np.random.default_rng(seed + 12345)
        conv = np.full(B, T + 1)
        hard_at_conv = np.zeros((B, self.n), dtype=np.uint8)
        prev_hard = (llr < 0).astype(np.uint8)
        prev_syn = None
        for it in range(T):
            # one BP step, then Jacobian probes at the CURRENT messages (finite-difference JVPs)
            new, tot, c2v = self.step(v2c, llr)
            for _ in range(probes):
                pert, _, _ = self.step(v2c + eps * u, llr)
                Ju = (pert - new) / eps
                nrm = np.linalg.norm(Ju, axis=1)
                u = Ju / np.maximum(nrm, 1e-30)[:, None]
                F['lam'][:, it] = nrm
            if it in probe_at:   # cheap variant: cold-start power iteration only at this checkpoint
                uc = rng_c.standard_normal((B, self.E)); uc /= np.linalg.norm(uc, axis=1, keepdims=True)
                for k in range(1, max(kcold) + 1):
                    pert, _, _ = self.step(v2c + eps * uc, llr)
                    Ju = (pert - new) / eps
                    nrm_c = np.linalg.norm(Ju, axis=1)
                    uc = Ju / np.maximum(nrm_c, 1e-30)[:, None]
                    if k in kcold:
                        F[f'lamc{k}'][:, it] = nrm_c
            hard = (tot < 0).astype(np.uint8)
            syn = np.asarray((self.Hs @ hard.T.astype(np.float64)).T) % 2
            sw = syn.sum(1)
            F['synd'][:, it] = sw
            F['mabs'][:, it] = np.abs(tot).mean(1)
            pa = 1.0 / (1.0 + np.exp(np.minimum(np.abs(tot), 700)))     # prob. of the less likely bit value
            F['ent'][:, it] = -(pa * np.log2(np.maximum(pa, 1e-300)) + (1 - pa) * np.log2(np.maximum(1 - pa, 1e-300))).sum(1)
            d = np.linalg.norm(c2v - (prev_msg if prev_msg is not None else 0), axis=1)
            F['dnorm'][:, it] = d
            F['ratio'][:, it] = d / np.maximum(prev_d, 1e-30) if prev_d is not None else 1.0
            F['nflip'][:, it] = (hard != prev_hard).sum(1)
            newly = (conv == T + 1) & (sw == 0)
            conv[newly] = it + 1
            hard_at_conv[newly] = hard[newly]
            prev_msg, prev_d, prev_hard = c2v, d, hard
            v2c = new
        return F, conv, hard_at_conv
