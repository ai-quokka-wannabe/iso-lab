"""CCSDS (128,64): per-iteration decoder-state features for every frame, saved for analysis."""
import sys, time, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist
from ldpc import awgn_llr
from bp_track import TrackBP
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
CHEAP = len(sys.argv) > 4 and sys.argv[4] == 'cheap'
H = read_alist('results/CCSDS_64_128.alist'); dec = TrackBP(H)
rng = np.random.default_rng(seed)
parts = []; t = time.time()
for c in range(N // 3000):
    llr = awgn_llr(3000, 128, eb, 0.5, rng)
    F, conv, hc = dec.run(llr, T=50, probes=0 if CHEAP else 2, seed=seed * 1000 + c, probe_at=(4, 9, 14) if CHEAP else (), kcold=(1, 2, 3, 5, 8, 12))
    parts.append((F, conv, hc.sum(1)))
F = {k: np.concatenate([p[0][k] for p in parts]) for k in parts[0][0]}
conv = np.concatenate([p[1] for p in parts]); und = np.concatenate([p[2] for p in parts])
np.savez_compressed(f'results/e05_eb{eb}_s{seed}{"_cheap" if CHEAP else ""}.npz', conv=conv, undetected_wt=und, **F)
print(f'Eb/N0={eb} seed={seed}: {len(conv)} frames, fails={int((conv>50).sum())}, undetected={int(((und>0)&(conv<=50)).sum())}, {time.time()-t:.0f}s', flush=True)
