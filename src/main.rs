// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod cli;
mod gwv;
mod validator;

use crate::cli::CommandLineInterface;

fn main() {
    let cli = CommandLineInterface::new();
    gwv::validate(cli.parse_arguments());
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;

    use crate::gwv;

    fn gwv() -> Command {
        Command::cargo_bin("gwv").unwrap()
    }

    fn project_dir() -> String {
        let root_dir = std::env::current_dir().unwrap();
        String::from(root_dir.to_str().unwrap())
    }

    #[test]
    fn should_report_all_wrappers_with_valid_checksums() {
        let valid_wrappers = format!("{}/test_data/valid", project_dir());
        let arguments = ["-p", &valid_wrappers];

        let assert = gwv().args(arguments).assert();

        assert
            .success()
            .stdout(contains("All Gradle wrappers have valid checksums"));
    }

    #[test]
    fn should_report_no_wrappers_found() {
        let no_wrappers = format!("{}/scripts", project_dir());
        let arguments = ["-p", &no_wrappers];

        let assert = gwv().args(arguments).assert();

        assert
            .failure()
            .code(gwv::EXIT_CODE_CANNOT_VALID_WRAPPERS)
            .stderr(contains("No wrappers found"));
    }

    #[test]
    fn should_report_tampered_wrapper_found() {
        let invalid_wrapper = format!("{}/test_data/invalid", project_dir());
        let arguments = ["-p", &invalid_wrapper];

        let assert = gwv().args(arguments).assert();

        assert
            .failure()
            .code(gwv::EXIT_CODE_TAMPERED_WRAPPER_FOUND)
            .stderr(contains("A Gradle wrapper with invalid checksum was found"));
    }

    #[test]
    fn should_show_help() {
        let assert = gwv().arg("--help").assert();

        let intro = "A validator for gradle/wrapper jar binaries";
        assert.success().stdout(contains(intro));
    }

    #[test]
    fn should_fail_without_arguments() {
        let no_arguments = "required arguments were not provided";

        let assert = gwv().assert();
        assert.failure().stderr(contains(no_arguments));
    }
}
