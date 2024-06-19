# Using

## If installed with `Cargo` or `Homebrew`

```bash
gwv --path <path/to/gradle/projects>
```

## One-off execution (current folder)

```bash
curl -sSf https://cdn.statically.io/gh/dotanuki-labs/gradle-wrapper-validator/main/run | bash
```

## One-off execution (custom folder)

```bash
curl -sSf https://cdn.statically.io/gh/dotanuki-labs/gradle-wrapper-validator/main/run |\
  bash -s -- <path/to/folder>
```
