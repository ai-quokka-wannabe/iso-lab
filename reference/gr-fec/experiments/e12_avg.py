import sys, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
H = read_alist('results/CCSDS_64_128.alist'); lb = LayeredBP(H)
rng = np.random.default_rng(seed); llr = np.concatenate([awgn_llr(6000, 128, eb, 0.5, rng) for _ in range(N // 6000)])
c0, h0 = lb.decode(llr, None, iters=50, reshuffle_rng=np.random.default_rng(seed + 5))
c1, h1 = lb.decode(llr, None, iters=50, reshuffle_rng=np.random.default_rng(seed + 5), avg_window=20)
f0 = c0 > 50; resc = f0 & (c1 <= 50); wrong = resc & (h1.sum(1) > 0)
print(f'Eb/N0={eb} seed={seed}: LAY-RND failures {f0.sum()} (FER {f0.mean():.5f}); AVG rescued {resc.sum()} ({resc.sum()/max(f0.sum(),1)*100:.1f}%), '
      f'of which wrong codewords {wrong.sum()}; FER after AVG {(f0 & ~(resc & ~wrong)).mean():.5f}; non-failed frames identical: {np.array_equal(c0[~f0], c1[~f0])}')
