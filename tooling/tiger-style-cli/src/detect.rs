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
    let rust = if status.rust { "active" } else { "inactive" };
    let python = if status.python { "active" } else { "inactive" };
    let typescript = if status.typescript { "active" } else { "inactive" };

    format!(
        "# Active Language Contracts Manifest\n\nThis file declares which language contracts are active for CI enforcement and reviewer expectations.\n\n## Rules\n\n1. Core contracts (`contracts/core/*`) are always active.\n2. Language contracts are active only when marked `active` below.\n3. If this manifest is absent, CI falls back to language autodetection by tracked file extensions.\n4. Keep statuses explicit to avoid silent scope drift.\n\n## Status\n\n- rust: {rust}\n- python: {python}\n- typescript: {typescript}\n"
    )
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
