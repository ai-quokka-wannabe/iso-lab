"""Project-wide statistics: ledger verdicts and pre-registered prediction outcomes."""
import re, glob, collections
rows = [l for l in open('results/LEDGER.md') if re.match(r'\| L\d+ ', l)]
cats = collections.Counter()
def cat(v):
    v = v.lower()
    for k, lab in (('refut', 'refuted'), ('not shown', 'not shown'), ('null', 'null'), ('suggestive', 'suggestive'), ('partial', 'partial'),
                   ('explained', 'explained / measured'), ('measured', 'explained / measured'), ('observed', 'explained / measured'),
                   ('confirm', 'confirmed'), ('genuine', 'confirmed'), ('known', 'known (prior art)'), ('revises', 'revision')):
        if k in v: return lab
    return 'other'
by_area = collections.defaultdict(collections.Counter)
for l in rows:
    parts = [p.strip() for p in l.split('|')]; n = int(parts[1][1:]); v = cat(parts[3])
    area = 'FEC/physics (L1–L52)' if n <= 52 else ('AI gate/memory (L53–L55, L71)' if n in (53, 54, 55, 71) else 'worm / embodied AI (L56–)')
    cats[v] += 1; by_area[area][v] += 1
print(f'Ledger rows: {len(rows)}'); print('verdicts overall:', dict(cats.most_common()))
for a, c in by_area.items(): print(f'  {a}: {dict(c.most_common())}')
P = F = 0; per = []
for f in sorted(glob.glob('results/PREREG*.md')):
    t = open(f).read(); o = t[t.find('Outcome'):] if 'Outcome' in t else ''
    p = len(re.findall(r'\bPASS', o)); q = len(re.findall(r'\bFAIL', o)); P += p; F += q; per.append((f.split('/')[-1], p, q))
print(f'\nPre-registered outcomes (counted from "Outcome" sections): PASS {P}, FAIL {F}, pass rate {P / max(P + F, 1):.0%}')
print('per file (PREREG number: passes/outcomes):', ', '.join(f'{(re.findall(r"\d+", n) or ["1"])[0]}:{p}/{p + q}' for n, p, q in per if p + q)); print(f'files with an Outcome section: {sum(1 for _, p, q in per if p + q)} of {len(per)} (earlier FEC pre-registrations recorded outcomes in the ledger instead)')
