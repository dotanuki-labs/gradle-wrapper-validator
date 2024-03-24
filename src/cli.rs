// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::validator::ValidationOutcome;
use anyhow::ensure;
use clap::Parser;
use human_panic::setup_panic;

pub struct ParsedRawPath(pub String);

pub struct CommandLineInterface {}

impl CommandLineInterface {
    pub fn parse_arguments(&self) -> ParsedRawPath {
        let parsed = ProgramArguments::parse().path;
        ParsedRawPath(parsed)
    }

    pub fn report(&self, outcomes: &[ValidationOutcome]) -> anyhow::Result<()> {
        ensure!(!outcomes.is_empty(), "No validations found!");
        ensure!(
            outcomes.iter().all(|outcome| outcome.has_valid_wrapper_checksum),
            "A Gradle wrapper with invalid checksum was found!"
        );

        println!("All Gradle wrappers have valid checksums!");
        Ok(())
    }

    pub fn new() -> CommandLineInterface {
        setup_panic!();
        CommandLineInterface {}
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ProgramArguments {
    #[arg(short, long)]
    pub path: String,
}
