"""PREREG19/19b (args: seed N sigmas flipfrac_lo,hi): gravity gate on a Hamming-space associative memory."""
import numpy as np
from sklearn.metrics import roc_auc_score
rng = np.random.default_rng(int(__import__("sys").argv[1])); N = int(__import__("sys").argv[2]); M = 6000
def thr_at(neg, pos, fa=0.01):
    t = np.quantile(neg, 1 - fa); return (pos > t).mean()
import sys
SIGS = [float(v) for v in sys.argv[3].split(",")]; FC = [float(v) for v in sys.argv[4].split(",")]
rows = []
for P in (16, 64):
    for sig in SIGS:
        X = rng.choice([-1, 1], (P, N)).astype(float)
        kind = rng.integers(0, 3, M); src = np.empty((M, N)); idx = rng.integers(0, P, M)
        for k in range(M):
            if kind[k] == 0: src[k] = X[idx[k]]
            elif kind[k] == 1:
                s = X[idx[k]].copy(); f = rng.choice(N, int(rng.uniform(*FC) * N), replace=False); s[f] *= -1; src[k] = s
            else: src[k] = rng.choice([-1, 1], N)
        y = 2 * (src + sig * rng.standard_normal((M, N))) / sig**2
        met = y @ X.T / 2; o = np.argsort(-met, 1); b, r2 = o[:, 0], o[:, 1]
        best = X[b]; gap = met[np.arange(M), b] - met[np.arange(M), r2]
        action = (np.abs(y) * (np.sign(y) != best)).sum(1)
        cos = (y * best).sum(1) / (np.linalg.norm(y, axis=1) * np.sqrt(N))
        r = gap / np.maximum(action, 1e-9)
        ok = (kind == 0) & (b == idx)
        res = {n: roc_auc_score(ok, v) for n, v in [('cos', cos), ('gap', gap), ('action', -action), ('r', r)]}
        acc = {n: thr_at(v[~ok], v[ok]) for n, v in [('cos', cos), ('gap', gap), ('action', -action), ('r', r)]}
        rows.append((P, sig, res, acc))
        print(f'P={P:3d} sig={sig}: correct {ok.mean():.3f} | AUROC ' + ' '.join(f'{n}={v:.4f}' for n, v in res.items())
              + ' | accepted-correct @1% halluc: ' + ' '.join(f'{n}={v:.3f}' for n, v in acc.items()))
p19 = sum(r['r'] >= r['cos'] - .01 and r['r'] >= r['gap'] - .01 for _, _, r, _ in rows)
s19 = sum(r['r'] >= r['cos'] + .02 for _, _, r, _ in rows)
print(f'P19: {p19}/6 cells -> {"PASS" if p19 >= 5 else "FAIL"};  S19: {s19}/6 -> {"PASS" if s19 >= 4 else "FAIL"}')
