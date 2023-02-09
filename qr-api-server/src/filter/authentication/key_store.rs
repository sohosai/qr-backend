use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use futures::future::TryFutureExt;
use jsonwebtoken::DecodingKey;
use reqwest::Client;
use serde::Deserialize;
use tokio::sync::RwLock;
use tracing::{event, span, Instrument, Level};
use url::Url;

#[derive(Debug, Clone)]
pub struct KeyStore {
    url: Url,
    client: Client,
    keys: Arc<RwLock<HashMap<String, DecodingKey<'static>>>>,
}

#[derive(Debug, Clone, Deserialize)]
struct Key {
    n: String,
    kid: String,
    e: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Response {
    keys: Vec<Key>,
}

impl KeyStore {
    pub fn new(url: Url) -> Self {
        KeyStore {
            url,
            client: Client::new(),
            keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn refresh(&self) -> Result<Option<u64>> {
        let response = self
            .client
            .get(self.url.clone())
            .send()
            .await
            .context("Failed to fetch from JWT keys URL")?;

        let status = response.status();
        if !status.is_success() {
            response
                .text()
                .map_ok(|text| event!(Level::ERROR, response = %text, %status))
        }
    }
}
