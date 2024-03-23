// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use serde::Deserialize;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, Clone, PartialEq)]
pub struct LocalGradleWrapper {
    gradle_version: String,
    wrapper_checksum: String,
    file_system_path: String,
}

impl LocalGradleWrapper {
    pub fn new(gradle_version: &str, wrapper_checksum: &str, file_system_path: &str) -> Self {
        Self {
            gradle_version: String::from(gradle_version),
            wrapper_checksum: String::from(wrapper_checksum),
            file_system_path: String::from(file_system_path),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GradleRelease {
    #[serde(rename(deserialize = "version"))]
    gradle_version: String,
    #[serde(rename(deserialize = "checksum"))]
    wrapper_checksum: String,
}

impl From<&LocalGradleWrapper> for GradleRelease {
    fn from(project: &LocalGradleWrapper) -> Self {
        let cloned = project.clone();
        GradleRelease {
            gradle_version: cloned.gradle_version,
            wrapper_checksum: cloned.wrapper_checksum,
        }
    }
}
