"""Collect per-bit failure participation for several random (3,6) codes."""
import sys, time, json, numpy as np
sys.path.insert(0, 'src')
from ldpc import random_regular_no4cycles, girth, BP, awgn_llr
n, dv, dc, R = 200, 3, 6, 0.5
SNRS = [3.0, 3.5]
TARGET = 400        # failures per (code, snr)
CAP = 1_500_000     # frames cap per (code, snr)
out = {}
for seed in range(6):
    rng = np.random.default_rng(1000 + seed)
    H = random_regular_no4cycles(n, dv, dc, rng)
    dec = BP(H)
    rec = {'H': H.tolist(), 'girth': int(girth(H))}
    for eb in SNRS:
        cnt = np.zeros(n); wts = []; frames = 0; fails = 0; t = time.time()
        pats = []
        while fails < TARGET and frames < CAP:
            llr = awgn_llr(20000, n, eb, R, rng)
            hard, done, used = dec.decode(llr, iters=50)
            bad = hard.sum(1) > 0
            frames += len(llr)
            if bad.any():
                cnt += hard[bad].sum(0)
                wts += hard[bad].sum(1).tolist()
                pats += [np.nonzero(h)[0].tolist() for h in hard[bad]]
                fails += int(bad.sum())
        rec[str(eb)] = {'frames': frames, 'fails': fails, 'cnt': cnt.tolist(), 'weights': wts, 'patterns': pats}
        print(f'seed {seed} girth {rec["girth"]} Eb/N0 {eb}: FER={fails/frames:.2e} ({fails} fails / {frames} frames) median wt={np.median(wts):.0f}  {time.time()-t:.0f}s', flush=True)
    out[seed] = rec
json.dump(out, open('results/e01_failures.json', 'w'))
