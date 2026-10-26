"""Exact (autodiff) curvature of a metric field g(x) in n dimensions, using JAX in float64."""
import jax
import jax.numpy as jnp
jax.config.update('jax_enable_x64', True)


def make_curvature(metric):
    """metric: x (n,) -> g (n, n). Returns function x -> dict(g, Ric, R, G)."""

    def christoffel(x):
        g = metric(x)
        ginv = jnp.linalg.inv(g)
        dg = jax.jacfwd(metric)(x)                    # dg[i, j, k] = d_k g_ij
        # Gamma^l_{ij} = 1/2 g^{lm} (d_i g_jm + d_j g_im - d_m g_ij)
        term = jnp.einsum('jmi->ijm', dg) + jnp.einsum('imj->ijm', dg) - dg   # (i, j, m)
        return 0.5 * jnp.einsum('lm,ijm->lij', ginv, term)

    def curvature(x):
        g = metric(x)
        Gam = christoffel(x)                          # (l, i, j)
        dGam = jax.jacfwd(christoffel)(x)             # (l, i, j, k) = d_k Gamma^l_ij
        # R^r_{s m n} = d_m Gam^r_{n s} - d_n Gam^r_{m s} + Gam^r_{m l} Gam^l_{n s} - Gam^r_{n l} Gam^l_{m s}
        Riem = (jnp.einsum('rnsm->rsmn', dGam) - jnp.einsum('rmsn->rsmn', dGam)
                + jnp.einsum('rml,lns->rsmn', Gam, Gam) - jnp.einsum('rnl,lms->rsmn', Gam, Gam))
        Ric = jnp.einsum('rsrn->sn', Riem)
        R = jnp.einsum('sn,sn->', jnp.linalg.inv(g), Ric)
        G = Ric - 0.5 * R * g
        return {'g': g, 'Ric': Ric, 'R': R, 'G': G}

    return jax.jit(curvature)


if __name__ == '__main__':
    # validation: unit 3-sphere in stereographic coordinates, g = 4/(1+|x|^2)^2 I
    # Ric = 2 g, R = 6, G = Ric - 3 g = -g
    import numpy as np
    curv = make_curvature(lambda x: 4.0 / (1 + x @ x) ** 2 * jnp.eye(3))
    out = curv(jnp.array([0.3, -0.2, 0.5]))
    print('R =', float(out['R']), '(expect 6)')
    print('max |G + g| =', float(jnp.abs(out['G'] + out['g']).max()), '(expect 0)')
    # 2D sphere: Einstein tensor must vanish identically
    c2 = make_curvature(lambda x: 4.0 / (1 + x @ x) ** 2 * jnp.eye(2))
    o2 = c2(jnp.array([0.4, 0.1])); print('2D: R =', float(o2['R']), '(expect 2), max|G| =', float(jnp.abs(o2['G']).max()), '(expect 0)')
