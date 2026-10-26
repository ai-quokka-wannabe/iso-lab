"""PREREG20: gravity gate on factual answers of a small LLM."""
import json, re, unicodedata, sys, numpy as np, torch
from transformers import AutoModelForCausalLM, AutoTokenizer
torch.manual_seed(2000); torch.set_num_threads(2)
MID = 'Qwen/Qwen2.5-0.5B-Instruct'
tok = AutoTokenizer.from_pretrained(MID); model = AutoModelForCausalLM.from_pretrained(MID, torch_dtype=torch.float32).eval()
END = tok.convert_tokens_to_ids('<|im_end|>')
F = json.load(open('results/e31_facts.json'))
DROP = {'Antilles néerlandaises', 'Bonaire, Sint Eustatius and Saba', 'Holy See (Vatican City State)', 'Vatican City State', 'Palestine',
        'Palestine, State of', 'Israel', 'Western Sahara', 'Serbia and Montenegro', 'Hong Kong', 'South Georgia',
        'British Indian Ocean Territory', 'Cocos (Keeling) Islands', 'Kiribati', 'Tokelau'}
RENAME = {'Republic of Macedonia': 'North Macedonia', 'Swaziland': 'Eswatini'}
ALIAS = {'Kazakhstan': ['Astana'], 'Bolivia': ['La Paz'], 'Sri Lanka': ['Sri Jayawardenepura Kotte', 'Kotte'],
         'South Africa': ['Cape Town', 'Bloemfontein'], 'Swaziland': ['Mbabane'], 'Benin': ['Cotonou'], 'Malaysia': ['Putrajaya'],
         'Montserrat': ['Brades'], 'Palau': ['Melekeok'], 'Ukraine': ['Kiev'], 'Mongolia': ['Ulan Bator'], 'Guam': ['Agana'],
         'United States': ['Washington'], 'San Marino': ['San Marino'], 'Kuwait': ['Kuwait'], 'Guatemala': ['Guatemala'],
         'Panama': ['Panama'], 'Myanmar': ['Nay Pyi Taw', 'Naypyitaw'], 'Yemen': ['Sanaa'], 'Chad': ['Ndjamena']}
EALIAS = {'Aluminum': ['Aluminium'], 'Cesium': ['Caesium'], 'Sulfur': ['Sulphur']}
def norm(s):
    s = unicodedata.normalize('NFKD', s).encode('ascii', 'ignore').decode().lower()
    return re.sub(r'[^a-z]', '', s.replace('st.', 'saint'))
Q = []
for name, sym in F['elements']:
    Q.append(('sym', f'What is the chemical symbol of {name}? Answer with the symbol only.', [sym]))
    Q.append(('elem', f'Which chemical element has the symbol {sym}? Answer with the element name only.', [name] + EALIAS.get(name, [])))
for c, cap in F['capitals']:
    if c in DROP: continue
    Q.append(('cap', f'What is the capital of {RENAME.get(c, c)}? Answer with the city name only.', [cap] + ALIAS.get(c, [])))
def correct(kind, ans, truth):
    if kind == 'sym':
        w = re.findall(r'[A-Za-z]+', ans); return bool(w) and w[0] in truth
    a = norm(ans); return len(a) >= 3 and any(norm(t) in a or a in norm(t) for t in truth if len(norm(t)) >= 3)
def same(kind, a, b):
    if kind == 'sym':
        wa, wb = re.findall(r'[A-Za-z]+', a), re.findall(r'[A-Za-z]+', b); return bool(wa) and bool(wb) and wa[0] == wb[0]
    na, nb = norm(a), norm(b); return na == nb or (len(na) >= 3 and len(nb) >= 3 and (na in nb or nb in na))
def prompt_ids(q):
    txt = tok.apply_chat_template([{'role': 'user', 'content': q}], add_generation_prompt=True, tokenize=False)
    return tok(txt, return_tensors='pt').input_ids
@torch.no_grad()
def seq_lp(pid, ans_ids):
    ids = torch.cat([pid, torch.tensor([ans_ids + [END]])], 1)
    lp = torch.log_softmax(model(ids).logits[0, :-1].float(), -1)
    t = ids[0, pid.shape[1]:]; v = lp[pid.shape[1] - 1:].gather(1, t[:, None])[:, 0]
    return v.sum().item(), v.mean().item()
def cut(ids):
    out = []
    for t in ids:
        if t in (END, tok.eos_token_id): break
        out.append(int(t))
    return out
@torch.no_grad()
def greedy_from(pid, first):
    ids = torch.cat([pid, torch.tensor([[first]])], 1)
    g = model.generate(ids, max_new_tokens=11, do_sample=False, pad_token_id=END)[0, pid.shape[1]:]
    return cut(g.tolist())
rows = []
for i, (kind, q, truth) in enumerate(Q):
    pid = prompt_ids(q)
    logits = model(pid).logits[0, -1]; top = torch.topk(logits, 5).indices.tolist()
    ans_ids = greedy_from(pid, top[0]); ans = tok.decode(ans_ids).strip()
    cands = [greedy_from(pid, t) for t in top[1:]]
    S = model.generate(pid, max_new_tokens=12, do_sample=True, temperature=1.0, top_k=0, top_p=1.0, num_return_sequences=8, pad_token_id=END)
    samples = [cut(s[pid.shape[1]:].tolist()) for s in S]
    agree = np.mean([same(kind, tok.decode(s).strip(), ans) for s in samples])
    lp, mlp = seq_lp(pid, ans_ids)
    others = {}
    for c in cands + samples:
        t = tok.decode(c).strip()
        if t and not same(kind, t, ans) and t not in others: others[t] = seq_lp(pid, c)[0]
    best_other = max(others.values()) if others else lp - 30.0
    rows.append(dict(kind=kind, q=q, truth=truth[0], ans=ans, ok=correct(kind, ans, truth), lp=lp, meanlp=mlp,
                     gap=lp - best_other, agree=float(agree), runner=max(others, key=others.get) if others else ''))
    if i % 25 == 0: print(i, len(Q), kind, repr(ans), rows[-1]['ok'], f"gap={rows[-1]['gap']:.2f} agree={agree:.2f}", flush=True)
json.dump(rows, open('results/e31_rows.json', 'w'), indent=0)
