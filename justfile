# General tasks driven by Just
# https://github.com/casey/just

# Install and activate Rust toolchain
toolchain:
    @echo "→ Install and activate Rust toolchain"
    rustup show active-toolchain
    @echo

# Install Cargo plugins (local)
cargo-plugins-local:
    @echo "→ Installing Cargo Binstall"
    ./scripts/cargo-binstaller.sh
    @echo

    @echo "→ Installing Cargo plugins (local)"
    yes | cargo binstall cargo-nextest --secure --force
    yes | cargo binstall cargo-get --secure --force
    @echo

# Performs setup for this project (local/stable)
setup-local: toolchain cargo-plugins-local
    @echo
    @echo "✅ Setup (local) concluded"
    @echo

# Performs setup for this project (nightly)
setup-nightly:
    @echo "→ Installing nightly toolchain and Miri"
    rustup toolchain install nightly
    @echo

# Check code formatting and smells
lint:
    @echo "→ Checking code formatting (rustfmt)"
    cargo fmt --check
    @echo

    @echo "→ Checking code smells (clippy)"
    cargo clippy --all-targets --all-features
    @echo

# Build project against the local toolchain
build:
    @echo "→ Compile project and build binary"
    cargo build
    @echo

# Run Tests
tests:
    @echo "→ Run project tests"
    cargo nextest run
    @echo

# Run project with additional runtime checks
dynamic-checks:
    @echo "→ Run project with additional runtime checks"
    cargo +nightly careful run -- --path test-data/valid
    @echo

# Emulates CI checks
emulate-ci: lint tests build
    @echo
    @echo "✅ Emulated a CI build with success"
    @echo

# Install required Cargo plugins (CI)
cargo-plugins-ci:
    @echo "→ Installing Cargo Binstall"
    ./scripts/cargo-binstaller.sh
    @echo

    @echo "→ Installing Cargo plugins (CI)"
    yes | cargo binstall cargo-deny --secure --force
    yes | cargo binstall cargo-cyclonedx --secure --force
    yes | cargo binstall cargo-zigbuild --secure --force
    yes | cargo binstall cargo-nextest --secure --force
    yes | cargo binstall cargo-get --secure --force
    yes | cargo binstall cargo-msrv --secure --force
    yes | cargo binstall cargo-careful --secure --force
    @echo

# Performs setup for this project (CI/stable)
setup-ci: toolchain cargo-plugins-ci
    @echo "✅ Setup (CI) concluded"
    @echo

# Build project against all supported targets
cross-build:
    @echo "→ Build project against all supported targets"
    ./scripts/cross-build.sh
    @echo

# Generates supply-chain related artifacts
supply-chain-checks:
    @echo "→ Checking supplying chain"
    cargo deny check
    @echo

    @echo "→ Generating SBOMs"
    cargo cyclonedx --format json
    @echo

# Check MSRV
msrv-check:
    @echo "→ Checking minimum supported Rust version (MSRV)"
    cargo msrv verify
    @echo
