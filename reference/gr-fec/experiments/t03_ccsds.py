import sys, numpy as np
sys.path.insert(0,'src')
from alist import read_alist, gf2_rank
from ldpc import girth
from curvature import node_ollivier, resistance_curvature, nb_closed_walks
H = read_alist('results/CCSDS_64_128.alist')
print('shape', H.shape, 'GF2 rank', gf2_rank(H), 'girth', girth(H), 'row wts', set(H.sum(1)), 'col wts', np.bincount(H.sum(0)))
# quasi-cyclic check: is H invariant under cyclic shift by 1 inside every 16-block?
P = 16
perm_c = np.array([ (i//P)*P + (i%P+1)%P for i in range(H.shape[1])])
perm_r = np.array([ (i//P)*P + (i%P+1)%P for i in range(H.shape[0])])
H2 = np.zeros_like(H); H2[np.ix_(perm_r, perm_c)] = H
print('QC with 16x16 circulants:', np.array_equal(H2, H))
f = nb_closed_walks(H)
F = {'c6':f[6],'c8':f[8],'c10':f[10],'oll2':node_ollivier(H,2),'oll3':node_ollivier(H,3),'res':resistance_curvature(H)}
for k,v in F.items():
    print(k, 'distinct values:', len(np.unique(np.round(v,8))), ' per block:', np.round([v[b*P] for b in range(8)],3))
