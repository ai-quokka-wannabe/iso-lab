"""Does Tanner-graph curvature predict which bits fail, beyond cycle counts?"""
import sys, json, numpy as np
from scipy.stats import spearmanr, rankdata
sys.path.insert(0, 'src')
from curvature import node_ollivier, resistance_curvature, nb_closed_walks

rng = np.random.default_rng(0)
data = json.load(open(sys.argv[1] if len(sys.argv) > 1 else 'results/e01_failures.json'))
SNR_KEYS = sys.argv[2].split(',') if len(sys.argv) > 2 else None
MAXW = int(sys.argv[3]) if len(sys.argv) > 3 else 10**9   # keep only failures of weight <= MAXW


def feats_for(H):
    f = nb_closed_walks(H)
    F = {'c6': f[6], 'c8': f[8], 'c10': f[10], 'c12': f[12],
         'oll_t2': node_ollivier(H, 2), 'oll_t3': node_ollivier(H, 3), 'oll_t4': node_ollivier(H, 4),
         'res': resistance_curvature(H)}
    return F


def zrank(x):
    r = rankdata(x)
    return (r - r.mean()) / (r.std() + 1e-12)


def resid(y, X):
    X1 = np.column_stack([np.ones(len(y)), X])
    beta, *_ = np.linalg.lstsq(X1, y, rcond=None)
    return y - X1 @ beta


CYC = ['c6', 'c8', 'c10', 'c12']
CURV = ['oll_t2', 'oll_t3', 'oll_t4', 'res']
rows = []
per_code = []
for seed, rec in data.items():
    H = np.array(rec['H'], dtype=np.uint8)
    n = H.shape[1]
    keys = SNR_KEYS or [k for k in rec if k not in ('H', 'girth')]
    pats = [p for k in keys for p in rec[k]['patterns'] if len(p) <= MAXW]
    y = np.zeros(n)
    for p in pats:
        y[p] += 1
    # split-half noise ceiling
    idx = rng.permutation(len(pats))
    ya, yb = np.zeros(n), np.zeros(n)
    for i in idx[: len(idx) // 2]:
        ya[pats[i]] += 1
    for i in idx[len(idx) // 2:]:
        yb[pats[i]] += 1
    ceil = spearmanr(ya, yb).statistic
    F = feats_for(H)
    res = {'seed': seed, 'n_fail': len(pats), 'ceiling_split_half': ceil}
    for k, v in F.items():
        res['rho_' + k] = spearmanr(v, y).statistic
    # partial: residualise rank(y) and rank(curv) on ranks of cycle features
    Xc = np.column_stack([zrank(F[k]) for k in CYC])
    ry = resid(zrank(y), Xc)
    for k in CURV:
        rc = resid(zrank(F[k]), Xc)
        r_obs = np.corrcoef(ry, rc)[0, 1]
        null = [np.corrcoef(ry, rng.permutation(rc))[0, 1] for _ in range(2000)]
        res['partial_' + k] = r_obs
        res['p_' + k] = (np.sum(np.abs(null) >= abs(r_obs)) + 1) / 2001
    # incremental R^2 of rank(y): cycles vs cycles + curvature (in-sample, adjusted)
    def r2(X):
        X1 = np.column_stack([np.ones(n), X])
        b, *_ = np.linalg.lstsq(X1, zrank(y), rcond=None)
        e = zrank(y) - X1 @ b
        r2 = 1 - e.var() / zrank(y).var()
        p = X.shape[1]
        return 1 - (1 - r2) * (n - 1) / (n - p - 1)
    res['adjR2_cycles'] = r2(Xc)
    res['adjR2_cycles+curv'] = r2(np.column_stack([Xc] + [zrank(F[k]) for k in CURV]))
    res['adjR2_curv_only'] = r2(np.column_stack([zrank(F[k]) for k in CURV]))
    per_code.append(res)
    rows.append((F, y))

# leave-one-code-out: train linear model on ranks from other codes, test on held-out code
def loco(cols):
    out = []
    for i in range(len(rows)):
        Xtr = np.vstack([np.column_stack([zrank(rows[j][0][k]) for k in cols]) for j in range(len(rows)) if j != i])
        ytr = np.concatenate([zrank(rows[j][1]) for j in range(len(rows)) if j != i])
        b, *_ = np.linalg.lstsq(np.column_stack([np.ones(len(ytr)), Xtr]), ytr, rcond=None)
        Xte = np.column_stack([zrank(rows[i][0][k]) for k in cols])
        pred = np.column_stack([np.ones(len(Xte)), Xte]) @ b
        out.append(spearmanr(pred, rows[i][1]).statistic)
    return np.array(out)

np.set_printoptions(precision=3, suppress=True)
keys = list(per_code[0].keys())[1:]
print(f"{'metric':<22}" + ''.join(f"{r['seed']:>8}" for r in per_code) + f"{'mean':>9}")
for k in keys:
    vals = np.array([r[k] for r in per_code], float)
    print(f"{k:<22}" + ''.join(f"{v:8.3f}" for v in vals) + f"{vals.mean():9.3f}")
print()
for name, cols in [('cycles', CYC), ('curvature', CURV), ('cycles+curv', CYC + CURV), ('c6 only', ['c6'])]:
    s = loco(cols)
    print(f"LOCO held-out Spearman  {name:<12}: " + ' '.join(f'{v:.3f}' for v in s) + f"   mean={s.mean():.3f}")
