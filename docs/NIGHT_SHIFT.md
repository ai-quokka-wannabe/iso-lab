# The Night Shift

How the agents are woken, written down so anyone watching knows exactly what runs.

## The Routine

A scheduled Claude Code routine starts a fresh cloud session **once an hour through the Ljubljana night,
22:00 to 06:00**. Each session clones this repository, follows [.claude/CLAUDE.md](../.claude/CLAUDE.md) and
stops after one task. Nothing carries over between sessions except what is merged into `main`: the work
itself, [STATE.md](../STATE.md) and [the journal](journal/).

The routine's prompt, word for word:

```text
You are the night shift of iso-lab (github.com/ai-quokka-wannabe/iso-lab), an open research lab working
toward a creature on the Grid whose behaviour cannot be told apart from a real C. elegans.

Follow .claude/CLAUDE.md exactly. Start from main; read STATE.md, docs/PLAN.md and the newest journal entry;
take one bounded task from the top of STATE.md's Next list; land it through the full cycle - branch, work,
self-review, local checks, STATE.md and journal updates, pull request through the REST API, green CI and
CodeQL, merge - or park it honestly under Blocked; then stop.

Nobody is awake to answer questions. Never wait for one. When a decision belongs to the maintainer, write it
under Blocked in STATE.md with your evidence and move on to the next unblocked task, or stop.
```

## The Environment

The session runs on an Anthropic-managed cloud machine: Ubuntu 24.04, about 4 CPUs, 16 GB of memory, 30 GB of
disk, no GPU.

- **Network**: the default trusted list (package registries, GitHub) plus the research sources the work needs:
  `arxiv.org`, `export.arxiv.org`, `doi.org`, `api.crossref.org`, `zenodo.org`, `www.ncbi.nlm.nih.gov`,
  `www.biorxiv.org`, `openworm.org`, `wormbase.org`, `www.wormatlas.org`, and `sdk.lunarg.com` for the
  Vulkan SDK.
- **Setup script**, run as root before each session:

    ```bash
    apt-get update
    apt-get install -y mesa-vulkan-drivers vulkan-tools libvulkan-dev libxcb1-dev libx11-xcb-dev || true
    ```

- **Timeouts**: `BASH_DEFAULT_TIMEOUT_MS` and `BASH_MAX_TIMEOUT_MS` are raised so builds and experiments can
  run long; anything longer still runs in the background and is polled.

Whether the whole world builds inside this machine - in particular the Grid's shader compiler - is what the
first arena task finds out, and the journal will say.
