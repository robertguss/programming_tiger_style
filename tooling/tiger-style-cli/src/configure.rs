use std::fs;

use anyhow::Context;

use crate::assets;
use crate::cli::{ConfigureArgs, ManifestMode};
use crate::conflicts;
use crate::conflicts::ConflictError;
use crate::detect::{autodetect_languages, render_manifest, LanguageStatus};
use crate::{ensure_target_exists, AppError};

pub fn run(args: &ConfigureArgs) -> Result<(), AppError> {
    ensure_target_exists(&args.target)?;

    let status = match args.manifest_mode {
        ManifestMode::Autodetect => autodetect_languages(&args.target)?,
        ManifestMode::AllInactive => LanguageStatus::all(false),
        ManifestMode::AllActive => LanguageStatus::all(true),
    };

    let manifest_bytes = render_manifest(status).into_bytes();
    let manifest_baseline = assets::manifest_template();
    let manifest_path = args.target.join("contracts/ACTIVE_LANGUAGE_CONTRACTS.md");
    let agents_path = args.target.join("AGENTS.md");
    let agents_template = assets::agents_template();
    let mut conflicts_found = Vec::new();

    let manifest_action = if manifest_path.exists() {
        let current = fs::read(&manifest_path)
            .with_context(|| format!("failed to read {}", manifest_path.display()))?;
        if current == manifest_bytes {
            ConfigureAction::Skip
        } else if current == manifest_baseline || args.force {
            ConfigureAction::Overwrite
        } else {
            conflicts_found.push(conflicts::from_bytes(
                &manifest_path,
                &current,
                &manifest_bytes,
            ));
            ConfigureAction::Skip
        }
    } else {
        ConfigureAction::Create
    };

    let agents_action = if agents_path.exists() {
        let current = fs::read(&agents_path)
            .with_context(|| format!("failed to read {}", agents_path.display()))?;
        if current == agents_template {
            ConfigureAction::Skip
        } else if args.force {
            ConfigureAction::Overwrite
        } else {
            conflicts_found.push(conflicts::from_bytes(
                &agents_path,
                &current,
                agents_template,
            ));
            ConfigureAction::Skip
        }
    } else {
        ConfigureAction::Create
    };

    if !conflicts_found.is_empty() {
        return Err(AppError::Conflict(ConflictError::new(conflicts_found)));
    }

    if args.dry_run {
        log_manifest_action(&manifest_action, status);
        log_agents_action(&agents_action);
        return Ok(());
    }

    match manifest_action {
        ConfigureAction::Skip => {
            println!("SKIP (unchanged): contracts/ACTIVE_LANGUAGE_CONTRACTS.md")
        }
        ConfigureAction::Create | ConfigureAction::Overwrite => {
            if let Some(parent) = manifest_path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create directory {}", parent.display()))?;
            }
            fs::write(&manifest_path, &manifest_bytes)
                .with_context(|| format!("failed to write {}", manifest_path.display()))?;
            match manifest_action {
                ConfigureAction::Create => {
                    println!("CREATE: contracts/ACTIVE_LANGUAGE_CONTRACTS.md")
                }
                ConfigureAction::Overwrite => {
                    println!("OVERWRITE: contracts/ACTIVE_LANGUAGE_CONTRACTS.md")
                }
                ConfigureAction::Skip => {}
            }
        }
    }

    match agents_action {
        ConfigureAction::Skip => {
            println!("SKIP (unchanged): AGENTS.md");
        }
        ConfigureAction::Overwrite => {
            fs::write(&agents_path, agents_template)
                .with_context(|| format!("failed to write {}", agents_path.display()))?;
            println!("OVERWRITE: AGENTS.md");
        }
        ConfigureAction::Create => {
            fs::write(&agents_path, agents_template)
                .with_context(|| format!("failed to write {}", agents_path.display()))?;
            println!("CREATE: AGENTS.md");
        }
    }

    Ok(())
}

fn log_manifest_action(action: &ConfigureAction, status: LanguageStatus) {
    let details = format!(
        "(rust={}, python={}, typescript={})",
        status_label(status.rust),
        status_label(status.python),
        status_label(status.typescript)
    );
    match action {
        ConfigureAction::Skip => {
            println!("DRY-RUN SKIP: contracts/ACTIVE_LANGUAGE_CONTRACTS.md {details}")
        }
        ConfigureAction::Create => {
            println!("DRY-RUN CREATE: contracts/ACTIVE_LANGUAGE_CONTRACTS.md {details}")
        }
        ConfigureAction::Overwrite => {
            println!("DRY-RUN OVERWRITE: contracts/ACTIVE_LANGUAGE_CONTRACTS.md {details}")
        }
    }
}

fn log_agents_action(action: &ConfigureAction) {
    match action {
        ConfigureAction::Skip => println!("DRY-RUN SKIP: AGENTS.md"),
        ConfigureAction::Create => println!("DRY-RUN CREATE: AGENTS.md"),
        ConfigureAction::Overwrite => println!("DRY-RUN OVERWRITE: AGENTS.md"),
    }
}

fn status_label(active: bool) -> &'static str {
    if active {
        "active"
    } else {
        "inactive"
    }
}

enum ConfigureAction {
    Skip,
    Create,
    Overwrite,
}
