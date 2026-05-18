<img width="1536" height="1024" alt="image" src="https://github.com/user-attachments/assets/58e4fb91-f9eb-4af3-939e-9b782aedc8fb" />

# liteml — O teu segundo cérebro offline com IA
> **"Se o meu PC não roda IA, vou construir uma que rode."**

Um binário CLI em Rust que indexa notas Markdown (vaults do Obsidian), faz busca semântica multilingue e sintetiza respostas com um modelo de linguagem de 1.5B parâmetros.  
Tudo **offline**, em **4 GB de RAM**, sem APIs pagas, sem dependência de cloud.

[![CI](https://github.com/LioExp/LiteML/actions/workflows/ci.yml/badge.svg)](https://github.com/LioExp/LiteML/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

*Leia este documento noutras línguas: [English](README.md)*

---

## 🧠 O que é o liteml?

Imagina um **NotebookLM pessoal**, mas que corre **apenas no teu computador**, sem internet, e que lê **apenas as tuas notas do Obsidian** (ou qualquer pasta de Markdown).  
Fazes uma pergunta em linguagem natural e o liteml:

- Procura nas tuas notas com **busca semântica multilingue** (funciona em português).
- Gera uma resposta sintetizada, **citando sempre a nota de origem**.
- Funciona **completamente offline** depois de descarregares os modelos uma vez.
- Não envia um único byte para a nuvem.

---

## 📦 Compatibilidade

| Hardware | RAM | Funciona? | Modelo padrão | Modo leve |
|----------|-----|-----------|---------------|-----------|
| PC modesto (4 GB) | 4 GB | ✅ Sim (com Obsidian aberto) | Qwen2.5‑1.5B Q4 (950 MB) | Qwen2.5‑0.5B (450 MB) |
| Portátil escolar (8 GB) | 8 GB | ✅ Confortável | 1.5B ou até Llama‑3.1‑8B | – |
| Desktop dev (16 GB) | 16 GB | ✅ Luxo | Pode correr 14B | – |

> 🔧 **CPU-only** (x86_64 com AVX2). Funciona em Linux (Ubuntu, Arch, Hyprland). Windows e macOS ainda não testados.

---

## ⚡ Instalação rápida

### Pré-requisitos (Ubuntu/Debian)

```bash
sudo apt install build-essential cmake git curl
```

### Instalar o liteml (do crates.io em breve)

```bash
cargo install liteml
```

> 📦 O primeiro build demora 10–20 minutos (compila o llama.cpp a partir do código). Os builds seguintes são incrementais e rápidos.

### Primeiros passos

```bash
# 1. Descarrega os modelos (requer internet, só uma vez)
liteml models download

# 2. Define a pasta das tuas notas
liteml config set vault_path ~/obsidian/vault

# 3. Indexa o vault
liteml index

# 4. Faz uma pergunta
liteml ask "O que aprendi sobre redes neuronais?"
```

Depois disto podes desligar o WiFi para sempre. O liteml funciona totalmente offline.

---

## 🖥️ Interface CLI

```
liteml index               # Indexa as notas
liteml ask "pergunta"      # Responde com base nas notas
liteml ask "..." --light   # Modo leve (0.5B) para RAM apertada
liteml status              # Estado do índice e modelos
liteml config show         # Configuração atual
liteml models list         # Modelos instalados
liteml enrich <URL>        # Transforma vídeos/artigos em notas (requer internet)
liteml dev                 # Assistente de código offline (modo interativo)
```

---

## 🌍 Exemplos reais de uso

### 1. O programador offline (4 GB de RAM)

```bash
# O Lio está sem internet, mas quer consultar as suas notas de programação.
lio@pc:~$ liteml ask "Como implementar uma fila em C?"
    [liteml] Recuperando notas... (2 encontradas)
    ...
    De acordo com as tuas notas, usas um array circular com ponteiros head/tail [NOTA 1].
```

### 2. A investigadora biomédica (16 GB de RAM)

```bash
# A Ana alimentou o vault com 100 papers. Agora pergunta offline.
ana@pc:~$ liteml ask "Mecanismo de ação do paracetamol segundo os meus artigos"
    [liteml] Recuperando notas... (4 encontradas)
    ...
    [NOTA 1] Estudo de 2022, [NOTA 2] Revisão de 2023.
```

### 3. Enriquecer o vault com um vídeo do YouTube (em casa, com internet)

```bash
pedro@laptop:~$ liteml enrich "https://youtube.com/watch?v=abc" --output "Rust"
    Pré-visualização da nota: ...
    Deseja guardar? [S]: s
    Nota guardada e indexada.
```

### 4. Assistente de código (inspirado no Claude Code)

```bash
pedro@laptop:~$ cd meu-projeto && liteml dev
    liteml dev · modelo: qwen-coder · offline · RAM livre: 11 GB
> Adiciona uma função que conta palavras.
    [lendo src/lib.rs...] Sugestão: ...
    Aplicar? [s/N]: s
> /run cargo test
    Todos os testes passaram.
> /exit
```

---

## 🔧 Como funciona (arquitetura resumida)

```
Ficheiros .md (vault)  →  Indexer (fastembed)  →  SQLite (embeddings)
                              │
Pergunta do utilizador  →  Retriever (cosine similarity)  →  Top‑K notas
                              │
Notas + pergunta  →  Generator (llama-cpp-2 + Qwen2.5‑1.5B)  →  Resposta com citações
```

- **Embedding**: Qwen3‑Embedding‑0.6B (multilingue, score MMTEB 64.33)
- **LLM**: Qwen2.5‑1.5B‑Instruct Q4_K_M (GGUF, 950 MB)
- **Motor de inferência**: llama.cpp (via crate `llama-cpp-2`)
- **Base de dados**: SQLite com vectors armazenados como BLOBs (512 dims)
- **Tudo offline** após o download dos modelos (~2.3 GB uma vez).

---

## 📂 Modelos suportados

O liteml aceita qualquer modelo **GGUF** compatível com llama.cpp. Podes instalar os teus próprios:

```bash
liteml models add unsloth/DeepSeek-R1-Distill-Qwen-1.5B-GGUF \
    --file DeepSeek-R1-Distill-Qwen-1.5B-Q4_K_M.gguf \
    --name "deepseek-r1"
liteml config set generation.active_model deepseek-r1
```

Vê a lista completa de modelos compatíveis em [docs/pt/MODELOS.md](docs/pt/MODELOS.md).

---

## 🧪 Modos de conhecimento

| Modo | Descrição |
|------|-----------|
| `strict` (padrão) | Responde **apenas com as tuas notas**. Sem alucinações. Se não souber, cala‑se. |
| `hybrid` | Usa as notas quando existem; se não, usa conhecimento do modelo e avisa. |
| `open` | Apenas o modelo (sem consultar o vault). Útil para perguntas genéricas offline. |

```bash
liteml mode hybrid   # ativa o modo híbrido
liteml mode strict   # volta ao modo seguro
```

---

## 📜 Licença & Privacidade

- **Licença:** MIT
- **Privacidade total:** Nenhum dado sai do teu computador. Não há telemetria, não há APIs, não há cloud.
- **Modelos:** Descarregados uma vez do HuggingFace. Depois, o WiFi pode ser desligado para sempre.

---

## 🚧 Roadmap (fases)

- [x] **Fase 0** — Setup e compilação
- [ ] **Fase 1** — MVP (fluxo completo em 50 notas)
- [ ] **Fase 2** — Robustez (múltiplos vaults, testes com 500 notas)
- [ ] **Fase 3** — Ecossistema (modo dev, enriquecimento, TUI opcional)

---

## 🤝 Contribuir

O liteml é um projeto comunitário. Se tens um PC com 4 GB e queres testar, ou queres ajudar com código, traduções, ou testes, vê [CONTRIBUTING.md](CONTRIBUTING.md).

---

## 📖 Documentação completa

- [PRD completo (português)](docs/pt/PRD.md)
- [Arquitetura (português)](docs/pt/ARCHITECTURE.md)
- [Modelos suportados](docs/pt/MODELOS.md)
- [Guia de contribuição](CONTRIBUTING.md)
- [Código de Conduta](CODE_OF_CONDUCT.md)
- [Política de Segurança](SECURITY.md)
- [Suporte](SUPPORT.md)

---

