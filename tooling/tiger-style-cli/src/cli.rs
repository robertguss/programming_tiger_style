use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "tiger-style")]
#[command(about = "Tiger Style Contract System v2 installer and validator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Install(InstallArgs),
    Configure(ConfigureArgs),
    Doctor(DoctorArgs),
    Bootstrap(BootstrapArgs),
}

#[derive(Debug, clap::Args)]
pub struct InstallArgs {
    #[arg(long)]
    pub target: PathBuf,
    #[arg(long, default_value_t = false)]
    pub force: bool,
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ManifestMode {
    Autodetect,
    AllInactive,
    AllActive,
}

#[derive(Debug, clap::Args)]
pub struct ConfigureArgs {
    #[arg(long)]
    pub target: PathBuf,
    #[arg(long, value_enum, default_value = "autodetect")]
    pub manifest_mode: ManifestMode,
    #[arg(long, default_value_t = false)]
    pub force: bool,
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, clap::Args)]
pub struct DoctorArgs {
    #[arg(long)]
    pub target: PathBuf,
    #[arg(long, default_value_t = false)]
    pub strict: bool,
    #[arg(long, value_enum, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Debug, clap::Args)]
pub struct BootstrapArgs {
    #[arg(long)]
    pub target: PathBuf,
    #[arg(long, default_value_t = false)]
    pub force: bool,
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}
