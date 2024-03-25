# gradle-wrapper-validator

> A validator for gradle/wrapper jar binaries for your CI pipelines.

## Why

This projects is a small and ergonomic re-implementation of
[gradle/wrapper-validator-action](https://github.com/gradle/wrapper-validation-action),
intended to be used within any CI pipeline environment. The aforementioned
project is great, but not quite portable outside Github. If you already use
it on Github Workflows, there is no need to change!

If you are into CircleCI, Bitrise, TeamCity, GitlabCI or others, this project
may be useful!

## Installing

Installing from [crates.io](https://crates.io) (requires Rust/Cargo):

```bash
$> cargo install gwv
```

More install methods to come! Stay tuned!

## Using

```bash
$> gwv --path <path/to/gradle/projects>
```

This tool will recursively walk the provided `path` and flag any `gradle/gradle-wrapper.jar`
files with
[unknown checksums](https://services.gradle.org/versions/all),
exiting with success otherwise.

## License

Copyright (c) 2024 - Dotanuki Labs - [The MIT license](https://choosealicense.com/licenses/mit)
