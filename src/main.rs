// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::validator::ValidationOutcome;
use std::process::exit;

mod cli;
mod validator;

pub static EXIT_CODE_CANNOT_VALID_WRAPPERS: i32 = 13;
pub static EXIT_CODE_TAMPERED_WRAPPER_FOUND: i32 = 19;

#[tokio::main]
async fn main() {
    let cli = cli::CommandLineInterface::new();
    match validator::validate(cli.parse_arguments().0.as_str()).await {
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
