import sys, numpy as np, json
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP; from servo import ServoLayeredBP
from scipy.stats import binomtest
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
H = read_alist('results/CCSDS_64_128.alist'); lb = LayeredBP(H); sv = ServoLayeredBP(H)
rng = np.random.default_rng(seed); llr = np.concatenate([awgn_llr(6000, 128, eb, 0.5, rng) for _ in range(N // 6000)])
c, h = lb.decode(llr, None, iters=50, reshuffle_rng=np.random.default_rng(seed + 5)); base = c > 50
c2, h2, _ = sv.decode(llr, np.random.default_rng(seed + 5), gain=0.2, mode='cool', remelt=True); f = c2 > 50
r, l = int((base & ~f).sum()), int((~base & f).sum())
p = binomtest(r, r + l, 0.5, alternative='greater').pvalue if r + l else 1.0
print(f'Eb/N0={eb} seed={seed} N={N}: LAY-RND FER={base.mean():.5f}  SERVO FER={f.mean():.5f}  rescued={r} lost={l}  McNemar one-sided p={p:.2e}  '
      f'undetected base={int(((~base)&(h.sum(1)>0)).sum())} servo={int(((~f)&(h2.sum(1)>0)).sum())}', flush=True)
