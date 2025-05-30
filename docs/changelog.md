# CHANGELOG

We follow the [Keep a Changelog](https://keepachangelog.com)
conventions for release notes. All notable project changes will be documented here.

## Version 0.4.0

Released at **2025-05-22**

### Added

- Provides installation and execution through Docker

## Version 0.3.2

Released at **2024-12-24**

### Fixed

- Fixes CD pipeline

## Version 0.3.1

Released at **2024-12-24**

### Changed

- Changed the underlying HTTP engine used to fetch checksums

### Fixed

- [Security vulnerability related to `idna`](https://rustsec.org/advisories/RUSTSEC-2024-0421.html)

## Version 0.3.0

Released at **2024-08-04**

### Added

- Binary artifacts and SBOMs now have provenance powered by Github Actions
- Project is now compliant with OpenSSF Best practices

## Version 0.2.0

Released at **2024-04-17**

### Added

- Implements runner script for one-off executions

### Fixed

- Changes URL provided by Gradle with aggregated checksums

## Version 0.1.0

Released at **2024-03-25**

### Added

- Full implementation as a CLI tool of [gradle/wrapper-validator-action](https://github.com/gradle/wrapper-validation-action)
- Compatibility with `Linux` and `macOS`, `x86_64` and `aarch64` platforms
