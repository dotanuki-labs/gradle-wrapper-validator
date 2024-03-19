#! /usr/bin/env bash
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

set -euo pipefail

readonly output_dir="target/ci"

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${dir%/*}"

rm -rf "$output_dir" && mkdir -p "$output_dir"

cross_compile() {
    local target="$1"

    rustup target add "$target"
    cargo zigbuild --release --target "$target"

    local binary="target/$target/release/gwv"
    cp "$binary" "$output_dir"/gwv-"$target"
    chmod +x "$output_dir"/gwv-"$target"
    sha256sum "$binary" >>"$output_dir"/gwv-"$target"-sha256
}

for platform in apple-darwin unknown-linux-gnu; do
    for arch in x86_64 aarch64; do
        cross_compile "$arch-$platform"
    done
done
