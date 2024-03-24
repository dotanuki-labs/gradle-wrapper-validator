// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod cli;
mod gwv;
mod validator;

fn main() {
    let cli = cli::CommandLineInterface::new();
    gwv::validate(cli.parse_arguments());
}
