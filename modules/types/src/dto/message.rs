use serde::{Deserialize, Serialize};

use super::{ChunkRagId, ConversationNodeID};

pub type MessageID = i64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoleType {
    User,
    Assistant,
    System,
}

impl RoleType {
    #[allow(clippy::inherent_to_string)]
    #[must_use] pub fn to_string(&self) -> String {
        match self {
            Self::User => "user".to_string(),
            Self::Assistant => "assistant".to_string(),
            Self::System => "system".to_string(),
        }
    }
    
    /// # Panics
    #[allow(dead_code)]
    #[must_use] pub fn from_string(role: &str) -> Self {
        match role {
            "user" => Self::User,
            "system" => Self::System,
            "assistant" => Self::Assistant,
            _ => panic!("Invalid role type"),
        }
    }
}

impl TryFrom<String> for RoleType {
    type Error = String;

    fn try_from(role: String) -> Result<Self, Self::Error> {
        match role.as_str() {
            "user" => Ok(Self::User),
            "system" => Ok(Self::System),
            "assistant" => Ok(Self::Assistant),
            _ => Err(format!("Invalid role type: {role}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageUsedRagChunk {
    pub chunk_id: ChunkRagId,
    pub dimension: usize,
    pub embedding_model: String,
}

#[derive(Debug, Clone, Hash)]
pub struct MessageDTO {
    pub id: MessageID,
    pub conversation_id: ConversationNodeID,
    pub content: String,
    pub reasoning: Option<String>,
    pub timestamp: String,
    pub role: RoleType,
    pub summary: Option<String>,
    pub chunks: Vec<MessageUsedRagChunk>,
}

impl Default for MessageDTO {
    fn default() -> Self {
        Self {
            id: 0,
            conversation_id: 0,
            content: String::new(),
            reasoning: None,
            timestamp: String::new(),
            role: RoleType::User,
            summary: None,
            chunks: Vec::new(),
        }
    }
}
