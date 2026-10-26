"""Does the BP-map Jacobian (curvature-linked stability) predict eventual non-convergence
better than standard stopping-rule statistics?

Usage: python3 a05_stopping.py TRAIN.npz [TEST.npz]
 - with one file: exploratory, 5-fold CV within it
 - with two files: fit on TRAIN, frozen, evaluate on TEST (confirmation)
"""
import sys, json, numpy as np
from sklearn.linear_model import LogisticRegression
from sklearn.preprocessing import StandardScaler
from sklearn.pipeline import make_pipeline
from sklearn.metrics import roc_auc_score
from sklearn.model_selection import StratifiedKFold

T = 50
CHECKS = [5, 10, 15, 20]


def load(path):
    d = np.load(path)
    return {k: d[k] for k in d.files}


def feats(d, t0):
    i = t0 - 1
    act = d['conv'] > t0
    y = (d['conv'] > T)[act].astype(int)
    g = lambda k, j=i: d[k][act, j]
    X = {
        'synd': g('synd'),
        'synd_trend': g('synd') - g('synd', i - 3),
        'mabs': np.log(g('mabs') + 1e-9),
        'mabs_trend': np.log(g('mabs') + 1e-9) - np.log(g('mabs', i - 3) + 1e-9),
        'nflip': g('nflip'),
        'dnorm': np.log(g('dnorm') + 1e-9),
        'ratio': np.log(g('ratio') + 1e-9),
        'lam': np.log(g('lam') + 1e-9),
        'lam_trend': np.log(g('lam') + 1e-9) - np.log(g('lam', i - 3) + 1e-9),
    }
    for j in range(t0):   # full trajectory history of standard statistics
        X[f'h_synd{j}'] = g('synd', j)
        X[f'h_mabs{j}'] = np.log(g('mabs', j) + 1e-9)
        X[f'h_dn{j}'] = np.log(g('dnorm', j) + 1e-9)
    return X, y, act


BASE = ['synd', 'synd_trend', 'mabs', 'mabs_trend', 'nflip', 'dnorm', 'ratio']
CURV = ['lam', 'lam_trend']


def model():
    return make_pipeline(StandardScaler(), LogisticRegression(max_iter=2000, C=1.0))


def mat(X, cols):
    return np.column_stack([X[c] for c in cols])


def stopping_tradeoff(d, t0, score, act, budget_rel=0.01):
    """Abandon active frames at t0 whose score exceeds a threshold chosen so that the number of
    abandoned would-have-converged frames is <= budget_rel * (#failures). Returns avg iterations
    with and without the rule, and the fraction of failures caught."""
    conv = d['conv']
    fail = conv > T
    n_fail = fail.sum()
    a_idx = np.nonzero(act)[0]
    order = np.argsort(-score)
    ok_lost = np.cumsum(~fail[a_idx[order]])
    k = np.searchsorted(ok_lost, int(np.floor(budget_rel * n_fail)), side='right')
    stop = a_idx[order[:k]]
    it = np.minimum(conv, T).astype(float)
    base_avg = it.mean()
    it2 = it.copy(); it2[stop] = t0
    caught = fail[stop].sum() / max(n_fail, 1)
    return base_avg, it2.mean(), caught, int((~fail[stop]).sum())


def run(train, test=None):
    out = {}
    for t0 in CHECKS:
        Xtr, ytr, acttr = feats(train, t0)
        row = {'t0': t0, 'n_active': int(len(ytr)), 'n_fail_active': int(ytr.sum())}
        # single-feature AUCs (on test if given, else train)
        Xev, yev, actev = feats(test, t0) if test is not None else (Xtr, ytr, acttr)
        for c in BASE + CURV:
            a = roc_auc_score(yev, Xev[c]); row['auc_' + c] = max(a, 1 - a) * (1 if a >= 0.5 else -1)
        HIST = [k for k in Xtr if k.startswith('h_')]
        for name, cols in [('base', BASE), ('base+curv', BASE + CURV), ('curv', CURV), ('hist', BASE + HIST), ('hist+curv', BASE + HIST + CURV)]:
            if test is None:
                pred = np.zeros(len(ytr))
                for tr, te in StratifiedKFold(5, shuffle=True, random_state=0).split(mat(Xtr, cols), ytr):
                    m = model().fit(mat(Xtr, cols)[tr], ytr[tr]); pred[te] = m.predict_proba(mat(Xtr, cols)[te])[:, 1]
                ev_y, ev_d, ev_act = ytr, train, acttr
            else:
                m = model().fit(mat(Xtr, cols), ytr); pred = m.predict_proba(mat(Xev, cols))[:, 1]
                ev_y, ev_d, ev_act = yev, test, actev
            row['AUC_' + name] = roc_auc_score(ev_y, pred)
            row['pred_' + name] = pred
            b, a, caught, lost = stopping_tradeoff(ev_d, t0, pred, ev_act)
            row['stop_' + name] = (b, a, caught, lost)
        # bootstrap CI for AUC difference (base+curv minus base)
        rng = np.random.default_rng(1); diffs = []
        for _ in range(1000):
            ii = rng.integers(0, len(ev_y), len(ev_y))
            if ev_y[ii].min() == ev_y[ii].max():
                continue
            diffs.append(roc_auc_score(ev_y[ii], row['pred_base+curv'][ii]) - roc_auc_score(ev_y[ii], row['pred_base'][ii]))
        row['dAUC_CI'] = np.percentile(diffs, [2.5, 97.5])
        diffs = []
        for _ in range(1000):
            ii = rng.integers(0, len(ev_y), len(ev_y))
            if ev_y[ii].min() == ev_y[ii].max():
                continue
            diffs.append(roc_auc_score(ev_y[ii], row['pred_hist+curv'][ii]) - roc_auc_score(ev_y[ii], row['pred_hist'][ii]))
        row['dAUC_hist_CI'] = np.percentile(diffs, [2.5, 97.5])
        out[t0] = row
    return out


def report(out):
    for t0, r in out.items():
        print(f"\n--- checkpoint t0={t0}: {r['n_active']} active frames, {r['n_fail_active']} will fail ---")
        print('  single-feature AUC (sign = direction):', '  '.join(f"{c}={r['auc_'+c]:+.3f}" for c in BASE + CURV))
        print(f"  model AUC: base={r['AUC_base']:.4f}  base+curv={r['AUC_base+curv']:.4f}  curv-only={r['AUC_curv']:.4f}"
              f"   dAUC 95% CI [{r['dAUC_CI'][0]:+.4f}, {r['dAUC_CI'][1]:+.4f}]")
        print(f"  model AUC: hist={r['AUC_hist']:.4f}  hist+curv={r['AUC_hist+curv']:.4f}   dAUC 95% CI [{r['dAUC_hist_CI'][0]:+.4f}, {r['dAUC_hist_CI'][1]:+.4f}]")
        for k in ['base', 'base+curv', 'hist', 'hist+curv']:
            b, a, caught, lost = r['stop_' + k]
            print(f"  stop rule ({k:<9}, <=1% of FER lost): avg iters {b:.2f} -> {a:.2f}  ({(1-a/b)*100:.1f}% saved), failures caught {caught*100:.1f}%, good frames lost {lost}")


if __name__ == '__main__':
    tr = load(sys.argv[1]); te = load(sys.argv[2]) if len(sys.argv) > 2 else None
    report(run(tr, te))
