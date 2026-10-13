# The Night Shift

How a night starts, written down so anyone watching knows exactly what runs.

## The Session

In the evening the maintainer starts a Claude Code session in the cloud, from the Claude app, on this
repository's `main`. The session follows [.claude/CLAUDE.md](../.claude/CLAUDE.md) and works **task after
task until 06:00 Ljubljana time**, each task through the full cycle and its own pull request. The maintainer
can look in on the session and step in; the agents never wait for that. Nothing carries over between nights
except what is merged into `main`: the work itself, [STATE.md](../STATE.md) and [the journal](journal/).

If the session reaches a usage limit it pauses until the maintainer resumes it. Work not yet pushed may be
lost; work already merged never is.

The session's prompt, word for word:

```text
You are the night shift of iso-lab (github.com/ai-quokka-wannabe/iso-lab), an open research lab working
toward a creature on the Grid whose behaviour cannot be told apart from a real C. elegans.

Follow .claude/CLAUDE.md exactly. Work through the night one task at a time: start from main; read STATE.md,
docs/PLAN.md and the newest journal entry; take one bounded task from the top of STATE.md's Next list; land it
through the full cycle - branch, work, self-review, local checks, STATE.md and journal updates, pull request
through the REST API, green CI, verification by a second agent, merge - or park it honestly under Blocked.
Then begin again from main with the next task. Start no new task after 05:00 Ljubljana time
(TZ=Europe/Ljubljana date); finish or park the one in hand, then stop. Stop sooner only when nothing is left
that you can move forward - and not before trying every Blocked item again from a new angle.

Nobody is awake to answer questions. Never wait for one. When a decision belongs to the maintainer, write it
under Blocked in STATE.md with your evidence and move on to the next unblocked task, or stop.
```

## The Environment

The session runs on an Anthropic-managed cloud machine - Ubuntu 24.04, about 4 CPUs, 16 GB of memory, 30 GB
of disk, no GPU - in an environment set up as follows.

- **Network**: the default trusted list (package registries, GitHub) plus the research sources the work needs:
  `arxiv.org`, `export.arxiv.org`, `doi.org`, `api.crossref.org`, `zenodo.org`, `www.ncbi.nlm.nih.gov`,
  `www.biorxiv.org`, `openworm.org`, `wormbase.org`, `www.wormatlas.org`, and `sdk.lunarg.com` for the
  Vulkan SDK; and, for papers' full texts, `pmc.ncbi.nlm.nih.gov`, `eutils.ncbi.nlm.nih.gov`, `europepmc.org`,
  `www.ebi.ac.uk` and `journals.plos.org`.
- **Setup script**, run as root before each session:

    ```bash
    apt-get update
    apt-get install -y mesa-vulkan-drivers vulkan-tools libvulkan-dev libxcb1-dev libx11-xcb-dev || true
    ```

- **Timeouts**: `BASH_DEFAULT_TIMEOUT_MS` and `BASH_MAX_TIMEOUT_MS` are raised so builds and experiments can
  run long; anything longer still runs in the background and is polled.

Whether the whole world builds inside this machine - in particular the Grid's shader compiler - is what the
first arena task finds out, and the journal will say.
