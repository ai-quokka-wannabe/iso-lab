"""PREREG19c: two-wall (novelty + confusion) detector on a Hamming-space memory."""
import numpy as np
from sklearn.metrics import roc_auc_score
from sklearn.linear_model import LogisticRegression
N = 64; M = 6000
def make(rng, P, sig):
    X = rng.choice([-1, 1], (P, N)).astype(float)
    kind = rng.integers(0, 3, M); idx = rng.integers(0, P, M); src = X[idx].copy()
    for k in np.nonzero(kind == 1)[0]:
        f = rng.choice(N, int(rng.uniform(.10, .30) * N), replace=False); src[k, f] *= -1
    src[kind == 2] = rng.choice([-1, 1], ((kind == 2).sum(), N))
    y = 2 * (src + sig * rng.standard_normal((M, N))) / sig**2
    met = y @ X.T / 2; o = np.argsort(-met, 1); b, r2 = o[:, 0], o[:, 1]; best = X[b]
    gap = met[np.arange(M), b] - met[np.arange(M), r2]; action = (np.abs(y) * (np.sign(y) != best)).sum(1)
    cos = (y * best).sum(1) / (np.linalg.norm(y, axis=1) * np.sqrt(N)); r = gap / np.maximum(action, 1e-9)
    return np.c_[cos, np.log(gap + 1e-9)], cos, r, (kind == 0) & (b == idx)
cells = [(P, s) for P in (16, 64) for s in (1.2, 1.6, 2.0)]; tr, te = np.random.default_rng(1903), np.random.default_rng(1904)
F, Y = [], []
for P, s in cells:
    f, _, _, ok = make(tr, P, s); F.append(np.c_[f, np.full(M, s)]); Y.append(ok)
clf = LogisticRegression(max_iter=1000).fit(np.vstack(F)[:, :2], np.concatenate(Y))
print('frozen weights (cos, log gap):', np.round(clf.coef_, 3), 'intercept', np.round(clf.intercept_, 3))
wins = 0
for P, s in cells:
    f, cos, r, ok = make(te, P, s); tw = clf.decision_function(f)
    a = {n: roc_auc_score(ok, v) for n, v in [('cos', cos), ('r', r), ('two-wall', tw)]}
    w = a['two-wall'] >= max(a['cos'], a['r']) + 0.005; wins += w
    t = np.quantile(tw[~ok], .99); tc = np.quantile(cos[~ok], .99)
    print(f'P={P:3d} sig={s}: ' + ' '.join(f'{n}={v:.4f}' for n, v in a.items()) + f' | accepted-correct @1% halluc: cos={(cos[ok]>tc).mean():.3f} two-wall={(tw[ok]>t).mean():.3f}' + (' WIN' if w else ''))
print(f'P19c: {wins}/6 -> {"PASS" if wins >= 4 else "FAIL"}')
