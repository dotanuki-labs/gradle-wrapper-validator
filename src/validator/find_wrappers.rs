// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::validator::LocalGradleWrapper;
use anyhow::{anyhow, ensure};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn find(raw_path: &str) -> anyhow::Result<Vec<LocalGradleWrapper>> {
    let mut found_wrappers: Vec<LocalGradleWrapper> = WalkDir::new(raw_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(parse_wrapper_info)
        .filter_map(|parsed| parsed.ok())
        .collect();

    ensure!(!&found_wrappers.is_empty(), "No wrappers found");

    found_wrappers.sort_by(|some, another| some.file_system_path.cmp(&another.file_system_path));
    Ok(found_wrappers)
}

fn parse_wrapper_info(entry: DirEntry) -> anyhow::Result<LocalGradleWrapper> {
    let raw_path = entry.path().to_string_lossy();
    let complete_path = format!("{}/gradle/wrapper/gradle-wrapper.jar", &raw_path);
    let wrapper_jar_path = Path::new(&complete_path);
    let wrapper_checksum = sha256::try_digest(wrapper_jar_path).map_err(|_| anyhow!(""))?;
    let wrapper = LocalGradleWrapper::new(complete_path.as_ref(), &wrapper_checksum);

    Ok(wrapper)
}

#[cfg(test)]
mod tests {
    use crate::validator::find_wrappers::find;
    use crate::validator::LocalGradleWrapper;

    fn project_dir() -> String {
        let root_dir = std::env::current_dir().unwrap();
        String::from(root_dir.to_str().unwrap())
    }

    #[test]
    fn should_locate_gradle_wrappers() {
        let test_data_dir = format!("{}/test-data/valid", project_dir());

        let found_wrappers = find(&test_data_dir).unwrap();

        let gradle7_wrapper_path = format!("{}/gradle7/gradle/wrapper/gradle-wrapper.jar", &test_data_dir);
        let gradle7_wrapper_sha256 = "575098db54a998ff1c6770b352c3b16766c09848bee7555dab09afc34e8cf590";
        let gradle7wrapper = LocalGradleWrapper::new(&gradle7_wrapper_path, gradle7_wrapper_sha256);

        let gradle8_wrapper_path = format!("{}/gradle8/gradle/wrapper/gradle-wrapper.jar", &test_data_dir);
        let gradle8_wrapper_sha256 = "0336f591bc0ec9aa0c9988929b93ecc916b3c1d52aed202c7381db144aa0ef15";
        let gradle8wrapper = LocalGradleWrapper::new(&gradle8_wrapper_path, gradle8_wrapper_sha256);

        let expected = vec![gradle7wrapper, gradle8wrapper];

        assert_eq!(found_wrappers, expected)
    }

    #[test]
    fn should_handle_no_wrappers_found() {
        let github_dir = format!("{}/.github", project_dir());

        let found_wrappers = find(&github_dir);

        assert!(found_wrappers.is_err());
    }
}
