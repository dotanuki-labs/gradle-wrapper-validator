// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::validator::models::{GradleRelease, Result};

static HOST: &str = "https://cdn.statically.io/gh";
static GITHUB_REPO: &str = "gradle/wrapper-validation-action";
static CHECKSUMS_COLLECTION: &str = "main/src/wrapper-checksums.json";

pub fn fetch() -> Result<Vec<GradleRelease>> {
    let releases_url = format!("{}/{}/{}", HOST, GITHUB_REPO, CHECKSUMS_COLLECTION);
    let releases: Vec<GradleRelease> = ureq::get(&releases_url).call()?.into_json()?;

    Ok(releases)
}
