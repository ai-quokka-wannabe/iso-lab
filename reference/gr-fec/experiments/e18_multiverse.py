import sys, time, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP; from osd import generator_from_H, osd_order_cands
from scipy.stats import binomtest
eb, seed, N, mv6 = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), sys.argv[4] == '1'
H = read_alist('results/CCSDS_64_128.alist'); G = generator_from_H(H); lb = LayeredBP(H)
t = time.time(); rec = {k: [] for k in ['single', 'mv2', 'mv6', 'mv2_gate_ok']}; nfail = 0; bp_und = 0
for b in range(N // 24000):
    llr = np.concatenate([awgn_llr(6000, 128, eb, 0.5, np.random.default_rng(seed * 100 + b * 4 + i)) for i in range(4)])
    rs = seed * 100 + b + 55
    c, h, fl, tot = lb.decode(llr, None, iters=20, reshuffle_rng=np.random.default_rng(rs), track_flips=8)
    _, _, _, tot5 = lb.decode(llr, None, iters=5, reshuffle_rng=np.random.default_rng(rs), track_flips=1)
    ok = c <= 20; bp_und += int((ok & (h.sum(1) > 0)).sum()); act = np.nonzero(~ok)[0]; nfail += len(act)
    for i in act:
        L, P, P5, F = llr[i], tot[i], tot5[i], fl[i]
        U = [(np.abs(L) + .25 * np.abs(P), L + .25 * P), (np.abs(P5), P5)]
        if mv6:
            U += [(np.abs(L), L), (np.abs(P), P), (np.abs(L) + .25 * np.abs(P) - 0.5 * F * np.abs(L).mean(), L + .25 * P), (np.abs(L) + np.abs(P), L + P)]
        R = [osd_order_cands(G, L, o, s, 2) for o, s in U]
        rec['single'].append(R[0][0].any())
        j2 = int(np.argmin([R[0][1], R[1][1]])); rec['mv2'].append(R[j2][0].any())
        rec['mv2_gate_ok'].append(R[j2][2] / max(R[j2][3], 1e-9) >= 0.5)
        if mv6:
            j6 = int(np.argmin([r[1] for r in R])); rec['mv6'].append(R[j6][0].any())
rec = {k: np.array(v) for k, v in rec.items()}
def mcn(a, b):
    r = int((a & ~b).sum()); l = int((~a & b).sum()); return r, l, (binomtest(r, r + l, 0.5, alternative='greater').pvalue if r + l else 1.0)
r, l, p = mcn(rec['single'], rec['mv2'])
print(f'Eb/N0={eb} seed={seed} N={N}: BP(20) FER {nfail/N:.5f}, BP undetected {bp_und}; single-blend FER {rec["single"].sum()/N:.5f}; '
      f'MV2 FER {rec["mv2"].sum()/N:.5f} (rescued {r}, lost {l}, p={p:.1e}); ratio {rec["mv2"].sum()/max(rec["single"].sum(),1):.3f}', flush=True)
g = rec['mv2_gate_ok']; aw = int((rec['mv2'] & g).sum())
print(f'  MV2 + gravity gate: accepted-but-wrong {aw}, detected FER {(~g).sum()/N:.5f} ({(~g).sum()/max(nfail,1):.3f} x BP-only)', flush=True)
if mv6:
    r, l, p = mcn(rec['mv2'], rec['mv6']); print(f'  MV6 FER {rec["mv6"].sum()/N:.5f} vs MV2 (rescued {r}, lost {l}, p={p:.1e})', flush=True)
print(f'  ({time.time()-t:.0f}s)')
