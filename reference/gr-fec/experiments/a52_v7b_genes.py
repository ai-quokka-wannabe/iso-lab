"""PREREG36: genes and gene–fitness statistics of v7b; P36b, P36c; grand-pattern check across runs."""
import json, sys, numpy as np
sys.path.insert(0, 'src')
from evolve2 import NAMES, CH, per, glob, DISCRETE
import os
d = json.load(open(os.environ.get('SRC', 'results/e49_general.json'))); H = d['history']; E = np.array(H[-1]['elite'])
v7 = np.array(json.load(open('results/e47_organic.json'))['history'][-1]['elite'])
print('elite fitness by generation (mean of cold / warm / rich lives):')
for h in H: print(f"  gen {h['gen']:2d}: elite {h['elite_mean']:.3f}  cold {h['elite_by_world'][0]:.3f} warm {h['elite_by_world'][1]:.3f} rich {h['elite_by_world'][2]:.3f}  pop {h['pop_mean']:.3f}")
leta = np.median(glob(E, 'leta')); tinst = np.median(per(E, 'inst')[:, CH.index('temp')])
print(f"P36b learning rate: v7b 10^{leta:.2f} vs v7 10^{np.median(glob(v7, 'leta')):.2f} -> {'PASS' if leta >= np.median(glob(v7, 'leta')) + 1 else 'FAIL'}")
print(f"P36c |temperature instinct| {abs(tinst):.2f} (< 1?) -> {'PASS' if abs(tinst) < 1 else 'FAIL'}")
print(' ' * 14 + ' '.join(f'{c[:6]:>6s}' for c in CH))
for k in ('rew', 'inst', 'trust', 'bias'):
    print(f'{k + " median":13s} ' + ' '.join(f'{v:+6.2f}' for v in np.median(per(E, k), 0)))
print(f"grand-pattern check, learned trust in smell: e45 2.92 | v7 {np.median(per(v7, 'trust')[:, 0]):+.2f} | this run {np.median(per(E, 'trust')[:, 0]):+.2f}")
# gene–fitness correlations pooled over all generations' full populations
X = np.vstack([np.array(h['all_genomes']) for h in H]); y = np.concatenate([np.array(h['all_fitness']) for h in H])
cont = [i for i in range(len(NAMES)) if i not in DISCRETE]
r = np.array([np.corrcoef(X[:, i], y)[0, 1] for i in cont])
print('strongest gene–fitness correlations (all individuals, all generations):')
for i in np.argsort(-np.abs(r))[:12]: print(f'  {NAMES[cont[i]]:16s} r = {r[i]:+.2f}')
print(f'genes with |r| < 0.05 (selection-neutral): {(np.abs(r) < 0.05).sum()} of {len(cont)}')
