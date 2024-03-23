// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use serde::Deserialize;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, Clone, PartialEq)]
pub struct LocalGradleWrapper {
    pub wrapper_checksum: String,
    pub file_system_path: String,
}

impl LocalGradleWrapper {
    pub fn new(file_system_path: &str, wrapper_checksum: &str) -> Self {
        Self {
            wrapper_checksum: String::from(wrapper_checksum),
            file_system_path: String::from(file_system_path),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OfficialWrapperChecksum {
    #[serde(rename(deserialize = "checksum"))]
    pub value: String,
}
