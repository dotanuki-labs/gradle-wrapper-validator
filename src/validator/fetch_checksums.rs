// Copyright 2025 Dotanuki Labs
// SPDX-License-Identifier: MIT

use crate::validator::OfficialWrapperChecksum;
use anyhow::Context;
use reqwest::header;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use std::time::Duration;

static HOST: &str = "https://cdn.statically.io/gh";
static GITHUB_REPO: &str = "gradle/actions";
static CHECKSUMS_COLLECTION: &str = "main/sources/src/wrapper-validation/wrapper-checksums.json";

static MAX_HTTP_RETRY_ATTEMPTS: u32 = 2;

pub async fn fetch() -> anyhow::Result<Vec<OfficialWrapperChecksum>> {
    let http_client = create_http_client()?;
    let releases_url = format!("{HOST}/{GITHUB_REPO}/{CHECKSUMS_COLLECTION}");

    let checksums = http_client
        .get(&releases_url)
        .send()
        .await
        .context("cannot fetch releases")?
        .error_for_status()?
        .json()
        .await
        .context("failed to parse gradle releases")?;

    Ok(checksums)
}

fn create_http_client() -> anyhow::Result<ClientWithMiddleware> {
    let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let mut headers = header::HeaderMap::new();
    let user_agent = header::HeaderValue::from_str(&user_agent).expect("invalid header value");
    let accept = header::HeaderValue::from_str("application/json").expect("invalid header value");
    headers.insert(header::USER_AGENT, user_agent);
    headers.insert(header::ACCEPT, accept);

    let base_http_client = reqwest::Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(60))
        .build()
        .expect("cannot build HTTP client");

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(MAX_HTTP_RETRY_ATTEMPTS);

    let retrier_http_client = reqwest_middleware::ClientBuilder::new(base_http_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    Ok(retrier_http_client)
}
