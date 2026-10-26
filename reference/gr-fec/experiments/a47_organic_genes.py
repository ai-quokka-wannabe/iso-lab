"""Read what evolution kept in the organic brain (PREREG35 P35d)."""
import json, sys, numpy as np
sys.path.insert(0, 'src')
from evolve2 import NAMES, CH, per, glob, C
d = json.load(open('results/e47_organic.json')); H = d['history']
E = np.array(H[-1]['elite']); E0 = np.array(H[0]['elite'])
print(f'elite fitness: first two gens {H[0]["elite_mean"]:.3f} ({H[0]["kind"]}), {H[1]["elite_mean"]:.3f} ({H[1]["kind"]}); '
      f'last two {H[-2]["elite_mean"]:.3f} ({H[-2]["kind"]}), {H[-1]["elite_mean"]:.3f} ({H[-1]["kind"]})')
def row(name, M): return f'{name:13s} ' + ' '.join(f'{v:+6.2f}' for v in M)
print(' ' * 14 + ' '.join(f'{c[:6]:>6s}' for c in CH))
for k in ('rew', 'inst', 'trust', 'bias'):
    print(row(k + ' median', np.median(per(E, k), 0)))
print(row('grows nodes %', 100 * (per(E, 'grow') > 0).mean(0)))
print(row('width (log10)', np.median(per(E, 'lwidth'), 0)))
for k in ('leta', 'lam', 'integ', 'lag', 'ls0', 'ltau', 'pmax', 'lnov'):
    print(f'{k:6s} median {np.median(glob(E, k)):+.3f}')
gA = E[:, NAMES.index('gA')].astype(int); gB = E[:, NAMES.index('gB')].astype(int)
on = np.r_[gA[glob(E, 'gA_on') > 0], gB[glob(E, 'gB_on') > 0]]
print('gate channels in use among elites:', {CH[c]: int((on == c).sum()) for c in range(C) if (on == c).any()}, f'(of {2 * len(E)} gate slots)')
