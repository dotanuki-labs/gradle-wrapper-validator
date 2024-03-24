// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod cli;
mod validator;

use crate::cli::CommandLineInterface;

fn main() -> anyhow::Result<()> {
    let cli = CommandLineInterface::new();
    let target_path = cli.parse_arguments();
    let validations = validator::locate_and_validate(&target_path.0)?;
    cli.report(&validations)
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;

    static TOOL: &str = "gwv";

    #[test]
    fn should_validate_test_wrappers() {
        let mut cmd = Command::cargo_bin(TOOL).unwrap();

        let project_dir = std::env::current_dir().unwrap();
        let test_data = format!("{}/test_data", &project_dir.to_string_lossy());

        let arguments = ["-p", &test_data];
        let assert = cmd.args(arguments).assert();

        let all_ok = "All Gradle wrappers have valid checksums";
        assert.success().stdout(contains(all_ok));
    }

    #[test]
    fn should_show_help() {
        let mut cmd = Command::cargo_bin(TOOL).unwrap();
        let description = "A validator for gradle/wrapper jar binaries, intended to be used in CI pipelines";

        let assert = cmd.arg("--help").assert();
        assert.success().stdout(contains(description));
    }

    #[test]
    fn should_fail_without_arguments() {
        let mut cmd = Command::cargo_bin(TOOL).unwrap();
        let instruction = "required arguments were not provided";

        let assert = cmd.assert();
        assert.failure().stderr(contains(instruction));
    }
}
