# Using

## From existing binaries

If installed with `Cargo`, `Homebrew` or from source

```bash
gwv --path <path/to/gradle/projects>
```

## Docker

The Docker image uses `/tmp` as `WORKDIR`, hence, just mount your Gradle project on that path:

```bash
docker run --rm -v "/path/to/my/project:/tmp" dotanuki-labs/gradle-wrapper-validator -p .
```

## One-off executions

In addition to existing installation methods, we also provide a standalone, one-off
execution script that may be handy for any CI environment.

- One-off execution (current folder)

```bash
curl -sSf https://cdn.statically.io/gh/dotanuki-labs/gradle-wrapper-validator/main/run | bash
```

- One-off execution (custom folder)

```bash
curl -sSf https://cdn.statically.io/gh/dotanuki-labs/gradle-wrapper-validator/main/run |\
  bash -s -- <path/to/folder>
```
