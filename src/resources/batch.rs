use crate::client::Client;
use crate::error;
use crate::types::{BatchParams, BatchResponse};

/// Resource for the batch decision endpoint.
pub struct BatchResource<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> BatchResource<'a> {
    /// `POST /v1/systemone` -- Multiple typed questions in one request.
    pub async fn create(&self, params: BatchParams) -> error::Result<BatchResponse> {
        self.client.post("/v1/systemone", &params).await
    }
}
