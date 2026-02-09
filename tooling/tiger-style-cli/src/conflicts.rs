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
