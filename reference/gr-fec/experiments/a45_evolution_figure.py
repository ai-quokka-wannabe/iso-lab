"""Evolution of fitness and genes (PREREG34)."""
import json, numpy as np, matplotlib
matplotlib.use('Agg'); import matplotlib.pyplot as plt
d = json.load(open('results/e45_evolve.json')); H = d['history']
g = np.array([h['gen'] for h in H]); hard = np.array([h['world'] == 'HardWorld' for h in H])
fig, ax = plt.subplots(1, 3, figsize=(16, 4.3))
for m, col, lab in ((hard, '#c0392b', 'harsh world'), (~hard, '#2a6fdb', 'rich world')):
    ax[0].plot(g[m], [H[i]['elite_mean'] for i in np.nonzero(m)[0]], '-o', color=col, ms=4, label=f'elite mean ({lab})')
    ax[0].plot(g[m], [H[i]['pop_mean'] for i in np.nonzero(m)[0]], '--', color=col, alpha=.6, label=f'population mean ({lab})')
ax[0].axhline(0.682, color='#8e44ad', ls=':', label='hand-built v6, harsh (72 h protocol)')
ax[0].set_xlabel('generation'); ax[0].set_ylabel('lifetime mean energy'); ax[0].legend(fontsize=7); ax[0].grid(alpha=.3); ax[0].set_title('Fitness rises; harsh-world population catches up', fontsize=9)
hand = dict(g_odour=2.354, g_salt=0.706, r_eat=4, r_hunger=1, trust_k=2, roam=0, curiosity=0)
for k, col in (('g_odour', '#e67e22'), ('g_salt', '#16a085'), ('trust_k', '#8e44ad'), ('roam', '#c0392b'), ('curiosity', '#2e9e5b'), ('r_eat', '#333'), ('r_hunger', '#999'), ('g_gut', '#2a6fdb')):
    ax[1].plot(g, [h['elite_median_genes'][k] for h in H], '-', color=col, label=k + (f' (hand {hand[k]})' if k in hand else ''))
ax[1].axhline(0, color='k', lw=.5); ax[1].set_xlabel('generation'); ax[1].set_ylabel('elite median gene'); ax[1].legend(fontsize=6.5, ncol=2); ax[1].grid(alpha=.3)
ax[1].set_title('What evolution chose (started random, incl. off / wrong sign)', fontsize=9)
for k, col in (('eta', '#c0392b'), ('s0', '#2a6fdb'), ('tau', '#2e9e5b'), ('integ', '#8e44ad')):
    ax[2].plot(g, [h['elite_median_genes'][k] for h in H], '-', color=col, label=k)
ax[2].set_yscale('log'); ax[2].set_xlabel('generation'); ax[2].legend(fontsize=7); ax[2].grid(alpha=.3)
ax[2].set_title('Brain settings (η and integration time hit their range limits)', fontsize=9)
fig.suptitle('Evolving the worm brain: 100 genomes × 20 generations, selected only by lifetime energy', fontsize=11)
plt.tight_layout(); plt.savefig('results/worm_evolution.png', dpi=140)
