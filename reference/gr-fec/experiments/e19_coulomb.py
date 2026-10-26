"""Coulomb repulsion ('anti-codeword') re-decoding: after the MV2 answer cw*, re-run BP+blended OSD-2 on channel LLRs with a
repulsive charge q at cw* (llr' = llr + q*(2*cw* - 1)), then keep whichever candidate is likelier under the TRUE channel."""
import sys, time, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP; from osd import generator_from_H, osd_order_cands
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]); qs = [float(q) for q in sys.argv[4].split(',')]
H = read_alist('results/CCSDS_64_128.alist'); G = generator_from_H(H); lb = LayeredBP(H)
llr = np.concatenate([awgn_llr(6000, 128, eb, 0.5, np.random.default_rng(seed + i)) for i in range(N // 6000)])
c, h, fl, tot = lb.decode(llr, None, iters=20, reshuffle_rng=np.random.default_rng(seed + 9), track_flips=8)
_, _, _, tot5 = lb.decode(llr, None, iters=5, reshuffle_rng=np.random.default_rng(seed + 9), track_flips=1)
act = np.nonzero(c > 20)[0]; t = time.time()
mv2 = []; best = []
for i in act:
    L, P, P5 = llr[i], tot[i], tot5[i]
    R = [osd_order_cands(G, L, np.abs(L) + .25 * np.abs(P), L + .25 * P, 2), osd_order_cands(G, L, np.abs(P5), P5, 2)]
    j = int(np.argmin([R[0][1], R[1][1]])); mv2.append(R[j][0].any()); best.append(R[j])
mv2 = np.array(mv2)
print(f'Eb/N0={eb} N={N}: BP failures {len(act)}; MV2 FER {mv2.sum()/N:.5f}  ({time.time()-t:.0f}s)', flush=True)
for q in qs:
    t = time.time(); Lr = llr[act].copy()
    for k, i in enumerate(act):
        Lr[k] += q * (2.0 * best[k][0] - 1.0)
    c2, h2, fl2, tot2 = lb.decode(Lr, None, iters=20, reshuffle_rng=np.random.default_rng(seed + 19), track_flips=1)
    out = []
    for k, i in enumerate(act):
        cand = [best[k]]
        if c2[k] <= 20:                    # BP on the repelled LLRs converged to a codeword
            cw = h2[k]; cand.append((cw, float((cw * llr[i]).sum())))
        else:
            r = osd_order_cands(G, llr[i], np.abs(Lr[k]) + .25 * np.abs(tot2[k]), Lr[k] + .25 * tot2[k], 2); cand.append((r[0], r[1]))
        j = int(np.argmin([x[1] for x in cand])); out.append(cand[j][0].any())
    out = np.array(out)
    print(f'  repulsion q={q}: FER {out.sum()/N:.5f} (rescued {int((mv2 & ~out).sum())}, lost {int((~mv2 & out).sum())})  ({time.time()-t:.0f}s)', flush=True)
