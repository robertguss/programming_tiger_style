use std::fs;
use std::path::Path;

use anyhow::Context;

use crate::assets;
use crate::cli::InstallArgs;
use crate::conflicts;
use crate::conflicts::ConflictError;
use crate::{ensure_target_exists, AppError};

pub fn run(args: &InstallArgs) -> Result<(), AppError> {
    ensure_target_exists(&args.target)?;

    let assets = assets::install_assets();
    let mut conflict_list = Vec::new();

    for asset in assets {
        let destination = args.target.join(&asset.relative_path);
        let existing = if destination.exists() {
            Some(
                fs::read(&destination)
                    .with_context(|| format!("failed to read {}", destination.display()))?,
            )
        } else {
            None
        };

        match existing {
            Some(current) if current == asset.contents => {
                println!("SKIP (unchanged): {}", asset.relative_path);
            }
            Some(current) if !args.force => {
                conflict_list.push(conflicts::from_bytes(
                    &destination,
                    &current,
                    &asset.contents,
                ));
            }
            Some(_) => {
                write_asset(
                    &destination,
                    &asset.contents,
                    asset.executable,
                    args.dry_run,
                )?;
                log_action("OVERWRITE", &asset.relative_path, args.dry_run);
            }
            None => {
                write_asset(
                    &destination,
                    &asset.contents,
                    asset.executable,
                    args.dry_run,
                )?;
                log_action("CREATE", &asset.relative_path, args.dry_run);
            }
        }
    }

    if !conflict_list.is_empty() {
        return Err(AppError::Conflict(ConflictError::new(conflict_list)));
    }

    Ok(())
}

fn write_asset(
    path: &Path,
    contents: &[u8],
    executable: bool,
    dry_run: bool,
) -> Result<(), AppError> {
    if dry_run {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }

    fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))?;

    #[cfg(unix)]
    if executable {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)
            .with_context(|| format!("failed to stat {}", path.display()))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)
            .with_context(|| format!("failed to set permissions on {}", path.display()))?;
    }

    Ok(())
}

fn log_action(action: &str, relative_path: &str, dry_run: bool) {
    if dry_run {
        println!("DRY-RUN {action}: {relative_path}");
    } else {
        println!("{action}: {relative_path}");
    }
}
