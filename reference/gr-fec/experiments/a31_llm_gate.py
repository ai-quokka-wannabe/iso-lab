"""PREREG20 analysis."""
import json, numpy as np
from sklearn.metrics import roc_auc_score
R = json.load(open('results/e31_rows.json'))
ok = np.array([r['ok'] for r in R]); kind = np.array([r['kind'] for r in R])
lp = np.array([r['lp'] for r in R]); mlp = np.array([r['meanlp'] for r in R]); gap = np.array([r['gap'] for r in R])
agree = np.array([r['agree'] for r in R]); action = -lp; r_ = gap / np.maximum(action, 1e-3)
S = {'meanlp': mlp, 'lp': lp, 'gap': gap, 'action(-)': -action, 'r': r_, 'agree': agree}
print(f'{len(R)} questions; accuracy {ok.mean():.3f} (' + ', '.join(f'{k} {ok[kind==k].mean():.2f}' for k in ('sym','elem','cap')) + ')')
A = {n: roc_auc_score(ok, v) for n, v in S.items()}
print('AUROC (all): ' + '  '.join(f'{n}={v:.3f}' for n, v in A.items()))
for k in ('sym', 'elem', 'cap'):
    m = kind == k
    if 0 < ok[m].sum() < m.sum(): print(f'  {k:4s}: ' + '  '.join(f'{n}={roc_auc_score(ok[m], v[m]):.3f}' for n, v in S.items()))
print(f"P20a: r {A['r']:.3f} vs meanlp {A['meanlp']:.3f} -> {'PASS' if A['r'] >= A['meanlp'] - .01 else 'FAIL'}")
print(f"P20b: r {A['r']:.3f} vs agree {A['agree']:.3f} -> {'PASS' if A['r'] >= A['agree'] - .01 else 'FAIL'}")
print(f"S20 : gap {A['gap']:.3f} vs meanlp {A['meanlp']:.3f} -> {'PASS' if A['gap'] > A['meanlp'] else 'FAIL'}")
w = ~ok; fz = agree >= .75; ch = agree <= .25
print(f'H-AI5: wrong answers {w.sum()}: frozen (agree>=0.75) {(w&fz).sum()} ({(w&fz).sum()/w.sum()*100:.0f}%), chaotic (agree<=0.25) {(w&ch).sum()} ({(w&ch).sum()/w.sum()*100:.0f}%)')
m = fz
if 0 < ok[m].sum() < m.sum(): print('   within frozen subset (agree>=0.75): ' + '  '.join(f'{n}={roc_auc_score(ok[m], v[m]):.3f}' for n, v in S.items() if n != 'agree'))
# accepted-correct at <=10% wrong accepted
for n, v in S.items():
    t = np.quantile(v[~ok], .90); print(f'   {n:10s}: keeps {(v[ok] > t).mean()*100:5.1f}% of correct answers while letting through 10% of wrong ones')
print('\nexamples of frozen hallucinations:'); [print('  ', r['q'][:60], '->', repr(r['ans']), '(truth', r['truth'] + ')', f"gap={r['gap']:.2f}") for r in R if not r['ok'] and r['agree'] >= .75][:12]
