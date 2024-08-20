// Copyright 2023 Salesforce, Inc. All rights reserved.
mod generated;

use anyhow::{anyhow, Result};

use pdk::hl::*;
use pdk::logger;
use pdk::cache::{Cache, CacheBuilder};

use urlencoding::encode;
use base64::encode as base64_encode;
use serde_json::Value;
use serde_json::json;
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize}; 
use std::fmt;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use uuid::Uuid;

use crate::generated::config::Config;

// This filter shows how to log a specific request header.
// You can extend the function and use the configurations exposed in config.rs file
async fn request_filter(request_state: RequestState, _config: &Config) {
    let headers_state = request_state.into_headers_state().await;
    let token = headers_state.handler().header("Token").unwrap_or_default();
    // Log the header value
    logger::info!("Header value: {token}");
}

#[entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> Result<()> {
    let config: Config = serde_json::from_slice(&bytes).map_err(|err| {
        anyhow!(
            "Failed to parse configuration '{}'. Cause: {}",
            String::from_utf8_lossy(&bytes),
            err
        )
    })?;
    let filter = on_request(|rs| request_filter(rs, &config));
    launcher.launch(filter).await?;
    Ok(())
}
