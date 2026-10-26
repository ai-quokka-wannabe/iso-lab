"""Cheap checkpoint-only Jacobian probe vs full-history baseline (PREREG3)."""
import sys, numpy as np
sys.path.insert(0, 'experiments')
from a05_stopping import feats, model, mat, BASE, stopping_tradeoff
from sklearn.metrics import roc_auc_score

tr = dict(np.load('results/e05_eb3.0_s100_cheap.npz')); te = dict(np.load('results/e05_eb3.0_s200_cheap.npz'))
ref_tr = np.load('results/e05_eb3.0_s100.npz'); ref_te = np.load('results/e05_eb3.0_s200.npz')
print('decoder trajectories identical to PREREG2 data:',
      np.array_equal(tr['conv'], ref_tr['conv']) and np.array_equal(te['conv'], ref_te['conv']),
      np.allclose(tr['synd'], ref_tr['synd']) and np.allclose(te['synd'], ref_te['synd']))
rng = np.random.default_rng(1)
for t0 in [5, 10, 15]:
    Xtr, ytr, atr = feats(tr, t0); Xte, yte, ate = feats(te, t0)
    HIST = BASE + [k for k in Xtr if k.startswith('h_')]
    m0 = model().fit(mat(Xtr, HIST), ytr); p0 = m0.predict_proba(mat(Xte, HIST))[:, 1]
    a0 = roc_auc_score(yte, p0)
    print(f'\nt0={t0}: test active={len(yte)}, will fail={yte.sum()}; AUC(hist)={a0:.4f}')
    for K in [1, 2, 3, 5, 8]:
        key = f'lamc{K}'
        Xtr[key] = np.log(tr[key][atr, t0 - 1] + 1e-9); Xte[key] = np.log(te[key][ate, t0 - 1] + 1e-9)
        single = roc_auc_score(yte, Xte[key]); single = max(single, 1 - single)
        m1 = model().fit(mat(Xtr, HIST + [key]), ytr); p1 = m1.predict_proba(mat(Xte, HIST + [key]))[:, 1]
        a1 = roc_auc_score(yte, p1)
        diffs = []
        for _ in range(1000):
            ii = rng.integers(0, len(yte), len(yte))
            diffs.append(roc_auc_score(yte[ii], p1[ii]) - roc_auc_score(yte[ii], p0[ii]))
        lo, hi = np.percentile(diffs, [2.5, 97.5])
        b, a, caught, lost = stopping_tradeoff(te, t0, p1, ate)
        b0, a0s, caught0, _ = stopping_tradeoff(te, t0, p0, ate)
        print(f'  K={K}: single AUC={single:.3f}  AUC(hist+lamc{K})={a1:.4f}  dAUC={a1-a0:+.4f} CI [{lo:+.4f},{hi:+.4f}]'
              f'   stop@1%: caught {caught0*100:.1f}% -> {caught*100:.1f}%')
