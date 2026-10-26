"""Ordered-statistics decoding (Fossorier-Lin) of order <= 2 for a binary linear code given by generator matrix G.
Also returns the ML 'lower-bound' test: whether the best found codeword beats the transmitted one (all-zero) in likelihood."""
import numpy as np
from itertools import combinations


def generator_from_H(H):
    """Systematic-ish generator matrix G (k x n) from parity-check H over GF(2)."""
    m, n = H.shape
    A = H.copy().astype(np.uint8); piv = []; r = 0
    for c in range(n):
        rows = np.nonzero(A[r:, c])[0]
        if len(rows) == 0: continue
        p = r + rows[0]; A[[r, p]] = A[[p, r]]
        others = np.nonzero(A[:, c])[0]; others = others[others != r]
        A[others] ^= A[r]; piv.append(c); r += 1
        if r == m: break
    free = [c for c in range(n) if c not in piv]
    k = len(free); G = np.zeros((k, n), np.uint8)
    for i, f in enumerate(free):
        G[i, f] = 1
        for j, pc in enumerate(piv):
            G[i, pc] = A[j, f]
    assert not ((H.astype(int) @ G.T.astype(int)) % 2).any()
    return G


def osd(G, llr, order=2):
    k, n = G.shape
    perm = np.argsort(-np.abs(llr))
    Gp = G[:, perm].copy(); cols = []; r = 0
    for c in range(n):                      # find k most reliable independent positions
        rows = np.nonzero(Gp[r:, c])[0]
        if len(rows) == 0: continue
        p = r + rows[0]; Gp[[r, p]] = Gp[[p, r]]
        others = np.nonzero(Gp[:, c])[0]; others = others[others != r]
        Gp[others] ^= Gp[r]; cols.append(c); r += 1
        if r == k: break
    hard = (llr[perm] < 0).astype(np.uint8)
    u0 = hard[cols]
    base = (u0 @ Gp) % 2
    cost = lambda cw: np.sum(np.abs(llr[perm]) * (cw != hard))
    best, bc = base, cost(base)
    teps = [()] + ([(i,) for i in range(k)] if order >= 1 else []) + (list(combinations(range(k), 2)) if order >= 2 else [])
    E = np.zeros((len(teps), k), np.uint8)
    for t, ix in enumerate(teps):
        E[t, list(ix)] = 1
    CW = ((u0[None, :] ^ E) @ Gp) % 2
    costs = (np.abs(llr[perm])[None, :] * (CW != hard[None, :])).sum(1)
    i = int(np.argmin(costs)); best = CW[i]
    out = np.zeros(n, np.uint8); out[perm] = best
    return out


def osd_blend(G, llr, post, w=0.25, order=2):
    """OSD ordered by reliability |llr| + w*|post| (channel + fraction of BP posterior), hard decisions from llr + w*post,
    candidate selection by the CHANNEL likelihood metric (sum of channel LLRs over bits set to 1)."""
    k, n = G.shape
    rel = np.abs(llr) + w * np.abs(post); s = np.sign(llr + w * post); s[s == 0] = 1
    perm = np.argsort(-rel); Gp = G[:, perm].copy(); cols = []; r = 0
    for c in range(n):
        rows = np.nonzero(Gp[r:, c])[0]
        if len(rows) == 0: continue
        p = r + rows[0]; Gp[[r, p]] = Gp[[p, r]]
        oth = np.nonzero(Gp[:, c])[0]; oth = oth[oth != r]; Gp[oth] ^= Gp[r]; cols.append(c); r += 1
        if r == k: break
    hard = (s[perm] < 0).astype(np.uint8); u0 = hard[cols]
    teps = [()] + ([(i,) for i in range(k)] if order >= 1 else []) + (list(combinations(range(k), 2)) if order >= 2 else [])
    E = np.zeros((len(teps), k), np.uint8)
    for t, ix in enumerate(teps):
        E[t, list(ix)] = 1
    CW = ((u0[None, :] ^ E) @ Gp) % 2
    best = CW[int(np.argmin((CW * llr[perm][None, :]).sum(1)))]
    out = np.zeros(n, np.uint8); out[perm] = best
    return out


