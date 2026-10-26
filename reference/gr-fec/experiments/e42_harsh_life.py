"""PREREG31: a worm's life in Hardland, where instinct is not enough."""
import sys, copy, json, warnings, numpy as np
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src'); sys.path.insert(0, 'experiments')
from embodied import Worm, World
from lifeworld import HardWorld
import importlib.util
src = open('experiments/e40_life.py').read().split('def live(')[0]
ns = {}; exec(src, ns); exam, value_curve = ns['exam'], ns['value_curve']
N, HOURS = 100, 72
def live(cond, seed, rw=None, extra=None, world_cls=HardWorld, hours=None):
    kw = dict(s0=0.01, integ=10, separate=True, **(extra or {}))
    HOURS_ = hours or HOURS
    if cond == 'F': kw['eta'] = 0.0
    if cond == 'R': kw['random_walk_p'] = rw
    w = Worm(N, seed, **kw); w.body.R = 50.0; world = world_cls(N); log = []; exams = {}
    TS = np.linspace(15, 25, 11); OS = np.linspace(0, 1, 6)
    for hr in range(0, HOURS_, 4):
        if hr in (0, 24, 48) and cond != 'R': exams[hr] = exam(w)
        onf = odl = pir = 0.0
        for t in range(14400):
            y, b = w.body.sense(world); p, _ = w.brain.step(y); w.body.act(p); w.body.metabolise(b); world.consume(w.body.intake)
            onf += (b > 0).mean(); odl += ((world.which >= 0) & (world.which < 4)).mean(); pir += p.mean()
        row = dict(hour=hr + 4, energy=float(w.body.E.mean()), on_food=onf / 14400, on_odourless=odl / 14400,
                   turns_per_min=pir / 14400 * 60, nodes=w.brain.cnt.mean(0).round(2).tolist(), x=float(w.body.xy[:, 0].mean()),
                   cool_share=float((w.body.xy[:, 0] < -25).mean()))
        if cond == 'L': row['temp_value'] = value_curve(w.brain, 3, TS).round(3).tolist(); row['odour_value'] = value_curve(w.brain, 0, OS).round(3).tolist()
        log.append(row)
        print(f"{cond} h{hr+4:2d}: energy {row['energy']:.2f} on-food {row['on_food']:.2f} (odourless {row['on_odourless']:.2f}) "
              f"cool-band share {row['cool_share']:.2f} mean x {row['x']:+.1f} turns {row['turns_per_min']:.1f}/min", flush=True)
    if cond != 'R': exams[HOURS_] = exam(w)
    return log, exams
import os
if os.environ.get('V6') == '1':
    from lifeworld import LifeWorld
    res = {}
    res['T'] = live('T', 18000, extra=dict(trust_k=2.0)); res['C'] = live('C', 18001, extra=dict(curiosity=1.0)); res['TC'] = live('TC', 18002, extra=dict(trust_k=2.0, curiosity=1.0))
    res['TC_rich'] = live('TC', 18003, extra=dict(trust_k=2.0, curiosity=1.0), world_cls=LifeWorld, hours=48); res['F_rich'] = live('F', 18004, world_cls=LifeWorld, hours=48)
    json.dump({k: {'log': v[0], 'exams': v[1]} for k, v in res.items()}, open('results/e43_v6.json', 'w'), indent=1)
    E = {k: np.mean([r['energy'] for r in v[0] if r['hour'] > 48]) for k, v in res.items() if 'rich' not in k}
    O = {k: np.mean([r['on_odourless'] for r in v[0] if r['hour'] > 48]) for k, v in res.items() if 'rich' not in k}
    Er = {k: np.mean([r['energy'] for r in v[0] if r['hour'] > 24]) for k, v in res.items() if 'rich' in k}
    print('\nenergy h48-72:', {k: round(v, 3) for k, v in E.items()}, ' odourless share:', {k: round(v, 3) for k, v in O.items()})
    print(f"P32a: TC {E['TC']:.3f} >= 0.454 and >= 0.419 -> {'PASS' if E['TC'] >= .454 else 'FAIL'}")
    print(f"P32b: TC odourless {O['TC']:.3f} >= 0.054 -> {'PASS' if O['TC'] >= .054 else 'FAIL'}")
    print(f"P32c: T {E['T']:.3f}, C {E['C']:.3f} between L5 0.404 and TC {E['TC']:.3f}: T {0.404 <= E['T'] <= E['TC']}, C {0.404 <= E['C'] <= E['TC']}")
    print(f"P32d (rich world, h24-48): TC {Er['TC_rich']:.3f} vs F {Er['F_rich']:.3f} -> {'PASS' if Er['TC_rich'] >= Er['F_rich'] - .02 else 'FAIL'}")
    print('exams:', json.dumps({k: v[1] for k, v in res.items()}))
    sys.exit()
res = {}
res['L'] = live('L', 17000); rw = np.mean([r['turns_per_min'] for r in res['L'][0]]) / 60
res['F'] = live('F', 17001); res['R'] = live('R', 17002, rw=rw)
json.dump({k: {'log': v[0], 'exams': v[1]} for k, v in res.items()}, open('results/e42_harsh.json', 'w'), indent=1)
E = {k: np.mean([r['energy'] for r in v[0] if r['hour'] > 48]) for k, v in res.items()}
O = {k: np.mean([r['on_odourless'] for r in v[0] if r['hour'] > 48]) for k, v in res.items()}
eL, eF = res['L'][1], res['F'][1]
print(f"\nP31a energy h48-72: L {E['L']:.3f} F {E['F']:.3f} R {E['R']:.3f} -> {'PASS' if E['L'] >= E['F'] + .05 and E['F'] > E['R'] else 'FAIL'}")
print(f"P31b thermal: L h72 {eL[72]['thermal']:.2f} vs h0 {eL[0]['thermal']:.2f} °C -> {'PASS' if eL[0]['thermal'] - eL[72]['thermal'] >= 1.5 else 'FAIL'}")
print(f"P31c odourless food share h48-72: L {O['L']:.3f} F {O['F']:.3f} R {O['R']:.3f} -> {'PASS' if O['L'] >= 2 * O['F'] else 'FAIL'}")
print(f"P31d odour exam h72: L {eL[72]['odour']:+.2f} vs F {eF[72]['odour']:+.2f} -> {'PASS' if eL[72]['odour'] <= eF[72]['odour'] - .1 else 'FAIL'}")
print('L exams:', json.dumps(eL)); print('F exams:', json.dumps(eF))
