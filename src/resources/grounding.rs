use crate::client::Client;
use crate::error;
use crate::types::{GroundDecision, GroundParams, VideoGroundDecision, VideoGroundParams};

/// Resource for visual grounding endpoints.
pub struct GroundingResource<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> GroundingResource<'a> {
    /// `POST /ground` -- Locate objects in images via bounding boxes.
    pub async fn image(&self, params: GroundParams) -> error::Result<GroundDecision> {
        self.client.post("/ground", &params).await
    }

    /// `POST /ground/video` -- Locate objects or events in video.
    pub async fn video(&self, params: VideoGroundParams) -> error::Result<VideoGroundDecision> {
        self.client.post("/ground/video", &params).await
    }
}
