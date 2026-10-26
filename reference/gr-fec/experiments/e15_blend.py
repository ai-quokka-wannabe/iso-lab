import sys, numpy as np, time
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP; from osd import generator_from_H, osd, osd_blend
from scipy.stats import binomtest
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
H = read_alist('results/CCSDS_64_128.alist'); G = generator_from_H(H); lb = LayeredBP(H)
llr = np.concatenate([awgn_llr(6000, 128, eb, 0.5, np.random.default_rng(seed + i)) for i in range(N // 6000)])
c, h, fl, tot = lb.decode(llr, None, iters=20, reshuffle_rng=np.random.default_rng(seed + 9), track_flips=1)
act = np.nonzero(c > 20)[0]; t = time.time()
ch = np.array([osd(G, llr[i], 2).any() for i in act]); bl = np.array([osd_blend(G, llr[i], tot[i], 0.25, 2).any() for i in act])
r = int((ch & ~bl).sum()); l = int((~ch & bl).sum()); p = binomtest(r, r + l, 0.5, alternative='greater').pvalue if r + l else 1.0
print(f'Eb/N0={eb} seed={seed} N={N}: BP(20) FER={len(act)/N:.5f}; hybrid FER channel-OSD2={ch.sum()/N:.5f}  blend-OSD2={bl.sum()/N:.5f}  '
      f'(rescued {r}, lost {l}, McNemar one-sided p={p:.2e})  {time.time()-t:.0f}s', flush=True)
