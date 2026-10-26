"""Servo-loop ('thermostat') layered BP: each frame has its own decoder temperature alpha_f (channel-LLR scale),
adjusted every iteration from the phase diagram measured in sprint 3:
  oscillating (hard decisions still flipping)        -> decoder too COLD -> heat: alpha *= (1 - gain)
  frozen (no flips, unchanged unsatisfied syndrome)  -> decoder too HOT  -> cool: alpha *= (1 + gain)
Active only after `start` iterations; alpha clamped to [amin, amax]. Random check order every iteration."""
import numpy as np
import scipy.sparse as sp


class ServoLayeredBP:
    def __init__(self, H):
        self.H = H.astype(np.uint8); self.m, self.n = H.shape
        self.rows = [np.nonzero(H[c])[0] for c in range(self.m)]
        self.Hs = sp.csr_matrix(self.H.astype(float))

    def decode(self, llr, rng, iters=50, gain=0.1, start=10, amin=0.5, amax=2.0, mode='both', hold=3, msg_gain=0.0, bmin=0.3, kick=0.0, remelt=False):
        B = llr.shape[0]
        alpha = np.ones(B); beta = np.ones(B)   # beta: per-frame message temperature (scales check messages)
        tot = llr.astype(float).copy()
        c2v = [np.zeros((B, len(r))) for r in self.rows]
        conv = np.full(B, iters + 1); active = np.ones(B, bool); hard_out = (llr < 0).astype(np.uint8)
        prev_hard = hard_out.copy(); prev_syn = np.full((B, self.m), -1); still = np.zeros(B, int); osc = np.zeros(B, int)
        alog = np.ones((B, iters))
        for it in range(iters):
            a = np.nonzero(active)[0]
            for c in rng.permutation(self.m):
                r = self.rows[c]
                v2c = tot[np.ix_(a, r)] - c2v[c][a]
                t = np.tanh(np.clip(v2c, -40, 40) / 2); t = np.where(np.abs(t) < 1e-15, 1e-15, t)
                p = np.prod(t, axis=1, keepdims=True)
                new = 2 * np.arctanh(np.clip(p / t, -1 + 1e-12, 1 - 1e-12)) * beta[a][:, None]
                tot[np.ix_(a, r)] = v2c + new; c2v[c][a] = new
            hard = (tot[a] < 0).astype(np.uint8)
            syn = (np.asarray((self.Hs @ hard.T.astype(float)).T) % 2).astype(int)
            ok = syn.sum(1) == 0
            flipped = (hard != prev_hard[a]).any(1)
            same_syn = (syn == prev_syn[a]).all(1)
            osc[a] = np.where(flipped, osc[a] + 1, 0)
            still[a] = np.where(~flipped & same_syn & ~ok, still[a] + 1, 0)
            if it >= start:
                new_alpha = alpha[a].copy()
                if mode in ('both', 'heat'):
                    new_alpha = np.where(osc[a] >= hold, new_alpha * (1 - gain), new_alpha)
                if mode in ('both', 'cool'):
                    new_alpha = np.where(still[a] >= hold, new_alpha * (1 + gain), new_alpha)
                new_alpha = np.clip(new_alpha, amin, amax)
                if msg_gain > 0:   # message thermostat: oscillating -> soften messages; calm -> relax back towards 1
                    beta[a] = np.clip(np.where(osc[a] >= hold, beta[a] * (1 - msg_gain), np.minimum(1.0, beta[a] * (1 + msg_gain))), bmin, 1.0)
                tot[a] += (new_alpha - alpha[a])[:, None] * llr[a]     # re-weight channel evidence in place
                alpha[a] = new_alpha
            if it >= start and kick > 0:      # Langevin BP: thermal kicks on every undecided frame
                und = a[~ok]
                tot[und] += kick * rng.standard_normal((len(und), self.n))
            if it >= start and remelt:        # re-melt frozen frames: restart from the channel evidence
                fr = a[(still[a] >= hold) & ~ok]
                for c in range(self.m):
                    c2v[c][fr] = 0.0
                tot[fr] = alpha[fr][:, None] * llr[fr]; still[fr] = 0
            alog[a, it] = alpha[a]
            prev_hard[a] = hard; prev_syn[a] = syn
            hard_out[a] = hard; conv[a[ok]] = it + 1; active[a[ok]] = False
            if not active.any():
                break
        return conv, hard_out, alog
