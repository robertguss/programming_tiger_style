use std::path::Path;

use similar::TextDiff;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Conflict {
    pub path: String,
    pub preview: String,
}

#[derive(Debug, Error)]
#[error("conflicts detected in {count} file(s)")]
pub struct ConflictError {
    pub count: usize,
    pub conflicts: Vec<Conflict>,
}

impl ConflictError {
    pub fn new(conflicts: Vec<Conflict>) -> Self {
        Self {
            count: conflicts.len(),
            conflicts,
        }
    }
}

pub fn from_bytes(path: &Path, existing: &[u8], incoming: &[u8]) -> Conflict {
    let preview = match (std::str::from_utf8(existing), std::str::from_utf8(incoming)) {
        (Ok(left), Ok(right)) => {
            let diff = TextDiff::from_lines(left, right)
                .unified_diff()
                .context_radius(2)
                .header("existing", "incoming")
                .to_string();

            const MAX_LINES: usize = 80;
            let mut lines = diff.lines();
            let snippet = lines
                .by_ref()
                .take(MAX_LINES)
                .collect::<Vec<_>>()
                .join("\n");
            if lines.next().is_some() {
                format!("{snippet}\n... (diff truncated)")
            } else {
                snippet
            }
        }
        _ => "binary or non-utf8 content differs".to_string(),
    };

    Conflict {
        path: path.display().to_string(),
        preview,
    }
}
