use std::path::Path;

use anyhow::Result;

#[derive(Debug, Clone, Copy, Default)]
pub struct LanguageStatus {
    pub rust: bool,
    pub python: bool,
    pub typescript: bool,
}

pub fn autodetect_languages(_root: &Path) -> Result<LanguageStatus> {
    Ok(LanguageStatus::default())
}
