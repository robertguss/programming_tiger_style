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

    let agents_path = args.target.join("AGENTS.md");
    let agents_template = assets::agents_template();

    if agents_path.exists() {
        let current = fs::read(&agents_path)
            .with_context(|| format!("failed to read {}", agents_path.display()))?;
        if current != agents_template && !args.force {
            let conflict = conflicts::from_bytes(&agents_path, &current, agents_template);
            return Err(AppError::Conflict(ConflictError::new(vec![conflict])));
        }
    }

    if args.dry_run {
        println!(
            "DRY-RUN WRITE: contracts/ACTIVE_LANGUAGE_CONTRACTS.md (rust={}, python={}, typescript={})",
            status_label(status.rust),
            status_label(status.python),
            status_label(status.typescript)
        );
        if agents_path.exists() {
            println!("DRY-RUN SKIP/OVERWRITE: AGENTS.md");
        } else {
            println!("DRY-RUN CREATE: AGENTS.md");
        }
        return Ok(());
    }

    let manifest_path = args.target.join("contracts/ACTIVE_LANGUAGE_CONTRACTS.md");
    if let Some(parent) = manifest_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }
    fs::write(&manifest_path, render_manifest(status))
        .with_context(|| format!("failed to write {}", manifest_path.display()))?;
    println!("WRITE: contracts/ACTIVE_LANGUAGE_CONTRACTS.md");

    if agents_path.exists() {
        let current = fs::read(&agents_path)
            .with_context(|| format!("failed to read {}", agents_path.display()))?;
        if current == agents_template {
            println!("SKIP (unchanged): AGENTS.md");
        } else {
            fs::write(&agents_path, agents_template)
                .with_context(|| format!("failed to write {}", agents_path.display()))?;
            println!("OVERWRITE: AGENTS.md");
        }
    } else {
        fs::write(&agents_path, agents_template)
            .with_context(|| format!("failed to write {}", agents_path.display()))?;
        println!("CREATE: AGENTS.md");
    }

    Ok(())
}

fn status_label(active: bool) -> &'static str {
    if active {
        "active"
    } else {
        "inactive"
    }
}
