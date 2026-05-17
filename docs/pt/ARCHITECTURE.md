# Arquitectura do liteml

*Read this document in English: [ARCHITECTURE.md](../ARCHITECTURE.md)* (em breve)

## Visão geral

O liteml é um binário CLI escrito em Rust que transforma uma pasta de notas Markdown
num segundo cérebro offline com IA. A arquitectura segue três camadas bem definidas:
**indexação**, **recuperação** e **geração**. Nenhuma depende da outra em runtime,
permitindo que o sistema funcione com picos de RAM controlados.

## Diagrama de módulos

```
liteml/
├── src/
│   ├── main.rs        # Ponto de entrada, parsing de argumentos (clap)
│   ├── cli.rs         # Definição dos comandos e flags
│   ├── config.rs      # Leitura/escrita de config.json, validação
│   ├── db.rs          # Conexão SQLite, criação de schema, queries
│   ├── indexer.rs     # Indexação: walkdir → hash → embedding → SQLite
│   ├── retriever.rs   # Busca: carregar embeddings → cosine similarity → top-K
│   ├── generator.rs   # Geração: prompt RAG → llama-cpp-2 → streaming output
│   ├── models.rs      # Download e verificação de modelos GGUF (hf-hub)
│   ├── error.rs       # Tipos de erro, conversões, mensagens amigáveis
│   └── utils.rs       # Funções auxiliares (truncagem, formatação, sanitização)
├── tests/
│   ├── integration_test.rs  # Fluxos completos com vault de exemplo
│   ├── retriever_test.rs    # Testes de similaridade com vetores conhecidos
│   └── config_test.rs       # Testes de leitura/escrita de config
├── test_data/
│   ├── notas/               # 50+ notas Markdown em português
│   └── expected_outputs/    # Saídas esperadas para testes de regressão
└── docs/
    ├── pt/                   # Documentação em português
    └── (documentação em inglês)
```

## Fluxo de dados — Indexação

1. `liteml index` lê `config.json` e abre `index.db` (SQLite).
2. `walkdir` percorre `vault_path` recursivamente.
3. Filtros aplicados:
   - Apenas extensão `.md`.
   - Pastas excluídas (`.obsidian`, `.git`, etc.).
   - Ficheiros com menos de `min_word_count` palavras.
4. Para cada ficheiro:
   - Calcula `SHA-256` dos primeiros 1024 bytes.
   - Se hash já existe → ignora (indexação incremental).
   - Se novo/modificado:
     - Lê conteúdo, extrai título.
     - Gera embedding via `fastembed` (batch).
     - Serializa `Vec<f32>` para BLOB.
     - Faz `UPSERT` na tabela `notes` + `embeddings`.
5. Barra de progresso (`indicatif`) actualizada a cada batch.
6. Resumo final: X indexadas, Y ignoradas, Z erros.

## Fluxo de dados — Pergunta (RAG)

1. `liteml ask "query"` verifica que `index.db` existe e tem dados.
2. Gera embedding da query (com `instruction_prefix` do config).
3. Carrega todos os embeddings da BD para memória (Vec<f32>).
4. Calcula `cosine similarity` entre a query e cada nota.
5. Ordena, filtra por `min_similarity`, selecciona `top_k`.
6. Carrega conteúdo das notas seleccionadas do disco.
7. Constrói prompt RAG (template ChatML, secção 13 do PRD).
8. Inicializa `llama-cpp-2` com o modelo GGUF configurado.
9. Gera resposta com streaming para o terminal.
10. Imprime lista de fontes (notas citadas) e métricas.
11. Descarrega o modelo LLM da RAM.

## Decisões de design (ADRs)

As decisões arquitecturais completas estão documentadas no
[PRD](PRD.md#21-decisões-arquitecturais-registadas-adrs). Em resumo:

- **Embedding:** Qwen3-Embedding-0.6B via `fastembed` (Candle) — evita bug do ONNX Runtime.
- **Busca:** Cosine similarity em memória (até 5.000 notas) — sem dependência de `sqlite-vec`.
- **Indexação incremental:** Presente desde o MVP — evita re-indexação desnecessária.
- **Operações mutuamente exclusivas:** Index e ask nunca correm juntos — lock no SQLite.
- **Modelo LLM:** Qwen2.5-1.5B-Instruct Q4_K_M (padrão), 0.5B (leve). Carregamento lazy e descarregamento após uso.

## Orçamento de RAM

O liteml foi desenhado para caber em **4 GB de RAM total**, mesmo com o Obsidian
aberto. Cada componente pesado (modelo de embedding, LLM) é carregado apenas
quando necessário e descarregado imediatamente a seguir. O pico máximo estimado
é **3.5 GB** (modo normal, Obsidian aberto). Ver tabela detalhada no PRD.

## Extensibilidade

- **Modelos personalizados:** `liteml models add` aceita qualquer GGUF compatível.
- **Templates de prompt:** O utilizador pode trocar o formato de chat via config.
- **Modos de conhecimento:** `strict` (apenas notas), `hybrid` (notas + pré-treino), `open` (apenas modelo).
- **Múltiplos vaults:** Suporte planeado para Fase 2.

---

*Este documento descreve a arquitectura alvo. Pode evoluir durante a implementação.*
