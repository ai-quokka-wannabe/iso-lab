"""Undetected-error audit of the hybrid (random-order BP 20 it -> blended OSD-2) with gate statistics."""
import sys, time, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP; from osd import generator_from_H, osd_blend_scored
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
H = read_alist('results/CCSDS_64_128.alist'); G = generator_from_H(H); lb = LayeredBP(H)
llr = np.concatenate([awgn_llr(6000, 128, eb, 0.5, np.random.default_rng(seed + i)) for i in range(N // 6000)])
c, h, fl, tot = lb.decode(llr, None, iters=20, reshuffle_rng=np.random.default_rng(seed + 9), track_flips=1)
bp_ok = c <= 20; bp_und = bp_ok & (h.sum(1) > 0)
act = np.nonzero(~bp_ok)[0]; t = time.time()
R = [osd_blend_scored(G, llr[i], tot[i], 0.25, 2) for i in act]
osd_wrong = np.array([r[0].any() for r in R]); gap = np.array([r[1] for r in R]); action = np.array([r[2] for r in R])
np.savez_compressed(f'results/e16_eb{eb}_s{seed}.npz', act=act, osd_wrong=osd_wrong, gap=gap, action=action,
                    bp_und=bp_und, N=N)
print(f'Eb/N0={eb} seed={seed} N={N}: BP(20) detected failures {len(act)} ({len(act)/N:.5f}), BP undetected {bp_und.sum()}; '
      f'OSD on failures: wrong (UNDETECTED) {osd_wrong.sum()} -> hybrid UER {(osd_wrong.sum()+bp_und.sum())/N:.2e}  ({time.time()-t:.0f}s)', flush=True)
