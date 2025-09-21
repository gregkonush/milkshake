use std::time::Duration;

use anyhow::{Context, Result};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_ENGINE;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;

const BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
const USER_AGENT_STR: &str = "milkshake-cli/0.1";

pub struct GoogleImageClient {
    client: Client,
    api_key: String,
}

impl GoogleImageClient {
    pub fn new(api_key: String, timeout: Duration) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STR));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .timeout(timeout)
            .build()
            .context("failed to initialise HTTP client")?;

        Ok(Self { client, api_key })
    }

    pub fn generate_image(&self, request: &GenerateImageRequest) -> Result<GeneratedImage> {
        let model_path = normalise_model_path(&request.model);
        let url = format!("{BASE_URL}/{model_path}:generateContent");

        let payload = serde_json::json!({
            "contents": [{
                "role": "user",
                "parts": [{
                    "text": request.prompt
                }]
            }],
            "generationConfig": {
                "responseModalities": ["TEXT", "IMAGE"]
            }
        });

        let response = self
            .client
            .post(url)
            .header("x-goog-api-key", &self.api_key)
            .json(&payload)
            .send()
            .context("request to Gemini API failed")?;

        let status = response.status();
        let bytes = response.bytes().context("failed to read response body")?;

        if !status.is_success() {
            let body = String::from_utf8_lossy(&bytes);
            anyhow::bail!("Gemini API error ({}): {}", status, body);
        }

        let parsed: GenerateContentResponse =
            serde_json::from_slice(&bytes).context("failed to decode Gemini API response")?;

        let candidate = parsed
            .candidates
            .and_then(|candidates| candidates.into_iter().find(|c| c.content.is_some()))
            .context("Gemini API returned no candidates")?;

        let content = candidate
            .content
            .context("Gemini API response missing content")?;

        let part = content
            .parts
            .into_iter()
            .find(|part| part.inline_data.is_some())
            .context("Gemini API response contained no image data")?;

        let inline = part.inline_data.unwrap();
        let data = BASE64_ENGINE
            .decode(inline.data)
            .context("failed to decode base64 image data")?;

        Ok(GeneratedImage {
            mime_type: inline.mime_type,
            data,
            safety_ratings: candidate.safety_ratings,
            prompt_feedback: parsed.prompt_feedback,
        })
    }
}

fn normalise_model_path(model: &str) -> String {
    if model.starts_with("models/") {
        model.to_string()
    } else {
        format!("models/{}", model)
    }
}

pub struct GenerateImageRequest {
    pub prompt: String,
    pub model: String,
}

pub struct GeneratedImage {
    pub mime_type: String,
    pub data: Vec<u8>,
    pub safety_ratings: Option<Vec<SafetyRating>>,
    pub prompt_feedback: Option<PromptFeedback>,
}

#[derive(Debug, Deserialize)]
struct GenerateContentResponse {
    candidates: Option<Vec<Candidate>>,
    #[serde(rename = "promptFeedback")]
    prompt_feedback: Option<PromptFeedback>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: Option<Content>,
    #[serde(rename = "safetyRatings")]
    safety_ratings: Option<Vec<SafetyRating>>,
}

#[derive(Debug, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
struct Part {
    #[serde(rename = "inlineData")]
    inline_data: Option<InlineData>,
}

#[derive(Debug, Deserialize)]
struct InlineData {
    #[serde(rename = "mimeType")]
    mime_type: String,
    data: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SafetyRating {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "probability")]
    pub probability: String,
    #[serde(rename = "blocked")]
    pub blocked: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PromptFeedback {
    #[serde(rename = "safetyRatings")]
    pub safety_ratings: Option<Vec<SafetyRating>>,
}
