use std::pin::Pin;

use futures_util::{Stream, StreamExt};
use reqwest_eventsource as SSE;
use serde::{Deserialize, Serialize};
use serde_json::json;

use database::{databases::{ProvidersDatabase, VectorDatabase}, DatabaseConnection};
use types::dto::{LmModel, MessageDTO, PresetDTO, ProviderID};

use crate::EmbeddingResponse;

use super::{ComplitationStatus, MessageChunk};

const REASONING_OPEN_TAGS: [&str; 4] = ["<reasoning>", "<think>", "<thinking>", "<reason>"];
const REASONING_CLOSE_TAGS: [&str; 4] = ["</reasoning>", "</think>", "</thinking>", "</reason>"];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionChunk {
    #[serde(default)]
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    #[serde(rename = "system_fingerprint", default)]
    pub system_fingerprint: String,
    pub choices: Vec<ChatCompletionChoice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionChoice {
    pub index: i64,
    pub delta: Option<ChatCompletionMessage>,
    pub message: Option<ChatCompletionMessage>, // Only present in first chunk
    pub logprobs: Option<serde_json::Value>,    // Can be null or object
    #[serde(rename = "finish_reason")]
    pub finish_reason: Option<String>, // Null until final chunk
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionMessage {
    #[serde(default)]
    pub role: String,
    pub content: Option<String>,
    #[serde(default)]
    pub reasoning_content: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GetModelsModel {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub owned_by: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetModelsResponse {
    pub object: String,
    pub data: Vec<GetModelsModel>,
}

#[derive(Debug, Clone)]
pub struct OpenAiApi {
    providers_db: ProvidersDatabase,
    vector_db: VectorDatabase,
}

impl OpenAiApi {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self {
            providers_db: ProvidersDatabase::new(connection.clone()),
            vector_db: VectorDatabase::new(connection),
        }
    }

    /// # Errors
    /// # Panics
    pub fn chat_completions(
        &self,
        lm_model: LmModel,
        messages_history: Vec<MessageDTO>,
        preset: Option<PresetDTO>,
        user_message: MessageDTO,
    ) -> Result<Pin<Box<dyn Stream<Item = ComplitationStatus> + Send>>, String> {
        if lm_model.provider.is_none() {
            log::error!("Provider is None");
            return Err("Provider not found".into());
        }

        let provider = lm_model.provider.unwrap();

        let client = match Self::build_client() {
            Ok(client) => client,
            Err(e) => {
                log::error!("Failed to build client: {e}");
                return Err(format!("Failed to build client: {e}"));
            }
        };

        let preset = preset.unwrap_or_default();

        let prompt_message = if preset.prompt.is_empty() {
            None
        } else {
            Some(MessageDTO {
                content: preset.prompt.clone(),
                conversation_id: 0,
                id: 0,
                reasoning: None,
                role: types::dto::RoleType::System,
                timestamp: String::new(),
                summary: None,
                chunks: Vec::new(),
            })
        };

        let mut messages = if let Some(prompt) = prompt_message {
            let mut messages = messages_history;
            messages.push(prompt);
            messages
        } else {
            messages_history
        };

        messages.push(user_message);

        let model = lm_model.model_name;
        let model = if model.is_empty() {
            log::error!("Model is empty");
            provider.default_model
        } else {
            model
        };

        let body = json!({
          "model": model,
          "messages": messages.iter().map(|message| json!({
            "content": message.content,
            "reasoning_content": message.reasoning,
            "role": message.role.to_string(),
          })).collect::<Vec<_>>(),
          "temperature": preset.temperature,
          "max_tokens": preset.max_tokens,
          "stream": true
        });

        log::debug!("Chat completion request body: {}", serde_json::to_string_pretty(&body).unwrap());

        let request = client
            .post(format!("{}/chat/completions", provider.url))
            .header("Accept", "text/event-stream")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", provider.api_key))
            .json(&body);

        let mut es = match SSE::EventSource::new(request) {
            Ok(source) => source,
            Err(e) => {
                return Err(format!("Failed to create event source: {e}"));
            }
        };

        let stream = Box::pin(async_stream::stream! {
            yield ComplitationStatus::Start;

            let mut state = SseEventSharedState::default();

            while let Some(event_result) = es.next().await {
                match handle_sse_event(event_result, &mut state) {
                    Ok(Some(event)) => {
                        let should_close = matches!(event, ComplitationStatus::Error(_) | ComplitationStatus::End);
                        yield event;

                        if should_close {
                            es.close();
                            break;
                        }
                    },
                    Ok(None) => continue,
                    Err(e) => {
                        yield ComplitationStatus::Error(e);
                        es.close();
                        break;
                    }
                }
            }
        });

        Ok(stream)
    }

    /// # Errors
    /// # Panics
    pub async fn embeddings(
        &self,
        embedding_lm_model: LmModel,
        inputs: Vec<String>,
    ) -> Result<EmbeddingResponse, String> {
        let client = Self::build_client().unwrap();

        let body = json!({
            "model": embedding_lm_model.model_name,
            "input": inputs,
            "encoding_format": "float",
        });

        let request = client
            .post(format!(
                "{}/embeddings",
                embedding_lm_model.clone().provider.unwrap().url
            ))
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", embedding_lm_model.provider.unwrap().api_key),
            )
            .json(&body);

        let response = request.send().await.expect("Failed to send request");

        if !response.status().is_success() {
            return Err(format!(
                "HTTP error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            ));
        }

        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {e}"))?;

        let json = serde_json::from_str::<EmbeddingResponse>(&text)
            .map_err(|e| format!("Failed to parse JSON: {e}"))?;

        if json.data.is_empty() {
            return Err("Empty data in response".to_string());
        }
        if json.data.len() != inputs.len() {
            return Err("Data length does not match input length".to_string());
        }

        Ok(json)
    }

    /// # Errors
    /// # Panics
    #[allow(dead_code)]
    pub async fn get_models(&self, provider_id: ProviderID) -> Result<Vec<String>, String> {
        let client = Self::build_client()?;

        let provider = self.providers_db.get_provider(provider_id);

        if provider.is_none() {
            return Err("Provider not found".into());
        }

        let provider = provider.unwrap();

        let response = client
            .get(format!("{}/models", provider.url))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", provider.api_key))
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?;

        if !response.status().is_success() {
            return Err(match response.status().as_u16() {
                401 => "Invalid API key".into(),
                429 => "Rate limit exceeded".into(),
                _ => format!(
                    "HTTP error: {} - {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            });
        }

        let text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {e}"))?;

        let json = serde_json::from_str::<GetModelsResponse>(&text)
            .map_err(|e| format!("Failed to parse JSON: {e}"))?;

        let models = json.data.into_iter().map(|model| model.id).collect();

        Ok(models)
    }

    /// # Errors
    /// # Panics
    fn build_client() -> Result<reqwest::Client, String> {
        reqwest::Client::builder()
            .build()
            .map_err(|err| err.to_string())
    }
}

#[derive(Debug, Default)]
struct SseEventSharedState {
    is_reasoning: bool,
}

fn handle_sse_event(
    event_result: Result<reqwest_eventsource::Event, reqwest_eventsource::Error>,
    state: &mut SseEventSharedState,
) -> Result<Option<ComplitationStatus>, String> {
    match event_result {
        Ok(SSE::Event::Open) => Ok(None),
        Ok(SSE::Event::Message(message_raw_event)) => {
            let message_raw = message_raw_event.data;

            if message_raw == "[DONE]" {
                return Ok(Some(ComplitationStatus::End));
            }

            match serde_json::from_str::<ChatCompletionChunk>(&message_raw) {
                Ok(message) => {
                    if message.choices.is_empty() {
                        return Err("Empty choices in response".to_string());
                    }

                    let choice = &message.choices[0];
                    let delta = match &choice.delta {
                        Some(delta) => delta.clone(),
                        None => return Err("Delta is None".to_string()),
                    };

                    let content_str = delta.content.clone().unwrap_or_default();
                    let content = content_str.as_str();

                    if REASONING_OPEN_TAGS.contains(&content) {
                        if state.is_reasoning {
                            return Err("Nested reasoning tags".to_string());
                        }

                        state.is_reasoning = true;
                        return Ok(None);
                    }

                    if REASONING_CLOSE_TAGS.contains(&content) {
                        if !state.is_reasoning {
                            return Err("Unmatched reasoning close tag".to_string());
                        }

                        state.is_reasoning = false;
                        return Ok(None);
                    }

                    if state.is_reasoning {
                        let mut reasoning_content =
                            delta.reasoning_content.clone().unwrap_or_default();
                        reasoning_content.push_str(&content_str);
                        return Ok(Some(ComplitationStatus::Message(MessageChunk {
                            role: delta.role,
                            content: String::new(),
                            reasoning_content,
                        })));
                    }

                    let message_chunk = MessageChunk {
                        role: delta.role.clone(),
                        content: content_str,
                        reasoning_content: delta.reasoning_content.unwrap_or_default(),
                    };

                    Ok(Some(ComplitationStatus::Message(message_chunk)))
                }
                Err(error) => Err(format!("Failed to parse message: {error}")),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}
