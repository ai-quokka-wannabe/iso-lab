"""Artefact test of the 'Einstein equation' G ~ kappa T in Hamming(7,4) code space, for many well shapes.

Original construction (Chat 1): E(x) = -sum_c f(|x-c|^2), metric g = Hess E, stress-energy
T_ij = sum_c P(c|x) (c_i-x_i)(c_j-x_j)/sigma^2 with Gaussian posterior, sigma^2 = 1/(2 beta).
Note: T does NOT depend on the well shape f; G does.  Correlation = Pearson over the 28 upper-triangular entries.
Controls at the same points: corr(G, a a^T) with a the pair axis (pure symmetry), corr(G, g), corr(T, a a^T),
and corr(G, T_mismatched) with T taken from another midpoint.
"""
import sys, itertools, json, numpy as np
sys.path.insert(0, 'src')
import jax, jax.numpy as jnp
from riemann import make_curvature
jax.config.update('jax_enable_x64', True)

H7 = np.array([[0, 0, 0, 1, 1, 1, 1], [0, 1, 1, 0, 0, 1, 1], [1, 0, 1, 0, 1, 0, 1]])
C = np.array([b for b in itertools.product([0, 1], repeat=7) if np.all((H7 @ np.array(b)) % 2 == 0)], float)
Cj = jnp.array(C)
u_dip = jnp.ones(7) / jnp.sqrt(7.0)


def profiles(beta):
    e = lambda s: jnp.exp(-beta * s)
    R2 = 1.2 ** 2
    return {
        'gaussian':          lambda s, d: e(s),
        'lorentzian':        lambda s, d: 1.0 / (1.0 + beta * s),
        'exponential':       lambda s, d: jnp.exp(1.0 - jnp.sqrt(1.0 + beta * s)),
        'bump(compact)':     lambda s, d: jnp.where(s < R2, jnp.exp(1.0 - 1.0 / (1.0 - jnp.minimum(s, R2 * 0.999999) / R2)), 0.0),
        "delta''-like (LoG)": lambda s, d: (1 - 2 * beta * s / 7) * e(s),
        "delta''''-like":    lambda s, d: (1 - 4 * beta * s / 7 + 4 * beta ** 2 * s ** 2 / 63) * e(s),
        "delta'-like dipole": lambda s, d: (d @ u_dip) * jnp.sqrt(2 * beta) * jnp.exp(0.5) * e(s),
    }


def energy_fn(f):
    def E(x):
        D = x[None, :] - Cj
        s = jnp.sum(D ** 2, axis=1)
        return -jnp.sum(jax.vmap(f)(s, D))
    return E


def T_tensor(x, beta):
    s2 = 1.0 / (2 * beta)
    D = C - x[None, :]
    lw = -np.sum(D ** 2, 1) / (2 * s2); w = np.exp(lw - lw.max()); w /= w.sum()
    return np.einsum('c,ci,cj->ij', w, D, D) / s2


def corr_ut(A, B):
    iu = np.triu_indices(7)
    a, b = np.asarray(A)[iu], np.asarray(B)[iu]
    if a.std() < 1e-300 or b.std() < 1e-300:
        return np.nan
    return np.corrcoef(a, b)[0, 1]


pairs = [(i, j) for i, j in itertools.combinations(range(16), 2) if np.sum(C[i] != C[j]) == 3]
rng = np.random.default_rng(0)
rand_pts = rng.uniform(0, 1, (40, 7))
results = {}
for beta in [1.0, 2.0, 4.0, 16.0, 64.0]:
    for name, f in profiles(beta).items():
        if beta in (16.0, 64.0) and name != 'gaussian':
            continue                                   # narrow-width sweep only for the delta limit
        E = energy_fn(f)
        metric = jax.hessian(E)
        curv = make_curvature(metric)
        rows = []
        for (i, j) in pairs:
            x = (C[i] + C[j]) / 2; a = C[i] - C[j]
            out = curv(jnp.array(x)); G = np.array(out['G']); g = np.array(out['g'])
            ev = np.linalg.eigvalsh(g)
            T = T_tensor(x, beta)
            k = pairs[(pairs.index((i, j)) + 7) % len(pairs)]; Tm = T_tensor((C[k[0]] + C[k[1]]) / 2, beta)
            rows.append(dict(cGT=corr_ut(G, T), cGaa=corr_ut(G, np.outer(a, a)), cGg=corr_ut(G, g),
                             cTaa=corr_ut(T, np.outer(a, a)), cGTm=corr_ut(G, Tm),
                             neg_eig=int((ev < 0).sum()), cond=float(np.abs(ev).max() / max(np.abs(ev).min(), 1e-300)),
                             finite=bool(np.all(np.isfinite(G)))))
        rr = []
        for x in rand_pts:
            out = curv(jnp.array(x)); G = np.array(out['G'])
            rr.append(corr_ut(G, T_tensor(x, beta)))
        key = f'{name} | beta={beta:g}'
        R = {k: np.array([r[k] for r in rows], float) for k in rows[0]}
        results[key] = dict(
            mid_cGT_median=float(np.nanmedian(R['cGT'])), mid_cGT_maxabs=float(np.nanmax(np.abs(R['cGT']))) if np.isfinite(R['cGT']).any() else np.nan,
            mid_cGaa_median=float(np.nanmedian(R['cGaa'])), mid_cGg_median=float(np.nanmedian(R['cGg'])),
            mid_cTaa_median=float(np.nanmedian(R['cTaa'])), mid_cGTm_median=float(np.nanmedian(R['cGTm'])),
            rand_cGT_median=float(np.nanmedian(rr)), neg_eig_median=float(np.median(R['neg_eig'])),
            log10_cond_median=float(np.log10(np.median(R['cond']))), finite_frac=float(np.mean(R['finite'])))
        r = results[key]
        print(f"{key:<36} G~T mid: med {r['mid_cGT_median']:+.3f} max|.| {r['mid_cGT_maxabs']:.3f} | "
              f"G~aa {r['mid_cGaa_median']:+.3f} | T~aa {r['mid_cTaa_median']:+.3f} | G~g {r['mid_cGg_median']:+.3f} | "
              f"G~T(wrong pt) {r['mid_cGTm_median']:+.3f} | G~T random pts {r['rand_cGT_median']:+.3f} | "
              f"neg eig {r['neg_eig_median']:.0f}, log10 cond {r['log10_cond_median']:.1f}", flush=True)
json.dump(results, open('results/e06_well_shapes.json', 'w'), indent=1)
