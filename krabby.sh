#! /usr/bin/env bash
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

set -e

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$dir"

readonly callinectes="ghcr.io/dotanuki-labs/callinectes:80da7991a90c3949881cef655bfba0e77eb031f4"
readonly task="$1"

usage() {
    echo
    echo "Available tasks:"
    echo
    echo "setup      # Installs required Cargo extensions"
    echo "lint       # Check code formatting and smells"
    echo "tests      # Run tests for Rust modules and integration tests"
    echo "assemble   # Builds binaries according to the environment (local or CI)"
    echo "security   # Run security checks and generates supply-chain artifacts"
    echo
}

setup_rust_toolchain() {
    echo "🔥 Installing and activating Rust toolchain"
    rustup show active-toolchain
    echo
}

check_code_smells() {
    echo
    echo "🔥 Checking code smells for Rust code"
    echo
    docker run --rm -v "${PWD}:/usr/src" "$callinectes" code
}

run_cargo_tests() {
    echo
    echo "🔥 Running unit + integration tests for Rust code"
    echo
    cargo test
    echo
}

build_binaries() {
    echo
    echo "🔥 Building project according to environment"
    echo
    local gha_runner="${RUNNER_OS:-local}"
    local platform

    echo "Detected environment → $gha_runner"

    case "$gha_runner" in
    "local")
        cargo build --release
        exit 0
        ;;
    "macOS")
        platform="apple-darwin"
        ;;
    "Linux")
        platform="unknown-linux-gnu"
        ;;
    *)
        echo "Error: unsupported environment → $gha_runner"
        echo
        exit 1
        ;;
    esac

    local output_dir="target/ci"

    for arch in x86_64 aarch64; do
        local target="$arch-$platform"
        rustup target add "$target"
        cargo build --release --target "$target"

        local binary="target/$target/release/gwv"
        cp "$binary" "$output_dir"/gwv-"$target"
        chmod +x "$output_dir"/gwv-"$target"
        sha256sum "$binary" >>"$output_dir"/gwv-"$target"-sha256
    done
}

check_supply_chain() {
    echo
    echo "🔥 Checking dependencies and supply-chain"
    echo
    docker run --rm -v "${PWD}:/usr/src" "$callinectes" deps
}

if [[ -z "$task" ]]; then
    usage
    exit 0
fi

case "$task" in
"setup")
    setup_rust_toolchain
    ;;
"lint")
    check_code_smells
    ;;
"tests")
    run_cargo_tests
    ;;
"assemble")
    build_binaries
    ;;
"security")
    check_supply_chain
    ;;
*)
    echo "Error: unsupported task → $task"
    usage
    exit 1
    ;;
esac
