import sys, time, numpy as np
sys.path.insert(0, 'src')
from ldpc import random_regular_no4cycles
from curvature import node_ollivier, resistance_curvature, nb_closed_walks
rng = np.random.default_rng(1)
H = random_regular_no4cycles(200, 3, 6, rng)
for t in [1,2,3]:
    s=time.time(); k = node_ollivier(H, t=t); print(f'Ollivier t={t}: mean={k.mean():.4f} std={k.std():.2e} min={k.min():.4f} max={k.max():.4f}  ({time.time()-s:.1f}s)')
s=time.time(); r = resistance_curvature(H); print(f'resistance: mean={r.mean():.4f} std={r.std():.2e} ({time.time()-s:.1f}s)')
s=time.time(); f = nb_closed_walks(H); print({L:(f[L].mean().round(2), f[L].std().round(2)) for L in f}, f'({time.time()-s:.1f}s)')
from scipy.stats import spearmanr
print('rho(oll t2, c6)', spearmanr(node_ollivier(H,t=2), f[6]).statistic, ' rho(res, c6)', spearmanr(r, f[6]).statistic, ' rho(res,c8)', spearmanr(r,f[8]).statistic)
