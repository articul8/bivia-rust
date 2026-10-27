use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

use crate::error::BiviaError;
use crate::resources::{
    BatchResource, DataResource, DecisionsResource, GroundingResource, ModelsResource,
};

const DEFAULT_BASE_URL: &str = "https://api.articul8.ai/bivia";
const DEFAULT_TIMEOUT_SECS: u64 = 60;
const DEFAULT_MAX_RETRIES: u32 = 2;

/// The Bivia API client.
///
/// Reads `A8_API_KEY` from the environment by default. Use [`ClientBuilder`] via
/// [`Client::builder()`] for full control over API key, base URL, timeout, and retries.
pub struct Client {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) max_retries: u32,
}

impl Client {
    /// Create a new client, reading `A8_API_KEY` from the environment.
    pub fn new() -> crate::error::Result<Self> {
        let api_key =
            std::env::var("A8_API_KEY").map_err(|_| BiviaError::MissingApiKey)?;
        Ok(ClientBuilder::default().api_key(api_key).build())
    }

    /// Create a builder for full configuration control.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Access the Decisions resource (boolean, choice, decide, score, set, span).
    pub fn decisions(&self) -> DecisionsResource<'_> {
        DecisionsResource { client: self }
    }

    /// Access the Grounding resource (image and video grounding).
    pub fn grounding(&self) -> GroundingResource<'_> {
        GroundingResource { client: self }
    }

    /// Access the Batch resource (multiple typed questions in one request).
    pub fn batch(&self) -> BatchResource<'_> {
        BatchResource { client: self }
    }

    /// Access the Data Science resource (decisions over raw signals).
    pub fn data(&self) -> DataResource<'_> {
        DataResource { client: self }
    }

    /// Access the Models resource (list models, get capabilities).
    pub fn models(&self) -> ModelsResource<'_> {
        ModelsResource { client: self }
    }

    /// Send a POST request with retries on 429/5xx.
    pub(crate) async fn post<T, R>(&self, path: &str, body: &T) -> crate::error::Result<R>
    where
        T: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut last_err = None;

        for _ in 0..=self.max_retries {
            match self.http.post(&url).json(body).send().await {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    let text = resp.text().await.unwrap_or_default();

                    if status >= 400 {
                        let message = serde_json::from_str::<serde_json::Value>(&text)
                            .ok()
                            .and_then(|v| {
                                v.get("detail")
                                    .or_else(|| v.get("error"))
                                    .and_then(|d| d.as_str())
                                    .map(String::from)
                            })
                            .unwrap_or_else(|| text.clone());
                        let err = BiviaError::Api { status, message, body: text };
                        if status == 429 || status >= 500 {
                            last_err = Some(err);
                            continue;
                        }
                        return Err(err);
                    }
                    return serde_json::from_str(&text).map_err(BiviaError::from);
                }
                Err(e) => {
                    last_err = Some(BiviaError::from(e));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }

    /// Send a GET request with retries on 429/5xx.
    pub(crate) async fn get<R>(&self, path: &str) -> crate::error::Result<R>
    where
        R: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut last_err = None;

        for _ in 0..=self.max_retries {
            match self.http.get(&url).send().await {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    let text = resp.text().await.unwrap_or_default();

                    if status >= 400 {
                        let message = serde_json::from_str::<serde_json::Value>(&text)
                            .ok()
                            .and_then(|v| {
                                v.get("detail")
                                    .or_else(|| v.get("error"))
                                    .and_then(|d| d.as_str())
                                    .map(String::from)
                            })
                            .unwrap_or_else(|| text.clone());
                        let err = BiviaError::Api { status, message, body: text };
                        if status == 429 || status >= 500 {
                            last_err = Some(err);
                            continue;
                        }
                        return Err(err);
                    }
                    return serde_json::from_str(&text).map_err(BiviaError::from);
                }
                Err(e) => {
                    last_err = Some(BiviaError::from(e));
                    continue;
                }
            }
        }
        Err(last_err.unwrap())
    }
}

/// Builder for configuring a [`Client`].
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Duration,
    max_retries: u32,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            max_retries: DEFAULT_MAX_RETRIES,
        }
    }
}

impl ClientBuilder {
    /// Set the API key directly (overrides `A8_API_KEY`).
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Override the base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the request timeout (default: 60s).
    pub fn timeout(mut self, d: Duration) -> Self {
        self.timeout = d;
        self
    }

    /// Set the max retries on 429/5xx (default: 2).
    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    /// Build the client.
    pub fn build(self) -> Client {
        let api_key = self.api_key
            .or_else(|| std::env::var("A8_API_KEY").ok())
            .unwrap_or_default();
        let base_url = self.base_url
            .or_else(|| std::env::var("BIVIA_BASE_URL").ok())
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
            .trim_end_matches('/')
            .to_string();

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(self.timeout)
            .build()
            .expect("failed to build HTTP client");

        Client { http, base_url, max_retries: self.max_retries }
    }
}
