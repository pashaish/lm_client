use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub mod open_ai_api;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageChunk
where
    Self: Sized + Send + 'static,
{
    pub role: String,
    pub content: String,
    pub reasoning_content: String,
}

impl MessageChunk {
    #[allow(dead_code)]
    #[must_use] pub const fn new(role: String, content: String, reasoning_content: String) -> Self {
        Self {
            role,
            content,
            reasoning_content,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ComplitationStatus
where
    Self: Sized + std::marker::Send + Sync + 'static,
{
    Start,
    End,
    Message(MessageChunk),
    Error(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbeddingResponse
    where
        Self: Sized + std::marker::Send + Sync + 'static,
{
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: EmbeddingUsage,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbeddingData
where
Self: Sized + std::marker::Send + Sync + 'static
{
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbeddingUsage
where
Self: Sized + std::marker::Send + Sync + 'static
{
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}
