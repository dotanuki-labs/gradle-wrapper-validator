// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod gradle_releases;
mod local_projects;
mod models;

use crate::validator::models::{LocalGradleWrapper, OfficialWrapperChecksum, Result};

pub fn locate_and_validate(path_name: &str) -> Result<Vec<ValidationOutcome>> {
    validate(path_name, local_projects::locate, gradle_releases::fetch)
}

#[derive(Debug, PartialEq)]
pub struct ValidationOutcome {
    pub local_project: LocalGradleWrapper,
    pub has_valid_wrapper_checksum: bool,
}

fn validate(
    base_path: &str,
    locate_gradle_projects: fn(&str) -> Result<Vec<LocalGradleWrapper>>,
    fetch_gradle_releases: fn() -> Result<Vec<OfficialWrapperChecksum>>,
) -> Result<Vec<ValidationOutcome>> {
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
    use crate::validator::gradle_releases::fetch;
    use crate::validator::{local_projects, validate};

    static FAKE_PATH_NAME: &str = "/usr/dev/my-projects";

    #[test]
    fn should_validate_local_project_when_checksum_matches() {
        let project_dir = std::env::current_dir().unwrap();
        let test_data = format!("{}/test_data/gradle8", &project_dir.to_string_lossy());
        let locator = local_projects::locate;

        let validations = validate(&test_data, locator, fetch).unwrap();
        let actual = validations.first().unwrap();
        assert!(actual.has_valid_wrapper_checksum)
    }

    #[test]
    fn should_validate_local_project_when_checksum_does_not_match() {
        let locator = local_projects::fakes::locate_tampered_project;
        let validations = validate(FAKE_PATH_NAME, locator, fetch).unwrap();
        let actual = validations.first().unwrap();
        assert!(!actual.has_valid_wrapper_checksum)
    }
}
