import sys, time, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP; from osd import generator_from_H, osd_blend_scored
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
H = read_alist('results/CCSDS_64_128.alist'); G = generator_from_H(H); lb = LayeredBP(H)
t = time.time(); acts = []; wrong = []; ratio = []; bp_und = 0; tot_fail = 0
for b in range(N // 24000):
    llr = np.concatenate([awgn_llr(6000, 128, eb, 0.5, np.random.default_rng(seed * 100 + b * 4 + i)) for i in range(4)])
    c, h, fl, tot = lb.decode(llr, None, iters=20, reshuffle_rng=np.random.default_rng(seed * 100 + b + 77), track_flips=1)
    ok = c <= 20; bp_und += int((ok & (h.sum(1) > 0)).sum()); act = np.nonzero(~ok)[0]; tot_fail += len(act)
    for i in act:
        cw, gap, action = osd_blend_scored(G, llr[i], tot[i], 0.25, 2)
        wrong.append(cw.any()); ratio.append(gap / max(action, 1e-9))
wrong = np.array(wrong); ratio = np.array(ratio); acc = ratio >= 0.5
aw = int((wrong & acc).sum()); det = int((~acc).sum())
print(f'Eb/N0={eb} seed={seed} N={N}: BP-only detected FER {tot_fail/N:.5f}, BP-only undetected {bp_und}', flush=True)
print(f'  ungated hybrid: undetected {int(wrong.sum())} (UER {(wrong.sum()+bp_und)/N:.1e})', flush=True)
print(f'  GATED (r>=0.5): accepted-but-wrong {aw} -> UER {(aw+bp_und)/N:.1e}; detected FER {det/N:.5f} = {det/max(tot_fail,1):.3f} x BP-only; '
      f'OSD wins kept {((~wrong)&acc).sum()/max((~wrong).sum(),1)*100:.1f}%', flush=True)
print(f'  P12(a) {"PASS" if aw <= max(2, bp_und) else "FAIL"};  P12(b) {"PASS" if det <= 0.3 * tot_fail else "FAIL"}   ({time.time()-t:.0f}s)', flush=True)
