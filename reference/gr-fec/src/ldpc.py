"""LDPC code construction and vectorised belief-propagation decoding.

Conventions: H is (m, n) binary; BPSK maps bit 0 -> +1, bit 1 -> -1;
the all-zero codeword is transmitted (valid for linear codes + symmetric decoders),
so any bit decoded as 1 is an error.
"""
import numpy as np
import scipy.sparse as sp


def random_regular_no4cycles(n, dv, dc, rng, iters=200000):
    """Random (dv, dc)-regular parity-check matrix by socket matching,
    with double edges and 4-cycles removed by random edge swaps."""
    m = n * dv // dc
    assert m * dc == n * dv
    vs = np.repeat(np.arange(n), dv)
    cs = np.repeat(np.arange(m), dc)
    rng.shuffle(cs)
    E = len(vs)
    for _ in range(iters):
        H = np.zeros((m, n), dtype=np.int64)
        np.add.at(H, (cs, vs), 1)
        bad_edges = _bad_edges(H, cs, vs)
        if len(bad_edges) == 0:
            return H.astype(np.uint8)
        e1 = rng.choice(bad_edges)
        e2 = rng.integers(E)
        cs[e1], cs[e2] = cs[e2], cs[e1]
    raise RuntimeError("could not remove 4-cycles")


def _bad_edges(H, cs, vs):
    bad = set()
    if H.max() > 1:
        for c, v in np.argwhere(H > 1):
            bad.update(np.nonzero((cs == c) & (vs == v))[0].tolist())
    Hb = (H > 0).astype(np.int64)
    G = Hb.T @ Hb
    np.fill_diagonal(G, 0)
    for v1, v2 in np.argwhere(G >= 2):
        if v1 < v2:
            shared = np.nonzero(Hb[:, v1] & Hb[:, v2])[0]
            for c in shared:
                bad.update(np.nonzero((cs == c) & (vs == v1))[0].tolist())
    return np.array(sorted(bad), dtype=np.int64)


def girth(H):
    """Girth of the Tanner graph via BFS from every variable node."""
    m, n = H.shape
    nv = [np.nonzero(H[:, j])[0] for j in range(n)]
    nc = [np.nonzero(H[i, :])[0] for i in range(m)]
    best = 10**9
    for s in range(n):
        dist = {('v', s): 0}
        par = {('v', s): None}
        frontier = [('v', s)]
        while frontier:
            nxt = []
            for node in frontier:
                kind, i = node
                nbrs = [('c', c) for c in nv[i]] if kind == 'v' else [('v', v) for v in nc[i]]
                for nb in nbrs:
                    if nb == par[node]:
                        continue
                    if nb in dist:
                        best = min(best, dist[node] + dist[nb] + 1)
                    else:
                        dist[nb] = dist[node] + 1
                        par[nb] = node
                        nxt.append(nb)
            frontier = nxt if (nxt and 2 * dist[nxt[0]] < best) else []
    return best


class BP:
    """Vectorised sum-product (tanh-rule) decoder over a batch of frames."""

    def __init__(self, H):
        self.H = H.astype(np.uint8)
        self.m, self.n = H.shape
        self.ec, self.ev = np.nonzero(H)
        self.E = len(self.ec)
        ones = np.ones(self.E)
        # (E x m) and (E x n) incidence matrices for group sums
        self.Sc = sp.csr_matrix((ones, (np.arange(self.E), self.ec)), shape=(self.E, self.m))
        self.Sv = sp.csr_matrix((ones, (np.arange(self.E), self.ev)), shape=(self.E, self.n))
        self.Hs = sp.csr_matrix(self.H.astype(np.float64))

    def decode(self, llr, iters=50):
        """Decode only frames that are still active; converged frames are dropped."""
        ec, ev = self.ec, self.ev
        B = llr.shape[0]
        hard = (llr < 0).astype(np.uint8)
        done = np.zeros(B, bool)
        used = np.full(B, iters)
        idx = np.arange(B)
        v2c = llr[:, ev].copy()
        L = llr
        for it in range(iters):
            t = np.tanh(np.clip(v2c, -40, 40) / 2)
            mag = np.maximum(np.abs(t), 1e-15)
            neg = (t < 0).astype(np.float64)
            logsum = (self.Sc.T @ np.log(mag).T).T          # (B, m)
            negsum = (self.Sc.T @ neg.T).T
            # extrinsic: divide out own term
            lex = logsum[:, ec] - np.log(mag)
            sgn = np.where(((negsum[:, ec] - neg) % 2) > 0.5, -1.0, 1.0)
            ex = np.clip(sgn * np.exp(lex), -1 + 1e-12, 1 - 1e-12)
            c2v = 2 * np.arctanh(ex)
            tot = L + (self.Sv.T @ c2v.T).T                   # (b, n)
            v2c = tot[:, ev] - c2v
            hard_new = (tot < 0).astype(np.uint8)
            syn = np.asarray((self.Hs @ hard_new.T.astype(np.float64)).T) % 2
            ok = syn.sum(1) == 0
            hard[idx] = hard_new
            done[idx[ok]] = True
            used[idx[ok]] = it + 1
            keep = ~ok
            if not keep.any():
                break
            idx, L, v2c = idx[keep], L[keep], v2c[keep]
        self.last_tot = None
        return hard, done, used


def awgn_llr(B, n, ebn0_db, rate, rng):
    """All-zero codeword, BPSK +1, AWGN at Eb/N0; returns channel LLRs."""
    sigma2 = 1.0 / (2 * rate * 10 ** (ebn0_db / 10))
    y = 1.0 + np.sqrt(sigma2) * rng.standard_normal((B, n))
    return 2 * y / sigma2
