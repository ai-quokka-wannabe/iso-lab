"""BP + OSD hybrid with early hand-off: frames unconverged at iteration t0 go straight to OSD-2."""
import sys, time, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP; from osd import generator_from_H, osd
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
H = read_alist('results/CCSDS_64_128.alist'); G = generator_from_H(H); lb = LayeredBP(H)
rng = np.random.default_rng(seed); llr = np.concatenate([awgn_llr(6000, 128, eb, 0.5, rng) for _ in range(N // 6000)])
c, h = lb.decode(llr, None, iters=50, reshuffle_rng=np.random.default_rng(seed + 9))   # random-order BP, full run
osd_res = {}
def osd_ok(i):
    if i not in osd_res:
        osd_res[i] = not osd(G, llr[i], order=2).any()
    return osd_res[i]
t = time.time()
print(f'Eb/N0={eb} seed={seed} N={N}: BP-only FER (50 it) = {np.mean(c>50):.5f}')
for t0 in [2, 3, 5, 10, 20, 50]:
    handed = np.nonzero(c > t0)[0]
    errs = sum(0 if osd_ok(i) else 1 for i in handed)
    bp_work = np.minimum(c, t0).mean()
    lost = sum(1 for i in handed if c[i] <= 50 and not osd_ok(i))   # BP would have succeeded later, OSD failed
    print(f'  hand-off at t0={t0:2d}: OSD calls {len(handed)/N*100:6.2f}% of frames, avg BP iterations {bp_work:5.2f}, hybrid FER {errs/N:.5f} '
          f'(frames BP would have saved but OSD lost: {lost})', flush=True)
print(f'  ({time.time()-t:.0f}s)')
