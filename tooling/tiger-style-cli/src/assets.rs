use include_dir::{include_dir, Dir, DirEntry};

#[derive(Debug, Clone)]
pub struct Asset {
    pub relative_path: String,
    pub contents: Vec<u8>,
    pub executable: bool,
}

static CONTRACTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../contracts");
static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../templates");
static CHECKLISTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../checklists");
static SCRIPTS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../scripts");

static PR_TEMPLATE: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../.github/pull_request_template.md"
));
static CONTRACT_GATES_WORKFLOW: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../.github/workflows/contract-gates.yml"
));
static AGENTS_TEMPLATE: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/templates/AGENTS_TEMPLATE.md"
));
static MANIFEST_TEMPLATE: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../contracts/ACTIVE_LANGUAGE_CONTRACTS.md"
));

pub fn install_assets() -> Vec<Asset> {
    let mut assets = Vec::new();
    collect_dir("contracts", &CONTRACTS_DIR, &mut assets);
    collect_dir("templates", &TEMPLATES_DIR, &mut assets);
    collect_dir("checklists", &CHECKLISTS_DIR, &mut assets);
    collect_dir("scripts", &SCRIPTS_DIR, &mut assets);

    assets.push(Asset {
        relative_path: ".github/pull_request_template.md".to_string(),
        contents: PR_TEMPLATE.to_vec(),
        executable: false,
    });

    assets.push(Asset {
        relative_path: ".github/workflows/contract-gates.yml".to_string(),
        contents: CONTRACT_GATES_WORKFLOW.to_vec(),
        executable: false,
    });

    assets.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    assets
}

pub fn agents_template() -> &'static [u8] {
    AGENTS_TEMPLATE
}

pub fn manifest_template() -> &'static [u8] {
    MANIFEST_TEMPLATE
}

fn collect_dir(prefix: &str, dir: &Dir<'_>, assets: &mut Vec<Asset>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::File(file) => {
                let rel = if prefix.is_empty() {
                    file.path().to_string_lossy().into_owned()
                } else {
                    format!("{}/{}", prefix, file.path().to_string_lossy())
                };
                if rel.ends_with(".DS_Store") {
                    continue;
                }
                let executable = rel.starts_with("scripts/") && rel.ends_with(".sh");
                assets.push(Asset {
                    relative_path: rel,
                    contents: file.contents().to_vec(),
                    executable,
                });
            }
            DirEntry::Dir(child) => {
                collect_dir(prefix, child, assets);
            }
        }
    }
}
