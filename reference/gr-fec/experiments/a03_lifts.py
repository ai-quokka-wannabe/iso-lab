"""Rank QC lifts by FER: curvature vs cycle-count design rules."""
import sys, json, glob, numpy as np
from scipy.stats import spearmanr, rankdata

res = []
PAT = sys.argv[1] if len(sys.argv) > 1 else 'results/e03_lifts_11.json'
for f in sorted(glob.glob(PAT)):
    res += json.load(open(f))
if len(sys.argv) > 2 and sys.argv[2] == 'no_ccsds':
    res = [r for r in res if r['name'] != 'ccsds']
fer = np.array([r['fer'] for r in res])
y = np.log(fer)
keys = ['c6', 'c8', 'c10', 'oll2_mean', 'oll2_min', 'oll3_mean', 'oll3_min', 'res_mean', 'res_min']
rng = np.random.default_rng(0)
print(f'{len(res)} codes; FER range {fer.min():.2e} .. {fer.max():.2e} (ratio {fer.max()/fer.min():.2f})')
ccsds = [r for r in res if r['name'] == 'ccsds']
if ccsds:
    rank = (fer < ccsds[0]['fer']).sum() + 1
    print(f"CCSDS code FER {ccsds[0]['fer']:.2e} ranks {rank} of {len(res)} (1 = best)")


def zr(x):
    r = rankdata(x); return (r - r.mean()) / (r.std() + 1e-12)


def resid(v, X):
    X1 = np.column_stack([np.ones(len(v)), X])
    b, *_ = np.linalg.lstsq(X1, v, rcond=None)
    return v - X1 @ b


Xc = np.column_stack([zr([r['c6'] for r in res]), zr([r['c8'] for r in res])])
ry = resid(zr(y), Xc)
print(f"{'feature':<10}{'rho(FER)':>10}{'  95% CI':>18}{'partial|c6,c8':>15}{'p_perm':>8}")
for k in keys:
    x = np.array([r[k] for r in res], float)
    rho = spearmanr(x, y).statistic
    bs = []
    for _ in range(2000):
        i = rng.integers(0, len(y), len(y))
        bs.append(spearmanr(x[i], y[i]).statistic)
    lo, hi = np.nanpercentile(bs, [2.5, 97.5])
    if k in ('c6', 'c8'):
        print(f'{k:<10}{rho:10.3f}   [{lo:6.3f},{hi:6.3f}]')
        continue
    rx = resid(zr(x), Xc)
    pr = np.corrcoef(ry, rx)[0, 1]
    null = [np.corrcoef(ry, rng.permutation(rx))[0, 1] for _ in range(5000)]
    p = (np.sum(np.abs(null) >= abs(pr)) + 1) / 5001
    print(f'{k:<10}{rho:10.3f}   [{lo:6.3f},{hi:6.3f}]{pr:15.3f}{p:8.3f}')
# measurement noise on FER: relative SE ~ 1/sqrt(fails)
print('median relative SE of FER estimates:', np.median(1 / np.sqrt([r['fails'] for r in res])).round(3))

# pre-registered one-sided test of H1: Spearman(oll3_mean, FER) < 0
x = np.array([r['oll3_mean'] for r in res]); rho = spearmanr(x, y).statistic
null = np.array([spearmanr(rng.permutation(x), y).statistic for _ in range(20000)])
print(f"H1 (pre-registered): rho(oll3_mean, FER) = {rho:.3f}, one-sided permutation p = {(np.sum(null <= rho)+1)/(len(null)+1):.4f}")
rx = resid(zr(x), Xc); pr = np.corrcoef(ry, rx)[0, 1]
nullp = np.array([np.corrcoef(ry, rng.permutation(rx))[0, 1] for _ in range(20000)])
print(f"H2 (pre-registered): partial rho | c6,c8 = {pr:.3f}, one-sided p = {(np.sum(nullp <= pr)+1)/(len(nullp)+1):.4f}")
