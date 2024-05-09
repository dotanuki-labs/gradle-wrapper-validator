# General SDLC tasks driven by Just
# https://just.systems

_default:
  @just --list --unsorted

# Setups Rust toolchain and Cargo extensions
setup:
    @echo "→ Installing and activate Rust toolchain"
    rustup show active-toolchain
    @echo
    @echo "→ Installing Cargo Binstall"
    ./scripts/cargo-binstaller.sh
    @echo
    @echo "→ Installing Cargo extensions"
    cargo binstall cargo-deny --no-confirm --force --quiet
    cargo binstall cargo-cyclonedx --no-confirm --force --quiet
    cargo binstall cargo-zigbuild --no-confirm --force --quiet
    cargo binstall cargo-nextest --no-confirm --force --quiet
    cargo binstall cargo-get --no-confirm --force --quiet
    cargo binstall cargo-msrv --no-confirm --force --quiet
    @echo

# Checks minimum supported Rust toolchain version
msrv:
    @echo "→ Checking minimum supported Rust toolchain version (MSRV)"
    cargo msrv verify
    @echo

# Checks code formatting and smells
lint:
    @echo "→ Checking code formatting (rustfmt)"
    cargo fmt --check
    @echo

    @echo "→ Checking code smells (clippy)"
    cargo clippy --all-targets --all-features -- -D warnings
    @echo

# Quick compiles this project and catches errors
compile:
    @echo "→ Compiling project and checking errors"
    cargo check
    @echo

# Runs project tests
tests:
    @echo "→ Running project tests"
    cargo nextest run
    @echo

# Builds release binaries for all supported compilation targets
assemble:
    @echo "→ Building binaries for all supported targets"
    ./scripts/cross-build.sh
    @echo

# Runs supply-chain checks and generates SecOps artifacts
security:
    @echo "→ Enforcing constraints over dependencies"
    cargo deny check
    @echo

    @echo "→ Generating SBOMs"
    cargo cyclonedx --format json
    @echo
