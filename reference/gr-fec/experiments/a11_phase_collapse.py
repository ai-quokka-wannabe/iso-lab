"""Rescaled-temperature collapse of BP failure phases across codes, decoders and SNRs."""
import numpy as np, matplotlib
matplotlib.use('Agg'); import matplotlib.pyplot as plt
def load(files, X):
    ft=[];fx=[];osc=[];dr=[];fer=[]
    for f in files:
        d=np.load(f); m=d['conv']>50; k='endclass' if 'endclass' in d.files else 'cls'
        c=np.bincount(d[k][m],minlength=3)/m.sum(); ft.append(np.median(d['ftle'][m])); fx.append(c[0]); dr.append(c[1]); osc.append(c[2]); fer.append(m.mean())
    lx=np.log(X); s=np.sign(ft); i=[k for k in range(len(X)-1) if s[k]<0<=s[k+1]][0]
    xc=np.exp(lx[i]+(0-ft[i])*(lx[i+1]-lx[i])/(ft[i+1]-ft[i]))
    return np.array(X)/xc, np.array(ft), np.array(fx), np.array(dr), np.array(osc), np.array(fer)
S=[]
A5=[0.5,0.7,0.85,1.0,1.2,1.5,2.0]; A8=[0.6,0.75,0.85,0.92,1.0,1.08,1.2,1.4]; A40=[0.55,0.6,0.65,0.7,0.75,0.8,0.9]
S.append(('CCSDS SP 3.0','#2a6fdb','o',load([f'results/e07_alpha{a}_s500.npz' for a in A5],A5)))
for eb in [2.5,3.5]: S.append((f'CCSDS SP {eb}','#2a6fdb','o',load([f'results/e07_eb{eb}_alpha{a}_s600.npz' for a in A8],A8)))
S.append(('CCSDS SP 4.0','#2a6fdb','o',load([f'results/e07_eb4.0_alpha{a}_s700.npz' for a in A40],A40)))
G1=[0.5,0.65,0.8,0.95,1.1,1.3,1.6]; G2=[0.5,0.6,0.7,0.8,0.9,1.0,1.15]
for eb,s,G in [(2.0,800,G1),(2.5,800,G1),(3.0,802,G2)]: S.append((f'(3,6) SP {eb}','#2e9e5b','s',load([f'results/e21_R36_200_eb{eb}_alpha{a}_s{s}.npz' for a in G],G)))
for eb,s,G in [(2.5,801,G1),(3.0,801,G1),(3.5,803,G2)]: S.append((f'(4,8) SP {eb}','#9b59b6','D',load([f'results/e21_R48_200_eb{eb}_alpha{a}_s{s}.npz' for a in G],G)))
B=[0.4,0.5,0.6,0.7,0.8,0.9,1.0]
S.append(('CCSDS min-sum 3.0 (β)','#c0392b','^',load([f'results/e22_beta{b}.npz' for b in B],B)))
for eb in [2.5,3.5]: S.append((f'CCSDS min-sum {eb} (β)','#c0392b','^',load([f'results/e22_eb{eb}_beta{b}.npz' for b in B],B)))
fig,ax=plt.subplots(1,3,figsize=(14,4.3))
seen=set()
for name,col,mk,(x,ft,fx,dr,osc,fer) in S:
    grp=name.split(' ')[0]+(' min-sum' if 'min-sum' in name else ' sum-product')
    lab=None if grp in seen else grp.replace('CCSDS sum-product','CCSDS (128,64) sum-product').replace('(3,6) sum-product','(3,6) n=200 sum-product').replace('(4,8) sum-product','(4,8) n=200 sum-product').replace('CCSDS min-sum','CCSDS (128,64) min-sum (knob = β)')
    seen.add(grp)
    ax[0].plot(x,ft,'-',marker=mk,color=col,alpha=.7,ms=4,lw=1,label=lab)
    ax[1].plot(x,fx*100,'-',marker=mk,color=col,alpha=.7,ms=4,lw=1); ax[1].plot(x,osc*100,'--',marker=mk,color=col,alpha=.5,ms=3,lw=1)
    ax[2].plot(x,fer/fer.min(),'-',marker=mk,color=col,alpha=.7,ms=4,lw=1)
for a in ax:
    a.set_xscale('log'); a.axvline(1,color='k',ls=':',lw=1); a.grid(alpha=.3); a.set_xlabel('temperature knob / its own chaos onset  (α/α_c or β/β_c)')
ax[0].axhline(0,color='k',lw=.6); ax[0].set_ylabel('median FTLE of failing frames'); ax[0].set_title('Chaos vs rescaled knob (13 sweeps)',fontsize=10); ax[0].legend(fontsize=7)
ax[1].set_ylabel('% of failures: frozen (solid) / oscillating (dashed)'); ax[1].set_title('Frozen → chaotic transition at the onset',fontsize=10)
ax[2].set_ylabel('FER / its minimum'); ax[2].set_ylim(0.95,2.2); ax[2].set_title('FER optimum at 0.96–1.42× onset (11/13 chaotic side)',fontsize=10)
fig.suptitle('Belief-propagation failure phases across 3 codes, 2 decoders, 2.0–4.0 dB',fontsize=11)
plt.tight_layout(); plt.savefig('results/phase_collapse.png',dpi=140)
# quantify collapse: spread of FTLE at fixed rescaled x
xs=np.linspace(0.75,1.3,12); vals=[]
for name,col,mk,(x,ft,*_) in S:
    vals.append(np.interp(np.log(xs),np.log(x),ft,left=np.nan,right=np.nan))
V=np.array(vals); print('FTLE spread across 13 sweeps at fixed rescaled knob (x, mean, sd):')
for xx,m,sd in zip(xs,np.nanmean(V,0),np.nanstd(V,0)): print(f'  x={xx:.2f}: mean {m:+.3f} sd {sd:.3f}')
opt=[x[int(np.argmin(fer))] for *_,(x,ft,fx,dr,osc,fer) in S]
print('FER-optimal rescaled knob per sweep:', np.round(opt,2), ' all > 1:', all(o>1 for o in opt))
