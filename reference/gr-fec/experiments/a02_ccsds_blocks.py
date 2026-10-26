"""Per-block analysis for CCSDS (128,64). Bits within a 16-bit circulant block are
equivalent under the code's cyclic symmetry, so within-block spread = pure noise."""
import sys, json, numpy as np
from scipy.stats import f_oneway, spearmanr
sys.path.insert(0, 'src')
from alist import read_alist
from curvature import node_ollivier, resistance_curvature, nb_closed_walks

H = read_alist('results/CCSDS_64_128.alist')
d = json.load(open('results/e02_ccsds_blocks.json'))
f = nb_closed_walks(H, lengths=(6, 8, 10))
feat = {'c6': f[6], 'c8': f[8], 'c10': f[10], 'oll2': node_ollivier(H, 2),
        'oll3': node_ollivier(H, 3), 'oll4': node_ollivier(H, 4), 'res': resistance_curvature(H)}
blkfeat = {k: np.array([v[b * 16] for b in range(8)]) for k, v in feat.items()}
print('block features (blocks 0-3 degree 5, 4-7 degree 3):')
for k, v in blkfeat.items():
    print(f'  {k:<5}', np.round(v, 4))
for eb, rec in d.items():
    cnt = np.array(rec['cnt']).reshape(8, 16)
    rate = cnt / rec['fails']          # bit-errors per failure frame
    print(f"\nEb/N0 {eb}: {rec['fails']} failures, {rec['frames']} frames, median weight {np.median(rec['weights']):.0f}")
    print('  per-block mean bit-error/failure:', np.round(rate.mean(1), 4), ' within-block SD:', np.round(rate.std(1, ddof=1), 4))
    for grp, blocks in [('degree-5', range(0, 4)), ('degree-3', range(4, 8))]:
        F = f_oneway(*[rate[b] for b in blocks])
        m = np.array([rate[b].mean() for b in blocks])
        line = f'  {grp}: ANOVA between blocks F={F.statistic:.2f} p={F.pvalue:.3g}; block means {np.round(m, 4)}'
        print(line)
        for k in ['c6', 'c8', 'oll3', 'oll4', 'res']:
            x = blkfeat[k][list(blocks)]
            print(f'      rho(block mean, {k:<4}) = {spearmanr(x, m).statistic:+.2f}')
