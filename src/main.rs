// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod validator;

use clap::Parser;
use human_panic::setup_panic;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ProgramArguments {
    #[arg(short, long)]
    path: String,
}

fn main() {
    setup_panic!();

    let arguments = ProgramArguments::parse();
    let validations = validator::validate(
        &arguments.path,
        validator::local_projects::locate,
        validator::gradle_releases::fetch,
    )
    .unwrap();
    println!("{:?}", validations);
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;

    static TOOL: &str = "gwv";

    #[test]
    fn should_show_help() {
        let mut cmd = Command::cargo_bin(TOOL).unwrap();
        let description = "An opinionated way to kick-off CLI apps powered by Rust";

        let assert = cmd.arg("--help").assert();
        assert.stdout(contains(description));
    }

    #[test]
    fn should_fail_without_arguments() {
        let mut cmd = Command::cargo_bin(TOOL).unwrap();
        let instruction = "required arguments were not provided";

        let assert = cmd.assert();
        assert.failure().stderr(contains(instruction));
    }
}
