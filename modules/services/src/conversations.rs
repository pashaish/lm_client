use std::{
    fmt::Debug, hash::Hash, sync::{Arc, RwLock}
};

use database::{
    databases::{ConversationDatabase, MessagesDatabase, PresetsDatabase, ProvidersDatabase}, DatabaseConnection
};
use types::dto::{
    ConversationNodeDTO, ConversationNodeID, MessageDTO, MessageID, PresetDTO, PresetId, ProviderDTO, ProviderID, RoleType
};
use utils::event_system::{Event, EventSystem};

use crate::VectorService;

struct SharedState {
    pub conversation_db: ConversationDatabase,
    pub messages_db: MessagesDatabase,
    pub preset_db: PresetsDatabase,
    pub vector_service: VectorService,
    pub provider_db: ProvidersDatabase,
}

#[derive(Clone)]
pub struct ConversationsService {
    state: Arc<RwLock<SharedState>>,
    event_system: EventSystem,
}

impl Debug for ConversationsService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConversationsService").finish()
    }
}

impl ConversationsService {
    pub fn new(
        connection: &DatabaseConnection,
        event_system: EventSystem,
        vector_service: VectorService,
    ) -> Self {
        Self {
            event_system,
            state: Arc::new(RwLock::new(SharedState {
                conversation_db: ConversationDatabase::new(connection.clone()),
                messages_db: MessagesDatabase::new(connection.clone()),
                preset_db: PresetsDatabase::new(connection.clone()),
                provider_db: ProvidersDatabase::new(connection.clone()),
                vector_service,
            })),
        }
    }

    /// # Errors
    /// # Panics
    pub fn update_conversation(
        &mut self,
        conversation_id: ConversationNodeID,
        new_dto: &ConversationNodeDTO,
    ) -> Result<(), String> {
        let state = self.state_write();

        let result = state
            .conversation_db
            .update(conversation_id, new_dto)
            .map_err(|e| format!("Failed to rename conversation: {e}"));

        let dto = state
            .conversation_db
            .get_conversation(conversation_id)
            .expect("Failed to get conversation after rename");

        self.event_system
            .clone()
            .dispatch(Event::ConversationUpdate(dto));

        result
    }

    /// # Errors
    pub fn delete_conversation(&self, conversation_id: ConversationNodeID) -> Result<(), String> {
        let state = self.state_read();

        self.delete_conversation_recursively(conversation_id, &state)
    }

    fn delete_conversation_recursively(&self, conversation_id: ConversationNodeID, state: &SharedState) -> Result<(), String> {
        let all_children = state.conversation_db.get_all_children_recursively(conversation_id)
            .map_err(|e| format!("Failed to get all children: {e}"))?;

        for child in all_children {
            self.delete_conversation(child.id)?;
        }

        state
            .conversation_db
            .delete(conversation_id)
            .map_err(|e| format!("Failed to delete conversation: {e}"))?;

        state
            .messages_db
            .delete_messages(conversation_id)
            .map_err(|e| format!("Failed to delete messages: {e}"))?;

        state.vector_service.delete_all_files(conversation_id)?;

        self.event_system
            .clone()
            .dispatch(Event::ConversationDelete(conversation_id));

        Ok(())
    }

    pub fn subscribe_delete_conversation<TMessage>(
        &self,
        conversation_id: ConversationNodeID,
        message: impl Fn(ConversationNodeID) -> TMessage + 'static,
    ) -> iced::Subscription<TMessage>
    where
        TMessage: Debug + Send + Clone + Hash + 'static,
    {
        self.event_system
            .subscribe(&Event::ConversationDelete(conversation_id), message)
    }

    /// # Errors
    pub fn move_conversation(
        &self,
        moving: ConversationNodeID,
        new_parent: ConversationNodeID,
        new_index: usize,
    ) -> Result<(), String> {
        let state = self.state_write();

        state
            .conversation_db
            .move_conversation(moving, new_parent, new_index)
            .map_err(|e| format!("Failed to move conversation: {e}"))
    }

    /// # Errors
    pub fn write_message(
        &self,
        conversation_id: ConversationNodeID,
        content: &str,
        reasoning: &str,
        role: &RoleType,
    ) -> Result<(), String> {
        let state = self.state_write();

        let result = state
            .messages_db
            .insert_message(
                conversation_id,
                content,
                reasoning,
                role,
                &[],
            )
            .map_err(|e| format!("Failed to write message: {e}"));

        let inseted_message = state
            .messages_db
            .get_last_messages(conversation_id, 0, 1)
            .map_err(|e| format!("Failed to get last messages: {e}"))?
            .into_iter()
            .next()
            .ok_or("Failed to get last message")?;

        self.event_system
            .clone()
            .dispatch(Event::ConversationReceiveMessage(inseted_message));

        result
    }

    /// # Errors
    pub fn get_last_messages(
        &self,
        conversation_id: ConversationNodeID,
        known_id: MessageID,
        limit: usize,
    ) -> Result<Vec<MessageDTO>, String> {
        let state = self.state_read();

        state
            .messages_db
            .get_last_messages(conversation_id, known_id, limit)
            .map_err(|e| format!("Failed to get last messages: {e}"))
    }

    /// # Errors
    pub fn get_conversation(
        &self,
        conversation_id: ConversationNodeID,
    ) -> Result<ConversationNodeDTO, String> {
        let state = self.state_read();

        state
            .conversation_db
            .get_conversation(conversation_id)
            .map_err(|e| format!("Failed to get conversation: {e}"))
    }

