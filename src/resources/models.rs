use crate::client::Client;
use crate::error;
use crate::types::{Capabilities, ModelList};

/// Resource for model discovery and capability introspection.
pub struct ModelsResource<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ModelsResource<'a> {
    /// `GET /v1/models` -- List available models.
    pub async fn list(&self) -> error::Result<ModelList> {
        self.client.get("/v1/models").await
    }

    /// `GET /capabilities` -- Discover available decision types and modalities.
    pub async fn capabilities(&self) -> error::Result<Capabilities> {
        self.client.get("/capabilities").await
    }
}
