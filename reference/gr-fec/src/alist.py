import numpy as np
def read_alist(path):
    tok = open(path).read().split()
    it = iter(tok)
    n, m = int(next(it)), int(next(it))
    maxv, maxc = int(next(it)), int(next(it))
    dv = [int(next(it)) for _ in range(n)]
    dc = [int(next(it)) for _ in range(m)]
    H = np.zeros((m, n), dtype=np.uint8)
    rest = [int(x) for x in it]
    # column lists: either padded to maxv (zeros) or exactly dv[j] entries
    padded = len(rest) == n * maxv + m * maxc
    pos = 0
    for j in range(n):
        k = maxv if padded else dv[j]
        for i in rest[pos:pos + k]:
            if i > 0: H[i - 1, j] = 1
        pos += k
    assert (H.sum(0) == np.array(dv)).all() and (H.sum(1) == np.array(dc)).all(), 'alist degree mismatch'
    return H
def gf2_rank(H):
    A = H.copy().astype(np.uint8); r = 0
    m, n = A.shape
    for c in range(n):
        piv = np.nonzero(A[r:, c])[0]
        if len(piv) == 0: continue
        p = r + piv[0]; A[[r, p]] = A[[p, r]]
        rows = np.nonzero(A[:, c])[0]; rows = rows[rows != r]
        A[rows] ^= A[r]
        r += 1
        if r == m: break
    return r
