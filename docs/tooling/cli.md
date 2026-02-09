# Tiger Style CLI (`tiger-style`)

The `tiger-style` CLI installs and configures Contract System v2 in downstream repositories without
manual copy commands.

## What It Does

- Installs contracts, templates, checklists, scripts, and GitHub workflow assets.
- Configures `contracts/ACTIVE_LANGUAGE_CONTRACTS.md` with explicit language status lines.
- Creates `AGENTS.md` from the upstream template.
- Runs environment checks (`doctor`) for required files and tooling.

## Install The CLI

### Option A: Download release binary (recommended)

Download the binary for your platform from GitHub Releases for tags matching
`tiger-style-cli-v*`.

Assets are published as:

- `tiger-style-x86_64-unknown-linux-gnu`
- `tiger-style-x86_64-apple-darwin`
- `tiger-style-aarch64-apple-darwin`
- `tiger-style-x86_64-pc-windows-msvc.exe`

Rename/move the file to `tiger-style` (or `tiger-style.exe`) on your `PATH`.

### Option B: Build locally from source

```bash
cargo build --release --manifest-path tooling/tiger-style-cli/Cargo.toml
cp target/release/tiger-style /usr/local/bin/tiger-style
```

### Option C: Install from this repository with Just/Cargo

```bash
just install-cargo
```

Or:

```bash
cargo install --path tooling/tiger-style-cli --force
```

## Quick Start

One-command setup in a target repository:

```bash
tiger-style bootstrap --target /absolute/path/to/your-repo
```

## Command Reference

### Install

```bash
tiger-style install --target <path> [--force] [--dry-run]
```

- Copies v1.1 contract assets into the target repository.
- Default safety behavior: conflict on changed existing files and exit with code `3`.
- `--force` overwrites conflicting files.
- `--dry-run` prints planned actions without writing files.

### Configure

```bash
tiger-style configure --target <path> [--manifest-mode autodetect|all-inactive|all-active] [--force] [--dry-run]
```

- Writes canonical language activation manifest status lines:
  - `- rust: active|inactive`
  - `- python: active|inactive`
  - `- typescript: active|inactive`
- Creates `AGENTS.md` from the template if missing.
- Conflicts on modified `AGENTS.md` unless `--force` is used.

### Doctor

```bash
tiger-style doctor --target <path> [--strict] [--format text|json]
```

- Validates required downstream files.
- Parses manifest status lines.
- Checks language-specific tools only for active languages.
- Validates shell validator script invocations when `bash` is available.
- If `bash` is missing:
  - non-strict mode: warning only
  - strict mode: validation failure

### Bootstrap

```bash
tiger-style bootstrap --target <path> [--force] [--dry-run]
```

Equivalent orchestration:

1. `install`
2. `configure --manifest-mode autodetect`
3. `doctor` (non-strict)

`--dry-run` applies to install/configure and skips doctor execution.

## Exit Codes

- `0`: success
- `1`: unexpected runtime failure
- `2`: validation failure
- `3`: file conflict detected

## Suggested Adoption Flow

1. Run `tiger-style bootstrap --target <repo>`.
2. Review and adjust `contracts/ACTIVE_LANGUAGE_CONTRACTS.md`.
3. Run `tiger-style doctor --target <repo> --format text`.
4. Commit the installed assets in the target repository.
