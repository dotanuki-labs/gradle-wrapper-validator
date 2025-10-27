// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

use clap::Parser;
use human_panic::{metadata, setup_panic};

pub struct ParsedRawPath(pub String);

pub struct CommandLineInterface {}

impl CommandLineInterface {
    pub fn parse_arguments(&self) -> ParsedRawPath {
        let parsed = ProgramArguments::parse().path;
        ParsedRawPath(parsed)
    }

    pub fn new() -> CommandLineInterface {
        let homepage_message = "https://github.io/dotanuki-labs/gradle-wrapper-validator";
        let support_message = format!("For support, reach out to {homepage_message}/issues");
        setup_panic!(metadata!().support(support_message.clone()).homepage(homepage_message));
        CommandLineInterface {}
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ProgramArguments {
    #[arg(short, long)]
    pub path: String,
}
