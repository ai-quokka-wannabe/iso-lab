import sys, time, numpy as np
sys.path.insert(0,'src')
from alist import read_alist
from ldpc import BP, awgn_llr
from bp_track import TrackBP
H = read_alist('results/CCSDS_64_128.alist')
rng = np.random.default_rng(3); llr = awgn_llr(3000, 128, 3.0, 0.5, rng)
t=time.time(); F, conv, hc = TrackBP(H).run(llr, T=50, probes=2); dt=time.time()-t
h, done, used = BP(H).decode(llr, iters=50)
print(f'{dt:.1f}s for 3000 frames x 50 it x (1+2 probes)')
print('convergence agrees with reference decoder:', np.array_equal(np.where(done, used, 51), conv))
print('fails:', (conv>50).sum(), ' undetected among converged:', int(((hc.sum(1)>0)&(conv<=50)).sum()))
for t0 in [5,10,20]:
    act = conv > t0
    f = conv > 50
    print(f't={t0}: active={act.sum()}, of which fail={f[act].sum()}; median lam active-fail={np.median(F["lam"][act&f,t0-1]):.3f} active-ok={np.median(F["lam"][act&~f,t0-1]):.3f}')
