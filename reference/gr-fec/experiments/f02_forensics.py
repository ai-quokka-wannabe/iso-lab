import sys, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist; from ldpc import awgn_llr; from layered import LayeredBP; from bp_track import TrackBP
from complex_bp import ComplexBP; from repair import final_state
from osd import generator_from_H, osd_order_cands
H = read_alist('results/CCSDS_64_128.alist'); lb = LayeredBP(H); tb = TrackBP(H)
llr = np.concatenate([awgn_llr(6000, 128, 3.0, 0.5, np.random.default_rng(2900 + i)) for i in range(2)])
c, h = lb.decode(llr, None, iters=50, reshuffle_rng=np.random.default_rng(1)); fail = c > 50
print('== F3 (fixed)  Is oscillation productive search? ==')
for t0 in [10, 20, 30]:
    a = c > t0; print(f'  unresolved at it.{t0}: {a.sum()}; converge later by it.50: {(a & ~fail).sum()} ({(a & ~fail).sum()/a.sum()*100:.0f}%)')
print('== F5 (extended)  Complex phase: how does the imaginary part evolve and how close does it get to tanh poles (|Im z/2| -> pi/2)? ==')
cb = ComplexBP(H); sub = llr[:3000]
for th in [0.0, 0.05, 0.2]:
    L = np.exp(1j * th) * sub.astype(complex); v2c = L[:, cb.ev].copy(); out = []
    for it in range(30):
        z = np.clip(v2c.real, -40, 40) + 1j * v2c.imag; tt = np.tanh(z / 2).reshape(len(sub), 64, 8)
        pre = np.ones_like(tt); suf = np.ones_like(tt)
        for k in range(1, 8):
            pre[:, :, k] = pre[:, :, k-1] * tt[:, :, k-1]; suf[:, :, 7-k] = suf[:, :, 8-k] * tt[:, :, 8-k]
        ex = (pre * suf).reshape(len(sub), -1); ex = np.where(np.abs(ex) > 1 - 1e-12, ex / np.abs(ex) * (1 - 1e-12), ex)
        c2v = 2 * np.arctanh(ex); tot = L + np.asarray((cb.Sv.T @ c2v.T).T); v2c = tot[:, cb.ev] - c2v
        if it in (2, 9, 29):
            wrong_bits = (tot.real < 0).sum(1)
            out.append(f'it{it+1}: median|Im tot|={np.median(np.abs(tot.imag)):.2f}, frac |Im v2c/2|>pi/4: {(np.abs(v2c.imag/2) > np.pi/4).mean():.3f}, frames with >0 wrong bits {np.mean(wrong_bits>0):.3f}')
    print(f'  theta={th}: ' + ' | '.join(out))
print('== F7  Lyapunov vector: where does it live? (flooding BP, failing frames, 50 it.) ==')
_, d, _ = tb.decode(llr, iters=50) if False else (None, None, None)
from ldpc import BP
hb, db, ub = BP(H).decode(llr, iters=50); fl_ = ~db
S = final_state(tb, llr[fl_], T=50, seed=3)
ly = S['lyap']; wrongb = S['tot'] < 0
pr = (ly.sum(1) ** 2) / (ly ** 2).sum(1)                     # participation ratio = effective number of bits
top = np.argsort(-ly, 1)[:, :8]
deg = H.sum(0)
print(f'  effective number of bits the Lyapunov sensitivity spans (participation ratio): median {np.median(pr):.1f} of 128')
print(f'  top-8 Lyapunov bits: fraction wrong {np.take_along_axis(wrongb, top, 1).mean():.3f} (base rate {wrongb.mean():.3f}); '
      f'fraction degree-5 {np.mean(deg[top] == 5):.3f} (base 0.5); mean |posterior| of top-8 {np.abs(np.take_along_axis(S["tot"], top, 1)).mean():.1f} vs all {np.abs(S["tot"]).mean():.1f}')
print('== F10  Why OSD-2 and repulsion fail on residual frames: errors inside the most reliable basis (MRB) ==')
G = generator_from_H(H)
c2, h2, fl2, tot2 = lb.decode(llr, None, iters=20, reshuffle_rng=np.random.default_rng(5), track_flips=1)
_, _, _, tot5 = lb.decode(llr, None, iters=5, reshuffle_rng=np.random.default_rng(5), track_flips=1)
act = np.nonzero(c2 > 20)[0]
def mrb_errors(order_vec, sign_vec, L):
    # number of hard-decision errors (vs truth = all zero) among the first k=64 independent most-reliable positions
    perm = np.argsort(-order_vec); Gp = G[:, perm].copy(); cols = []; r = 0
    for cc in range(128):
        rows = np.nonzero(Gp[r:, cc])[0]
        if len(rows) == 0: continue
        p = r + rows[0]; Gp[[r, p]] = Gp[[p, r]]; oth = np.nonzero(Gp[:, cc])[0]; oth = oth[oth != r]; Gp[oth] ^= Gp[r]; cols.append(cc); r += 1
        if r == 64: break
    return int((sign_vec[perm][cols] < 0).sum())
rows = []
for i in act:
    L = llr[i]; P = tot2[i]; P5 = tot5[i]
    e_bl = mrb_errors(np.abs(L) + .25 * np.abs(P), L + .25 * P, L); e5 = mrb_errors(np.abs(P5), P5, L); e_ch = mrb_errors(np.abs(L), L, L)
    rows.append((e_ch, e_bl, e5))
R = np.array(rows)
for j, nm in enumerate(['channel', 'blend', 'posterior it5']):
    print(f'  ordering {nm:<14}: MRB errors over {len(act)} BP failures: P(0)={np.mean(R[:,j]==0):.3f} P(1)={np.mean(R[:,j]==1):.3f} '
          f'P(2)={np.mean(R[:,j]==2):.3f} P(>=3)={np.mean(R[:,j]>=3):.4f}  -> OSD-2 can only fix <=2')
both = (R[:, 1] >= 3) & (R[:, 2] >= 3)
print(f'  frames with >=3 MRB errors in blend: {int((R[:,1]>=3).sum())}; of those, <=2 in the it5 universe: {int(((R[:,1]>=3)&(R[:,2]<=2)).sum())}; >=3 in BOTH: {int(both.sum())}')
