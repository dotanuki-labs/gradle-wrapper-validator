#! /usr/bin/env bash
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

set -e

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$dir"

readonly callinectes="ghcr.io/dotanuki-labs/callinectes:latest@sha256:3ddbd15d6293c3243eac5e624740fc69863a39ec7a640ab0e0a7468432ff0d28"
readonly docker_image="ghcr.io/dotanuki-labs/gradle-wrapper-validator"
readonly output_dir="artifacts"

readonly task="$1"
readonly argument="$2"

usage() {
    echo
    echo "Available tasks:"
    echo
    echo "setup             # Installs required Cargo extensions"
    echo "lint              # Check code formatting and smells"
    echo "tests             # Run tests for Rust modules and integration tests"
    echo "docker-build      # Builds and runs Docker image (local or CI)"
    echo "assemble          # Builds binaries according to the environment (local or CI)"
    echo "security          # Run security checks and generates supply-chain artifacts"
    echo "sbom              # Generate a CycloneDX SBOM from Rust dependencies"
    echo "prepare-release   # Prepares metadata for releasing artifacts"
    echo "docker-manifest   # Updates Docker manifest for multiarch images"
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
    docker run --rm -v "${PWD}/test-data/valid:/tmp" dotanuki-labs/gwv -p .
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

prepare_docker_release() {
    export_release_version
    case "$RUNNER_ARCH" in
    *ARM64*)
        echo "platform=arm64" >>"$GITHUB_OUTPUT"
        ;;
    *X64*)
        echo "platform=amd64" >>"$GITHUB_OUTPUT"
        ;;
    *)
        echo "Error: unsupported runner → $RUNNER_ARCH"
        exit 1
        ;;
    esac
}

merge_and_push_docker_manifest() {
    local package="$docker_image:$argument"
    local latest="$docker_image:latest"

    docker manifest create "$latest" --amend "$package"-amd64 --amend "$package"-arm64
    docker manifest annotate --arch amd64 --os linux "$latest" "$package"-amd64
    docker manifest annotate --arch arm64 --os linux "$latest" "$package"-arm64
    docker manifest inspect "$latest"
    docker manifest push "$latest"
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
"docker-build")
    build_docker
    ;;
"security")
    check_supply_chain
    ;;
"sbom")
    generate_cyclonedx_sbom
    ;;
"prepare-github-release")
    prepare_github_release
    ;;
"prepare-docker-release")
    prepare_docker_release
    ;;
"docker-manifest")
    merge_and_push_docker_manifest
    ;;
*)
    echo "Error: unsupported task → $task"
    usage
    exit 1
    ;;
esac
