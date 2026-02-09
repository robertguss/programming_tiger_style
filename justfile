set positional-arguments

cli_manifest := "tooling/tiger-style-cli/Cargo.toml"
cli_bin := "target/release/tiger-style"

# List available recipes.
default:
  @just --list

# Build the CLI in debug mode.
build:
  cargo build --manifest-path {{cli_manifest}}

# Build the CLI in release mode.
build-release:
  cargo build --release --manifest-path {{cli_manifest}}

# Run the CLI with arbitrary arguments.
run *args:
  cargo run --manifest-path {{cli_manifest}} -- {{args}}

# Run bootstrap against a target repository.
bootstrap target=".":
  cargo run --manifest-path {{cli_manifest}} -- bootstrap --target {{target}}

# Run install against a target repository.
install-target target=".":
  cargo run --manifest-path {{cli_manifest}} -- install --target {{target}}

# Run configure against a target repository.
configure target="." manifest_mode="autodetect":
  cargo run --manifest-path {{cli_manifest}} -- configure --target {{target}} --manifest-mode {{manifest_mode}}

# Run bootstrap as a non-mutating preview.
bootstrap-dry-run target=".":
  cargo run --manifest-path {{cli_manifest}} -- bootstrap --target {{target}} --dry-run

# Run doctor against a target repository.
doctor target="." format="text":
  cargo run --manifest-path {{cli_manifest}} -- doctor --target {{target}} --format {{format}}

# Run doctor in strict mode.
doctor-strict target="." format="text":
  cargo run --manifest-path {{cli_manifest}} -- doctor --target {{target}} --strict --format {{format}}

# Check formatting without modifying files.
fmt-check:
  cargo fmt --all -- --check

# Format Rust code.
fmt:
  cargo fmt --all

# Run clippy with contract-level strictness.
clippy:
  cargo clippy --workspace --all-targets --all-features -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::undocumented_unsafe_blocks

# Run all Rust tests in the workspace.
test:
  cargo test --workspace --all-features

# Run full local validation for the CLI.
check: fmt-check clippy test

# Install the CLI binary to a local bin directory.
install prefix="$HOME/.local/bin": build-release
  mkdir -p {{prefix}}
  cp {{cli_bin}} {{prefix}}/tiger-style
  chmod +x {{prefix}}/tiger-style
  @echo "Installed {{prefix}}/tiger-style"

# Install the CLI with cargo (managed by cargo install).
install-cargo:
  cargo install --path tooling/tiger-style-cli --force

# Build mdBook docs to docs/book.
book-build:
  mdbook build

# Serve mdBook docs with live reload.
book-serve:
  mdbook serve --open

# Install mdBook locally with cargo.
book-install:
  cargo install mdbook

# Remove the locally installed CLI binary.
uninstall prefix="$HOME/.local/bin":
  rm -f {{prefix}}/tiger-style
  @echo "Removed {{prefix}}/tiger-style"

# Build release artifacts for common targets.
release-artifacts:
  cargo build --release --manifest-path {{cli_manifest}} --target x86_64-unknown-linux-gnu
  cargo build --release --manifest-path {{cli_manifest}} --target x86_64-apple-darwin
  cargo build --release --manifest-path {{cli_manifest}} --target aarch64-apple-darwin
  cargo build --release --manifest-path {{cli_manifest}} --target x86_64-pc-windows-msvc

# Clean build artifacts.
clean:
  cargo clean
