use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::{
    fs,
    path::PathBuf,
    process::Command as ProcessCommand,
};
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
    /// Comprueba si Restic está instalado y muestra su versión. No crea backups.
    ResticVersion,
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

fn check_restic_version() -> anyhow::Result<()> {
    tracing::info!("Comprobando disponibilidad de Restic");
    let output = ProcessCommand::new("restic")
        .arg("version")
        .output()
        .context("No se encontró el ejecutable 'restic' en PATH. Instálalo y vuelve a ejecutar 'restic-version'.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(
            "Restic respondió con código {} al consultar la versión: {}",
            output.status,
            stderr.trim()
        );
    }

    let version = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if version.is_empty() {
        bail!("Restic terminó correctamente, pero no devolvió información de versión.");
    }

    println!("Restic disponible: {version}");
    tracing::info!(version = %version, "Restic detectado");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let cli = Cli::parse();

    // Esta comprobación no necesita leer la configuración ni accede al repositorio.
    if let Command::ResticVersion = cli.command {
        return check_restic_version();
    }

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
        Command::ResticVersion => unreachable!("se procesa antes de cargar la configuración"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_accepts_restic_version_command() {
        let cli = Cli::try_parse_from(["linux-protegido", "restic-version"])
            .expect("el subcomando debe reconocerse");
        assert!(matches!(cli.command, Command::ResticVersion));
    }
}
