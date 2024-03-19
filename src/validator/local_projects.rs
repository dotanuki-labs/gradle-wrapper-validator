// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::validator::models::DistributionType;
use crate::validator::LocalGradleProject;

pub(crate) fn locate(path: &str) -> anyhow::Result<Vec<LocalGradleProject>> {
    let wrapper_checksum = "d3b261c2820e9e3d8d639ed084900f11f4a86050a8f83342ade7b6bc9b0d2bdd";
    let gradle_version = "8.5";
    let fake = LocalGradleProject::new(gradle_version, DistributionType::Stable, wrapper_checksum, path);
    Ok(vec![fake])
}
