import sys, time, numpy as np
sys.path.insert(0, 'src')
from qc_layered import read_qc, QCLayeredBP
from scipy.stats import binomtest
eb, seed, N = float(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
S, Z, p = read_qc('results/AR4JA_4096_8192.qc'); dec = QCLayeredBP(S, Z, p)
rng = np.random.default_rng(seed); sig2 = 1 / (2 * 0.5 * 10 ** (eb / 10))
F = {'fixed': [], 'random': []}; t = time.time()
for b in range(N // 500):
    llr = 2 * (1 + np.sqrt(sig2) * rng.standard_normal((500, 8192))) / sig2
    c, h = dec.decode(llr, iters=50); F['fixed'].append(c > 50)
    c, h = dec.decode(llr, iters=50, rng=np.random.default_rng(seed * 1000 + b)); F['random'].append(c > 50)
F = {k: np.concatenate(v) for k, v in F.items()}
r = int((F['fixed'] & ~F['random']).sum()); l = int((~F['fixed'] & F['random']).sum())
pv = binomtest(r, r + l, 0.5, alternative='greater').pvalue if r + l else 1.0
print(f'AR4JA (8192,4096) Eb/N0={eb} seed={seed} N={N}: fixed-order FER={F["fixed"].mean():.4f}  random-layer-order FER={F["random"].mean():.4f}  '
      f'rescued={r} lost={l}  McNemar one-sided p={pv:.2e}  ({time.time()-t:.0f}s)', flush=True)
