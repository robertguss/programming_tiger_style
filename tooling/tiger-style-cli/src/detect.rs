use std::path::Path;

use anyhow::Result;
use serde::Serialize;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct LanguageStatus {
    pub rust: bool,
    pub python: bool,
    pub typescript: bool,
}

impl LanguageStatus {
    pub fn all(active: bool) -> Self {
        Self {
            rust: active,
            python: active,
            typescript: active,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.rust && self.python && self.typescript
    }
}

pub fn autodetect_languages(root: &Path) -> Result<LanguageStatus> {
    let mut status = LanguageStatus::default();

    let walker = WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !should_skip(entry));

    for item in walker {
        let entry = item?;
        if !entry.file_type().is_file() {
            continue;
        }

        if let Some(ext) = entry.path().extension().and_then(|value| value.to_str()) {
            match ext {
                "rs" => status.rust = true,
                "py" => status.python = true,
                "ts" | "tsx" | "js" | "jsx" => status.typescript = true,
                _ => {}
            }
        }

        if status.is_complete() {
            break;
        }
    }

    Ok(status)
}

pub fn render_manifest(status: LanguageStatus) -> String {
    format!(
        "# Active Language Contracts Manifest\n\nThis file declares which language contracts are active for CI enforcement and reviewer expectations.\n\n## Rules\n\n1. Core contracts (`contracts/core/*`) are always active.\n2. Language contracts are active only when marked `active` below.\n3. If this manifest is absent, CI falls back to language autodetection by tracked file extensions.\n4. Keep statuses explicit to avoid silent scope drift.\n\n## Status\n\n- rust: {}\n- python: {}\n- typescript: {}\n",
        status_label(status.rust),
        status_label(status.python),
        status_label(status.typescript)
    )
}

pub fn parse_manifest(contents: &str) -> Result<LanguageStatus, String> {
    let mut rust = None;
    let mut python = None;
    let mut typescript = None;

    for line in contents.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("- rust:") {
            rust = Some(parse_status(value)?);
        } else if let Some(value) = trimmed.strip_prefix("- python:") {
            python = Some(parse_status(value)?);
        } else if let Some(value) = trimmed.strip_prefix("- typescript:") {
            typescript = Some(parse_status(value)?);
        }
    }

    match (rust, python, typescript) {
        (Some(rust), Some(python), Some(typescript)) => Ok(LanguageStatus {
            rust,
            python,
            typescript,
        }),
        _ => Err(
            "manifest must contain '- rust: active|inactive', '- python: active|inactive', and '- typescript: active|inactive'"
                .to_string(),
        ),
    }
}

fn parse_status(raw: &str) -> Result<bool, String> {
    match raw.trim() {
        "active" => Ok(true),
        "inactive" => Ok(false),
        other => Err(format!(
            "invalid language status '{other}', expected active|inactive"
        )),
    }
}

fn status_label(active: bool) -> &'static str {
    if active {
        "active"
    } else {
        "inactive"
    }
}

fn should_skip(entry: &DirEntry) -> bool {
    if entry.depth() == 0 {
        return false;
    }

    let Some(name) = entry.file_name().to_str() else {
        return false;
    };

    matches!(
        name,
        ".git" | "node_modules" | ".venv" | "venv" | "target" | ".mypy_cache" | ".pytest_cache"
    )
}
