"""Forensics: test the explanations of failed ideas with targeted measurements (CCSDS (128,64), 3.0 dB)."""
import sys, numpy as np, itertools
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP; from bp_track import TrackBP
from complex_bp import ComplexBP
H = read_alist('results/CCSDS_64_128.alist'); lb = LayeredBP(H); tb = TrackBP(H)
llr = np.concatenate([awgn_llr(6000, 128, 3.0, 0.5, np.random.default_rng(2900 + i)) for i in range(2)])
chan_err = (llr < 0).sum(1)                         # channel hard-decision errors (all-zero sent)
c, h, fl, tot = lb.decode(llr, None, iters=50, reshuffle_rng=np.random.default_rng(1), track_flips=50)
fail = c > 50
print('== F1  Why are failures not localised (curvature could not predict which bits fail)? ==')
wrong = h[fail]; blocks = np.array([[wrong[i, b*16:(b+1)*16].any() for b in range(8)] for i in range(len(wrong))])
print(f'  failing frames: median {np.median(wrong.sum(1)):.0f} wrong bits, touching median {np.median(blocks.sum(1)):.0f} of 8 circulant blocks')
print(f'  channel errors: failing frames median {np.median(chan_err[fail]):.0f} vs successful {np.median(chan_err[~fail]):.0f} (of 128 bits)')
print('== F2  Why does temperature diversity fail (same frames fail at every temperature)? ==')
for a in [0.7, 1.4]:
    ca, ha = lb.decode(a * llr, None, iters=50, reshuffle_rng=np.random.default_rng(1)); fa = ca > 50
    print(f'  alpha={a}: failures shared with alpha=1: {(fa & fail).sum()}/{fail.sum()};  channel errors in shared failures median {np.median(chan_err[fa & fail]):.0f}')
q = np.quantile(chan_err, [0.5, 0.9, 0.99]); print(f'  channel-error quantiles over all frames (50/90/99%): {q}; FER by channel-error count:')
for lo, hi in [(0, 8), (9, 11), (12, 14), (15, 17), (18, 40)]:
    m = (chan_err >= lo) & (chan_err <= hi); print(f'     {lo:2d}-{hi:2d} errors: {m.sum():5d} frames, FER {fail[m].mean() if m.any() else 0:.3f}')
print('== F3  Why does heating oscillating frames hurt (is oscillation productive search)? ==')
osc15 = fl[:, 10:20].sum(1) > 0                  # still flipping during iterations 11-20
late_ok = (c > 20) & (c <= 50)
print(f'  frames still flipping in it. 11-20: {osc15.sum()}; of those, converge later (it. 21-50): {(osc15 & late_ok).sum()} '
      f'({(osc15 & late_ok).sum()/max(osc15.sum(),1)*100:.0f}%); never converge: {(osc15 & fail).sum()}')
print('== F4  Why does time-crystal averaging rescue nothing? ==')
idx = np.nonzero(fail)[0]
avg_hard = (np.sign(tot[idx]) < 0)  # placeholder (final state); recompute average below
# rerun failing frames tracking the running average of posteriors over it. 31-50
B = len(idx); T2 = llr[idx].copy(); c2v = [np.zeros((B, len(r))) for r in lb.rows]; acc = np.zeros_like(T2); rng = np.random.default_rng(1)
# NB: different random stream from the run above; failing set re-identified within this replay
for it in range(50):
    for cc in rng.permutation(64):
        r = lb.rows[cc]; v = T2[:, r] - c2v[cc]; t = np.tanh(np.clip(v, -40, 40) / 2); t = np.where(np.abs(t) < 1e-15, 1e-15, t)
        new = 2 * np.arctanh(np.clip(np.prod(t, 1, keepdims=True) / t, -1 + 1e-12, 1 - 1e-12)); T2[:, r] = v + new; c2v[cc] = new
    if it >= 30: acc += T2
fin_hard = T2 < 0; avg_h = acc < 0
syn = lambda x: (np.asarray((lb.Hs @ x.T.astype(float)).T) % 2).sum(1)
still = syn(fin_hard) > 0
print(f'  replayed failing frames still unconverged: {still.sum()}; wrong bits: final state median {np.median(fin_hard[still].sum(1)):.0f}, '
      f'cycle-average median {np.median(avg_h[still].sum(1)):.0f}; unsatisfied checks: final {np.median(syn(fin_hard[still])):.0f}, average {np.median(syn(avg_h[still])):.0f}')
print('== F5  Why do complex phases hurt? (message magnitude / information destroyed by interference) ==')
cb = ComplexBP(H); sub = llr[:2000]
for th in [0.0, 0.05, 0.2]:
    L = np.exp(1j * th) * sub.astype(complex); v2c = L[:, cb.ev].copy()
    for it in range(3):
        z = np.clip(v2c.real, -40, 40) + 1j * v2c.imag; tt = np.tanh(z / 2).reshape(len(sub), 64, 8)
        pre = np.ones_like(tt); suf = np.ones_like(tt)
        for k in range(1, 8):
            pre[:, :, k] = pre[:, :, k-1] * tt[:, :, k-1]; suf[:, :, 7-k] = suf[:, :, 8-k] * tt[:, :, 8-k]
        ex = (pre * suf).reshape(len(sub), -1); c2v_ = 2 * np.arctanh(np.where(np.abs(ex) > 1 - 1e-12, ex / np.abs(ex) * (1 - 1e-12), ex))
        totc = L + np.asarray((cb.Sv.T @ c2v_.T).T); v2c = totc[:, cb.ev] - c2v_
    print(f'  theta={th}: after 3 it. mean Re(c2v)={np.abs(c2v_.real).mean():.3f}, mean |Im(c2v)|={np.abs(c2v_.imag).mean():.3f}, '
          f'fraction of check messages with WRONG sign of Re: {(c2v_.real < 0).mean():.4f}')
print('== F6  Why did redundant "multipole" checks wreck BP? ==')
Hu = H.astype(int); pairs = [(i, j) for i, j in itertools.combinations(range(64), 2) if (Hu[i] & Hu[j]).sum() == 1]
rngp = np.random.default_rng(0); R = np.array([Hu[pairs[s][0]] ^ Hu[pairs[s][1]] for s in rngp.choice(len(pairs), 64, replace=False)])
for name, M in [('original', Hu), ('+64 redundant', np.vstack([Hu, R]))]:
    Gm = M @ M.T; np.fill_diagonal(Gm, 0); four = int((Gm * (Gm - 1) // 2).sum() // 2)
    print(f'  {name:<14}: check pairs sharing >=2 bits {int((Gm >= 2).sum() // 2)}, 4-cycles {four}, max bits shared by two checks {Gm.max()}')
