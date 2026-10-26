"""Figure for worm agent v1: trajectories (thermotaxis memory, salt learning), foraging, exploration temperature."""
import sys, functools, warnings, numpy as np, matplotlib
warnings.filterwarnings('ignore'); matplotlib.use('Agg'); import matplotlib.pyplot as plt
sys.path.insert(0, 'src'); sys.path.insert(0, 'experiments')
import worm
worm.Worms.__init__ = functools.partialmethod(worm.Worms.__init__, rectify=True)
import e33_worm as E; E.Worms = worm.Worms
S0 = 0.03; n = 12
def track(w, plate, T=1200):
    w.place(np.zeros((w.n, 2))); P = [w.xy.copy()]
    for _ in range(T): w.step(plate); P.append(w.xy.copy())
    return np.array(P)
fig, ax = plt.subplots(1, 4, figsize=(17, 4.4))
th = np.linspace(0, 2 * np.pi, 200)
# panel 1: thermotaxis memory
X, Y = np.meshgrid(np.linspace(-25, 25, 200), np.linspace(-25, 25, 200)); Tm = 20 + 5 * X / 25; Tm[X**2 + Y**2 > 625] = np.nan
ax[0].imshow(Tm, extent=(-25, 25, -25, 25), origin='lower', cmap='coolwarm', alpha=.35)
for tc, col in ((15, '#1f5fbf'), (25, '#c0392b')):
    w = worm.Worms(n, np.random.default_rng(5000 + tc), s0=S0); E.cultivate(w, temp=tc); P = track(w, worm.Plate(thermal=True))
    for i in range(n): ax[0].plot(P[:, i, 0], P[:, i, 1], color=col, lw=.6, alpha=.7)
    ax[0].plot([], [], color=col, label=f'raised at {tc} °C')
ax[0].set_title('Thermotaxis memory (no thermotaxis code)\n15 °C ← plate temperature → 25 °C', fontsize=9); ax[0].legend(fontsize=7, loc='lower left')
# panel 2: salt learning
for cond, col, su in (('mock-starved', '#2e9e5b', None), ('starved in salt', '#8e44ad', 0.8)):
    w = worm.Worms(n, np.random.default_rng(5100 + (su is not None)), s0=S0); E.cultivate(w, food=False, secs=3600, salt_uniform=su)
    P = track(w, worm.Plate(salt=True))
    for i in range(n): ax[1].plot(P[:, i, 0], P[:, i, 1], color=col, lw=.6, alpha=.7)
    ax[1].plot([], [], color=col, label=cond)
ax[1].add_patch(plt.Circle((15, 0), 8, fill=False, ls='--', color='k')); ax[1].text(15, 9.5, 'salt peak', ha='center', fontsize=7)
ax[1].set_title('Salt associative learning (no salt-learning code)', fontsize=9); ax[1].legend(fontsize=7, loc='lower left')
for a in ax[:2]: a.plot(25 * np.cos(th), 25 * np.sin(th), 'k', lw=.8); a.set_aspect('equal'); a.set_xticks([]); a.set_yticks([])
# panel 3: foraging
for rw, col, lab in ((None, '#333', 'agent'), (4.09 / 60, '#aaa', 'random walker')):
    w = worm.Worms(100, np.random.default_rng(5200), s0=S0, random_walk_p=rw); E.cultivate(w, food=True, secs=1800)
    w.place(np.zeros((100, 2))); p = worm.Plate(); r = []
    for _ in range(1200): r.append(w.step(p)[0].mean())
    r = np.array(r).reshape(20, 60).sum(1)
    ax[2].plot(np.arange(20) + .5, r, '-o', ms=3, color=col, label=lab)
ax[2].set_xlabel('minutes after leaving food'); ax[2].set_ylabel('pirouettes per minute'); ax[2].legend(fontsize=7)
ax[2].set_title('Foraging: local search, then roaming', fontsize=9); ax[2].grid(alpha=.3)
# panel 4: exploration temperature (from e34 log)
T = [0.03, 0.1, 0.3, 1, 3, 10]; m = [0.554, 0.538, 0.574, 0.522, 0.405, 0.135]; se = [0.019, 0.011, 0.015, 0.014, 0.019, 0.013]
ax[3].errorbar(T, m, yerr=se, fmt='-o', color='#333', capsize=3); ax[3].set_xscale('log'); ax[3].grid(alpha=.3)
ax[3].set_xlabel('exploration temperature τ'); ax[3].set_ylabel('chemotaxis index (8 seeds, ±SE)')
ax[3].set_title('No edge-of-chaos peak: plateau, then decline', fontsize=9)
fig.suptitle('Worm-level hypergraph agent v1 — simulated C. elegans assays', fontsize=11)
plt.tight_layout(); plt.savefig('results/worm_v1.png', dpi=140)
