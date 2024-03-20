// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::validator::models::{DistributionType, GradleRelease, Result};

pub fn fetch() -> Result<Vec<GradleRelease>> {
    let gradle_84 = GradleRelease::new(
        "8.4",
        DistributionType::Stable,
        "0336f591bc0ec9aa0c9988929b93ecc916b3c1d52aed202c7381db144aa0ef15",
    );

    let gradle_85 = GradleRelease::new(
        "8.5",
        DistributionType::Stable,
        "d3b261c2820e9e3d8d639ed084900f11f4a86050a8f83342ade7b6bc9b0d2bdd",
    );

    let releases = vec![gradle_84, gradle_85];
    Ok(releases)
}
