# gr-fec — instructions for Claude Code

**Start by reading `docs/HANDOFF.md`** (state, code map, exact commands, gotchas, open threads), then `docs/principles.md`
and `docs/ai_branch.md` §0 and §8.

Working rules agreed with the user (Matej):
- British English; warm but brutally honest; flag "mirage or real?" on ideas, including his.
- Pre-register every prediction in `results/PREREG<n>.md` before running; append an "## Outcome" with PASS/FAIL; replicate
  surprising wins on fresh seeds; never reuse a seed block for a new claim.
- Write everything down after each step: ledger row in `results/LEDGER.md`, section in `docs/ai_branch.md` §6, update §0/§8,
  `docs/principles.md` if a principle changes, dated line in the brief (`/home/claude/GR_FEC_research_brief.md` → copy to
  `docs/brief.md`), rebuild the zip.
- The worm brain may receive information **only through simulated senses** (World → Body → Brain); no cheats.
- Prefer organic growth/evolution over hard-coding, **but keep the mind an interpretable, printable hypergraph** (no LLMs or
  opaque networks as the brain).
- Current focus: the worm (C. elegans level); FEC/EDAC work is paused.
- Gotchas: never `pkill -f` a pattern that appears in your own command; wait with `until …; do sleep 30; done` loops;
  long runs need `run_in_background` or must be watched within the turn.
