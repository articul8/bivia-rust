//! Request and response types for the Bivia API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Input references ────────────────────────────────────────────

/// An image provided by URL or base64.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
}

/// An audio clip provided by URL or base64.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
}

/// A document provided by URL or base64.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
}

/// A raw numeric signal for data-science decisions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub data: Vec<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtype: Option<String>,
}

/// A video clip provided by URL or base64.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VideoRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_edge: Option<i64>,
}

// ── Decision request params ─────────────────────────────────────

/// Parameters for `POST /boolean`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct BooleanParams {
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<AudioRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_abstain: Option<bool>,
}

/// Parameters for `POST /choice`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ChoiceParams {
    pub question: String,
    pub choices: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<AudioRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_abstain: Option<bool>,
}

/// Parameters for `POST /decide`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DecideParams {
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<AudioRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_abstain: Option<bool>,
}

/// Parameters for `POST /score`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ScoreParams {
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<[f64; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<AudioRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_abstain: Option<bool>,
}

/// Parameters for `POST /set` (multi-select).
#[derive(Debug, Clone, Serialize, Default)]
pub struct SetParams {
    pub question: String,
    pub options: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<AudioRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_abstain: Option<bool>,
}

/// Parameters for `POST /span`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct SpanParams {
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

/// Parameters for `POST /ground`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GroundParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

/// Parameters for `POST /ground/video`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct VideoGroundParams {
    pub video: VideoRef,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coarse_step: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine_resolution: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boxes_on_hits: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_probes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_abstain: Option<bool>,
}

/// Parameters for `POST /v1/systemone` (batch).
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<AudioRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signals: Option<Vec<SignalRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub questions: Option<HashMap<String, BatchQuestion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// A single question within a batch request.
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchQuestion {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub question_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<i64>,
}

/// Parameters for `POST /ds`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DataDecisionParams {
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signals: Option<Vec<SignalRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<DocumentRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_abstain: Option<bool>,
}

// ── Response types ──────────────────────────────────────────────

/// Verification status from deep mode.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Verification {
    pub performed: bool,
    pub result: String,
}

/// A character span in the input text.
#[derive(Debug, Clone, Deserialize)]
pub struct EvidenceSpan {
    pub start: i64,
    pub end: i64,
    pub text: String,
}

/// Evidence supporting a decision.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Evidence {
    #[serde(default)]
    pub spans: Vec<EvidenceSpan>,
}

/// Per-document extraction status.
#[derive(Debug, Clone, Deserialize)]
pub struct DocumentNote {
    pub document: String,
    pub status: String,
    #[serde(default)]
    pub chars: Option<i64>,
    #[serde(default)]
    pub tokens: Option<i64>,
    #[serde(default)]
    pub truncated: Option<bool>,
    #[serde(default)]
    pub reason: Option<String>,
}

/// Response for boolean, choice, and decide endpoints.
#[derive(Debug, Clone, Deserialize)]
pub struct CategoricalDecision {
    #[serde(rename = "type")]
    pub decision_type: String,
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub probability: Option<f64>,
    #[serde(default)]
    pub distribution: Option<HashMap<String, f64>>,
    pub status: String,
    #[serde(default)]
    pub abstain_reason: Option<String>,
    pub mode: String,
    #[serde(default)]
    pub think_allowed: Option<bool>,
    pub thought: bool,
    #[serde(default)]
    pub grounded: Option<bool>,
    pub verification: Verification,
    pub evidence: Evidence,
    pub latency_ms: f64,
    pub model: String,
    #[serde(default)]
    pub input_tokens: Option<i64>,
    #[serde(default)]
    pub image_tokens: Option<i64>,
    #[serde(default)]
    pub document_tokens: Option<i64>,
    #[serde(default)]
    pub billable_tokens: Option<i64>,
    #[serde(default)]
    pub document_notes: Option<Vec<DocumentNote>>,
}

/// Response for the score endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct ScoreDecision {
    #[serde(rename = "type")]
    pub decision_type: String,
    pub value: Option<f64>,
    #[serde(default)]
    pub levels: Option<HashMap<String, f64>>,
    pub status: String,
    #[serde(default)]
    pub abstain_reason: Option<String>,
    pub mode: String,
    #[serde(default)]
    pub think_allowed: Option<bool>,
    pub thought: bool,
    #[serde(default)]
    pub grounded: Option<bool>,
    pub verification: Verification,
    pub evidence: Evidence,
    pub latency_ms: f64,
    pub model: String,
    #[serde(default)]
    pub input_tokens: Option<i64>,
    #[serde(default)]
    pub image_tokens: Option<i64>,
    #[serde(default)]
    pub document_tokens: Option<i64>,
    #[serde(default)]
    pub billable_tokens: Option<i64>,
    #[serde(default)]
    pub document_notes: Option<Vec<DocumentNote>>,
}

