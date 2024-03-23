// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::validator::models::{GradleRelease, Result};
use anyhow::anyhow;

static HOST: &str = "https://cdn.statically.io/gh";
static GITHUB_REPO: &str = "gradle/wrapper-validation-action";
static CHECKSUMS_COLLECTION: &str = "main/src/wrapper-checksums.json";

pub fn fetch() -> Result<Vec<GradleRelease>> {
    let releases_url = format!("{}/{}/{}", HOST, GITHUB_REPO, CHECKSUMS_COLLECTION);

    let raw_response = ureq::get(&releases_url)
        .call()
        .map_err(|_| anyhow!("Failed to fetch Gradle checksums"))?;

    let releases = raw_response
        .into_json()
        .map_err(|_| anyhow!("Cannot parse Gradle checksums"))?;

    Ok(releases)
}
