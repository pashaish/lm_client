use std::{collections::HashMap, hash::Hash, ops::Range};

use framework::{
    services::MessagingEvent, types::{common::ProgressStatus, dto::{ConversationNodeDTO, ConversationNodeID, MessageDTO, MessageID}}
};
use iced::{
    Task,
    widget::{scrollable::Viewport, text_editor},
};

use super::message_viewer::{self, MessageViewer};

#[derive(Debug, Clone)]
pub enum Message {
    #[allow(clippy::enum_variant_names)]
    UpdateMessage(MessageID, message_viewer::Message),
    #[allow(clippy::enum_variant_names)]
    UpdateGatheringMessage(message_viewer::Message),
    StartLoading,
    LoadedChat(Option<ConversationNodeDTO>),
    LoadedBatchMessages(Vec<MessageDTO>),
    EndLoadingMessages,
    #[allow(clippy::enum_variant_names)]
    CommitGatheringMessage(MessageDTO),
    #[allow(clippy::enum_variant_names)]
    OnScrollMessageList(Viewport),
    UpdateTextEditor(text_editor::Action),
    #[allow(clippy::enum_variant_names)]
    SendMessage,
    ChatUpdate(ConversationNodeDTO),
    #[allow(clippy::enum_variant_names)]
    DeleteMessage(MessageID),
    MessagingServiceEvent(MessagingEvent),
    DeleteConversation(ConversationNodeID),
    ToggleSettings(bool),
    LoadingFilesStatus(ProgressStatus),
    StopMessageLoading,
    Summarized(MessageDTO),
    StartSummarizing,
}

impl Hash for Message {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct Chat {
    // State
    pub(super) conversation_id: ConversationNodeID,
    pub(super) chat: Option<ConversationNodeDTO>,
    pub(super) messages: HashMap<MessageID, MessageViewer>,
    pub(super) gathering_message: Option<MessageViewer>,
    pub(super) is_loaded_all_messages: bool,
    pub(super) last_message_id: MessageID,
    pub(super) text_editor_content: text_editor::Content,
    pub(super) gathering_message_process: bool,
    pub(super) shared_messages_state: message_viewer::SharedState,
    pub(super) loading_file: bool,
    pub(super) text_editor_id: &'static str,
    pub(super) loading_progress: Option<(String, Range<f32>, f32)>,
    pub(super) gathering_message_aborter: Option<iced::task::Handle>,
    pub(super) is_need_generate: bool,
    pub(super) sorted_messages_ids: Vec<MessageID>,
}

impl Chat {
    pub fn new(conversation_id: ConversationNodeID) -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        tasks.push(Task::done(Message::StartLoading));

        (
            Self {
                is_need_generate: false,
                gathering_message_aborter: None,
                loading_progress: None,
                // text_editor_id: format!("text_editor_{conversation_id}"),
                text_editor_id: format!("text_editor_{}", conversation_id).leak(),
                conversation_id,
                chat: None,
                sorted_messages_ids: vec![],
                messages: HashMap::new(),
                is_loaded_all_messages: true,
                last_message_id: 0,
                text_editor_content: text_editor::Content::new(),
                gathering_message: None,
                gathering_message_process: false,
                shared_messages_state: message_viewer::SharedState::default(),
                loading_file: false,
            },
            iced::Task::batch(tasks),
        )
    }

    pub(super) fn get_sorted_messages(&self) -> Vec<&MessageViewer> {
        self.sorted_messages_ids
            .iter()
            .map(|id| self.messages.get(id).expect("Failed to get message"))
            .collect()
    }
}