/// Response for the set endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct SetDecision {
    #[serde(rename = "type")]
    pub decision_type: String,
    #[serde(default)]
    pub value: Vec<serde_json::Value>,
    #[serde(default)]
    pub probabilities: Option<HashMap<String, f64>>,
    pub status: String,
    #[serde(default)]
    pub abstain_reason: Option<String>,
    pub mode: String,
    #[serde(default)]
    pub think_allowed: Option<bool>,
    pub thought: bool,
    #[serde(default)]
    pub grounded: Option<bool>,
    pub verification: Verification,
    pub evidence: Evidence,
    pub latency_ms: f64,
    pub model: String,
    #[serde(default)]
    pub input_tokens: Option<i64>,
    #[serde(default)]
    pub billable_tokens: Option<i64>,
    #[serde(default)]
    pub document_notes: Option<Vec<DocumentNote>>,
}

/// Response for the span endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct SpanDecision {
    #[serde(rename = "type")]
    pub decision_type: String,
    #[serde(default)]
    pub value: Vec<EvidenceSpan>,
    pub status: String,
    #[serde(default)]
    pub abstain_reason: Option<String>,
    pub mode: String,
    #[serde(default)]
    pub think_allowed: Option<bool>,
    pub thought: bool,
    #[serde(default)]
    pub grounded: Option<bool>,
    pub verification: Verification,
    pub evidence: Evidence,
    pub latency_ms: f64,
    pub model: String,
    #[serde(default)]
    pub input_tokens: Option<i64>,
    #[serde(default)]
    pub billable_tokens: Option<i64>,
    #[serde(default)]
    pub document_notes: Option<Vec<DocumentNote>>,
}

/// A detected bounding-box region in an image.
#[derive(Debug, Clone, Deserialize)]
pub struct GroundRegion {
    pub label: String,
    #[serde(rename = "box")]
    pub box_coords: Option<Vec<f64>>,
    #[serde(default)]
    pub score: Option<f64>,
}

/// Response for the ground endpoint.
///
/// The live reply omits the decision envelope (`status`, `mode`, `thought`,
/// `verification`, `evidence`, `latency_ms`, `model`). Those fields stay on the
/// type and default to empty values when absent. Other decision types still
/// require them.
#[derive(Debug, Clone, Deserialize)]
pub struct GroundDecision {
    #[serde(rename = "type")]
    pub decision_type: String,
    #[serde(default)]
    pub value: Vec<GroundRegion>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub abstain_reason: Option<String>,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub thought: bool,
    #[serde(default)]
    pub grounded: Option<bool>,
    #[serde(default)]
    pub verification: Verification,
    #[serde(default)]
    pub evidence: Evidence,
    #[serde(default)]
    pub latency_ms: f64,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub input_tokens: Option<i64>,
    #[serde(default)]
    pub image_tokens: Option<i64>,
    #[serde(default)]
    pub billable_tokens: Option<i64>,
}

/// A temporal hit in a video scan.
#[derive(Debug, Clone, Deserialize)]
pub struct VideoHit {
    #[serde(default)]
    pub start: Option<f64>,
    #[serde(default)]
    pub end: Option<f64>,
    #[serde(default)]
    pub peak_t: Option<f64>,
    #[serde(default)]
    pub onset_t: Option<f64>,
    #[serde(default)]
    pub confidence: Option<f64>,
    #[serde(default)]
    pub boxes: Option<Vec<serde_json::Value>>,
}

/// Response for the ground/video endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct VideoGroundDecision {
    #[serde(rename = "type")]
    pub decision_type: String,
    pub kind: String,
    pub duration_s: f64,
    #[serde(default)]
    pub scanned: Option<i64>,
    #[serde(default)]
    pub truncated: Option<bool>,
    #[serde(default)]
    pub hits: Vec<VideoHit>,
    #[serde(default)]
    pub input_tokens: Option<i64>,
    #[serde(default)]
    pub image_tokens: Option<i64>,
    #[serde(default)]
    pub billable_tokens: Option<i64>,
}

// ── Batch response types ────────────────────────────────────────

/// Token usage for a batch request.
#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub questions: i64,
    pub input_tokens: i64,
    #[serde(default)]
    pub output_tokens: Option<i64>,
    pub billable_tokens: i64,
}

/// Response for `POST /v1/systemone`.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchResponse {
    pub model: String,
    pub answers: HashMap<String, serde_json::Value>,
    pub usage: Usage,
}

// ── Models response types ───────────────────────────────────────

/// A model entry from `GET /v1/models`.
#[derive(Debug, Clone, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
}

/// Response for `GET /v1/models`.
#[derive(Debug, Clone, Deserialize)]
pub struct ModelList {
    pub models: Vec<ModelInfo>,
}

/// Modality capability flags.
#[derive(Debug, Clone, Deserialize)]
pub struct ModalityInfo {
    #[serde(default)]
    pub available: Option<bool>,
    #[serde(default)]
    pub grounding_available: Option<bool>,
}

/// Response for `GET /capabilities`.
#[derive(Debug, Clone, Deserialize)]
pub struct Capabilities {
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub decision_types: Option<Vec<String>>,
    #[serde(default)]
    pub modalities: Option<HashMap<String, ModalityInfo>>,
    #[serde(default)]
    pub evidence: Option<serde_json::Value>,
    #[serde(default)]
    pub abstention: Option<bool>,
}
