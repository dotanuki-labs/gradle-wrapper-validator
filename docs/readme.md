# Gradle Wrapper Validator

## What

This projects is a small and ergonomic re-implementation of
[gradle/wrapper-validator-action](https://github.com/gradle/wrapper-validation-action),
intended to be used within any CI pipeline environment. The aforementioned
project is great, but not quite portable outside Github. If you already use
it on Github Workflows, there is no need to change!

If you are into CircleCI, Bitrise, TeamCity, GitlabCI or others, this project
may be useful!

## How

This tool will recursively walk the provided `path` and flag any `gradle/gradle-wrapper.jar`
files with
[unknown checksums](https://services.gradle.org/versions/all),
exiting with success otherwise.
