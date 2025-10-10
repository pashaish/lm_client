use super::{PresetId, ProviderID};

pub type ConversationNodeID = i64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversationType {
    #[allow(dead_code)]
    Chat,
    Folder,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationNodeDTO {
    pub id: ConversationNodeID,
    #[allow(dead_code)]
    pub parent_id: ConversationNodeID,
    pub name: String,
    #[allow(dead_code)]
    pub tp: ConversationType,
    #[allow(dead_code)]
    pub order: i32,
    pub preset_id: Option<PresetId>,
    pub max_messages: usize,
    pub embedding_model: Option<String>,
    pub embedding_provider: Option<ProviderID>,
    pub rag_chunk_size: usize,
    pub rag_chunks_count: usize,
    pub summary_enabled: bool,
    pub summary_model: Option<String>,
    pub summary_provider: Option<ProviderID>,
    pub provider: Option<ProviderID>,
    pub model: Option<String>,
    pub prompt: String,
}

impl ConversationNodeDTO {
    #[must_use] pub const fn is_chat(&self) -> bool {
        matches!(self.tp, ConversationType::Chat)
    }

    #[must_use] pub const fn empty_with_id(id: ConversationNodeID) -> Self {
        Self {
            id,
            parent_id: 0,
            name: String::new(),
            tp: ConversationType::Chat,
            order: 0,
            preset_id: None,
            max_messages: 20,
            embedding_model: None,
            embedding_provider: None,
            rag_chunk_size: 512,
            rag_chunks_count: 2,
            summary_enabled: false,
            summary_model: None,
            summary_provider: None,

            provider: None,
            model: None,
            prompt: String::new(),
        }
    }
}
