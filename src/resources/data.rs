use crate::client::Client;
use crate::error;
use crate::types::{CategoricalDecision, DataDecisionParams};

/// Resource for the data-science decision endpoint.
pub struct DataResource<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> DataResource<'a> {
    /// `POST /ds` -- Decision from raw numeric signals.
    pub async fn decide(&self, params: DataDecisionParams) -> error::Result<CategoricalDecision> {
        self.client.post("/ds", &params).await
    }
}
