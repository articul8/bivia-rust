use crate::client::Client;
use crate::error;
use crate::types::{
    BooleanParams, CategoricalDecision, ChoiceParams, DecideParams, ScoreDecision,
    ScoreParams, SetDecision, SetParams, SpanDecision, SpanParams,
};

/// Resource for the six core decision endpoints.
pub struct DecisionsResource<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> DecisionsResource<'a> {
    /// `POST /boolean` -- Yes/no decision with calibrated probability.
    pub async fn boolean(&self, params: BooleanParams) -> error::Result<CategoricalDecision> {
        self.client.post("/boolean", &params).await
    }

    /// `POST /choice` -- Pick one option from a list.
    pub async fn choice(&self, params: ChoiceParams) -> error::Result<CategoricalDecision> {
        self.client.post("/choice", &params).await
    }

    /// `POST /decide` -- Auto-detect the best decision type.
    pub async fn create(&self, params: DecideParams) -> error::Result<CategoricalDecision> {
        self.client.post("/decide", &params).await
    }

    /// `POST /score` -- Numeric score on a scale.
    pub async fn score(&self, params: ScoreParams) -> error::Result<ScoreDecision> {
        self.client.post("/score", &params).await
    }

    /// `POST /set` -- Multi-select from a list.
    pub async fn multi_select(&self, params: SetParams) -> error::Result<SetDecision> {
        self.client.post("/set", &params).await
    }

    /// `POST /span` -- Locate text spans in the input.
    pub async fn span(&self, params: SpanParams) -> error::Result<SpanDecision> {
        self.client.post("/span", &params).await
    }
}
