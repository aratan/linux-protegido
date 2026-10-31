use anyhow::Context;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{fs, path::PathBuf};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "linux-protegido", version, about = "Backups seguros para Linux")]
struct Cli {
    #[arg(short, long, default_value = "config/example.toml")]
    config: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Comprueba que el archivo TOML puede leerse.
    ConfigCheck,
    /// Muestra la configuración cargada sin revelar secretos.
    Status,
}

#[derive(Debug, Deserialize)]
struct Config {
    backup: BackupConfig,
    source: SourceConfig,
    repository: RepositoryConfig,
}

#[derive(Debug, Deserialize)]
struct BackupConfig {
    name: String,
    schedule: String,
}

#[derive(Debug, Deserialize)]
struct SourceConfig {
    paths: Vec<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct RepositoryConfig {
    #[serde(rename = "type")]
    kind: String,
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let cli = Cli::parse();
    let raw = fs::read_to_string(&cli.config)
        .with_context(|| format!("No se pudo leer {}", cli.config.display()))?;
    let config: Config = toml::from_str(&raw)
        .with_context(|| format!("Configuración TOML inválida: {}", cli.config.display()))?;

    match cli.command {
        Command::ConfigCheck => {
            tracing::info!(config = %cli.config.display(), "Configuración válida");
            println!("Configuración válida.");
        }
        Command::Status => {
            println!("Proyecto: {}", config.backup.name);
            println!("Planificación: {}", config.backup.schedule);
            println!("Origen(es): {}", config.source.paths.len());
            println!("Repositorio: {} ({})", config.repository.path.display(), config.repository.kind);
        }
    }
    Ok(())
}
