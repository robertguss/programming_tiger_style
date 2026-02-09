use std::fs;
use std::path::Path;

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn cli() -> Command {
    Command::cargo_bin("tiger-style").expect("binary should compile")
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent");
    }
    fs::write(path, content).expect("write file");
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("read file")
}

fn source_file(path: &str) -> String {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    fs::read_to_string(repo_root.join(path)).expect("read source file")
}

#[test]
fn install_writes_required_tree() {
    let temp = TempDir::new().expect("temp dir");

    cli()
        .arg("install")
        .arg("--target")
        .arg(temp.path())
        .assert()
        .success();

    let required = [
        "contracts/core/AI_AGENT_CORE_CONTRACT.md",
        "contracts/languages/RUST_CODING_CONTRACT.md",
        "contracts/ACTIVE_LANGUAGE_CONTRACTS.md",
        "templates/TASK_PACKET_TEMPLATE.md",
        "checklists/PR_CONTRACT_CHECKLIST.md",
        "scripts/validate_tdd_cycle.sh",
        ".github/pull_request_template.md",
        ".github/workflows/contract-gates.yml",
    ];

    for rel in required {
        assert!(temp.path().join(rel).exists(), "missing {rel}");
    }
}

#[test]
fn install_conflict_returns_exit_3() {
    let temp = TempDir::new().expect("temp dir");

    cli()
        .arg("install")
        .arg("--target")
        .arg(temp.path())
        .assert()
        .success();

    let changed = temp
        .path()
        .join("contracts/core/AI_AGENT_CORE_CONTRACT.md");
    write_file(&changed, "local drift\n");

    cli()
        .arg("install")
        .arg("--target")
        .arg(temp.path())
        .assert()
        .code(3)
        .stderr(contains("conflicts detected"));
}

#[test]
fn install_force_overwrites_conflicts() {
    let temp = TempDir::new().expect("temp dir");

    cli()
        .arg("install")
        .arg("--target")
        .arg(temp.path())
        .assert()
        .success();

    let changed = temp
        .path()
        .join("contracts/core/AI_AGENT_CORE_CONTRACT.md");
    write_file(&changed, "local drift\n");

    cli()
        .arg("install")
        .arg("--target")
        .arg(temp.path())
        .arg("--force")
        .assert()
        .success();

    assert_eq!(
        read_file(&changed),
        source_file("contracts/core/AI_AGENT_CORE_CONTRACT.md")
    );
}

#[test]
fn install_dry_run_does_not_write_files() {
    let temp = TempDir::new().expect("temp dir");

    cli()
        .arg("install")
        .arg("--target")
        .arg(temp.path())
        .arg("--dry-run")
        .assert()
        .success();

    assert!(
        !temp
            .path()
            .join("contracts/core/AI_AGENT_CORE_CONTRACT.md")
            .exists()
    );
}
