"""PREREG33: day-2 batch. Usage: python3 e44_day2.py <job> ; jobs: A (F,R,L5,T), B (TC,TR,switch-T,switch-F), C (lab assays on T)."""
import sys, json, warnings, numpy as np
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src'); sys.path.insert(0, 'experiments')
from embodied import Worm, World, NCH, MX, NT
from lifeworld import HardWorld, LifeWorld
ns = {}; exec(open('experiments/e40_life.py').read().split('def live(')[0], ns); exam, value_curve = ns['exam'], ns['value_curve']
N = 100; BASE = dict(s0=0.01, integ=10, separate=True)
def trust(br, hm):
    c0 = NCH * MX; wl = br.W[:, c0:c0 + 2] + hm[:, None] * br.W[:, NT + c0:NT + c0 + 2]
    return np.clip(1 + (br.trust_k or 0) * wl, 0, 1.5).mean(0) if br.trust_k else np.array([1.0, 1.0])
def live(name, seed, extra=None, schedule=((72, HardWorld),)):
    w = Worm(N, seed, **BASE, **(extra or {})); w.body.R = 50.0; log = []; exams = {}; hour = 0; learning = (extra or {}).get('eta', 1) != 0 and 'random_walk_p' not in (extra or {})
    for hours, WC in schedule:
        world = WC(N)
        for blk in range(0, hours, 4):
            if hour in (0, 36, 72) and 'random_walk_p' not in (extra or {}): exams[hour] = exam(w)
            onf = odl = pir = 0.0
            for t in range(14400):
                y, b = w.body.sense(world); p, _ = w.brain.step(y); w.body.act(p); w.body.metabolise(b); world.consume(w.body.intake)
                onf += (b > 0).mean(); pir += p.mean()
                if isinstance(world, HardWorld): odl += ((world.which >= 0) & (world.which < 4)).mean()
            hour += 4
            row = dict(hour=hour, world=WC.__name__, energy=float(w.body.E.mean()), on_food=onf / 14400, on_odourless=odl / 14400,
                       turns_per_min=pir / 14400 * 60, cool_share=float((w.body.xy[:, 0] < -25).mean()), x=float(w.body.xy[:, 0].mean()))
            if learning:
                row['temp_value'] = value_curve(w.brain, 3, np.linspace(15, 25, 11)).round(3).tolist()
                row['odour_value'] = value_curve(w.brain, 0, np.linspace(0, 1, 6)).round(3).tolist()
                row['trust'] = trust(w.brain, w.body.hormone).round(3).tolist()
            log.append(row)
            print(f"{name} h{hour:2d} [{WC.__name__}]: energy {row['energy']:.2f} on-food {row['on_food']:.2f} odourless {row['on_odourless']:.3f} "
                  f"cool {row['cool_share']:.2f} turns {row['turns_per_min']:.1f}" + (f" trust {row['trust']}" if 'trust' in row else ''), flush=True)
    if 'random_walk_p' not in (extra or {}): exams[hour] = exam(w)
    return dict(log=log, exams=exams)
job = sys.argv[1]; out = {}
if job == 'A':
    out['L5'] = live('L5', 19002); rw = np.mean([r['turns_per_min'] for r in out['L5']['log']]) / 60
    out['F'] = live('F', 19000, dict(eta=0.0)); out['R'] = live('R', 19001, dict(random_walk_p=rw)); out['T'] = live('T', 19003, dict(trust_k=2.0))
elif job == 'B':
    out['TC'] = live('TC', 19004, dict(trust_k=2.0, curiosity=1.0)); out['TR'] = live('TR', 19005, dict(trust_k=2.0, roam=3.0))
    sw = ((36, HardWorld), (36, LifeWorld))
    out['swT'] = live('swT', 19600, dict(trust_k=2.0), sw); out['swF'] = live('swF', 19601, dict(eta=0.0), sw)
elif job == 'C':
    import e39_embodied as E39, functools
    E39.Worm.__init__ = functools.partialmethod(E39.Worm.__init__, integ=10, separate=True, trust_k=2.0)
    s0 = 0.01; R = []
    ci, pr = E39.a1(19501, s0); R.append(ci >= .3)
    t = E39.a2(19510, s0); d = t[25].mean() - t[15].mean(); R.append(d >= 3)
    m = E39.a3(19520, s0); R.append(m['mock'] >= .15 and m['salt'] <= m['mock'] - .3)
    f, l, rec = E39.a4(19530, s0); R.append(l <= .5 * f and rec >= .8 * f)
    e, la = E39.a5(19540, s0); R.append(e >= 2 * la)
    out = dict(A1=ci, A2=d, A3=m, A4=(f, l, rec), A5=(e, la), passes=R)
    print(f"A1 CI {ci:+.2f} | A2 diff {d:+.2f} °C | A3 mock {m['mock']:+.2f} salt {m['salt']:+.2f} | A4 {f:.2f}->{l:.2f}->{rec:.2f} | A5 {e:.2f} vs {la:.2f} (x{e/max(la,1e-9):.2f})")
    print(f"P33c: {sum(R)}/5 pass -> {'PASS' if sum(R) >= 4 else 'FAIL'}  {R}")
json.dump(out, open(f'results/e44_{job}.json', 'w'), indent=1, default=float)
