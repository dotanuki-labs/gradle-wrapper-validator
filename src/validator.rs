// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod gradle_releases;
mod local_projects;
mod models;

use crate::validator::models::{GradleRelease, LocalGradleProject, Result};

pub fn locate_and_validate(path_name: &str) -> Result<Vec<ValidationOutcome>> {
    validate(path_name, local_projects::locate, gradle_releases::fetch)
}

#[derive(Debug, PartialEq)]
pub struct ValidationOutcome {
    local_project: LocalGradleProject,
    has_valid_wrapper_checksum: bool,
}

fn validate(
    base_path: &str,
    locate_gradle_projects: fn(&str) -> Result<Vec<LocalGradleProject>>,
    fetch_gradle_releases: fn() -> Result<Vec<GradleRelease>>,
) -> Result<Vec<ValidationOutcome>> {
    let gradle_releases = fetch_gradle_releases()?;
    let local_projects = locate_gradle_projects(base_path)?;

    let mut validations: Vec<ValidationOutcome> = Vec::new();

    for project in local_projects {
        let expected_release = GradleRelease::from(&project);
        let validation = gradle_releases.contains(&expected_release);

        validations.push(ValidationOutcome {
            local_project: project,
            has_valid_wrapper_checksum: validation,
        })
    }

    Ok(validations)
}

#[cfg(test)]
mod tests {
    use crate::validator::gradle_releases::fetch;
    use crate::validator::local_projects::fakes::{locate_tampered_project, locate_valid_project};
    use crate::validator::validate;

    static FAKE_PATH_NAME: &str = "/usr/dev/my-projects";

    #[test]
    fn should_validate_local_project_when_checksum_matches() {
        let fake_locator = locate_valid_project;
        let validations = validate(FAKE_PATH_NAME, fake_locator, fetch).unwrap();
        let actual = validations.first().unwrap();
        assert!(actual.has_valid_wrapper_checksum)
    }

    #[test]
    fn should_validate_local_project_when_checksum_does_not_match() {
        let fake_locator = locate_tampered_project;
        let validations = validate(FAKE_PATH_NAME, fake_locator, fetch).unwrap();
        let actual = validations.first().unwrap();
        assert!(!actual.has_valid_wrapper_checksum)
    }
}
