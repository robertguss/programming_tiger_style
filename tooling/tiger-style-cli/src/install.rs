use crate::cli::InstallArgs;
use crate::{ensure_target_exists, AppError};

pub fn run(args: &InstallArgs) -> Result<(), AppError> {
    ensure_target_exists(&args.target)?;
    Err(AppError::Validation(
        "install command is not implemented yet".to_string(),
    ))
}
