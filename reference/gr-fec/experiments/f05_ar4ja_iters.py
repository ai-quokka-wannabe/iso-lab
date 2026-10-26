import sys, numpy as np
sys.path.insert(0, 'src')
from qc_layered import read_qc, QCLayeredBP
S, Z, p = read_qc('results/AR4JA_4096_8192.qc'); dec = QCLayeredBP(S, Z, p)
mode = sys.argv[1]; rng = np.random.default_rng(2000); sig2 = 1 / (2 * 0.5 * 10 ** (0.9 / 10)); C = []
for b in range(3):
    llr = 2 * (1 + np.sqrt(sig2) * rng.standard_normal((500, 8192))) / sig2
    C.append(dec.decode(llr, iters=150, rng=None if mode == 'fixed' else np.random.default_rng(77 + b))[0])
C = np.concatenate(C)
ok50 = C <= 50
print(f'{mode:<6}: FER@50 {np.mean(C>50):.4f}  FER@100 {np.mean(C>100):.4f}  FER@150 {np.mean(C>150):.4f}  '
      f'mean iterations of frames converging by 50: {C[ok50].mean():.1f} (median {np.median(C[ok50]):.0f})', flush=True)
