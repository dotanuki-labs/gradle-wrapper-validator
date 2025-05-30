// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod fetch_checksums;
mod find_wrappers;

pub fn locate_and_validate(path_name: &str) -> anyhow::Result<Vec<ValidationOutcome>> {
    validate(path_name, find_wrappers::find, fetch_checksums::fetch)
}

use serde::Deserialize;

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

#[derive(Debug, PartialEq)]
pub struct ValidationOutcome {
    pub local_project: LocalGradleWrapper,
    pub has_valid_wrapper_checksum: bool,
}

fn validate(
    base_path: &str,
    locate_gradle_projects: fn(&str) -> anyhow::Result<Vec<LocalGradleWrapper>>,
    fetch_gradle_releases: fn() -> anyhow::Result<Vec<OfficialWrapperChecksum>>,
) -> anyhow::Result<Vec<ValidationOutcome>> {
    let available_checksums = fetch_gradle_releases()?;
    let local_projects = locate_gradle_projects(base_path)?;

    let mut validations: Vec<ValidationOutcome> = Vec::new();

    for project in local_projects {
        let validation = available_checksums
            .iter()
            .find(|wrapped| wrapped.value.eq(&project.wrapper_checksum));

        validations.push(ValidationOutcome {
            local_project: project,
            has_valid_wrapper_checksum: validation.is_some(),
        })
    }

    Ok(validations)
}

#[cfg(test)]
mod tests {
    use crate::validator::{fetch_checksums, find_wrappers, validate};

    fn project_dir() -> String {
        let root_dir = std::env::current_dir().unwrap();
        String::from(root_dir.to_str().unwrap())
    }

    #[test]
    fn should_validate_local_project_when_checksum_matches() {
        let valid_wrapper = format!("{}/test-data/valid/gradle8", project_dir());

        let validations = validate(&valid_wrapper, find_wrappers::find, fetch_checksums::fetch).unwrap();
        let actual = validations.first().unwrap();
        assert!(actual.has_valid_wrapper_checksum)
    }

    #[test]
    fn should_validate_local_project_when_checksum_does_not_match() {
        let valid_wrapper = format!("{}/test-data/invalid/tampered", project_dir());

        let validations = validate(&valid_wrapper, find_wrappers::find, fetch_checksums::fetch).unwrap();
        let actual = validations.first().unwrap();
        assert!(!actual.has_valid_wrapper_checksum)
    }
}
