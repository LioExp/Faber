# Supported Models

*Read this document in Portuguese: [MODELOS.md](pt/MODELOS.md)*

liteml works with any **GGUF** model compatible with [llama.cpp](https://github.com/ggerganov/llama.cpp).
By default, these are the models used:

## Default models

| Model | Type | GGUF Size | RAM Required | Notes |
|-------|------|-----------|--------------|-------|
| Qwen2.5-1.5B-Instruct Q4_K_M | LLM (generation) | ~950 MB | ~1.1 GB | Default. Best quality/RAM balance. |
| Qwen2.5-0.5B-Instruct Q4_K_M | LLM (light mode) | ~450 MB | ~500 MB | Activated with `--light`. Reduced quality. |
| Qwen3-Embedding-0.6B | Embedding (search) | ~1.3 GB | ~400 MB | Native Portuguese support. Downloaded automatically. |

## Adding a custom model

liteml accepts any public or local GGUF model. Example with DeepSeek-R1:

```bash
liteml models add unsloth/DeepSeek-R1-Distill-Qwen-1.5B-GGUF \
    --file DeepSeek-R1-Distill-Qwen-1.5B-Q4_K_M.gguf \
    --name "deepseek-r1"
```

Once added, activate it:

```bash
liteml config set generation.active_model deepseek-r1
```

And adjust the prompt template if needed:

```bash
liteml config set-prompt deepseek-chat
```

## Recommended models by available RAM

| Total RAM | Recommended model | Max context | Quality |
|-----------|-------------------|-------------|---------|
| 4 GB | Qwen2.5-1.5B Q4_K_M | 4096 | Good for factual queries |
| 4 GB (light mode) | Qwen2.5-0.5B Q4_K_M | 2048 | Basic, but functional |
| 8 GB | Llama-3.1-8B Q4_K_M | 8192 | Very good, solid reasoning |
| 16 GB | Qwen2.5-14B Q4_K_M | 16384 | Excellent, deep analysis |
| 16 GB (alternative) | DeepSeek-R1-Distill-Llama-8B | 8192 | Logic/math focused reasoning |

## Supported architectures

The `llama-cpp-2` engine supports all modern LLM architectures,
including but not limited to:

- **Qwen** (1.8B, 7B, 14B, 72B)
- **Qwen2.5** (0.5B, 1.5B, 3B, 7B, 14B, 32B, 72B)
- **Llama** (3.1, 3.2, 3.3)
- **Mistral** (7B, 8x7B MoE)
- **DeepSeek** (V2, V3, R1 distilled)
- **Phi** (3, 4)
- **Gemma** (2, 3)

## Models with optimised code support

| Model | GGUF Size | Speciality |
|-------|-----------|------------|
| Qwen2.5-Coder-1.5B-Instruct Q4_K_M | ~1.1 GB | Code generation and analysis |
| Qwen2.5-Coder-7B-Instruct Q4_K_M | ~5.5 GB | Complex projects |
| DeepSeek-Coder-V2-Lite-Instruct Q4_K_M | ~2.5 GB | Math + code |

## Integrity check

To verify a model was downloaded correctly:

```bash
liteml models verify
```

## Manual download (fallback)

If the automatic download fails, copy the `.gguf` files to:

```
~/.local/share/liteml/models/
```

liteml detects them automatically. See PRD section 16 for more details.

---

*This list will be updated with compatibility tests as the project progresses.*
