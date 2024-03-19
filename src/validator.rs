// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

pub(crate) mod gradle_releases;
pub(crate) mod local_projects;
mod models;

use crate::validator::models::{GradleRelease, LocalGradleProject};

pub(crate) use gradle_releases::fetch;
pub(crate) use local_projects::locate;

#[derive(Debug, PartialEq)]
pub struct ValidationOutcome {
    local_project: LocalGradleProject,
    has_valid_wrapper_checksum: bool,
}

impl ValidationOutcome {
    pub fn new(local_project: LocalGradleProject, has_valid_wrapper_checksum: bool) -> Self {
        Self {
            local_project,
            has_valid_wrapper_checksum,
        }
    }
}

pub fn validate(
    base_path: &str,
    locate_gradle_projects: fn(&str) -> anyhow::Result<Vec<LocalGradleProject>>,
    fetch_gradle_releases: fn() -> anyhow::Result<Vec<GradleRelease>>,
) -> anyhow::Result<Vec<ValidationOutcome>> {
    let gradle_releases = fetch_gradle_releases()?;
    let local_projects = locate_gradle_projects(&base_path)?;

    let mut validations: Vec<ValidationOutcome> = Vec::new();

    for project in local_projects {
        let expected_release = GradleRelease::from(project.clone());
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
    use crate::validator::models::{DistributionType, LocalGradleProject};
    use crate::validator::validate;

    static FAKE_PATH_NAME: &str = "/usr/dev/my-projects";

    fn locate_tampered_project(path_name: &str) -> anyhow::Result<Vec<LocalGradleProject>> {
        let tampered_project = LocalGradleProject::new(
            "8.5",
            DistributionType::Stable,
            "84900f11f4a86050a8f83342ade7b6bc9b0d2bdd-tampered",
            path_name,
        );

        Ok(vec![tampered_project])
    }

    fn locate_valid_project(path_name: &str) -> anyhow::Result<Vec<LocalGradleProject>> {
        let valid_project = LocalGradleProject::new(
            "8.5",
            DistributionType::Stable,
            "d3b261c2820e9e3d8d639ed084900f11f4a86050a8f83342ade7b6bc9b0d2bdd",
            path_name,
        );

        Ok(vec![valid_project])
    }

    #[test]
    fn should_validate_local_project_when_checksum_matches() {
        let validations = validate(FAKE_PATH_NAME, locate_valid_project, fetch).unwrap();
        let actual = validations.get(0).unwrap();
        assert_eq!(actual.has_valid_wrapper_checksum, true)
    }

    #[test]
    fn should_validate_local_project_when_checksum_does_not_match() {
        let validations = validate(FAKE_PATH_NAME, locate_tampered_project, fetch).unwrap();
        let actual = validations.get(0).unwrap();
        assert_eq!(actual.has_valid_wrapper_checksum, false)
    }
}
