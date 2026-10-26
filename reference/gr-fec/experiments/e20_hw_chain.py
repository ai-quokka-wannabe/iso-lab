"""Hardware-like chain: 6-bit channel LLRs, normalised min-sum (x0.75) layered BP, fixed order, 20 it.
-> MV2 OSD-2 (blend of quantised LLR + 0.25 posterior; posterior at it. 5) -> gravity gate r >= 0.5."""
import sys, time, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP; from osd import generator_from_H, osd_order_cands
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
H = read_alist('results/CCSDS_64_128.alist'); G = generator_from_H(H); lb = LayeredBP(H)
llr0 = np.concatenate([awgn_llr(6000, 128, eb, 0.5, np.random.default_rng(seed + i)) for i in range(N // 6000)])
llr = np.clip(np.round(llr0 * 2) / 2, -15.5, 15.5)
kw = dict(minsum=0.75, qclip=15.5)
c, h, fl, tot = lb.decode(llr, np.arange(64), iters=20, track_flips=1, **kw)
_, _, _, tot5 = lb.decode(llr, np.arange(64), iters=5, track_flips=1, **kw)
ok = c <= 20; bp_und = int((ok & (h.sum(1) > 0)).sum()); act = np.nonzero(~ok)[0]; t = time.time()
wrong1 = []; wrong2 = []; gate = []
for i in act:
    L, P, P5 = llr[i], tot[i], tot5[i]
    R = [osd_order_cands(G, L, np.abs(L) + .25 * np.abs(P), L + .25 * P, 2), osd_order_cands(G, L, np.abs(P5) + 1e-3 * np.abs(L), P5 + 1e-3 * L, 2)]
    j = int(np.argmin([R[0][1], R[1][1]]))
    wrong1.append(R[0][0].any()); wrong2.append(R[j][0].any()); gate.append(R[j][2] / max(R[j][3], 1e-9) >= 0.5)
w1, w2, g = map(np.array, (wrong1, wrong2, gate))
print(f'Eb/N0={eb} N={N} [6-bit LLRs, min-sum 0.75, fixed order, 20 it]: BP FER {len(act)/N:.5f} (undetected {bp_und}); '
      f'+blend OSD-2 FER {w1.sum()/N:.5f}; +MV2 FER {w2.sum()/N:.5f}; MV2+gate: accepted-wrong {int((w2 & g).sum())}, '
      f'retransmissions {(~g).sum()/N:.5f} ({(~g).sum()/max(len(act),1):.3f} x BP-only)  ({time.time()-t:.0f}s)', flush=True)
