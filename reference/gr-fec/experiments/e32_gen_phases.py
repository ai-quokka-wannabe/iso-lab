"""PREREG21: frozen vs chaotic phases of GPT-2 generation vs temperature; quality judged by Qwen2.5-0.5B."""
import json, sys, numpy as np, torch
from transformers import AutoModelForCausalLM, AutoTokenizer
torch.set_num_threads(2)
PROMPTS = ["The old lighthouse keeper", "In the year 2050, cities", "My grandmother always said that", "The experiment began when",
 "Deep in the forest there was", "The best way to learn a language is", "When the satellite lost contact,", "The recipe calls for",
 "Scientists have discovered a new", "The train was late again, so", "Once upon a time in a small village", "The meeting started with",
 "Mountains in winter are", "The history of the bicycle", "She opened the letter and", "The engineer looked at the data and",
 "Every morning the market", "The museum's newest exhibit", "Rain fell on the city as", "The captain announced that",
 "A good teacher knows", "The ancient map showed", "The software update caused", "Football fans gathered outside",
 "The river had flooded twice", "Our neighbours bought a", "The robot walked into the room and", "Music in the eighteenth century",
 "The detective noticed a small", "The problem with modern cars is", "Last summer we travelled to", "The garden was full of",
 "The president of the club", "The storm knocked out power", "Learning to cook requires", "At the edge of the desert",
 "The children built a", "The telescope revealed", "Coffee prices have risen because", "The violinist played",
 "The village council decided", "The spacecraft entered orbit", "He had never seen snow before", "The factory produced",
 "The library closes at", "Wolves were reintroduced to", "The bridge was designed by", "Her first novel was about",
 "The election results showed", "Bees are important because", "The doctor explained that", "The island has no roads, so",
 "The computer crashed while", "In the kitchen, the smell of", "The mountain rescue team", "The startup raised money to",
 "The old photograph showed", "The language of mathematics", "Night fell over the harbour", "The town's only cinema"]
TS = [0.3, 0.5, 0.7, 0.85, 1.0, 1.15, 1.3, 1.6]; NEW = 130; KF = 10
gt = AutoTokenizer.from_pretrained('gpt2'); g = AutoModelForCausalLM.from_pretrained('gpt2').eval()
@torch.no_grad()
def twins(pid, T, u):
    ids = torch.tensor([pid, pid]); out = [[], []]; past = None; x = ids
    for t in range(NEW):
        o = g(x, past_key_values=past, use_cache=True); past = o.past_key_values
        p = torch.softmax(o.logits[:, -1].double() / T, -1); c = torch.cumsum(p, -1)
        nxt = torch.searchsorted(c, torch.full((2, 1), u[t], dtype=torch.float64)).clamp(max=p.shape[1] - 1)[:, 0]
        if t == KF:
            top2 = torch.topk(p[1], 2).indices; nxt[1] = top2[1] if top2[0] == nxt[0] else top2[0]
        for b in range(2): out[b].append(int(nxt[b]))
        x = nxt[:, None]
    return out
def grams(s, n=4): return [tuple(s[i:i + n]) for i in range(len(s) - n + 1)]
def loopflag(s):
    tail = grams(s[-60:]); seen = set(grams(s[:-60 + 3])); early = set()
    hits = 0
    for i, gm in enumerate(tail):
        if gm in seen or gm in early: hits += 1
        early.add(gm)
    return hits / len(tail) >= 0.5
def jacc(a, b):
    A, B = set(grams(a[-60:])), set(grams(b[-60:])); return len(A & B) / max(len(A | B), 1)
R = {}
for T in TS:
    rows = []
    for k, pr in enumerate(PROMPTS):
        pid = gt(pr)['input_ids']; u = np.random.default_rng(2100 + k).random(NEW)
        A, B = twins(pid, T, u)
        rows.append(dict(p=k, loop=loopflag(A), J=jacc(A, B), text=gt.decode(A)))
    R[T] = rows
    print(f'T={T}: loop share {np.mean([r["loop"] for r in rows]):.2f}  median J {np.median([r["J"] for r in rows]):.2f}', flush=True)
del g
jt = AutoTokenizer.from_pretrained('Qwen/Qwen2.5-0.5B-Instruct'); j = AutoModelForCausalLM.from_pretrained('Qwen/Qwen2.5-0.5B-Instruct', torch_dtype=torch.float32).eval()
@torch.no_grad()
def quality(prompt, text):
    pi = jt(prompt)['input_ids']; gi = jt(text)['input_ids']; ids = torch.tensor([pi + gi])
    lp = torch.log_softmax(j(ids).logits[0, :-1].float(), -1).gather(1, ids[0, 1:, None])[:, 0][len(pi) - 1:].numpy()
    seen = set(); good = 0
    for i in range(len(gi)):
        g4 = tuple(gi[i - 3:i + 1]) if i >= 3 else None
        novel = g4 is None or g4 not in seen
        if g4 is not None: seen.add(g4)
        good += bool(novel and lp[i] >= np.log(0.01))
    return good / len(gi), float(lp.mean())
for T in TS:
    for r in R[T]:
        r['Q'], r['judge_lp'] = quality(PROMPTS[r['p']], r['text'])
    print(f'T={T}: mean Q {np.mean([r["Q"] for r in R[T]]):.3f}  judge lp {np.mean([r["judge_lp"] for r in R[T]]):.2f}', flush=True)
json.dump({str(k): v for k, v in R.items()}, open('results/e32_rows.json', 'w'))
