<img width="1536" height="1024" alt="image" src="https://github.com/user-attachments/assets/58e4fb91-f9eb-4af3-939e-9b782aedc8fb" />

# liteml — O agente de código que escala com o teu hardware

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange)](https://www.rust-lang.org)

**Tudo o que o Pi faz. Tudo o que o Hermes faz. No teu hardware, seja ele qual for.**

---

## O que é

Um agente de código no terminal. 4 ferramentas (`read`, `write`, `edit`, `bash`). Modelo local quantizado. Binário único de ~35 MB. Offline. Corre em 4 GB de RAM, escala até onde o teu hardware aguentar.

---

## Instalação

```bash
cargo install liteml
liteml models download
liteml dev
```

---

## Benchmark rápido

|  | liteml (4 GB) | Pi (16 GB) | Hermes (16 GB) |
|--|---------------|------------|----------------|
| RAM idle | **30 MB** | 144 MB | 256 MB |
| RAM tarefa | **1.1 GB** | 800 MB | 900 MB |
| Binário | **35 MB** | 100 MB | 256 MB |
| Offline | ✅ Nativo | ⚠️ Parcial | ⚠️ Parcial |
| Memória cross-session | ✅ Sim | ❌ | ✅ Sim |

---

## Exemplo

```bash
$ cd ~/projetos/auth-service
$ liteml dev
    liteml dev · Rust · 35 MB · RAM livre: 14 GB
    Modo: hybrid

> Lê o src/auth.rs, encontra o bug de validação JWT e corrige.

[read_file] → Bug na linha 42. Corrigir? [s/N]: s
[edit_file] → ✅ +3, -1. Testes: 5/5 passaram. ✅
> /exit
    MEMORY.md atualizado.
```

---

## Modos

| Modo | Leitura | Escrita | Execução |
|------|---------|---------|----------|
| `strict` | ✅ | ❌ | ❌ |
| `hybrid` | ✅ | ✅ | ⚠️ com confirmação |
| `open` | ✅ | ✅ | ✅ |

---

## Extensibilidade

Hooks Lua (10 linhas para bloquear `rm -rf`). Tools WASM (plugins em qualquer linguagem). Tudo carregado em runtime, sem reiniciar.

---

## Roadmap

- **Fase 0**: Setup, compilação 🔜
- **Fase 1 (MVP)**: Agente de código, 4 ferramentas, modos, plugins Lua 🎯
- **Fase 2**: Vault Obsidian, RAG, português nativo
- **Fase 3**: Model Merging, MCP, TUI, `liteml-hub`

---

## Documentação

[PRD completo](docs/PRD.md) · [Arquitetura](docs/ARCHITECTURE.md) · [Modelos](docs/MODELOS.md) · [Contribuir](CONTRIBUTING.md)

---

**Feito com ❤️ em Angola. O teu agente de código, offline, sem desculpas.**
```
