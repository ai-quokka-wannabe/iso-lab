"""PREREG29: a worm's life in Agarland. Brain = embodied v4b; nothing tuned. Exams on clones only."""
import sys, copy, json, warnings, numpy as np
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
from embodied import Worm, World, MX, NT, WID
from lifeworld import LifeWorld
import os
SEP = os.environ.get('SEP') == '1'; SEED0 = int(os.environ.get('SEED0', '15000')); OUT = os.environ.get('OUT', 'results/e40_life.json')
N, HOURS = 100, 48
def CI(xy): return ((np.linalg.norm(xy - [15, 0], axis=1) < 8).sum() - (np.linalg.norm(xy - [-15, 0], axis=1) < 8).sum()) / len(xy)
def value_curve(br, m, xs, h=0.5):
    """Read the brain's learned value of a sensation from its own weights (no node creation)."""
    exist = np.arange(MX)[None] < br.cnt[:, m][:, None]; out = []
    for x in xs:
        act = np.exp(-(br.proto[:, m] - x) ** 2 / (2 * WID[m] ** 2)) * exist
        v = (br.W[:, m * MX:(m + 1) * MX] * act).sum(1) + h * (br.W[:, NT + m * MX:NT + (m + 1) * MX] * act).sum(1)
        if m < 3: v += x * (br.W[:, NT - 3 + m] + h * br.W[:, 2 * NT - 3 + m])   # learned part only
        out.append(v.mean())
    return np.array(out)
def exam(w):
    r = {}
    for name, world in (('thermal', World(thermal=True)), ('salt', World(salt=True)), ('odour', World(odour=True))):
        c = copy.deepcopy(w); c.body.R = 25.0; c.transfer(); c.run(world, 1200)
        r[name] = float((20 + 5 * c.body.xy[:, 0] / 25).mean()) if name == 'thermal' else float(CI(c.body.xy))
    return r
def live(cond, seed, rw=None):
    kw = dict(s0=0.01, integ=10, separate=SEP)
    if cond == 'F': kw['eta'] = 0.0
    if cond == 'R': kw['random_walk_p'] = rw
    w = Worm(N, seed, **kw); w.body.R = 50.0; world = LifeWorld(N); log = []; exams = {}
    TS = np.linspace(15, 25, 11); SS = np.linspace(0, 1, 6)
    for hr in range(0, HOURS, 2):
        if hr in (0, 6, 24) and cond != 'R': exams[hr] = exam(w)
        onf = 0.0; pir = 0.0
        for t in range(7200):
            y, b = w.body.sense(world); p, _ = w.brain.step(y); w.body.act(p); w.body.metabolise(b); world.consume(w.body.intake)
            onf += (b > 0).mean(); pir += p.mean()
        row = dict(hour=hr + 2, energy=float(w.body.E.mean()), on_food=onf / 7200, turns_per_min=pir / 7200 * 60,
                   nodes=w.brain.cnt.mean(0).round(2).tolist(), x=float(w.body.xy[:, 0].mean()))
        if cond == 'L': row['temp_value'] = value_curve(w.brain, 3, TS).round(3).tolist(); row['salt_value'] = value_curve(w.brain, 1, SS).round(3).tolist()
        log.append(row)
        print(f"{cond} h{hr+2:2d}: energy {row['energy']:.2f} on-food {row['on_food']:.2f} turns {row['turns_per_min']:.1f}/min nodes {row['nodes']} mean x {row['x']:+.1f}", flush=True)
    if cond != 'R': exams[48] = exam(w)
    return log, exams, pir / 7200
res = {}
res['L'] = live('L', SEED0); rw = np.mean([r['turns_per_min'] for r in res['L'][0]]) / 60
res['F'] = live('F', SEED0 + 1); res['R'] = live('R', SEED0 + 2, rw=rw)
json.dump({k: {'log': v[0], 'exams': v[1]} for k, v in res.items()}, open(OUT, 'w'), indent=1)
E = {k: np.mean([r['energy'] for r in v[0] if r['hour'] > 24]) for k, v in res.items()}
print(f"\nP29a energy h24-48: L {E['L']:.3f}  F {E['F']:.3f}  R {E['R']:.3f} -> {'PASS' if E['L'] > E['F'] > E['R'] else 'FAIL'}")
ex = res['L'][1]; print('L exams:', json.dumps(ex)); print('F exams:', json.dumps(res['F'][1]))
print(f"P29b thermal: h48 {ex[48]['thermal']:.2f} vs h0 {ex[0]['thermal']:.2f} °C -> {'PASS' if ex[48]['thermal'] - ex[0]['thermal'] >= 1.5 else 'FAIL'}")
print(f"P29c salt: CI h48 {ex[48]['salt']:+.2f} vs h0 {ex[0]['salt']:+.2f} -> {'PASS' if ex[0]['salt'] - ex[48]['salt'] >= 0.2 else 'FAIL'}")
nl = res['L'][0]; n24 = sum(nl[11]['nodes']); n0 = 0; n48 = sum(nl[-1]['nodes'])
print(f"P29d nodes: added h0-24 {n24 - n0:.1f}, h24-48 {n48 - n24:.1f} -> {'PASS' if (n48 - n24) < 0.5 * (n24 - n0) else 'FAIL'}")
