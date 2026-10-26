"""Idea B: do bit-selection rules (Lyapunov vector vs classics) find the bits that rescue failed frames?"""
import sys, time, json, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist
from ldpc import BP, awgn_llr
from bp_track import TrackBP
from repair import final_state, guess_redecode
from sklearn.metrics import roc_auc_score
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
H = read_alist('results/CCSDS_64_128.alist'); bp = BP(H); tb = TrackBP(H)
rng = np.random.default_rng(seed); t = time.time()
L = []
for c in range(N // 6000):
    llr = awgn_llr(6000, 128, eb, 0.5, rng); h, d, u = bp.decode(llr, iters=50); L.append(llr[~d])
llr = np.concatenate(L); B = len(llr)
S = final_state(tb, llr, T=50, seed=seed)
wrong = (S['tot'] < 0)                       # bits wrong at the final iteration (all-zero sent)
print(f'Eb/N0={eb} seed={seed}: {B} failed frames out of {N}; median wrong bits/frame {np.median(wrong.sum(1)):.0f}  ({time.time()-t:.0f}s)', flush=True)
rules = ['lyap', 'unrel', 'osc', 'chan', 'random']
S['random'] = rng.random(llr.shape)
loc = {}
for r in rules:
    aucs = [roc_auc_score(wrong[i], S[r][i]) for i in range(B) if 0 < wrong[i].sum() < 128]
    loc[r] = float(np.mean(aucs))
print('  localisation: mean per-frame AUC for finding currently-wrong bits:', {r: round(v, 3) for r, v in loc.items()}, flush=True)
res = {'B': B, 'loc': loc}
for r in rules:
    order = np.argsort(-S[r], axis=1)
    for k in [1, 2, 4, 8]:
        cor, wr = guess_redecode(bp, llr, order, k, S['tot'])
        res[f'{r}_k{k}'] = (cor.tolist(), wr.tolist())
        print(f'  rule={r:<6} k={k}: rescued correctly {cor.sum():4d}/{B} ({cor.mean()*100:5.1f}%), converged to WRONG codeword {wr.sum():3d}', flush=True)
json.dump(res, open(f'results/e08_eb{eb}_s{seed}.json', 'w'))
print(f'done {time.time()-t:.0f}s')
