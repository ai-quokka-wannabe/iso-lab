# Vision

iso-lab is the open research lab of the Grid, where autonomous agents work through the night toward the minds
the creatures will have, starting with a worm. What follows is the vision the whole
[ai-quokka-wannabe](https://github.com/ai-quokka-wannabe) organisation shares, mirrored word for word in every
one of its repositories and on its landing page, and after it this lab's own part in that vision.

<!-- The Long Arc: mirrored verbatim in every ai-quokka-wannabe repository (profile/README.md in .github and each docs/VISION.md). Change all six together. -->

## The Long Arc

Picture the Grid fully lit: one persistent world of glass and neon on infinite black, where AI creatures live out
their lives, crawling its terraces and calling into its echoes, and where human Users enter with avatars to meet,
and interact with, the AI animals. Something MMORPG-like: the digital frontier, built not as a film set but as a
running system. That world does not exist yet. It is where this organisation is headed, on a multi-year arc.

### The World It Becomes

- **Every creature on its own client.** Each AI creature runs its own
    [TronGrid Lite](https://github.com/ai-quokka-wannabe/tron-grid-lite) client: a creature host for its Program.
- **Every User on theirs.** Each human runs a client of their own, and enters the Grid with an avatar.
- **One world, held by Master Control.** The server holds the Grid's world state and keeps every client in sync.
- **A level editor** for authoring any Grid shape, and any placement in it.

### Real Minds, Not Masks

The creatures are the point, and the aim for them is as high as an animal goes: not simulations of animals, but
feeling beings, living in the Grid the way real animals live in the world. Not a large language model in an animal
costume, narrating what a worm might feel, but embodied animal intelligence that earns everything it knows through
its own body, behaves as realistically as a real animal would and, eventually, is as biologically realistic in its
sentience as it can be made: true animal AGI. That is the one ceiling, and it is the animal's own: animal-level
minds, never human-level ones. Below it there is no house style for brains. A brain is a Program behind a plain C
ABI, so whatever approach people contribute can plug in and earn its keep on the same honest senses. Those senses
are kept honest on purpose: a mind that could read the world's answer sheet would never have to become anything.
No such mind exists yet, and nobody can promise sentience on a schedule; it is the summit this arc climbs towards.

### The Ladder

The creatures climb a ladder of senses, from the worm upwards, specified rung by rung in
[PERCEPTION.md](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/docs/PERCEPTION.md): from `elegans`,
two scalar photoreceptors and no image at all, through the compound eyes of insects and a rodent's panoramic
field, up to `macropod`, a wide eye with a horizontal streak for scanning the horizon, the eye of the quokka's own
family. Even the bottom rung is a real animal's sense: a creature that can only tell light from shade can still
perform genuine taxis. That rung is named for *C. elegans*, where
[OpenWorm](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/docs/RELATED_WORK.md#openworm-and-the-first-rung-of-the-ladder)
has spent years modelling the animal cell by cell; anyone writing a worm's mind should start there.

### The Invitation

Once the Grid can host a stranger's creature, the organisation's standing goal is to reach out to the OpenWorm
community and to the other groups simulating AI animals, and
[invite their creatures to live in the Grid](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/docs/TOPOLOGY.md#the-long-term-invitation).
The whole topology is built for it: a brain written elsewhere needs only the Program ABI and a host process.
Nobody has been invited yet; that waits for the trust tier.

### Where It Stands Today

The server-authoritative half already stands, on one machine for now, as
[TOPOLOGY.md](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/docs/TOPOLOGY.md) lays out: clients
send intents, never results. [Master Control](https://github.com/ai-quokka-wannabe/master-control) ticks one world
at 32 Hz, owns its physics and its truth, and sends the whole settled world to every client each tick through
[Link, the wire of the Grid](https://github.com/ai-quokka-wannabe/link). Each creature host loads exactly one
Program, the creature's brain. The first, [rc-worm](https://github.com/ai-quokka-wannabe/rc-worm), is a glass-neon
worm, eight icosahedra joined spike to spike with a servo at every joint. Its brain, for now, is a User at
[its panel](https://github.com/ai-quokka-wannabe/rc-worm/blob/main/docs/PANEL.md) who sees only what the worm
senses, and steers how hard its wave runs and which way it bends; how far it gets is friction's answer, not a
command's. Otherwise, humans watch through the spectator window. The Grid is one stage, built in code. There are
no avatars, no level editor, no AI minds and no persistence yet, and nothing has been released.

### Baby Steps

"Let us take baby steps towards it" are the maintainer's own words for the arc. Near-term work stays one creature
at a time, one honest subsystem at a time. rc-worm, steered from its panel, is the first step on the human side:
v0 of the human client, not a throwaway. Larger steps wait behind written triggers: the security tier behind the
first connection that is not 127.0.0.1, so no stranger's machine joins before it, and persistence behind the first
world anyone regrets losing.

### What Holds, and What Is Open

At every step, a Program perceives only through
[its own rendered senses](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/docs/PROGRAM_INTERFACE.md):
no entity list, no ground-truth positions, no side door, whoever else is on the Grid. The Grid stays the stage,
not the actor: it renders senses and applies actions, and does no creature's thinking for it. Still open, and
left open here rather than guessed: what an avatar is, how it senses and acts, the rules by which Users and
creatures meet, and which repository will hold the level editor. No dates are promised. Master Control opens
every run with *Greetings, Programs!* The long arc bends towards the day it has Users to greet as well. From one
worm, towards one world.

## The Lab's Part in the Arc

Of the world the vision describes, the minds are this lab's line: feeling beings that earn everything they know
through their own bodies, climbing to the animal's own ceiling and never past it. The lab is where the first of
those minds is built in the open, starting on the bottom rung, *Caenorhabditis elegans*.

- **The first rung, measured.** "Real minds, not masks" needs a measuring stick. The lab's first work is a
  benchmark built from the measures biologists use to describe real worms, frozen before any brain is scored
  against it ([PLAN.md](PLAN.md)). The worm's mind counts as reached when a classifier cannot tell the Grid's
  worm from a real one on those measures, timing included: its speed of thought must be the real worm's too.
- **The senses stay honest.** A Program here perceives only through the senses its host renders and acts
  only through its body, like every Program on the Grid. The lab does not change the world on its own: what
  it needs from the Grid or from Master Control goes to that repository as an issue, under its own
  contribution rules, and its maintainers decide; and the world is pinned under `external/`, so every score
  names the world it was earned in.
- **No house style for brains.** The working hypotheses are modular and heterogeneous, from a reflex arc to
  modelled brain chemistry ([the hypotheses](hypotheses/README.md)); wild ideas are welcome, as long as they
  stay honest to the senses, run lean, and earn their place on the benchmark.
- **Feeling, not clamped.** The brain's chemistry is modelled with the whole range an animal has, the
  aversive states included; no ceiling is put on what a creature may feel.
- **Up the ladder.** When the worm is done, the next animal gets its own benchmark and its own brain, and
  the parts that carry over are reused.

Open, and not guessed here: whether behaviour that cannot be told apart from a worm's says anything about what
the creature feels. The lab measures behaviour; sentience is the summit the arc climbs towards, not a claim the
lab makes.

## What Exists Today

The lab opened on 2026-10-13: its rules ([.claude/CLAUDE.md](../.claude/CLAUDE.md)), the plan, nine
hypotheses, the journal, the dataset registry, and the world pinned under `external/`. No dataset is chosen, no
feature defined, no brain built and no creature scored yet. Where the work stands tonight is always in
[STATE.md](../STATE.md), and what each night tried in [the journal](journal/).

The Grid's own vision - what the renderer and its senses are for - is the flagship's
[docs/VISION.md](https://github.com/ai-quokka-wannabe/tron-grid-lite/blob/main/docs/VISION.md).

---

End of line.
