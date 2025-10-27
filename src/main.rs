// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::cli::ParsedRawPath;
use crate::validator::ValidationOutcome;
use std::process::exit;

mod cli;
mod validator;

pub static EXIT_CODE_CANNOT_VALID_WRAPPERS: i32 = 13;
pub static EXIT_CODE_TAMPERED_WRAPPER_FOUND: i32 = 19;

fn main() {
    let cli = cli::CommandLineInterface::new();
    validate_wrappers(cli.parse_arguments());
}

fn validate_wrappers(target_path: ParsedRawPath) {
    match validator::locate_and_validate(&target_path.0) {
        Ok(outcomes) => ensure_no_issues(outcomes),
        Err(wrapped) => {
            eprintln!("{}", &wrapped.to_string());
            exit(EXIT_CODE_CANNOT_VALID_WRAPPERS)
        },
    }
}

fn ensure_no_issues(outcomes: Vec<ValidationOutcome>) {
    let issues: Vec<&ValidationOutcome> = outcomes
        .iter()
        .filter(|check| !check.has_valid_wrapper_checksum)
        .collect();

    if !issues.is_empty() {
        eprintln!("A Gradle wrapper with invalid checksum was found!");

        for invalid in issues {
            println!("{}", &invalid.local_project.file_system_path);
        }

        exit(EXIT_CODE_TAMPERED_WRAPPER_FOUND)
    }

    println!("All Gradle wrappers have valid checksums");
}
