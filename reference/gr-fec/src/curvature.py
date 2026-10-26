"""Discrete curvatures and cycle-structure baselines on the Tanner graph.

Tanner graph nodes: variables 0..n-1, checks n..n+m-1.
"""
import numpy as np
import scipy.sparse as sp
from scipy.sparse.csgraph import shortest_path
import ot


def tanner_adj(H):
    m, n = H.shape
    A = np.zeros((n + m, n + m))
    A[:n, n:] = H.T
    A[n:, :n] = H
    return A


def walk_measure(P, x, t, alpha):
    """Distribution of a t-step lazy random walk from x (idleness alpha)."""
    N = P.shape[0]
    mu = np.zeros(N)
    mu[x] = 1.0
    Pl = alpha * np.eye(N) + (1 - alpha) * P
    for _ in range(t):
        mu = mu @ Pl
    return mu


def ollivier_edge_curvature(H, t=1, alpha=0.5, tol=1e-12):
    """Ollivier-Ricci curvature of every Tanner edge, measures = t-step lazy walks.
    Returns (edges as (var, check_index), kappa)."""
    A = tanner_adj(H)
    n = H.shape[1]
    deg = A.sum(1)
    P = A / deg[:, None]
    D = shortest_path(sp.csr_matrix(A), unweighted=True)
    N = A.shape[0]
    Pl = alpha * np.eye(N) + (1 - alpha) * P
    Pt = np.linalg.matrix_power(Pl, t)
    ec, ev = np.nonzero(H)
    kap = np.zeros(len(ec))
    for k, (c, v) in enumerate(zip(ec, ev)):
        x, y = v, n + c
        mx, my = Pt[x], Pt[y]
        sx, sy = np.nonzero(mx > tol)[0], np.nonzero(my > tol)[0]
        a, b = mx[sx] / mx[sx].sum(), my[sy] / my[sy].sum()
        W = ot.emd2(a, b, D[np.ix_(sx, sy)])
        kap[k] = 1.0 - W / D[x, y]
    return ev, ec, kap


def node_ollivier(H, t=1, alpha=0.5):
    """Variable-node curvature = mean Ollivier curvature of incident edges."""
    ev, ec, kap = ollivier_edge_curvature(H, t, alpha)
    n = H.shape[1]
    return np.bincount(ev, weights=kap, minlength=n) / np.bincount(ev, minlength=n)


def resistance_curvature(H):
    """Devriendt-Lambiotte node resistance curvature p_i = 1 - 1/2 sum_{j~i} R_ij,
    evaluated on variable nodes."""
    A = tanner_adj(H)
    Lap = np.diag(A.sum(1)) - A
    Lp = np.linalg.pinv(Lap)
    d = np.diag(Lp)
    R = d[:, None] + d[None, :] - 2 * Lp
    p = 1 - 0.5 * (A * R).sum(1)
    return p[: H.shape[1]]


def nb_closed_walks(H, lengths=(6, 8, 10, 12)):
    """Per-variable-node counts of closed non-backtracking walks of given lengths
    (the standard local cycle-structure features)."""
    A = tanner_adj(H)
    n = H.shape[1]
    src, dst = np.nonzero(A)
    E2 = len(src)
    # B[e -> f] = 1 if dst(e) == src(f) and f is not reverse of e
    out_by_node = {}
    for i, s in enumerate(src):
        out_by_node.setdefault(s, []).append(i)
    rows, cols = [], []
    for e in range(E2):
        for f in out_by_node[dst[e]]:
            if dst[f] != src[e]:
                rows.append(e); cols.append(f)
    B = sp.csr_matrix((np.ones(len(rows)), (rows, cols)), shape=(E2, E2))
    feats = {}
    Bk = sp.identity(E2, format='csr')
    maxL = max(lengths)
    for L in range(1, maxL + 1):
        Bk = Bk @ B
        if L in lengths:
            diag = Bk.diagonal()
            feats[L] = np.bincount(src, weights=diag, minlength=A.shape[0])[:n]
    return feats
