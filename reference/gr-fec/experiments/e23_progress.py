"""PREREG17: progress ratio of failing frames: AR4JA (long) vs CCSDS (128,64) (short), layered sum-product, fixed order."""
import sys, numpy as np
sys.path.insert(0, 'src')
which = sys.argv[1]
if which == 'ar4ja':
    from qc_layered import read_qc, QCLayeredBP
    S, Z, p = read_qc('results/AR4JA_4096_8192.qc'); dec = QCLayeredBP(S, Z, p)
    rng = np.random.default_rng(3000); sig2 = 1 / (2 * 0.5 * 10 ** (0.9 / 10)); rows = []
    for b in range(3):
        llr = 2 * (1 + np.sqrt(sig2) * rng.standard_normal((500, 8192))) / sig2
        B = 500; L = np.zeros((B, dec.n)); L[:, dec.tx] = llr; tot = L.copy(); c2v = [np.zeros((B, Z, V.shape[1])) for V in dec.layers]; u = {}
        for it in range(50):
            for l in range(dec.mb):
                V = dec.layers[l]; v2c = tot[:, V] - c2v[l]
                t = np.tanh(np.clip(v2c, -40, 40) / 2); t = np.where(np.abs(t) < 1e-15, 1e-15, t)
                new = 2 * np.arctanh(np.clip(np.prod(t, axis=2, keepdims=True) / t, -1 + 1e-12, 1 - 1e-12)); upd = v2c + new
                for k in range(V.shape[1]): tot[:, V[:, k]] = upd[:, :, k]
                c2v[l] = new
            if it + 1 in (25, 50):
                hard = (tot < 0).astype(np.uint8); u[it + 1] = sum((hard[:, V].sum(2) % 2).sum(1) for V in dec.layers)
        rows.append(np.stack([u[25], u[50]], 1))
    U = np.concatenate(rows)
else:
    from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP
    H = read_alist('results/CCSDS_64_128.alist'); lb = LayeredBP(H)
    llr = np.concatenate([awgn_llr(6000, 128, 3.0, 0.5, np.random.default_rng(3001 + i)) for i in range(4)])
    B = len(llr); tot = llr.copy(); c2v = [np.zeros((B, len(r))) for r in lb.rows]; u = {}
    for it in range(50):
        for cc in range(64):
            r = lb.rows[cc]; v = tot[:, r] - c2v[cc]; t = np.tanh(np.clip(v, -40, 40) / 2); t = np.where(np.abs(t) < 1e-15, 1e-15, t)
            new = 2 * np.arctanh(np.clip(np.prod(t, 1, keepdims=True) / t, -1 + 1e-12, 1 - 1e-12)); tot[:, r] = v + new; c2v[cc] = new
        if it + 1 in (25, 50):
            u[it + 1] = (np.asarray((lb.Hs @ (tot < 0).T.astype(float)).T) % 2).sum(1)
    U = np.stack([u[25], u[50]], 1)
fail = U[:, 1] > 0; rho = U[fail, 1] / np.maximum(U[fail, 0], 1)
np.save(f'results/e23_{which}_rho.npy', rho)
print(f'{which}: frames {len(U)}, failures at 50 it {fail.sum()}; progress ratio rho = unsat(50)/unsat(25): median {np.median(rho):.3f}; '
      f'fraction rho<0.5 (still progressing) {np.mean(rho<0.5):.3f}; fraction rho>=1 (stuck/worse) {np.mean(rho>=1):.3f}', flush=True)
