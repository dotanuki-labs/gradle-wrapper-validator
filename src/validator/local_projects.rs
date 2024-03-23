// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::validator::models::{LocalGradleWrapper, Result};

pub fn locate(path: &str) -> Result<Vec<LocalGradleWrapper>> {
    let wrapper_checksum = "d3b261c2820e9e3d8d639ed084900f11f4a86050a8f83342ade7b6bc9b0d2bdd";
    let gradle_version = "8.5";
    let fake = LocalGradleWrapper::new(gradle_version, wrapper_checksum, path);
    Ok(vec![fake])
}

#[cfg(test)]
pub mod fakes {
    use crate::validator::models::{LocalGradleWrapper, Result};

    pub fn locate_tampered_project(path_name: &str) -> Result<Vec<LocalGradleWrapper>> {
        let fake = LocalGradleWrapper::new("8.5", "84900f11f4a86050a8f83342ade7b6bc9b0d2bdd-tampered", path_name);

        Ok(vec![fake])
    }

    pub fn locate_valid_project(path_name: &str) -> Result<Vec<LocalGradleWrapper>> {
        let fake = LocalGradleWrapper::new(
            "8.5",
            "d3b261c2820e9e3d8d639ed084900f11f4a86050a8f83342ade7b6bc9b0d2bdd",
            path_name,
        );

        Ok(vec![fake])
    }
}
