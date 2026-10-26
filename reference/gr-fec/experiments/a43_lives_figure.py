"""Energy over life for every 'life' experiment (PREREG29–32)."""
import json, numpy as np, matplotlib
matplotlib.use('Agg'); import matplotlib.pyplot as plt
fig, ax = plt.subplots(1, 3, figsize=(16, 4.4), sharey=True)
def plot(a, path, keys, title):
    d = json.load(open(path))
    for k, lab, col, ls in keys:
        if k in d:
            L = d[k]['log']; a.plot([r['hour'] for r in L], [r['energy'] for r in L], ls, color=col, label=lab, lw=1.6)
    a.set_title(title, fontsize=9); a.set_xlabel('hours of life'); a.grid(alpha=.3); a.legend(fontsize=7)
plot(ax[0], 'results/e40_life.json', [('L', 'learning (instinct inside learner)', '#c0392b', '-'), ('F', 'instinct only', '#2a6fdb', '-'), ('R', 'random walker', '#999', '--')],
     'Rich world, v4b: learning hurts (learns food zone is "disappointing")')
plot(ax[1], 'results/e41_life_v5.json', [('L', 'learning (instinct separate)', '#c0392b', '-'), ('F', 'instinct only', '#2a6fdb', '-'), ('R', 'random walker', '#999', '--')],
     'Rich world, v5: damage gone, nothing to learn')
plot(ax[2], 'results/e42_harsh.json', [('L', 'learning v5', '#c0392b', '-'), ('F', 'instinct only (trapped by decoys)', '#2a6fdb', '-'), ('R', 'random walker', '#999', '--')],
     'Harsh world: learning beats instinct, v6 adds trust + curiosity')
try:
    d = json.load(open('results/e43_v6.json'))
    for k, lab, col in (('TC', 'v6 trust + curiosity', '#2e9e5b'), ('T', 'v6 trust only', '#8e44ad'), ('C', 'v6 curiosity only', '#e67e22')):
        L = d[k]['log']; ax[2].plot([r['hour'] for r in L], [r['energy'] for r in L], '-', color=col, label=lab, lw=1.6)
    ax[2].legend(fontsize=7)
except FileNotFoundError: pass
ax[0].set_ylabel('mean energy of 100 worms')
fig.suptitle("A worm's life: energy over time in each world and brain version", fontsize=11)
plt.tight_layout(); plt.savefig('results/worm_lives.png', dpi=140)
