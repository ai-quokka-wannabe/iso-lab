"""PREREG4: cheap checkpoint probe (K=8) on fresh frames, both SNRs."""
import sys, numpy as np
sys.path.insert(0, 'experiments')
from a05_stopping import feats, model, mat, BASE, stopping_tradeoff
from sklearn.metrics import roc_auc_score

rng = np.random.default_rng(7)
t0 = 10
for eb, s_tr, s_te in [(3.0, 100, 300), (3.5, 101, 301)]:
    tr = dict(np.load(f'results/e05_eb{eb}_s{s_tr}_cheap.npz')); te = dict(np.load(f'results/e05_eb{eb}_s{s_te}_cheap.npz'))
    Xtr, ytr, atr = feats(tr, t0); Xte, yte, ate = feats(te, t0)
    HIST = BASE + [k for k in Xtr if k.startswith('h_')]
    p0 = model().fit(mat(Xtr, HIST), ytr).predict_proba(mat(Xte, HIST))[:, 1]
    a0 = roc_auc_score(yte, p0)
    n = len(te['conv']); act_frac = ate.mean()
    avg_it = np.minimum(te['conv'], 50).mean()
    print(f'\n{eb} dB  train seed {s_tr} -> test seed {s_te}: {n} frames, FER={np.mean(te["conv"]>50):.4f}, '
          f'active at t0={t0}: {ate.sum()} ({act_frac*100:.1f}%), of which fail {yte.sum()}; avg decoder iterations {avg_it:.2f}')
    print(f'  AUC(hist) = {a0:.4f}')
    for K in [5, 8, 12]:
        key = f'lamc{K}'
        Xtr[key] = np.log(tr[key][atr, t0 - 1] + 1e-9); Xte[key] = np.log(te[key][ate, t0 - 1] + 1e-9)
        p1 = model().fit(mat(Xtr, HIST + [key]), ytr).predict_proba(mat(Xte, HIST + [key]))[:, 1]
        a1 = roc_auc_score(yte, p1)
        diffs = []
        for _ in range(2000):
            ii = rng.integers(0, len(yte), len(yte))
            diffs.append(roc_auc_score(yte[ii], p1[ii]) - roc_auc_score(yte[ii], p0[ii]))
        lo, hi = np.percentile(diffs, [2.5, 97.5])
        overhead = K * act_frac          # extra BP-step equivalents per frame, probing only active frames once
        tag = '  <-- PRE-REGISTERED' if K == 8 else ''
        print(f'  K={K:>2}: AUC(hist+lamc{K}) = {a1:.4f}   dAUC = {a1-a0:+.4f}  95% CI [{lo:+.4f}, {hi:+.4f}]   '
              f'overhead = {overhead:.2f} steps/frame ({overhead/avg_it*100:.1f}% of avg iterations){tag}')
