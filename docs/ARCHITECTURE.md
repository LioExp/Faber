# liteml Architecture

*Read this document in Portuguese: [ARQUITETURA.md](pt/ARCHITECTURE.md)*

## Overview

liteml is a Rust CLI binary that turns a folder of Markdown notes into an
offline second brain with AI. The architecture follows three well-defined layers:
**indexing**, **retrieval** and **generation**. None of them depend on each other at runtime,
keeping RAM peaks under control.

## Module diagram

```
liteml/
├── src/
│   ├── main.rs        # Entry point, CLI argument parsing (clap)
│   ├── cli.rs         # Command and flag definitions
│   ├── config.rs      # config.json read/write, validation
│   ├── db.rs          # SQLite connection, schema creation, queries
│   ├── indexer.rs     # Indexing: walkdir → hash → embedding → SQLite
│   ├── retriever.rs   # Retrieval: load embeddings → cosine similarity → top-K
│   ├── generator.rs   # Generation: RAG prompt → llama-cpp-2 → streaming output
│   ├── models.rs      # GGUF model download and verification (hf-hub)
│   ├── error.rs       # Error types, conversions, user-friendly messages
│   └── utils.rs       # Helpers (truncation, formatting, sanitisation)
├── tests/
│   ├── integration_test.rs  # End-to-end flows with example vault
│   ├── retriever_test.rs    # Similarity tests with known vectors
│   └── config_test.rs       # Config read/write tests
├── test_data/
│   ├── notas/               # 50+ Markdown notes in Portuguese
│   └── expected_outputs/    # Expected outputs for regression tests
└── docs/
    ├── pt/                   # Portuguese documentation
    └── (English documentation)
```

## Data flow — Indexing

1. `liteml index` reads `config.json` and opens `index.db` (SQLite).
2. `walkdir` traverses `vault_path` recursively.
3. Filters applied:
   - Only `.md` extension.
   - Excluded directories (`.obsidian`, `.git`, etc.).
   - Files with fewer than `min_word_count` words.
4. For each file:
   - Computes `SHA-256` of the first 1024 bytes.
   - If hash already exists → skip (incremental indexing).
   - If new/modified:
     - Reads content, extracts title.
     - Generates embedding via `fastembed` (batch).
     - Serialises `Vec<f32>` to BLOB.
     - `UPSERT` into `notes` + `embeddings` tables.
5. Progress bar (`indicatif`) updated every batch.
6. Final summary: X indexed, Y skipped, Z errors.

## Data flow — Question (RAG)

1. `liteml ask "query"` checks that `index.db` exists and has data.
2. Generates query embedding (with `instruction_prefix` from config).
3. Loads all embeddings from DB into memory (Vec<f32>).
4. Computes `cosine similarity` between query and every note.
5. Sorts, filters by `min_similarity`, selects `top_k`.
6. Loads content of selected notes from disk.
7. Builds RAG prompt (ChatML template, PRD section 13).
8. Initialises `llama-cpp-2` with the configured GGUF model.
9. Generates response with streaming to terminal.
10. Prints source list (cited notes) and metrics.
11. Unloads LLM model from RAM.

## Design decisions (ADRs)

Full architectural decisions are documented in the
[PRD](PRD.md#21-decisões-arquitecturais-registadas-adrs) (Portuguese only for now).
Summary:

- **Embedding:** Qwen3-Embedding-0.6B via `fastembed` (Candle) — avoids ONNX Runtime bug.
- **Search:** In-memory cosine similarity (up to 5,000 notes) — no `sqlite-vec` dependency.
- **Incremental indexing:** Present from MVP — avoids unnecessary re-indexing.
- **Mutually exclusive operations:** Index and ask never run together — SQLite lock.
- **LLM model:** Qwen2.5-1.5B-Instruct Q4_K_M (default), 0.5B (light). Lazy loading and unloading after use.

## RAM budget

liteml is designed to fit in **4 GB total RAM**, even with Obsidian open.
Every heavy component (embedding model, LLM) is loaded only when needed and
unloaded immediately afterwards. The estimated maximum peak is **3.5 GB**
(normal mode, Obsidian open). See detailed table in the PRD.

## Extensibility

- **Custom models:** `liteml models add` accepts any compatible GGUF.
- **Prompt templates:** Users can swap chat format via config.
- **Knowledge modes:** `strict` (notes only), `hybrid` (notes + pre-training), `open` (model only).
- **Multiple vaults:** Support planned for Phase 2.

---

*This document describes the target architecture. It may evolve during implementation.*
