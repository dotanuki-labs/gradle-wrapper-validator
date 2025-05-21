#! /usr/bin/env bash
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

set -e

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$dir"

readonly callinectes="ghcr.io/dotanuki-labs/callinectes:latest@sha256:dbf5bdc784779c13e2c8ce9a559ee9f3f6276314b3312cc234b672560b26c4f2"
readonly task="$1"
readonly output_dir="artifacts"

usage() {
    echo
    echo "Available tasks:"
    echo
    echo "setup             # Installs required Cargo extensions"
    echo "lint              # Check code formatting and smells"
    echo "tests             # Run tests for Rust modules and integration tests"
    echo "docker            # Builds and runs Docker image (local or CI)"
    echo "assemble          # Builds binaries according to the environment (local or CI)"
    echo "security          # Run security checks and generates supply-chain artifacts"
    echo "sbom              # Generate a CycloneDX SBOM from Rust dependencies"
    echo "prepare-release   # Prepares metadata for releasing artifacts"
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
    docker run --rm -v "${PWD}:/usr/src" "$callinectes" fmt clippy
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

    rm -rf "$output_dir" && mkdir -p "$output_dir"

    for arch in x86_64 aarch64; do
        local target="$arch-$platform"
        rustup target add "$target"
        cargo build --release --target "$target"

        local binary="target/$target/release/gwv"
        cp "$binary" "$output_dir"/gwv-"$target"
        chmod +x "$output_dir"/gwv-"$target"
    done
}

build_docker() {
    echo
    echo "🔥 Building Docker image"
    echo
    docker build -t dotanuki-labs/gwv .

    echo
    echo "🔥 Running Docker image"
    docker run --rm -v "${PWD}/test-data/valid:/usr/src" "dotanuki-labs/gwv"
    echo
}

check_supply_chain() {
    echo
    echo "🔥 Checking dependencies and supply-chain"
    echo
    docker run --rm -v "${PWD}:/usr/src" "$callinectes" msrv deny machete
}

generate_cyclonedx_sbom() {
    echo
    echo "🔥 Generating cyclonedx SBOM"
    echo
    docker run --rm -v "${PWD}:/usr/src" "$callinectes" cyclonedx
}

compute_checksums() {
    readonly checksums="checksums.txt"

    cd "$output_dir"
    touch "$checksums"
    find . -name 'gwv-*' -exec sha256sum {} \; |
        sed "s/\.\///g" |
        sed "s/gwv-binaries-macOS\///g" |
        sed "s/gwv-binaries-Linux\///g" >"$checksums"
}

export_release_version() {
    version=$(grep 'version' Cargo.toml | head -1 | sed "s/version[[:space:]]=[[:space:]]//g" | tr -d '"')
    echo "version=$version" >>"$GITHUB_OUTPUT"
}

prepare_github_release() {
    compute_checksums
    export_release_version
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
"docker")
    build_docker
    ;;
"security")
    check_supply_chain
    ;;
"sbom")
    generate_cyclonedx_sbom
    ;;
"prepare-release")
    prepare_github_release
    ;;
*)
    echo "Error: unsupported task → $task"
    usage
    exit 1
    ;;
esac
