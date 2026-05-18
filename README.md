<img width="1536" height="1024" alt="image" src="https://github.com/user-attachments/assets/d66cb23e-d9b0-4fa1-ad20-a37fc5f05231" />

# liteml
> *Lightweight AI runtime for your offline second brain.*

**liteml** is a single-binary Rust CLI that indexes your Markdown notes (e.g., an [Obsidian](https://obsidian.md) vault), performs multilingual semantic search, and synthesises answers with a local 1.5B parameter language model — all **offline**, on as little as **4 GB of RAM**, without paid APIs or cloud dependencies.

*Read this document in another language: [Portuguese](README.pt.md)*

![Rust](https://img.shields.io/badge/rust-1.75+-orange) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

## Why liteml?

| Problem | liteml |
| :--- | :--- |
| Most local AI tools need 16 GB+ RAM | ✅ Runs comfortably on 4 GB |
| Second‑brain assistants require internet | ✅ Works forever offline after initial model download |
| Embedding models degrade on non‑English text | ✅ Built on Qwen3‑Embedding – strong Portuguese & multilingual support |
| Proprietary, expensive, or privacy‑invasive | ✅ MIT‑licensed, zero telemetry, no API keys |

> *“If my PC can’t run AI, I’ll build one that can.”*

---

## Features

- **🔒 Privacy‑first** – your notes never leave your device.
- **🌐 Multilingual semantic search** – Portuguese, English, and 100+ languages.
- **📝 RAG with citations** – every answer references the exact notes it used.
- **⚡ Incremental indexing** – re‑index only changed files.
- **🪶 Ultra‑light mode** (`--light`) – uses a 0.5B model when RAM is tight.
- **🧩 Bring your own models** – add any llama.cpp‑compatible GGUF file.
- **📦 Single binary** – `cargo install liteml`, no runtime dependencies.

---

## Quick start

### 1. Install system dependencies (Ubuntu/Debian example)

```bash
sudo apt install build-essential cmake curl git
```

### 2. Install Rust (if not already)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 3. Install liteml

```bash
cargo install liteml      # first build compiles llama.cpp (~15 min)
```

### 4. Download models (internet required once)

```bash
liteml models download
```

### 5. Configure your vault

```bash
liteml config set vault_path /home/user/obsidian/vault
```

### 6. Index your notes

```bash
liteml index
```

### 7. Ask a question

```bash
liteml ask "What did I learn about neural networks?"
```

**Output:**
```
[liteml] Retrieving notes… (3 found)
[liteml] Generating answer…
Neural networks are computational systems inspired by the brain [NOTE 1]…
─────────────────────────────────────────
Sources:
  [NOTE 1] Deep Learning Introduction — /vault/ia/dl-intro.md
  [NOTE 2] Backpropagation — /vault/ia/backprop.md
─────────────────────────────────────────
Time: 8.3s · Tokens: 127 · Model: Qwen2.5-1.5B Q4_K_M
```

---

## Commands

| Command | Purpose |
| :--- | :--- |
| `liteml index [--force]` | Walk the vault, compute embeddings, store in SQLite. |
| `liteml ask "<query>" [--light] [--top-k N]` | Semantic search + RAG‑powered answer. |
| `liteml models download` | Fetch default LLM (1.5B Qwen2.5) and embedding model. |
| `liteml models add <repo> --file <gguf>` | Add your own GGUF model. |
| `liteml config set <key> <value>` | Change settings (temperature, vault path, etc.). |
| `liteml status` | View vault size, index freshness, installed models. |

---

## Bringing your own models

liteml works with any llama.cpp‑compatible GGUF model. For example:

```bash
liteml models add unsloth/DeepSeek-R1-Distill-Qwen-1.5B-GGUF \
    --file DeepSeek-R1-Distill-Qwen-1.5B-Q4_K_M.gguf --name "deepseek-r1"
liteml config set generation.active_model deepseek-r1
```

The system checks available RAM before downloading and warns you if a model won’t fit.

---

## Architecture

```
liteml/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── cli.rs           # clap derive definitions
│   ├── config.rs        # JSON config handling
│   ├── db.rs            # SQLite schema & queries
│   ├── indexer.rs       # Markdown → embedding → DB
│   ├── retriever.rs     # Cosine similarity, top‑K
│   ├── generator.rs     # Prompt builder, llama‑cpp‑2 streaming
│   ├── models.rs        # HuggingFace download & verification
│   └── error.rs         # Error types
├── Cargo.toml
└── README.md
```

**Stack:** Rust · `clap` · `fastembed` (Qwen3‑Embedding‑0.6B, Candle backend) · `llama-cpp-2` · `rusqlite` · `hf-hub` · `walkdir`  

**Embedding model:** Qwen3‑Embedding‑0.6B (MMTEB multilingual score 64.33, 512‑dim vectors via Matryoshka).  
**LLM:** Qwen2.5‑1.5B‑Instruct Q4_K_M (~950 MB). Light mode: Qwen2.5‑0.5B‑Instruct Q4_K_M (~450 MB).

---

## RAM budget (4 GB target)

| Scenario | RAM used |
| :--- | :--- |
| `liteml index` (Obsidian open) | ~2.4–3.0 GB |
| `liteml ask` (Obsidian open) | ~2.9–3.5 GB |
| `liteml ask --light` (Obsidian open) | ~2.4–3.0 GB |

On 16 GB machines you can run models up to 13–14B without issues.

---

## Roadmap

- **Phase 1 (MVP)** – Full pipeline: index, retrieve, generate, status, basic config.  
- **Phase 2 (Robustness)** – Incremental indexing, `--light` mode, friendly error messages, first external beta tester.  
- **Phase 3 (Ecosystem)** – `liteml dev` (interactive code assistant), `liteml enrich` (turn URLs into Obsidian notes), TUI, background indexing, publishing on crates.io.

---

## Contributing

Pull requests are welcome! The project is in early development – check the issues page for good first issues. Please open a discussion before adding large features.

---

## License

MIT © LioExp
