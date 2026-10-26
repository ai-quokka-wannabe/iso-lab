"""Forensics for PREREG19b: which score catches which kind of hallucination (novelty vs confusion)."""
import numpy as np, sys
from sklearn.metrics import roc_auc_score
rng = np.random.default_rng(1902); N = 64; M = 30000; P = 16
for sig in (1.2, 1.6, 2.0):
    X = rng.choice([-1, 1], (P, N)).astype(float)
    kind = rng.integers(0, 3, M); idx = rng.integers(0, P, M); src = X[idx].copy()
    for k in np.nonzero(kind == 1)[0]:
        f = rng.choice(N, int(rng.uniform(.10, .30) * N), replace=False); src[k, f] *= -1
    src[kind == 2] = rng.choice([-1, 1], ((kind == 2).sum(), N))
    y = 2 * (src + sig * rng.standard_normal((M, N))) / sig**2
    met = y @ X.T / 2; o = np.argsort(-met, 1); b, r2 = o[:, 0], o[:, 1]; best = X[b]
    gap = met[np.arange(M), b] - met[np.arange(M), r2]; action = (np.abs(y) * (np.sign(y) != best)).sum(1)
    cos = (y * best).sum(1) / (np.linalg.norm(y, axis=1) * np.sqrt(N)); r = gap / np.maximum(action, 1e-9)
    ok = (kind == 0) & (b == idx); conf = (kind == 0) & (b != idx)
    print(f'sig={sig}: stored-cue confusions {conf.sum()} ({conf.sum()/(kind==0).sum()*100:.1f}% of stored cues)')
    for name, bad in [('novelty: flying cat', kind == 1), ('novelty: random', kind == 2), ('confusion (stored cue, wrong memory)', conf)]:
        sel = ok | bad
        print(f'   {name:38s} AUROC ' + ' '.join(f'{n}={roc_auc_score(ok[sel], v[sel]):.3f}' for n, v in [('cos', cos), ('gap', gap), ('action', -action), ('r', r)]))
