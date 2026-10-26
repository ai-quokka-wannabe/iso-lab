import sys, numpy as np
sys.path.insert(0, 'src')
from qc_layered import read_qc, QCLayeredBP
from alist import read_alist; from ldpc import awgn_llr; from bp_track import TrackBP; from sched import decode_sched
print('== F9  AR4JA: why does random layer order hurt? (information delivery to punctured nodes) ==')
S, Z, p = read_qc('results/AR4JA_4096_8192.qc'); dec = QCLayeredBP(S, Z, p)
sig2 = 1 / (2 * 0.5 * 10 ** (0.9 / 10)); rng = np.random.default_rng(31)
llr = 2 * (1 + np.sqrt(sig2) * rng.standard_normal((300, 8192))) / sig2
punct = ~dec.tx
for name, rr in [('fixed', None), ('random', np.random.default_rng(5))]:
    # replicate decode for a few iterations, measuring mean |posterior| on punctured nodes and fraction of punctured nodes still wrong
    B = len(llr); L = np.zeros((B, dec.n)); L[:, dec.tx] = llr; tot = L.copy(); c2v = [np.zeros((B, Z, V.shape[1])) for V in dec.layers]; prof = []
    for it in range(6):
        order = rr.permutation(dec.mb) if rr is not None else range(dec.mb)
        for l in order:
            V = dec.layers[l]; v2c = tot[:, V] - c2v[l]
            t = np.tanh(np.clip(v2c, -40, 40) / 2); t = np.where(np.abs(t) < 1e-15, 1e-15, t)
            new = 2 * np.arctanh(np.clip(np.prod(t, axis=2, keepdims=True) / t, -1 + 1e-12, 1 - 1e-12)); upd = v2c + new
            for k in range(V.shape[1]): tot[:, V[:, k]] = upd[:, :, k]
            c2v[l] = new
        prof.append(f'it{it+1}: |post| punct {np.abs(tot[:, punct]).mean():5.2f}, wrong punct {np.mean(tot[:, punct] < 0):.3f}, wrong tx {np.mean(tot[:, dec.tx] < 0):.4f}')
    print(f'  {name:<6}: ' + ' | '.join(prof[i] for i in (0, 1, 2, 5)))
# which layers touch punctured columns?
pc = np.nonzero(p == 0)[0]
print('  layers (block rows) touching punctured block-columns:', [r for r in range(S.shape[0]) if (S[r, pc] >= 0).any()],
      ' natural order processes them at positions:', [r for r in range(S.shape[0]) if (S[r, pc] >= 0).any()])
print('== F12  Why do tempering cycles hurt? FER vs cycle period (flooding BP, 3.0 dB) ==')
H = read_alist('results/CCSDS_64_128.alist'); tb = TrackBP(H)
llr2 = np.concatenate([awgn_llr(6000, 128, 3.0, 0.5, np.random.default_rng(3100 + i)) for i in range(2)])
t = np.arange(50)
for per in [4, 10, 25, 50, 100, 10**9]:
    al = np.exp(0.4 * np.sin(2 * np.pi * t / per)) if per < 10**9 else np.ones(50)
    conv = decode_sched(tb, llr2, al, 0.0); print(f'  period {per if per<10**9 else "inf (constant)":>15}: FER {np.mean(conv > 50):.4f}')
for amp in [0.05, 0.1, 0.2, 0.4]:
    conv = decode_sched(tb, llr2, np.exp(amp * np.sin(2 * np.pi * t / 10)), 0.0); print(f'  period 10, amplitude {amp}: FER {np.mean(conv > 50):.4f}')
