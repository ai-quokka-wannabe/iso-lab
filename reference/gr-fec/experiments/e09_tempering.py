import sys, time, json, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist
from ldpc import awgn_llr
from bp_track import TrackBP
from sched import decode_sched, schedules
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
names = sys.argv[4].split('|') if len(sys.argv) > 4 else None
H = read_alist('results/CCSDS_64_128.alist'); dec = TrackBP(H)
rng = np.random.default_rng(seed); llr = np.concatenate([awgn_llr(6000, 128, eb, 0.5, rng) for _ in range(N // 6000)])
S = schedules(); out = {}
for name, (al, d) in S.items():
    if names and name not in names:
        continue
    t = time.time(); conv, hard = decode_sched(dec, llr, al, d, return_hard=True)
    fail = conv > 50; und = (~fail) & (hard.sum(1) > 0)
    out[name] = fail.astype(int).tolist()
    print(f'{name:<22} FER={fail.mean():.5f} ({fail.sum()})  undetected={und.sum()}  avg iters={np.minimum(conv,50).mean():.2f}  {time.time()-t:.0f}s', flush=True)
json.dump(out, open(f'results/e09_eb{eb}_s{seed}.json', 'w'))
