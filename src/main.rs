// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod cli;
mod validator;

use crate::cli::CommandLineInterface;
use crate::validator::ValidationOutcome;
use std::process::exit;

fn main() {
    let cli = CommandLineInterface::new();
    let target_path = cli.parse_arguments();

    match validator::locate_and_validate(&target_path.0) {
        Ok(outcomes) => ensure_no_issues(outcomes),
        Err(wrapped) => {
            eprintln!("{}", &wrapped.to_string());
            exit(42)
        },
    }
}

fn ensure_no_issues(outcomes: Vec<ValidationOutcome>) {
    let issues = outcomes.iter().any(|check| !check.has_valid_wrapper_checksum);

    if issues {
        eprintln!("A Gradle wrapper with invalid checksum was found!");
        exit(37)
    }

    println!("All Gradle wrappers have valid checksums");
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

        assert
            .success()
            .stdout(contains("All Gradle wrappers have valid checksums"));
    }

    #[test]
    fn should_report_custom_errors() {
        let mut cmd = Command::cargo_bin(TOOL).unwrap();

        let project_dir = std::env::current_dir().unwrap();
        let no_wrappers = format!("{}/scripts", &project_dir.to_string_lossy());

        let arguments = ["-p", &no_wrappers];
        let assert = cmd.args(arguments).assert();

        assert.failure().stderr(contains("No wrappers found"));
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
