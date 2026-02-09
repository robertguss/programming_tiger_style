use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Context;
use serde::Serialize;

use crate::cli::{DoctorArgs, OutputFormat};
use crate::detect::{parse_manifest, LanguageStatus};
use crate::{ensure_target_exists, AppError};

#[derive(Debug, Serialize)]
pub struct DoctorCheck {
    pub name: String,
    pub passed: bool,
    pub required: bool,
    pub details: String,
}

#[derive(Debug, Serialize)]
pub struct DoctorReport {
    pub ok: bool,
    pub active_languages: LanguageStatus,
    pub missing_tools: Vec<String>,
    pub warnings: Vec<String>,
    pub checks: Vec<DoctorCheck>,
}

pub fn run(args: &DoctorArgs) -> Result<DoctorReport, AppError> {
    ensure_target_exists(&args.target)?;

    let mut checks = Vec::new();
    let mut warnings = Vec::new();
    let mut missing_tools = Vec::new();

    for rel in required_paths() {
        let full = args.target.join(rel);
        let passed = full.exists();
        checks.push(DoctorCheck {
            name: format!("required path: {rel}"),
            passed,
            required: true,
            details: if passed {
                "present".to_string()
            } else {
                format!("missing: {}", full.display())
            },
        });
    }

    let manifest_path = args.target.join("contracts/ACTIVE_LANGUAGE_CONTRACTS.md");
    let active_languages = if manifest_path.exists() {
        let content = fs::read_to_string(&manifest_path)
            .with_context(|| format!("failed to read {}", manifest_path.display()))?;
        match parse_manifest(&content) {
            Ok(status) => {
                checks.push(DoctorCheck {
                    name: "manifest format".to_string(),
                    passed: true,
                    required: true,
                    details: "manifest status lines parsed successfully".to_string(),
                });
                status
            }
            Err(message) => {
                checks.push(DoctorCheck {
                    name: "manifest format".to_string(),
                    passed: false,
                    required: true,
                    details: message,
                });
                LanguageStatus::default()
            }
        }
    } else {
        checks.push(DoctorCheck {
            name: "manifest format".to_string(),
            passed: false,
            required: true,
            details: format!("missing: {}", manifest_path.display()),
        });
        LanguageStatus::default()
    };

    for (tool, required, reason) in [
        ("cargo", active_languages.rust, "Rust is active"),
        ("ruff", active_languages.python, "Python is active"),
        ("mypy", active_languages.python, "Python is active"),
        ("pytest", active_languages.python, "Python is active"),
        ("node", active_languages.typescript, "TypeScript is active"),
        ("npm", active_languages.typescript, "TypeScript is active"),
        ("npx", active_languages.typescript, "TypeScript is active"),
    ] {
        check_tool(tool, required, &mut checks, &mut missing_tools, reason);
    }

    let bash_ready = bash_is_usable();

    run_validator_check(
        &args.target,
        bash_ready,
        args.strict,
        &mut checks,
        &mut warnings,
        "scripts/validate_tdd_cycle.sh",
    )?;
    run_validator_check(
        &args.target,
        bash_ready,
        args.strict,
        &mut checks,
        &mut warnings,
        "scripts/validate_evidence_packet.sh",
    )?;

    let ok = checks.iter().all(|check| !check.required || check.passed);

    let report = DoctorReport {
        ok,
        active_languages,
        missing_tools,
        warnings,
        checks,
    };

    emit_report(&report, args.format)?;
    Ok(report)
}

fn required_paths() -> &'static [&'static str] {
    &[
        "contracts/core",
        "contracts/languages",
        "contracts/ACTIVE_LANGUAGE_CONTRACTS.md",
        "templates",
        "checklists",
        "scripts",
        ".github/pull_request_template.md",
        ".github/workflows/contract-gates.yml",
    ]
}

fn check_tool(
    tool: &str,
    required: bool,
    checks: &mut Vec<DoctorCheck>,
    missing_tools: &mut Vec<String>,
    required_reason: &str,
) {
    if !required {
        checks.push(DoctorCheck {
            name: format!("tool: {tool}"),
            passed: true,
            required: false,
            details: "not required (language inactive)".to_string(),
        });
        return;
    }

    let passed = command_exists(tool);
    if !passed {
        missing_tools.push(tool.to_string());
    }

    checks.push(DoctorCheck {
        name: format!("tool: {tool}"),
        passed,
        required: true,
        details: if passed {
            format!("available in PATH ({required_reason})")
        } else {
            format!("missing from PATH ({required_reason})")
        },
    });
}

fn run_validator_check(
    target: &Path,
    bash_ready: bool,
    strict: bool,
    checks: &mut Vec<DoctorCheck>,
    warnings: &mut Vec<String>,
    script: &str,
) -> Result<(), AppError> {
    if bash_ready {
        let status = Command::new("bash")
            .arg(script)
            .arg("--help")
            .current_dir(target)
            .status()
            .with_context(|| format!("failed to run validator help command for {script}"))?;

        checks.push(DoctorCheck {
            name: format!("validator: {script}"),
            passed: status.success(),
            required: true,
            details: if status.success() {
                "help command executed successfully".to_string()
            } else {
                format!("help command exited with status {status}")
            },
        });
    } else {
        let message = format!("bash unavailable; skipped validator check for {script}");
        warnings.push(message.clone());
        checks.push(DoctorCheck {
            name: format!("validator: {script}"),
            passed: !strict,
            required: strict,
            details: if strict {
                "bash unavailable and strict mode enabled".to_string()
            } else {
                "bash unavailable; warning only in non-strict mode".to_string()
            },
        });
    }

    Ok(())
}

fn emit_report(report: &DoctorReport, format: OutputFormat) -> Result<(), AppError> {
    match format {
        OutputFormat::Text => {
            println!("doctor: {}", if report.ok { "PASS" } else { "FAIL" });
            for check in &report.checks {
                let status = if check.passed { "PASS" } else { "FAIL" };
                println!("[{status}] {} - {}", check.name, check.details);
            }
            for warning in &report.warnings {
                println!("[WARN] {warning}");
            }
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(report)
                    .context("failed to serialize doctor report as JSON")?
            );
        }
    }
    Ok(())
}

fn command_exists(command: &str) -> bool {
    let Some(paths) = env::var_os("PATH") else {
        return false;
    };

    let exts = windows_extensions();

    for dir in env::split_paths(&paths) {
        for ext in &exts {
            let candidate = dir.join(format!("{command}{ext}"));
            if is_executable_file(&candidate) {
                return true;
            }
        }
    }

    false
}

fn bash_is_usable() -> bool {
    if !command_exists("bash") {
        return false;
    }

    Command::new("bash")
        .arg("--version")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[cfg(windows)]
fn windows_extensions() -> Vec<String> {
    let exts = env::var("PATHEXT").unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_string());
    exts.split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(|entry| entry.trim().to_ascii_lowercase())
        .collect()
}

#[cfg(not(windows))]
fn windows_extensions() -> Vec<String> {
    vec!["".to_string()]
}

fn is_executable_file(path: &PathBuf) -> bool {
    if !path.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        if let Ok(meta) = fs::metadata(path) {
            return meta.permissions().mode() & 0o111 != 0;
        }
        false
    }

    #[cfg(not(unix))]
    {
        true
    }
}
