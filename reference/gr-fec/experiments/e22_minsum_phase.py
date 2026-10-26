"""PREREG16: phase picture vs normalisation factor beta in flooding min-sum (CCSDS (128,64), 3.0 dB)."""
import sys, time, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from track_minsum import TrackMinSum
betas = [float(b) for b in sys.argv[1].split(',')]
EB = float(sys.argv[2]) if len(sys.argv) > 2 else 3.0; SEED = int(sys.argv[3]) if len(sys.argv) > 3 else 900; NCH = int(sys.argv[4]) if len(sys.argv) > 4 else 4
H = read_alist('results/CCSDS_64_128.alist')
for b in betas:
    dec = TrackMinSum(H, b); rng = np.random.default_rng(SEED); acc = {k: [] for k in ['conv', 'ftle', 'cls']}; t = time.time()
    for c in range(NCH):
        llr = awgn_llr(3000, 128, EB, 0.5, rng)
        F, conv, hc = dec.run(llr, T=50, probes=1, seed=SEED * 100 + c)
        ftle = np.log(np.maximum(F['lam'][:, 30:50], 1e-30)).mean(1)
        dn = F['dnorm'][:, 40:].max(1); fl = F['nflip'][:, 40:].sum(1)
        acc['conv'].append(conv); acc['ftle'].append(ftle); acc['cls'].append(np.where(fl > 0, 2, np.where(dn < 1e-3, 0, 1)))
    o = {k: np.concatenate(v) for k, v in acc.items()}; np.savez_compressed(f'results/e22_eb{EB}_beta{b}.npz', **o)
    f = o['conv'] > 50; cl = np.bincount(o['cls'][f], minlength=3) / max(f.sum(), 1)
    print(f'beta={b}: FER={f.mean():.4f} ({f.sum()})  FTLE(fail) median={np.median(o["ftle"][f]):+.3f}  fixed/drift/osc = {cl[0]*100:.1f}/{cl[1]*100:.1f}/{cl[2]*100:.1f}%  {time.time()-t:.0f}s', flush=True)
