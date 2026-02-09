use crate::cli::ConfigureArgs;
use crate::{ensure_target_exists, AppError};

pub fn run(args: &ConfigureArgs) -> Result<(), AppError> {
    ensure_target_exists(&args.target)?;
    Err(AppError::Validation(
        "configure command is not implemented yet".to_string(),
    ))
}
