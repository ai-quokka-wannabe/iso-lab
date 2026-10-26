import sys, time, json, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist
from ldpc import awgn_llr
from bp_track import TrackBP
from sched import decode_sched
from layered import LayeredBP
from scipy.stats import binomtest
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
H = read_alist('results/CCSDS_64_128.alist'); tb = TrackBP(H); lb = LayeredBP(H)
rng = np.random.default_rng(seed); llr = np.concatenate([awgn_llr(6000, 128, eb, 0.5, rng) for _ in range(N // 6000)])
R = {}; W = {}
t = time.time()
def chunked(d):
    return np.concatenate([decode_sched(tb, llr[i:i + 12000], np.ones(50), d) for i in range(0, len(llr), 12000)])
c = chunked(0.0); R['FLOOD'] = c > 50; W['FLOOD'] = np.minimum(c, 50).mean()
c = chunked(0.3); R['FLOOD-D'] = c > 50; W['FLOOD-D'] = np.minimum(c, 50).mean()
c, h = lb.decode(llr, np.arange(64), iters=50); R['LAY-FIX'] = c > 50; W['LAY-FIX'] = np.minimum(c, 50).mean()
c, h, flips, tot = lb.decode(llr, None, iters=50, reshuffle_rng=np.random.default_rng(seed + 1), track_flips=20)
R['LAY-RND'] = c > 50; W['LAY-RND'] = np.minimum(c, 50).mean()
und = {k: 0 for k in R}
# pipeline
fail = np.nonzero(c > 50)[0]; work = np.minimum(c, 50).astype(float)
score = flips[fail] + 1e-3 * (-np.abs(tot[fail])); order = np.argsort(-score, axis=1)
done = np.zeros(len(fail), bool); correct = np.zeros(len(fail), bool); prng = np.random.default_rng(seed + 2)
for j in range(4):
    for attempt in (0, 1):
        act = np.nonzero(~done)[0]
        if len(act) == 0: break
        bits = order[act, j]; cur = np.sign(tot[fail[act], bits]); cur[cur == 0] = 1
        L = llr[fail[act]].copy(); L[np.arange(len(act)), bits] = (-cur if attempt == 0 else cur) * 50.0
        cc, hh = lb.decode(L, None, iters=50, reshuffle_rng=prng)
        work[fail[act]] += np.minimum(cc, 50)
        ok = cc <= 50; done[act[ok]] = True; correct[act[ok]] = hh[ok].sum(1) == 0
pipe_fail = np.zeros(len(llr), bool); pipe_fail[fail[~correct]] = True
R['PIPE'] = pipe_fail; W['PIPE'] = work.mean(); und['PIPE'] = int((done & ~correct).sum())
print(f'Eb/N0={eb} seed={seed}: {len(llr)} frames ({time.time()-t:.0f}s)')
for k in R: print(f'  {k:<8} FER={R[k].mean():.5f} ({R[k].sum():5d})  avg work={W[k]:.2f} it/frame  undetected={und[k]}')
def mcn(a, b):   # one-sided: b better than a
    n10 = int((R[a] & ~R[b]).sum()); n01 = int((~R[a] & R[b]).sum())
    return n10, n01, binomtest(n10, n10 + n01, 0.5, alternative='greater').pvalue if n10 + n01 else 1.0
for a, b in [('LAY-FIX', 'LAY-RND'), ('FLOOD-D', 'LAY-RND'), ('FLOOD', 'LAY-RND')]:
    n10, n01, p = mcn(a, b); print(f'  McNemar {b} better than {a}: rescued {n10}, lost {n01}, one-sided p={p:.2e}')
print(f'  PIPE vs FLOOD: FER ratio {R["FLOOD"].mean()/max(R["PIPE"].mean(),1e-12):.2f}x lower')
json.dump({k: v.astype(int).tolist() for k, v in R.items()}, open(f'results/e10_eb{eb}_s{seed}.json', 'w'))
