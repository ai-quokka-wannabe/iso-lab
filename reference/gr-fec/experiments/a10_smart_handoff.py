"""Exploratory: smart BP->OSD hand-off at t0=10 using frozen PREREG4 models (hist vs hist+lamc8).
Flooding BP (as in PREREG2-4). Train: seed 100 (3.0 dB). Test: seed 300 (3.0 dB) frames regenerated exactly."""
import sys, numpy as np, time
sys.path.insert(0, 'src'); sys.path.insert(0, 'experiments')
from alist import read_alist; from ldpc import awgn_llr; from osd import generator_from_H, osd
from a05_stopping import feats, model, mat, BASE
t0 = 10
tr = dict(np.load('results/e05_eb3.0_s100_cheap.npz')); te = dict(np.load('results/e05_eb3.0_s300_cheap.npz'))
Xtr, ytr, atr = feats(tr, t0); Xte, yte, ate = feats(te, t0)
HIST = BASE + [k for k in Xtr if k.startswith('h_')]
for X, d, a in [(Xtr, tr, atr), (Xte, te, ate)]:
    X['lamc8'] = np.log(d['lamc8'][a, t0 - 1] + 1e-9)
pb = model().fit(mat(Xtr, HIST), ytr).predict_proba(mat(Xte, HIST))[:, 1]
pc = model().fit(mat(Xtr, HIST + ['lamc8']), ytr).predict_proba(mat(Xte, HIST + ['lamc8']))[:, 1]
# regenerate test LLRs exactly as e05 did (seed 300, chunks of 3000)
H = read_alist('results/CCSDS_64_128.alist'); G = generator_from_H(H)
rng = np.random.default_rng(300); llr = np.concatenate([awgn_llr(3000, 128, 3.0, 0.5, rng) for _ in range(8)])
conv = te['conv']; N = len(conv); act = np.nonzero(ate)[0]
t = time.time(); osd_ok = np.array([not osd(G, llr[i], 2).any() for i in act]); print(f'OSD on {len(act)} active frames: {time.time()-t:.0f}s; OSD success {osd_ok.mean()*100:.1f}%')
bp_ok_later = conv[act] <= 50
base_work = np.minimum(conv, 50).astype(float)
def evaluate(handoff):            # handoff: bool over active frames
    work = base_work.copy(); work[act[handoff]] = t0
    fail_bp_cont = ~handoff & ~bp_ok_later      # continued, BP failed at 50 -> OSD at 50
    ok = np.where(handoff, osd_ok, bp_ok_later | osd_ok)
    errors = (~ok).sum()
    calls = handoff.sum() + fail_bp_cont.sum()
    return errors / N, work.mean(), calls / N
print(f'{"policy":<34}{"FER":>9}{"BP it/frame":>13}{"OSD calls":>11}')
for name, h in [('no early hand-off (t0=50)', np.zeros(len(act), bool)), ('hand off ALL undecided at 10', np.ones(len(act), bool))]:
    f, w, c = evaluate(h); print(f'{name:<34}{f:9.5f}{w:13.2f}{c*100:10.2f}%')
# equal hand-off budgets: hand off the top-q fraction by predicted failure probability
for q in [0.3, 0.5, 0.6, 0.7]:
    k = int(q * len(act))
    for lab, p in [('hist', pb), ('hist+lamc8', pc)]:
        h = np.zeros(len(act), bool); h[np.argsort(-p)[:k]] = True
        f, w, c = evaluate(h); print(f'{"top "+str(int(q*100))+"% by "+lab:<34}{f:9.5f}{w:13.2f}{c*100:10.2f}%')
