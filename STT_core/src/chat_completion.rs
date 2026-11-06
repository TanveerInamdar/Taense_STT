//! OpenAI-compatible `POST /v1/chat/completions` response (llama-server / S1-mini).

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<ChatChoice>,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub object: String,
    pub usage: Usage,
    pub id: String,
    /// Present on llama.cpp server responses; omitted on some OpenAI-compatible APIs.
    pub timings: Option<Timings>,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub finish_reason: String,
    pub index: u32,
    pub message: AssistantMessage,
}

#[derive(Debug, Deserialize)]
pub struct AssistantMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
    pub prompt_tokens_details: Option<PromptTokenDetails>,
}

#[derive(Debug, Deserialize)]
pub struct PromptTokenDetails {
    pub cached_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct Timings {
    pub cache_n: u32,
    pub prompt_n: u32,
    pub prompt_ms: f64,
    pub prompt_per_token_ms: f64,
    pub prompt_per_second: f64,
    pub predicted_n: u32,
    pub predicted_ms: f64,
    pub predicted_per_token_ms: f64,
    pub predicted_per_second: f64,
}

impl ChatCompletionResponse {
    /// Normalized text from the first choice (what you usually paste).
    pub fn assistant_content(&self) -> Option<&str> {
        self.choices
            .first()
            .map(|choice| choice.message.content.as_str())
    }
}

pub fn parse_chat_completion(body: &str) -> Result<ChatCompletionResponse, serde_json::Error> {
    serde_json::from_str(body)
}
