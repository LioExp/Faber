use clap::{Parser, Subcommand, ValueEnum};
use colored::*;

// ─────────────────────────────────────────────
//  Tipos públicos
// ─────────────────────────────────────────────

#[derive(Debug, Clone, ValueEnum)]
pub enum Mode {
    Strict,
    Hybrid,
    Open,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Strict => write!(f, "strict"),
            Mode::Hybrid => write!(f, "hybrid"),
            Mode::Open   => write!(f, "open"),
        }
    }
}

// ─────────────────────────────────────────────
//  CLI principal
// ─────────────────────────────────────────────

/// Faber — o agente de código que escala com o teu hardware.
/// Offline. Frugal. Extensível.
#[derive(Parser, Debug)]
#[command(
    name    = "faber",
    version = env!("CARGO_PKG_VERSION"),
    author  = "LioExp",
    about   = "Agente de código local. Sem cloud. Sem custos por token.",
    long_about = None,
    after_help = "EXEMPLOS:\n  faber                         Abre o REPL interativo\n  faber 'explica este ficheiro' main.rs\n                                Executa tarefa pontual\n  faber models list             Lista modelos disponíveis\n  faber models download qwen    Descarrega modelo padrão",
)]
pub struct Cli {
    /// Modo de operação (padrão: hybrid)
    #[arg(long, value_enum, global = true)]
    pub mode: Option<Mode>,

    /// Modelo GGUF a usar nesta sessão
    #[arg(long, global = true)]
    pub model: Option<String>,

    /// Mostrar informação de debug (tokens, RAM, latência)
    #[arg(long, global = true, default_value_t = false)]
    pub debug: bool,

    /// Tarefa a executar (modo não-interativo)
    #[arg(trailing_var_arg = true)]
    pub task: Vec<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Gerir modelos GGUF locais
    Models {
        #[command(subcommand)]
        action: ModelAction,
    },

    /// Ver e editar a configuração
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Mostrar estado da memória e skills do projecto atual
    Status,

    /// Limpar contexto de sessão (não apaga memória)
    Clear,
}

#[derive(Subcommand, Debug)]
pub enum ModelAction {
    /// Listar modelos instalados
    List,
    /// Descarregar um modelo de HuggingFace
    Download {
        /// Nome do modelo (ex: qwen-coder-1.5b, nanbeige-3b, agent-nano-2b)
        name: String,
    },
    /// Adicionar modelo a partir de ficheiro local
    Add {
        /// Caminho para o ficheiro .gguf
        #[arg(long)]
        path: String,
    },
    /// Remover um modelo instalado
    Remove {
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Mostrar configuração atual
    Show,
    /// Editar configuração no editor padrão
    Edit,
    /// Repor configuração padrão
    Reset,
}

// ─────────────────────────────────────────────
//  Entry point
// ─────────────────────────────────────────────

pub fn run(args: Cli) {
    print_banner();

    let mode = args.mode.as_ref()
        .map(|m| m.to_string())
        .unwrap_or_else(|| "hybrid".to_string());

    match &args.command {
        Some(Commands::Models { action }) => handle_models(action),
        Some(Commands::Config { action }) => handle_config(action),
        Some(Commands::Status)           => handle_status(),
        Some(Commands::Clear)            => handle_clear(),
        None => {
            if args.task.is_empty() {
                // Modo REPL interativo
                println!("{}", format!("  modo: {}  |  modelo: {}  |  debug: {}",
                    mode.cyan(),
                    args.model.as_deref().unwrap_or("qwen-coder-1.5b").yellow(),
                    if args.debug { "on".green() } else { "off".dimmed() }
                ));
                println!();
                println!("  {}", "REPL ainda não implementado (Fase 0 — em curso)".dimmed());
                println!("  {}", "→ Próximo: faber models download qwen-coder-1.5b".dimmed());
            } else {
                // Modo tarefa pontual: faber 'faz X' ficheiro.rs
                let task = args.task.join(" ");
                println!("  {} {}", "tarefa:".dimmed(), task.white());
                println!("  {}", "Inferência ainda não disponível (Fase 0)".dimmed());
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Handlers de subcomandos
// ─────────────────────────────────────────────

fn handle_models(action: &ModelAction) {
    match action {
        ModelAction::List => {
            println!("{}", "  modelos instalados:".bold());
            // TODO Fase 0: ler ~/.local/share/faber/models/
            println!("  {}", "(nenhum modelo instalado ainda)".dimmed());
            println!();
            println!("  {}", "→ faber models download qwen-coder-1.5b".cyan());
        }
        ModelAction::Download { name } => {
            println!("  {} {}", "a descarregar:".bold(), name.yellow());
            println!("  {}", "Download ainda não implementado (Fase 0)".dimmed());
            println!("  URL: https://huggingface.co/Qwen/Qwen2.5-Coder-1.5B-Instruct-GGUF");
        }
        ModelAction::Add { path } => {
            println!("  {} {}", "a adicionar modelo:".bold(), path.yellow());
            println!("  {}", "Ainda não implementado (Fase 0)".dimmed());
        }
        ModelAction::Remove { name } => {
            println!("  {} {}", "a remover:".bold(), name.red());
            println!("  {}", "Ainda não implementado (Fase 0)".dimmed());
        }
    }
}

fn handle_config(action: &ConfigAction) {
    match action {
        ConfigAction::Show => {
            match crate::config::load() {
                Ok(cfg) => crate::config::display(&cfg),
                Err(e)  => eprintln!("  erro ao ler config: {}", e),
            }
        }
        ConfigAction::Edit => {
            let path = crate::config::config_path();
            // Garante que o ficheiro existe antes de abrir
            let _ = crate::config::load();
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".into());
            let status = std::process::Command::new(&editor)
                .arg(&path)
                .status();
            match status {
                Ok(_)  => println!("  {}", "Config guardada.".green()),
                Err(_) => println!("  config em: {}", path.display()),
            }
        }
        ConfigAction::Reset => {
            let cfg = crate::config::FaberConfig::default();
            match crate::config::save(&cfg) {
                Ok(_)  => println!("  {}", "Config reposta para defaults.".green()),
                Err(e) => eprintln!("  erro: {}", e),
            }
        }
    }
}

fn handle_status() {
    println!("{}", "  estado do projecto:".bold());
    println!("  {}", "MEMORY.md:    não encontrado (.faber/ não inicializado)".dimmed());
    println!("  {}", "Skills:       0".dimmed());
    println!("  {}", "Tarefa ativa: nenhuma".dimmed());
}

fn handle_clear() {
    println!("  {}", "Contexto de sessão limpo.".green());
    println!("  {}", "Memória e skills preservadas.".dimmed());
}

// ─────────────────────────────────────────────
//  Banner
// ─────────────────────────────────────────────

fn print_banner() {
    println!();
    println!("  {}  {}", "faber".bold().red(), format!("v{}", env!("CARGO_PKG_VERSION")).dimmed());
    println!("  {}", "o agente de código que escala com o teu hardware".dimmed());
    println!();
}
