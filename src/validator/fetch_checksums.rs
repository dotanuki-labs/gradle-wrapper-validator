// Copyright 2024 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::validator::models::OfficialWrapperChecksum;
use anyhow::anyhow;

static HOST: &str = "https://cdn.statically.io/gh";
static GITHUB_REPO: &str = "gradle/actions";
static CHECKSUMS_COLLECTION: &str = "main/sources/src/wrapper-validation/wrapper-checksums.json";

pub fn fetch() -> anyhow::Result<Vec<OfficialWrapperChecksum>> {
    let releases_url = format!("{}/{}/{}", HOST, GITHUB_REPO, CHECKSUMS_COLLECTION);

    let raw_response = reqwest::blocking::get(&releases_url).expect("Cannot fetch checksums");

    let checksums = raw_response
        .json()
        .map_err(|_| anyhow!("Cannot parse Gradle checksums"))?;

    Ok(checksums)
}
