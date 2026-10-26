import sys, numpy as np
sys.path.insert(0, 'src')
from qc_layered import read_qc, QCLayeredBP
S, Z, p = read_qc('results/AR4JA_4096_8192.qc'); dec = QCLayeredBP(S, Z, p)
which = sys.argv[1]
rng = np.random.default_rng(2000); sig2 = 1 / (2 * 0.5 * 10 ** (0.9 / 10))
class Fixed:
    def __init__(s, o): s.o = np.array(o)
    def permutation(s, m): return s.o
orders = {'natural': None, 'fixed perm A': Fixed(np.random.default_rng(1).permutation(12)), 'fixed perm B': Fixed(np.random.default_rng(2).permutation(12)),
          'reversed': Fixed(np.arange(11, -1, -1))}
F = []
for b in range(4):
    llr = 2 * (1 + np.sqrt(sig2) * rng.standard_normal((500, 8192))) / sig2
    F.append(dec.decode(llr, iters=50, rng=orders[which])[0] > 50)
F = np.concatenate(F)
print(f'AR4JA 0.9 dB, same 2000 frames as e13: order {which:<13} {"" if orders[which] is None else orders[which].o}: FER {F.mean():.4f}', flush=True)
