"""Layered BP for quasi-cyclic codes, one layer = one block-row of circulants (updated in parallel, as in hardware).
Supports punctured variable nodes (channel LLR = 0)."""
import numpy as np


def read_qc(path):
    tok = open(path).read().split()
    nb, mb, Z = int(tok[0]), int(tok[1]), int(tok[2])
    vals = list(map(int, tok[3:3 + nb * mb]))
    S = np.array(vals).reshape(mb, nb)
    punct = np.array(list(map(int, tok[3 + nb * mb:3 + nb * mb + nb]))) if len(tok) > 3 + nb * mb else np.ones(nb, int)
    return S, Z, punct


class QCLayeredBP:
    def __init__(self, S, Z, punct):
        self.S, self.Z = S, Z
        self.mb, self.nb = S.shape
        self.n = self.nb * Z
        self.tx = np.repeat(punct.astype(bool), Z)                  # transmitted variable mask
        self.layers = []
        for r in range(self.mb):
            cols = [c for c in range(self.nb) if S[r, c] >= 0]
            # variable index for (check j in layer, edge position) : block col c, row j -> var c*Z + (j + shift) % Z
            V = np.stack([c * Z + (np.arange(Z) + S[r, c]) % Z for c in cols], axis=1)   # (Z, d)
            self.layers.append(V)

    def syndrome_ok(self, hard):
        ok = np.ones(hard.shape[0], bool)
        for V in self.layers:
            ok &= (hard[:, V].sum(2) % 2 == 0).all(1)
        return ok

    def decode(self, llr_tx, iters=50, rng=None, random_from=0):
        """llr_tx: (B, n_tx) channel LLRs of transmitted bits. rng: None -> natural layer order, else random each iteration."""
        B = llr_tx.shape[0]
        L = np.zeros((B, self.n)); L[:, self.tx] = llr_tx
        tot = L.copy()
        c2v = [np.zeros((B, self.Z, V.shape[1])) for V in self.layers]
        conv = np.full(B, iters + 1); idx = np.arange(B); hard_out = np.zeros((B, self.n), np.uint8)
        for it in range(iters):
            order = rng.permutation(self.mb) if (rng is not None and it >= random_from) else range(self.mb)
            for l in order:
                V = self.layers[l]
                v2c = tot[:, V] - c2v[l]                                  # (b, Z, d)
                t = np.tanh(np.clip(v2c, -40, 40) / 2); t = np.where(np.abs(t) < 1e-15, 1e-15, t)
                new = 2 * np.arctanh(np.clip(np.prod(t, axis=2, keepdims=True) / t, -1 + 1e-12, 1 - 1e-12))
                np.put_along_axis  # (no-op reference)
                upd = v2c + new
                for k in range(V.shape[1]):                               # columns of V are disjoint variable sets
                    tot[:, V[:, k]] = upd[:, :, k]
                c2v[l] = new
            hard = (tot < 0).astype(np.uint8)
            ok = self.syndrome_ok(hard)
            hard_out[idx] = hard; conv[idx[ok]] = it + 1
            keep = ~ok
            if not keep.any():
                break
            idx = idx[keep]; tot = tot[keep]; c2v = [c[keep] for c in c2v]
        return conv, hard_out
