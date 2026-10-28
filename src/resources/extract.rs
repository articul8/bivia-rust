use crate::client::Client;
use crate::error;
use crate::types::{ExtractDecision, ExtractParams};

/// Resource for the `/extract` endpoint.
pub struct ExtractResource<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> ExtractResource<'a> {
    /// `POST /extract` -- Extract exact field values from document text.
    pub async fn extract(&self, params: ExtractParams) -> error::Result<ExtractDecision> {
        self.client.post("/extract", &params).await
    }
}
