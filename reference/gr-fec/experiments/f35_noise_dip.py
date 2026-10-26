"""Forensics for the PREREG24 dip: is k=0 a lucky single realisation of a deterministic system?"""
import sys, numpy as np, warnings
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src'); sys.path.insert(0, 'experiments')
import importlib.util
spec = importlib.util.spec_from_file_location('e35', 'experiments/e35_sensor_noise.py')
src = open('experiments/e35_sensor_noise.py').read().split('KS = [')[0]
ns = {}; exec(src, ns); DetWorm, TrapPlate, Plate = ns['DetWorm'], ns['TrapPlate'], ns['Plate']
def run(k, seed, P, offset):
    w = DetWorm(100, np.random.default_rng(seed), k); w.place(np.zeros((100, 2)))
    w.head = w.head + offset; turns = 0; dist = []
    for t in range(1200):
        turns += w.step(P).mean()
        if t % 60 == 59: dist.append(np.linalg.norm(w.xy - [15, 0], axis=1).mean())
    nt = (np.linalg.norm(w.xy - [15, 0], axis=1) < 8).sum(); nc = (np.linalg.norm(w.xy - [-10, 0], axis=1) < 8).sum()
    return (nt - nc) / 100, turns / 1200 * 60, np.mean(dist)
for name, P in (('W1', Plate(odour=True)), ('W2', TrapPlate(odour=True))):
    for k in (0, 0.1, 0.3, 1, 3):
        R = np.array([run(k, 7000 + i, P, np.random.default_rng(7500 + i).uniform(0, 2 * np.pi / 100)) for i in range(12)])
        print(f'{name} k={k:<4}: CI {R[:,0].mean():+.3f} ± {R[:,0].std()/np.sqrt(12):.3f} (spread {R[:,0].min():+.2f}..{R[:,0].max():+.2f})  turns/min {R[:,1].mean():.2f}  mean dist to target {R[:,2].mean():.1f} mm')

print('\nWhere do worms end up? (W1: distance to target; W2: share near target / near decoy)')
def ends(k, seed, P):
    w = DetWorm(100, np.random.default_rng(seed), k); w.place(np.zeros((100, 2))); traj = []
    for t in range(1200):
        w.step(P)
        if t >= 1080: traj.append(w.xy.copy())
    traj = np.array(traj); d = np.linalg.norm(traj - [15, 0], axis=2)
    return d[-1], d.std(0), np.linalg.norm(w.xy - [-10, 0], axis=1)
for k in (0, 0.1, 1):
    d, sd, dd = zip(*[ends(k, 7100 + i, Plate(odour=True)) for i in range(6)]); d = np.concatenate(d); sd = np.concatenate(sd)
    q = np.percentile(d, [10, 25, 50, 75, 90])
    print(f'W1 k={k:<4}: final distance percentiles 10/25/50/75/90 = {np.round(q,1)};  within 8 mm {(d<8).mean():.2f};  distance wobble over last 2 min (median sd) {np.median(sd):.2f} mm')
for k in (0, 0.1, 1):
    r = [ends(k, 7200 + i, TrapPlate(odour=True)) for i in range(6)]
    t = np.concatenate([x[0] for x in r]); dc = np.concatenate([x[2] for x in r])
    print(f'W2 k={k:<4}: near target {(t<8).mean():.2f}, near decoy {(dc<8).mean():.2f}')
