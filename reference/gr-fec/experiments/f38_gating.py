"""Forensics for P27c: did the hyperedges learn hunger gating, and is the test at ceiling?"""
import sys, warnings, numpy as np
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src'); sys.path.insert(0, 'experiments')
import worm; from worm3 import Worm3, MX, NT
import e33_worm as E
w = Worm3(100, np.random.default_rng(12000), s0=0.01, hyper=True, gates_only=True)
for sess in range(20):
    w.set_state(hunger=w.rng.uniform(0, 1))
    for _ in range(300): w.step(worm.Plate(salt_uniform=0.8), on_food=True, move=False)
    for _ in range(300): w.step(worm.Plate(), on_food=False, move=False)
sl = slice(MX, 2 * MX); W = w.W.mean(0)
print('salt tuned nodes per worm:', w.cnt[:, 1].mean())
print(f'additive salt-node weight (sum) {W[sl].sum():+.3f}, salt intensity additive {W[3*MX+1]:+.3f}')
print(f'hunger-gated salt-node weight (sum) {W[NT+2:][sl].sum():+.3f}, gated salt intensity {W[NT+2+3*MX+1]:+.3f}')
for hv in (0.1, 0.9):
    x = np.linspace(0.46, 1.0, 5)
    print(f'h={hv}: predicted value along the salt gradient 0.46→1.0:', np.round([W[3*MX+1]*xx + W[NT+2+3*MX+1]*xx*hv for xx in x], 3), '(intensity part only)')
