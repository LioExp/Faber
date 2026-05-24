use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

// ─────────────────────────────────────────────
//  Estrutura principal
// ─────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FaberConfig {
    pub inference: InferenceConfig,
    pub memory:    MemoryConfig,
    pub skills:    SkillsConfig,
    pub security:  SecurityConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InferenceConfig {
    /// Path para o modelo GGUF activo
    pub model_path: Option<PathBuf>,
    /// Pasta onde os modelos estão guardados
    pub models_dir: PathBuf,
    /// Modo de operação: strict | hybrid | open
    pub mode: String,
    /// Modo de inferência: local | api
    pub inference_mode: String,
    /// Threads a usar na inferência (0 = auto)
    pub threads: u32,
    /// Contexto máximo em tokens
    pub context_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryConfig {
    /// Cap do MEMORY.md em caracteres
    pub memory_cap: usize,
    /// Cap do ERRORS.md em caracteres
    pub errors_cap: usize,
    /// Nível de log: metadata | full
    pub log_level: String,
    /// Threshold para criação automática de skills
    pub skill_threshold: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkillsConfig {
    /// Cap de cada skill em caracteres
    pub skill_cap: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityConfig {
    /// Comandos sempre bloqueados (qualquer modo)
    pub blocked_commands: Vec<String>,
}

// ─────────────────────────────────────────────
//  Defaults — valores do PRD
// ─────────────────────────────────────────────

impl Default for FaberConfig {
    fn default() -> Self {
        Self {
            inference: InferenceConfig {
                model_path:     None,
                models_dir:     default_models_dir(),
                mode:           "hybrid".into(),
                inference_mode: "local".into(),
                threads:        0,
                context_size:   4096,
            },
            memory: MemoryConfig {
                memory_cap:      2000,
                errors_cap:      1500,
                log_level:       "metadata".into(),
                skill_threshold: 3,
            },
            skills: SkillsConfig {
                skill_cap: 500,
            },
            security: SecurityConfig {
                blocked_commands: vec![
                    "rm -rf".into(),
                    "mkfs".into(),
                    "dd if=".into(),
                    "chmod 777 /".into(),
                ],
            },
        }
    }
}

// ─────────────────────────────────────────────
//  Paths
// ─────────────────────────────────────────────

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("faber")
        .join("config.toml")
}

fn default_models_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("~/.local/share"))
        .join("faber")
        .join("models")
}

// ─────────────────────────────────────────────
//  Load / Save
// ─────────────────────────────────────────────

/// Carrega a config. Se não existir, cria com defaults e devolve-a.
pub fn load() -> Result<FaberConfig> {
    let path = config_path();

    if !path.exists() {
        let cfg = FaberConfig::default();
        save(&cfg).context("Falhou ao criar config.toml com defaults")?;
        return Ok(cfg);
    }

    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("Não foi possível ler {}", path.display()))?;

    let cfg: FaberConfig = toml::from_str(&raw)
        .with_context(|| format!("config.toml inválido em {}", path.display()))?;

    Ok(cfg)
}

/// Guarda a config em disco.
pub fn save(cfg: &FaberConfig) -> Result<()> {
    let path = config_path();

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Não foi possível criar {}", parent.display()))?;
    }

    let content = toml::to_string_pretty(cfg)
        .context("Falhou ao serializar config")?;

    std::fs::write(&path, content)
        .with_context(|| format!("Não foi possível escrever {}", path.display()))?;

    Ok(())
}

// ─────────────────────────────────────────────
//  Display helpers (usado no CLI config show)
// ─────────────────────────────────────────────

pub fn display(cfg: &FaberConfig) {
    use colored::*;

    println!("  {}  {}", "config:".bold(), config_path().display().to_string().dimmed());
    println!();
    println!("  {} {}",  "[inference]".cyan().bold(), "");
    println!("  modelo:    {}", fmt_opt(&cfg.inference.model_path));
    println!("  models_dir:{}", cfg.inference.models_dir.display());
    println!("  modo:      {}", cfg.inference.mode.yellow());
    println!("  threads:   {}", if cfg.inference.threads == 0 {
        "auto".into()
    } else {
        cfg.inference.threads.to_string()
    });
    println!("  contexto:  {} tokens", cfg.inference.context_size);
    println!();
    println!("  {} ", "[memory]".cyan().bold());
    println!("  memory_cap:      {} chars", cfg.memory.memory_cap);
    println!("  errors_cap:      {} chars", cfg.memory.errors_cap);
    println!("  log_level:       {}", cfg.memory.log_level.yellow());
    println!("  skill_threshold: {} ocorrências", cfg.memory.skill_threshold);
    println!();
    println!("  {}", "[security]".cyan().bold());
    println!("  blocked_commands: {}", cfg.security.blocked_commands.join(", ").dimmed());
}

fn fmt_opt(p: &Option<PathBuf>) -> String {
    match p {
        Some(path) => path.display().to_string(),
        None       => "(não configurado)".into(),
    }
}
