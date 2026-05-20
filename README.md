<img width="2172" height="724" alt="image" src="https://github.com/user-attachments/assets/8cbedbcd-8809-4f9c-9d56-4642c44e1adc" />

# Faber — The coding agent that scales with your hardware

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange)](https://www.rust-lang.org)

**Everything Pi does. Everything Hermes does. On your hardware, whatever it is.**

> read in another language [Portuguese](README.pt.md)
---

## What it is

A coding agent in your terminal. 4 tools (`read`, `write`, `edit`, `bash`). Local quantized model. Single ~35 MB binary. Offline. Runs on 4 GB of RAM, scales up to whatever your hardware can handle.

---

## Install

```bash
cargo install Faber
liteml models download
liteml dev
```

---

## Quick benchmark

|  | Faber (4 GB) | Pi (16 GB) | Hermes (16 GB) |
|--|---------------|------------|----------------|
| RAM idle | **30 MB** | 144 MB | 256 MB |
| RAM task | **1.1 GB** | 800 MB | 900 MB |
| Binary | **35 MB** | 100 MB | 256 MB |
| Offline | ✅ Native | ⚠️ Partial | ⚠️ Partial |
| Cross‑session memory | ✅ Yes | ❌ | ✅ Yes |

---

## Example

```bash
$ cd ~/projects/auth-service
$ Faber dev
    Faber dev · Rust · 35 MB · RAM free: 14 GB
    Mode: hybrid

> Read src/auth.rs, find the JWT validation bug and fix it.

[read_file] → Bug at line 42. Apply fix? [y/N]: y
[edit_file] → ✅ +3, -1. Tests: 5/5 passed. ✅
> /exit
    MEMORY.md updated.
```

---

## Modes

| Mode | Read | Write | Execute |
|------|------|-------|---------|
| `strict` | ✅ | ❌ | ❌ |
| `hybrid` | ✅ | ✅ | ⚠️ with confirmation |
| `open` | ✅ | ✅ | ✅ |

---

## Extensibility

Lua hooks (10 lines to block `rm -rf`). WASM tools (plugins in any language). All loaded at runtime, no restart needed.

---

## Roadmap

- **Phase 0**: Setup, compilation 🔜
- **Phase 1 (MVP)**: Coding agent, 4 tools, modes, Lua plugins 🎯
- **Phase 2**: Obsidian vault, RAG, native Portuguese
- **Phase 3**: Model Merging, MCP, TUI, `liteml-hub`

---

## Docs

· [Architecture](docs/ARCHITECTURE.md) · [Models](docs/MODELOS.md) · [Contribute](CONTRIBUTING.md)

