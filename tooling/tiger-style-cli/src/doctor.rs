use serde::Serialize;

use crate::cli::{DoctorArgs, OutputFormat};
use crate::{ensure_target_exists, AppError};

#[derive(Debug, Serialize)]
pub struct DoctorReport {
    pub ok: bool,
}

pub fn run(args: &DoctorArgs) -> Result<DoctorReport, AppError> {
    ensure_target_exists(&args.target)?;
    let report = DoctorReport { ok: false };

    match args.format {
        OutputFormat::Text => {
            println!("doctor is not implemented yet");
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string()));
        }
    }

    Ok(report)
}
