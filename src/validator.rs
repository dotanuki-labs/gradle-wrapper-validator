// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

mod fetch_checksums;
mod find_wrappers;

use serde::Deserialize;

type ChecksumCheck<'check> = (LocalGradleWrapper, Option<&'check OfficialWrapperChecksum>);

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

    fn match_checksum(self, checksums: &'_ [OfficialWrapperChecksum]) -> ChecksumCheck<'_> {
        let maybe_validated = checksums
            .iter()
            .find(|wrapped| wrapped.value.eq(&self.wrapper_checksum));
        (self, maybe_validated)
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

impl From<ChecksumCheck<'_>> for ValidationOutcome {
    fn from(checksum_check: ChecksumCheck<'_>) -> Self {
        let (local_project, validation) = checksum_check;
        ValidationOutcome {
            local_project,
            has_valid_wrapper_checksum: validation.is_some(),
        }
    }
}

pub async fn validate(base_path: &str) -> anyhow::Result<Vec<ValidationOutcome>> {
    let available_checksums = fetch_checksums::fetch().await?;
    let local_projects = find_wrappers::find(base_path)?;

    let validations = local_projects
        .into_iter()
        .map(|local_project| local_project.match_checksum(&available_checksums))
        .map(ValidationOutcome::from)
        .collect::<Vec<_>>();

    Ok(validations)
}

#[cfg(test)]
mod tests {
    use crate::validator::validate;

    fn project_dir() -> String {
        let root_dir = std::env::current_dir().unwrap();
        String::from(root_dir.to_str().unwrap())
    }

    #[tokio::test]
    async fn should_validate_local_project_when_checksum_matches() {
        let valid_wrapper = format!("{}/test-data/valid/gradle8", project_dir());

        let validations = validate(&valid_wrapper).await.unwrap();
        let actual = validations.first().unwrap();
        assert!(actual.has_valid_wrapper_checksum)
    }

    #[tokio::test]
    async fn should_validate_local_project_when_checksum_does_not_match() {
        let valid_wrapper = format!("{}/test-data/invalid/tampered", project_dir());

        let validations = validate(&valid_wrapper).await.unwrap();
        let actual = validations.first().unwrap();
        assert!(!actual.has_valid_wrapper_checksum)
    }
}
