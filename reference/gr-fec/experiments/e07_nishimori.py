"""PREREG5: decoder temperature mismatch (LLR scaling alpha) vs FER and decoder chaos."""
import sys, time, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist
from ldpc import awgn_llr
from bp_track import TrackBP
seed, N = int(sys.argv[1]), int(sys.argv[2])
eb = float(sys.argv[4]) if len(sys.argv) > 4 else 3.0
alphas = [float(a) for a in sys.argv[3].split(',')]
code = sys.argv[5] if len(sys.argv) > 5 else 'ccsds'
H = read_alist('results/CCSDS_64_128.alist') if code == 'ccsds' else np.load(f'results/H_{code}.npy'); dec = TrackBP(H); n = H.shape[1]
for a in alphas:
    rng = np.random.default_rng(seed)          # same channel noise for every alpha (paired design)
    t = time.time(); keep = {}
    acc = {k: [] for k in ['conv', 'ftle', 'endclass', 'undet', 'ent_end', 'ent10']}
    for c in range(N // 3000):
        llr = awgn_llr(3000, n, eb, 0.5, rng)
        F, conv, hc = dec.run(a * llr, T=50, probes=1, seed=seed * 100 + c)
        ftle = np.log(np.maximum(F['lam'][:, 30:50], 1e-30)).mean(1)
        dn_end = F['dnorm'][:, 40:].max(1); fl_end = F['nflip'][:, 40:].sum(1)
        cls = np.where(fl_end > 0, 2, np.where(dn_end < 1e-3, 0, 1))     # 0 fixed, 1 drifting, 2 oscillating
        acc['conv'].append(conv); acc['ftle'].append(ftle); acc['endclass'].append(cls)
        acc['undet'].append(((hc.sum(1) > 0) & (conv <= 50)).astype(int))
        acc['ent_end'].append(F['ent'][:, 49]); acc['ent10'].append(F['ent'][:, 9])
    out = {k: np.concatenate(v) for k, v in acc.items()}
    np.savez_compressed(((f'results/e07_alpha{a}_s{seed}.npz' if eb == 3.0 else f'results/e07_eb{eb}_alpha{a}_s{seed}.npz') if code == 'ccsds' else f'results/e21_{code}_eb{eb}_alpha{a}_s{seed}.npz'), **out)
    fail = out['conv'] > 50
    print(f'alpha={a}: FER={fail.mean():.4f} ({fail.sum()}), undetected={out["undet"].sum()}, '
          f'FTLE(fail) median={np.median(out["ftle"][fail]):+.3f}, classes(fail) fixed/drift/osc = '
          f'{np.bincount(out["endclass"][fail], minlength=3).tolist()}  {time.time()-t:.0f}s', flush=True)
