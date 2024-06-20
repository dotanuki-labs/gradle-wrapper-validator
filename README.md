# gradle-wrapper-validator

[![OSSF Best](https://www.bestpractices.dev/projects/8869/badge)](https://www.bestpractices.dev/projects/8869)
[![DeepSource](https://app.deepsource.com/gh/dotanuki-labs/gradle-wrapper-validator.svg/?label=active+issues&show_trend=false&token=RkvGszk0c0X5b_NOtG5k501L)](https://app.deepsource.com/gh/dotanuki-labs/gradle-wrapper-validator/)
[![CI](https://github.com/dotanuki-labs/gradle-wrapper-validator/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/dotanuki-labs/gradle-wrapper-validator/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rustc-1.74+-FF8000.svg?logo=rust&logoColor=white)](https://rustup.rs/)
[![Latest Version](https://img.shields.io/crates/v/gwv)](https://crates.io/crates/gwv)
[![License](https://img.shields.io/github/license/dotanuki-labs/norris)](https://choosealicense.com/licenses/mit)

> A validator for gradle/wrapper jar binaries for your CI pipelines.

## Why

This projects is a small and ergonomic re-implementation of
[gradle/wrapper-validator-action](https://github.com/gradle/wrapper-validation-action),
intended to be used within CI pipelines. The aforementioned
project is great, but not quite portable outside Github.

If you are into CircleCI, Bitrise, TeamCity, GitlabCI or others, this project
may be useful!

Check our
[documentation](https://dotanuki-labs.github.io/gradle-wrapper-validator/)
to learn more.

## License

Copyright (c) 2024 - Dotanuki Labs - [The MIT license](https://choosealicense.com/licenses/mit)