    /// # Errors
    pub fn delete_message(&mut self, message_id: MessageID) -> Result<(), String> {
        let state = self.state_write();

        let result = state
            .messages_db
            .delete_message(message_id)
            .map_err(|e| format!("Failed to delete message: {e}"));

        self.event_system
            .clone()
            .dispatch(Event::MessageDelete(message_id));

        result
    }

    /// # Errors
    pub fn insert_message_dto(&self, message_dto: &MessageDTO) -> Result<(), String> {
        let state = self.state_read();

        state
            .messages_db
            .insert_message_dto(message_dto.clone())
            .map_err(|e| format!("Failed to upsert message DTO: {e}"))
    }

    /// # Errors
    pub fn update_message_dto(&self, message_dto: &MessageDTO) -> Result<(), String> {
        let state = self.state_read();

        self.event_system
            .clone()
            .dispatch(Event::ConversationReceiveMessage(message_dto.clone()));

        log::debug!("Updating message DTO: {message_dto:?}");

        state
            .messages_db
            .update_message_dto(message_dto.clone())
            .map_err(|e| format!("Failed to upsert message DTO: {e}"))

    }

    /// # Panics
    /// # Errors
    pub fn get_message(&self, message_id: MessageID) -> Result<MessageDTO, String> {
        let state = self.state_read();

        state
            .messages_db
            .get_message(message_id)
            .map_err(|e| format!("Failed to get message: {e}"))
    }

    /// # Errors
    /// # Panics
    pub fn get_last_summary(
        &self,
        conversation_id: ConversationNodeID,
    ) -> Result<Option<MessageDTO>, String> {
        let state = self.state_read();

        let messages = state
            .messages_db
            .get_last_messages(conversation_id, 0, 5)
            .map_err(|e| format!("Failed to get last messages: {e}"))?;

        for message in messages.iter().rev() {
            if let Some(summary) = &message.summary {
                if !summary.is_empty() {
                    return Ok(Some(message.clone()));
                }
            }
        }

        Ok(None)
    }

    /// # Errors
    #[must_use] pub fn get_preset(
        &self,
        conversation_id: ConversationNodeID,
    ) -> Option<PresetDTO> {
        let state = self.state_read();

        let conversation = state
            .conversation_db
            .get_conversation(conversation_id);

        if let Ok(ref conversation) = conversation {
            if let Some(preset_id) = conversation.preset_id {
                let preset = state
                    .preset_db
                    .get_preset(preset_id);
    
                if let Ok(preset) = preset {
                    return Some(preset);
                }
            }
        }

        None
    }

    /// # Errors
    pub fn get_children(
        &self,
        conversation_id: ConversationNodeID,
    ) -> Result<Vec<ConversationNodeDTO>, String> {
        let state = self.state_read();

        state
            .conversation_db
            .get_children(conversation_id)
            .map_err(|e| format!("Failed to get children: {e}"))
    }

    /// # Errors
    pub fn add_folder(
        &self,
        name: &str,
        parent_id: ConversationNodeID,
    ) -> Result<ConversationNodeDTO, String> {
        let state = self.state_read();

        state
            .conversation_db
            .add_folder(name, parent_id, 20)
            .map_err(|e| format!("Failed to add folder: {e}"))
    }

    /// # Errors
    pub fn add_chat(
        &self,
        name: &str,
        parent_id: ConversationNodeID,
    ) -> Result<ConversationNodeDTO, String> {
        let state = self.state_read();

        state
            .conversation_db
            .add_chat(name, parent_id, 20)
            .map_err(|e| format!("Failed to add chat: {e}"))
    }

    pub fn update_subscribe<TMessage>(
        &self,
        chat: &ConversationNodeDTO,
        message: impl Fn(ConversationNodeDTO) -> TMessage + 'static,
    ) -> iced::Subscription<TMessage>
    where
        TMessage: Debug + Send + Clone + Hash + 'static,
    {
        self.event_system
            .subscribe(&Event::ConversationUpdate(chat.clone()), message)
    }

    pub fn update_subscribe_by_id<TMessage>(
        &self,
        chat_id: ConversationNodeID,
        message: impl Fn(ConversationNodeDTO) -> TMessage + 'static,
    ) -> iced::Subscription<TMessage>
    where
        TMessage: Debug + Send + Clone + Hash + 'static,
    {
        self.event_system.subscribe(
            &Event::ConversationUpdate(ConversationNodeDTO::empty_with_id(chat_id)),
            message,
        )
    }

    #[must_use] pub fn get_provider_dto(&self, provider_id: ProviderID) -> Option<ProviderDTO> {
        let state = self.state_read();

        state.provider_db.get_provider(provider_id)
    }

    /// # Errors
    /// # Panics
    pub fn set_preset(
        &self,
        id: ConversationNodeID,
        preset_id: Option<PresetId>,
    ) -> Result<(), String> {
        let state = self.state_write();

        state
            .conversation_db
            .set_preset(id, preset_id)
            .map_err(|e| format!("Failed to set preset: {e}"))
    }

    fn state_write(&self) -> std::sync::RwLockWriteGuard<SharedState> {
        self.state.write().expect("Failed to write to state")
    }

    fn state_read(&self) -> std::sync::RwLockReadGuard<SharedState> {
        self.state.read().expect("Failed to read from state")
    }
}
