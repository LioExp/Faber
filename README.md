<!-- Logo — colocar aqui quando finalizada -->
<!-- <p align="center"><img src="assets/logo.svg" alt="Faber" width="80" /></p> -->

<h1 align="center">FABER</h1>

<p align="center">
  <em>a code agent that runs on the hardware you have</em>
</p>

<p align="center">
  <img alt="built in rust" src="https://img.shields.io/badge/built_in-rust-3C3489?style=flat-square&labelColor=1A1A2E&color=534AB7">
  <img alt="4 GB RAM" src="https://img.shields.io/badge/RAM-4_GB_min-E94560?style=flat-square&labelColor=1A1A2E&color=E94560">
  <img alt="offline" src="https://img.shields.io/badge/offline-first-1D9E75?style=flat-square&labelColor=1A1A2E&color=1D9E75">
  <img alt="license MIT" src="https://img.shields.io/badge/license-MIT-888780?style=flat-square&labelColor=1A1A2E&color=5F5E5A">
</p>

<p align="center">
  <code>cargo install faber</code>
</p>

> read in another idioma [Portuguese](README.pt.md)
---

Most code agents assume 8–16 GB of RAM, a stable connection, and a paid API key.  
On modest hardware, an unstable connection, or without monthly costs — they don't start.

Faber is a terminal code agent written in Rust. It runs a local quantized model.  
**No API key. No internet after setup. No Node.js. One binary under 40 MB.**

It reads files, writes files, applies diffs, and runs shell commands — with permission modes you control. It builds a checklist before executing any complex task and doesn't move on until each step is verifiably complete. It learns from use: after three successful similar tasks, it writes its own skill file and uses it next time.

The benchmark is the product. See [`BENCHMARKS.md`](./BENCHMARKS.md).

---

## vs. the alternatives

|                          | Claude Code | Pi    | Hermes | **Faber** |
|--------------------------|:-----------:|:-----:|:------:|:---------:|
| Works offline            | No          | No    | No     | **Yes**   |
| No API key required      | No          | No    | No     | **Yes**   |
| Runs on 4 GB RAM         | No          | No*   | No     | **Yes**   |
| Learns across sessions   | No          | No    | Yes    | **Yes**   |
| Single binary, no deps   | No          | No    | No     | **Yes**   |
| Open source              | No          | Yes   | Yes    | **Yes**   |

*Pi's process is lightweight but uses external API models — it doesn't run inference locally.

---

## getting started

**1. Install**

```bash
cargo install faber
```

Requires Rust. Install at [rustup.rs](https://rustup.rs) if needed.

**2. Download the model** (one time, ~950 MB)

```bash
faber models download
```

Faber detects your available RAM and recommends the right model automatically.  
No internet after this step.

For offline/air-gapped environments:
```bash
# Download the model file manually, then point Faber to it
faber models add --path ./qwen2.5-1.5b-instruct-q4_k_m.gguf
```

**3. Start working**

```bash
faber "add error handling to this function" src/handler.rs
```

---

## what it does

**Reads, writes, edits, executes** — four base tools that cover most real development tasks.

```
read_file   — reads a file into context
write_file  — creates or overwrites a file
edit_file   — applies a diff with preview before committing
bash        — runs a shell command
```

**Permission modes** — you choose the trust level per session or permanently in config:

| Mode     | Read | Write | Bash              |
|----------|:----:|:-----:|:-----------------:|
| `strict` | Yes  | No    | No                |
| `hybrid` | Yes  | Yes + confirm | Yes + confirm |
| `open`   | Yes  | Yes   | Yes               |

Default is `hybrid`. Switch with `/mode` or set permanently in `~/.config/faber/config.toml`.

**Till Done** — before executing any complex task, Faber generates a checklist of subtasks. It is blocked from progressing to the next subtask until the current one has a verifiable output. No premature success declarations.

**Memory** — decisions and patterns are persisted in `.faber/MEMORY.md` per project, with a hard cap that forces active summarization. It stays useful instead of becoming noise.

**Skills** — after three successful similar tasks, Faber writes a `.faber/skills/SKILL_*.md` file documenting the pattern. It uses that skill on the next similar task. If it finds a better approach, it updates the skill.

**Hooks** — intercept any lifecycle event with Lua scripts. Block dangerous commands, validate writes, automate flows. Hot-reload without restarting the process.

**Plugins** — WASM extensions for complex tools. The model sees only the tool name and description — the logic runs outside AI context, saving tokens for reasoning.

---

## honest limitations

Faber uses a **1.5B parameter model** by default (~950 MB RAM).

It handles well:
- Debugging and tracing errors
- Refactoring individual functions
- Writing and updating tests
- Automating repetitive file and git tasks
- Explaining and documenting existing code

It struggles with:
- Large-scale architectural refactoring across many files
- Complex multi-step reasoning chains (8+ dependent steps)
- Tasks where Claude Code with Sonnet would produce significantly better results

With 8+ GB RAM, switch to a 7B model for noticeably better results on complex tasks:

```bash
faber models download qwen2.5-7b
faber --model qwen2.5-7b "refactor the auth module"
```

The full benchmark results are in [`BENCHMARKS.md`](./BENCHMARKS.md).  
We document what works and what doesn't. You decide if it fits your use case.

---

## extending faber

**Lua hooks** — lightweight scripts for lifecycle events:

```lua
-- .faber/hooks/bash_guard.lua
function on_bash_before(cmd)
  local blocked = {"rm -rf", "mkfs", "dd if="}
  for _, pattern in ipairs(blocked) do
    if cmd:find(pattern) then
      return { block = true, reason = "blocked: " .. pattern }
    end
  end
  return { block = false }
end
```

**Manual skills** — create `.faber/skills/SKILL_custom.md` with any workflow. Faber loads it when the task matches the trigger you define.

**WASM plugins** — complex tools in any language that compiles to WASM. Zero runtime overhead on the model's context window.

---

## project files

| File | Location | Purpose |
|------|----------|---------|
| `FABER.md` | `.faber/` | Project profile: language, conventions, context |
| `MEMORY.md` | `.faber/` | Cross-session memory (capped at 2,000 chars) |
| `ERRORS.md` | `.faber/` | Error log and resolutions (capped at 1,500 chars) |
| `feedback.jsonl` | `.faber/` | Task log used to generate skills |
| `SKILL_*.md` | `.faber/skills/` | Auto-generated and manual skill files |
| `config.toml` | `~/.config/faber/` | Global config: model, mode, thresholds |

---

## roadmap

- [x] PRD v2.0 — architecture defined
- [ ] Phase 0 — repository, build, model download
- [ ] Phase 1 — MVP: agent, tools, hooks, memory, skills
- [ ] Phase 2 — Obsidian vault RAG, LSP integration, scheduled tasks
- [ ] Phase 3 — MCP client, TUI, faber-hub community skills

---

## contributing

Faber is early. The most valuable contributions right now are:

- Running Phase 1 on modest hardware and reporting real results in [`BENCHMARKS.md`](./BENCHMARKS.md)
- Finding tasks where the 1.5B model fails and documenting them clearly
- Writing hooks and skills for your own workflows

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) for guidelines.

---

<p align="center">
  <a href="./BENCHMARKS.md">benchmarks</a> ·
  <a href="./CONTRIBUTING.md">contributing</a> ·
</p>

<p align="center">
  <sub>LioExp · MIT License</sub>
</p>
