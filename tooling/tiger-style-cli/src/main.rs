mod assets;
mod cli;
mod configure;
mod conflicts;
mod detect;
mod doctor;
mod install;

use std::path::Path;

use clap::Parser;
use thiserror::Error;

use cli::{Cli, Commands};

const EXIT_SUCCESS: i32 = 0;
const EXIT_UNEXPECTED: i32 = 1;
const EXIT_VALIDATION: i32 = 2;
const EXIT_CONFLICT: i32 = 3;

#[derive(Debug, Error)]
enum AppError {
    #[error("{0}")]
    Conflict(#[from] conflicts::ConflictError),
    #[error("{0}")]
    Validation(String),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl AppError {
    fn exit_code(&self) -> i32 {
        match self {
            AppError::Conflict(_) => EXIT_CONFLICT,
            AppError::Validation(_) => EXIT_VALIDATION,
            AppError::Unexpected(_) => EXIT_UNEXPECTED,
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let code = match run(cli) {
        Ok(()) => EXIT_SUCCESS,
        Err(err) => {
            print_error(&err);
            err.exit_code()
        }
    };
    std::process::exit(code);
}

fn print_error(err: &AppError) {
    match err {
        AppError::Conflict(conflicts) => {
            eprintln!("{conflicts}");
            for conflict in &conflicts.conflicts {
                eprintln!("\\n-- {} --", conflict.path);
                eprintln!("{}", conflict.preview);
            }
        }
        _ => eprintln!("{err}"),
    }
}

fn run(cli: Cli) -> Result<(), AppError> {
    match cli.command {
        Commands::Install(args) => install::run(&args),
        Commands::Configure(args) => configure::run(&args),
        Commands::Doctor(args) => {
            let report = doctor::run(&args)?;
            if !report.ok {
                return Err(AppError::Validation("doctor checks failed".to_string()));
            }
            Ok(())
        }
        Commands::Bootstrap(args) => {
            let install_args = cli::InstallArgs {
                target: args.target.clone(),
                force: args.force,
                dry_run: args.dry_run,
            };
            install::run(&install_args)?;

            let configure_args = cli::ConfigureArgs {
                target: args.target.clone(),
                manifest_mode: cli::ManifestMode::Autodetect,
                force: args.force,
                dry_run: args.dry_run,
            };
            configure::run(&configure_args)?;

            if args.dry_run {
                println!("DRY-RUN SKIP: doctor");
                return Ok(());
            }

            let doctor_args = cli::DoctorArgs {
                target: args.target,
                strict: false,
                format: cli::OutputFormat::Text,
            };
            let report = doctor::run(&doctor_args)?;
            if !report.ok {
                return Err(AppError::Validation("doctor checks failed".to_string()));
            }
            Ok(())
        }
    }
}

pub(crate) fn ensure_target_exists(target: &Path) -> Result<(), AppError> {
    if target.exists() && target.is_dir() {
        return Ok(());
    }

    Err(AppError::Validation(format!(
        "target path must exist and be a directory: {}",
        target.display()
    )))
}
