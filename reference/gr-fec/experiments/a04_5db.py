"""5 dB re-simulation of the confirmation batch: do code differences grow towards the floor?"""
import glob, json, numpy as np
from scipy.stats import spearmanr

R = []
for f in sorted(glob.glob('results/e04_5db_part*.json')):
    R += json.load(open(f))
R = [r for r in R if r['name'] != 'ccsds']
f4 = np.array([r['fer'] for r in R]); f5 = np.array([r['fer5'] for r in R])
n4 = np.array([r['fails'] for r in R]); n5 = np.array([r['fails5'] for r in R])
print(f'{len(R)} lifts re-simulated at 5 dB')
for lab, f, n in [('4 dB', f4, n4), ('5 dB', f5, n5)]:
    y = np.log(f); obs = y.var(ddof=1); noise = np.mean(1 / np.maximum(n, 1))
    rel = max(0, (obs - noise) / obs)
    print(f'  {lab}: FER {f.min():.2e}..{f.max():.2e} (ratio {f.max()/f.min():.2f}), var(logFER)={obs:.4f}, noise={noise:.4f}, reliability={rel:.2f}')
w4 = np.median([np.median(r['wts5']) for r in R if r['wts5']])
print(f'  median failure weight at 5 dB: {w4:.0f} bits')
rho = spearmanr(f4, f5).statistic
rng = np.random.default_rng(0)
null = [spearmanr(rng.permutation(f4), f5).statistic for _ in range(20000)]
print(f'  rank agreement FER(4 dB) vs FER(5 dB): rho = {rho:+.3f}, one-sided p = {(np.sum(np.array(null) >= rho)+1)/20001:.4f}')
print('  feature correlations with FER at 5 dB (exploratory):')
for k in ['c6', 'c8', 'c10', 'oll2_mean', 'oll3_mean', 'oll3_min', 'res_mean', 'res_min']:
    print(f'    rho({k:<9}, FER5) = {spearmanr([r[k] for r in R], f5).statistic:+.3f}')
