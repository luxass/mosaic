#!/usr/bin/env -S just --justfile

# set shell configurations
set windows-shell := ["powershell"]
set shell := ["bash", "-cu"]

# default target: list all tasks with updated information
_default:
    just --list -u

# setup environment
setup:
  # install tools
  cargo install cargo-binstall
  cargo binstall cargo-deny tokio-console -y

  @echo '✅ Setup complete!'

# check readiness of the project
ready:
  just fmt
  just check
  just lint
  just test
  @echo '✅ All passed!'

# format
fmt:
    cargo fmt --all -- --emit=files

# lint
lint:
    cargo clippy --workspace --all-targets -- --deny warnings

# check for compilation errors
check:
    cargo check --workspace

# run tests
test:
    cargo test --no-fail-fast
