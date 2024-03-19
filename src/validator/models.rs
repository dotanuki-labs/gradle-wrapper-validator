// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq)]
pub enum DistributionType {
    Stable,
    Rc,
    Beta,
    Milestone,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalGradleProject {
    gradle_version: String,
    distribution_type: DistributionType,
    wrapper_checksum: String,
    file_system_path: String,
}

impl LocalGradleProject {
    pub fn new(
        gradle_version: &str,
        distribution_type: DistributionType,
        wrapper_checksum: &str,
        file_system_path: &str,
    ) -> Self {
        Self {
            gradle_version: String::from(gradle_version),
            distribution_type,
            wrapper_checksum: String::from(wrapper_checksum),
            file_system_path: String::from(file_system_path),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradleRelease {
    gradle_version: String,
    distribution_type: DistributionType,
    wrapper_checksum: String,
}

impl GradleRelease {
    pub fn new(gradle_version: &str, distribution_type: DistributionType, wrapper_checksum: &str) -> Self {
        Self {
            gradle_version: String::from(gradle_version),
            distribution_type,
            wrapper_checksum: String::from(wrapper_checksum),
        }
    }
}

impl From<LocalGradleProject> for GradleRelease {
    fn from(project: LocalGradleProject) -> Self {
        GradleRelease {
            gradle_version: project.gradle_version,
            distribution_type: project.distribution_type,
            wrapper_checksum: project.wrapper_checksum,
        }
    }
}
