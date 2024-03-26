# gradle-wrapper-validator

[![rustfmt](https://img.shields.io/badge/code%20style-%E2%9D%A4-FF4081.svg)](https://rust-lang.github.io/rustfmt)
[![DeepSource](https://app.deepsource.com/gh/dotanuki-labs/gradle-wrapper-validator.svg/?label=active+issues&show_trend=false&token=RkvGszk0c0X5b_NOtG5k501L)](https://app.deepsource.com/gh/dotanuki-labs/gradle-wrapper-validator/)
[![CI](https://github.com/dotanuki-labs/gradle-wrapper-validator/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/dotanuki-labs/gradle-wrapper-validator/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/gwv)](https://crates.io/crates/gwv)
[![License](https://img.shields.io/github/license/dotanuki-labs/norris)](https://choosealicense.com/licenses/mit)

> A validator for gradle/wrapper jar binaries for your CI pipelines.

## Why

This projects is a small and ergonomic re-implementation of
[gradle/wrapper-validator-action](https://github.com/gradle/wrapper-validation-action),
intended to be used within any CI pipeline environment. The aforementioned
project is great, but not quite portable outside Github. If you already use
it on Github Workflows, there is no need to change!

If you are into CircleCI, Bitrise, TeamCity, GitlabCI or others, this project
may be useful!

## What

This tool will recursively walk the provided `path` and flag any `gradle/gradle-wrapper.jar`
files with
[unknown checksums](https://services.gradle.org/versions/all),
exiting with success otherwise.

## Installing

Installing from [crates.io](https://crates.io) (requires Rust/Cargo):

```bash
cargo install gwv
```

More install methods to come! Stay tuned!

## Using

On-off execution (current folder)

```bash
curl -sSf https://cdn.statically.io/gh/dotanuki-labs/gradle-wrapper-validator/main/run.sh | bash
```

On-off execution (custom folder)

```bash
curl -sSf https://cdn.statically.io/gh/dotanuki-labs/gradle-wrapper-validator/main/run.sh | bash -s -- <path/to/folder>
```

If installed with Cargo (or other)

```bash
gwv --path <path/to/gradle/projects>
```

## License

Copyright (c) 2024 - Dotanuki Labs - [The MIT license](https://choosealicense.com/licenses/mit)