def osd_blend_scored(G, llr, post, w=0.25, order=2):
    """As osd_blend, but also returns gate statistics on the code 'landscape':
    gap    = likelihood-metric difference between the runner-up and the best candidate codeword (well-depth gap);
    action = soft distance the decoder 'tunnelled': sum |llr| over bits where the best codeword disagrees with the
             channel hard decision (cost of reaching that well from the received point)."""
    k, n = G.shape
    rel = np.abs(llr) + w * np.abs(post); s = np.sign(llr + w * post); s[s == 0] = 1
    perm = np.argsort(-rel); Gp = G[:, perm].copy(); cols = []; r = 0
    for c in range(n):
        rows = np.nonzero(Gp[r:, c])[0]
        if len(rows) == 0: continue
        p = r + rows[0]; Gp[[r, p]] = Gp[[p, r]]
        oth = np.nonzero(Gp[:, c])[0]; oth = oth[oth != r]; Gp[oth] ^= Gp[r]; cols.append(c); r += 1
        if r == k: break
    hard = (s[perm] < 0).astype(np.uint8); u0 = hard[cols]
    teps = [()] + ([(i,) for i in range(k)] if order >= 1 else []) + (list(combinations(range(k), 2)) if order >= 2 else [])
    E = np.zeros((len(teps), k), np.uint8)
    for t, ix in enumerate(teps):
        E[t, list(ix)] = 1
    CW = ((u0[None, :] ^ E) @ Gp) % 2
    lp = llr[perm]
    costs = (CW * lp[None, :]).sum(1)
    two = np.partition(costs, 1)[:2]
    i = int(np.argmin(costs)); best = CW[i]
    chan_hard = (lp < 0).astype(np.uint8)
    action = float(np.abs(lp)[best != chan_hard].sum())
    out = np.zeros(n, np.uint8); out[perm] = best
    return out, float(two[1] - two[0]), action


def osd3_best(G, llr, chunk=8000):
    """Order-3 OSD (channel ordering), chunked; returns best codeword by channel likelihood metric."""
    k, n = G.shape
    perm = np.argsort(-np.abs(llr)); Gp = G[:, perm].copy(); cols = []; r = 0
    for c in range(n):
        rows = np.nonzero(Gp[r:, c])[0]
        if len(rows) == 0: continue
        p = r + rows[0]; Gp[[r, p]] = Gp[[p, r]]
        oth = np.nonzero(Gp[:, c])[0]; oth = oth[oth != r]; Gp[oth] ^= Gp[r]; cols.append(c); r += 1
        if r == k: break
    lp = llr[perm]; hard = (lp < 0).astype(np.uint8); u0 = hard[cols]
    Gf = Gp.astype(np.float32)
    teps = [()] + [(i,) for i in range(k)] + list(combinations(range(k), 2)) + list(combinations(range(k), 3))
    best, bc = None, np.inf
    for s in range(0, len(teps), chunk):
        T = teps[s:s + chunk]; E = np.zeros((len(T), k), np.uint8)
        for t, ix in enumerate(T):
            E[t, list(ix)] = 1
        CW = (((u0[None, :] ^ E).astype(np.float32) @ Gf) % 2).astype(np.uint8)
        costs = CW @ lp
        i = int(np.argmin(costs))
        if costs[i] < bc:
            bc, best = costs[i], CW[i]
    out = np.zeros(n, np.uint8); out[perm] = best
    return out


def osd_order_cands(G, llr, order_vec, sign_vec, order=2):
    """OSD with an arbitrary reliability ordering/hard-decision vector; candidates judged by the channel metric.
    Returns (best codeword, best cost, runner-up cost)."""
    k, n = G.shape
    perm = np.argsort(-order_vec); Gp = G[:, perm].copy(); cols = []; r = 0
    for c in range(n):
        rows = np.nonzero(Gp[r:, c])[0]
        if len(rows) == 0: continue
        p = r + rows[0]; Gp[[r, p]] = Gp[[p, r]]
        oth = np.nonzero(Gp[:, c])[0]; oth = oth[oth != r]; Gp[oth] ^= Gp[r]; cols.append(c); r += 1
        if r == k: break
    hard = (sign_vec[perm] < 0).astype(np.uint8); u0 = hard[cols]
    teps = [()] + ([(i,) for i in range(k)] if order >= 1 else []) + (list(combinations(range(k), 2)) if order >= 2 else [])
    E = np.zeros((len(teps), k), np.uint8)
    for t, ix in enumerate(teps):
        E[t, list(ix)] = 1
    CW = ((u0[None, :] ^ E) @ Gp) % 2
    lp = llr[perm]
    costs = (CW * lp[None, :]).sum(1)
    two = np.partition(costs, 1)[:2]
    i = int(np.argmin(costs)); out = np.zeros(n, np.uint8); out[perm] = CW[i]
    action = float(np.abs(lp)[CW[i] != (lp < 0)].sum())
    return out, float(costs[i]), float(two[1] - two[0]), action
