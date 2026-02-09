use std::fs;
use std::path::Path;

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;

fn cli() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("tiger-style"))
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|err| panic!("create parent: {err}"));
    }
    fs::write(path, content).unwrap_or_else(|err| panic!("write file: {err}"));
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("read file: {err}"))
}

fn source_file(path: &str) -> String {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    fs::read_to_string(repo_root.join(path)).unwrap_or_else(|err| panic!("read source file: {err}"))
}

fn assert_manifest(path: &Path, rust: bool, python: bool, typescript: bool) {
    let content = read_file(path);
    let rust_expected = if rust { "active" } else { "inactive" };
    let python_expected = if python { "active" } else { "inactive" };
    let typescript_expected = if typescript { "active" } else { "inactive" };

    assert!(
        content.contains(&format!("- rust: {rust_expected}")),
        "manifest missing rust status"
    );
    assert!(
        content.contains(&format!("- python: {python_expected}")),
        "manifest missing python status"
    );
    assert!(
        content.contains(&format!("- typescript: {typescript_expected}")),
        "manifest missing typescript status"
    );
}

#[test]
fn install_writes_required_tree() {
    let temp = TempDir::new().unwrap_or_else(|err| panic!("temp dir: {err}"));

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
    let temp = TempDir::new().unwrap_or_else(|err| panic!("temp dir: {err}"));

    cli()
        .arg("install")
        .arg("--target")
        .arg(temp.path())
        .assert()
        .success();

    let changed = temp.path().join("contracts/core/AI_AGENT_CORE_CONTRACT.md");
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
    let temp = TempDir::new().unwrap_or_else(|err| panic!("temp dir: {err}"));

    cli()
        .arg("install")
        .arg("--target")
        .arg(temp.path())
        .assert()
        .success();

    let changed = temp.path().join("contracts/core/AI_AGENT_CORE_CONTRACT.md");
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
    let temp = TempDir::new().unwrap_or_else(|err| panic!("temp dir: {err}"));

    cli()
        .arg("install")
        .arg("--target")
        .arg(temp.path())
        .arg("--dry-run")
        .assert()
        .success();

    assert!(!temp
        .path()
        .join("contracts/core/AI_AGENT_CORE_CONTRACT.md")
        .exists());
}

#[test]
fn configure_autodetect_sets_manifest_statuses() {
    let cases = vec![
        (vec![], (false, false, false)),
        (vec![("src/lib.rs", "fn x() {}")], (true, false, false)),
        (
            vec![("scripts/tool.py", "print('x')")],
            (false, true, false),
        ),
        (
            vec![("web/app.ts", "export const x = 1;")],
            (false, false, true),
        ),
        (
            vec![
                ("src/main.rs", "fn main() {}"),
                ("web/app.tsx", "export default 1;"),
            ],
            (true, false, true),
        ),
    ];

    for (files, expected) in cases {
        let temp = TempDir::new().unwrap_or_else(|err| panic!("temp dir: {err}"));
        for (path, content) in files {
            write_file(&temp.path().join(path), content);
        }

        cli()
            .arg("configure")
            .arg("--target")
            .arg(temp.path())
            .arg("--manifest-mode")
            .arg("autodetect")
            .assert()
            .success();

        assert_manifest(
            &temp.path().join("contracts/ACTIVE_LANGUAGE_CONTRACTS.md"),
            expected.0,
            expected.1,
            expected.2,
        );
        assert!(temp.path().join("AGENTS.md").exists());
    }
}

#[test]
fn configure_conflicts_when_agents_exists_without_force() {
    let temp = TempDir::new().unwrap_or_else(|err| panic!("temp dir: {err}"));
    write_file(&temp.path().join("AGENTS.md"), "custom agents content\n");

    cli()
        .arg("configure")
        .arg("--target")
        .arg(temp.path())
        .assert()
        .code(3)
        .stderr(contains("conflicts detected"));
}

#[test]
fn configure_force_overwrites_agents() {
    let temp = TempDir::new().unwrap_or_else(|err| panic!("temp dir: {err}"));
    write_file(&temp.path().join("AGENTS.md"), "custom agents content\n");

    cli()
        .arg("configure")
        .arg("--target")
        .arg(temp.path())
        .arg("--force")
        .assert()
        .success();

    assert_eq!(
        read_file(&temp.path().join("AGENTS.md")),
        source_file("docs/templates/AGENTS_TEMPLATE.md")
    );
}

#[test]
fn bootstrap_installs_configures_and_passes_doctor() {
    let temp = TempDir::new().unwrap_or_else(|err| panic!("temp dir: {err}"));

    cli()
        .arg("bootstrap")
        .arg("--target")
        .arg(temp.path())
        .assert()
        .success();

    assert!(temp
        .path()
        .join("contracts/core/AI_AGENT_CORE_CONTRACT.md")
        .exists());
    assert!(temp.path().join("AGENTS.md").exists());
    assert_manifest(
        &temp.path().join("contracts/ACTIVE_LANGUAGE_CONTRACTS.md"),
        false,
        false,
        false,
    );
}

#[test]
fn bootstrap_dry_run_does_not_write_files() {
    let temp = TempDir::new().unwrap_or_else(|err| panic!("temp dir: {err}"));

    cli()
        .arg("bootstrap")
        .arg("--target")
        .arg(temp.path())
        .arg("--dry-run")
        .assert()
        .success();

    assert!(!temp.path().join("contracts").exists());
    assert!(!temp.path().join("AGENTS.md").exists());
}

#[test]
fn doctor_json_reports_missing_tools_for_active_languages() {
    let temp = TempDir::new().unwrap_or_else(|err| panic!("temp dir: {err}"));

    cli()
        .arg("install")
        .arg("--target")
        .arg(temp.path())
        .assert()
        .success();

    write_file(
        &temp.path().join("contracts/ACTIVE_LANGUAGE_CONTRACTS.md"),
        "# Active Language Contracts Manifest\n\n## Status\n\n- rust: active\n- python: inactive\n- typescript: inactive\n",
    );

    cli()
        .arg("doctor")
        .arg("--target")
        .arg(temp.path())
        .arg("--format")
        .arg("json")
        .env("PATH", "")
        .assert()
        .code(2)
        .stdout(contains("\"ok\": false"))
        .stdout(contains("\"cargo\""));
}

#[test]
fn doctor_strict_fails_when_bash_is_missing() {
    let temp = TempDir::new().unwrap_or_else(|err| panic!("temp dir: {err}"));

    cli()
        .arg("install")
        .arg("--target")
        .arg(temp.path())
        .assert()
        .success();

    write_file(
        &temp.path().join("contracts/ACTIVE_LANGUAGE_CONTRACTS.md"),
        "# Active Language Contracts Manifest\n\n## Status\n\n- rust: inactive\n- python: inactive\n- typescript: inactive\n",
    );

    cli()
        .arg("doctor")
        .arg("--target")
        .arg(temp.path())
        .arg("--strict")
        .env("PATH", "")
        .assert()
        .code(2)
        .stderr(contains("doctor checks failed"));
}
