# Modelos suportados pelo liteml

*Read this document in English: [MODELOS.md](../MODELOS.md)* (em breve)

O liteml funciona com qualquer modelo **GGUF** compatível com [llama.cpp](https://github.com/ggerganov/llama.cpp).
Por defeito, são estes os modelos usados:

## Modelos padrão

| Modelo | Tipo | Tamanho GGUF | RAM necessária | Nota |
|--------|------|-------------|----------------|------|
| Qwen2.5-1.5B-Instruct Q4_K_M | LLM (geração) | ~950 MB | ~1.1 GB | Padrão. Melhor equilíbrio qualidade/RAM. |
| Qwen2.5-0.5B-Instruct Q4_K_M | LLM (modo leve) | ~450 MB | ~500 MB | Ativado com `--light`. Qualidade reduzida. |
| Qwen3-Embedding-0.6B | Embedding (busca) | ~1.3 GB | ~400 MB | Suporte nativo a Português. Descarregado automaticamente. |

## Adicionar um modelo personalizado

O liteml aceita qualquer modelo GGUF público ou local. Exemplo com DeepSeek-R1:

```bash
liteml models add unsloth/DeepSeek-R1-Distill-Qwen-1.5B-GGUF \
    --file DeepSeek-R1-Distill-Qwen-1.5B-Q4_K_M.gguf \
    --name "deepseek-r1"
```

Depois de adicionado, ativa-o:

```bash
liteml config set generation.active_model deepseek-r1
```

E ajusta o template de prompt se necessário:

```bash
liteml config set-prompt deepseek-chat
```

## Modelos recomendados por RAM disponível

| RAM total | Modelo recomendado | Contexto máximo | Qualidade |
|-----------|--------------------|-----------------|-----------|
| 4 GB | Qwen2.5-1.5B Q4_K_M | 4096 | Boa para perguntas factuais |
| 4 GB (modo leve) | Qwen2.5-0.5B Q4_K_M | 2048 | Básica, mas funcional |
| 8 GB | Llama-3.1-8B Q4_K_M | 8192 | Muito boa, raciocínio sólido |
| 16 GB | Qwen2.5-14B Q4_K_M | 16384 | Excelente, análises profundas |
| 16 GB (alternativa) | DeepSeek-R1-Distill-Llama-8B | 8192 | Foco em raciocínio lógico/matemática |

## Arquitecturas suportadas

O motor `llama-cpp-2` suporta todas as arquitecturas modernas de LLMs,
incluindo mas não limitado a:

- **Qwen** (1.8B, 7B, 14B, 72B)
- **Qwen2.5** (0.5B, 1.5B, 3B, 7B, 14B, 32B, 72B)
- **Llama** (3.1, 3.2, 3.3)
- **Mistral** (7B, 8x7B MoE)
- **DeepSeek** (V2, V3, R1 destilados)
- **Phi** (3, 4)
- **Gemma** (2, 3)

## Modelos com suporte otimizado a código

| Modelo | Tamanho GGUF | Especialidade |
|--------|-------------|---------------|
| Qwen2.5-Coder-1.5B-Instruct Q4_K_M | ~1.1 GB | Geração e análise de código |
| Qwen2.5-Coder-7B-Instruct Q4_K_M | ~5.5 GB | Projectos complexos |
| DeepSeek-Coder-V2-Lite-Instruct Q4_K_M | ~2.5 GB | Matemática + código |

## Verificação de integridade

Para verificar se um modelo foi descarregado corretamente:

```bash
liteml models verify
```

## Download manual (fallback)

Se o download automático falhar, copia os ficheiros `.gguf` para:

```
~/.local/share/liteml/models/
```

O liteml detecta automaticamente. Ver a secção 16 do PRD para mais detalhes.

---

*Esta lista será atualizada com testes de compatibilidade à medida que o projecto avança.*
