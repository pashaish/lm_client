use std::hash::Hash;

use framework::{
    services::ConversationsService,
    types::dto::{MessageDTO, MessageID, RoleType}, utils::take_component,
};
use iced::widget::{text_editor};

use crate::app::common::markdown_viewer::{self, MarkdownViewer};

#[derive(Debug, Clone)]
pub enum Message {
    UpdateMessageDTO(MessageDTO),

    ReasoningExpanded(bool),

    StartEdit,
    EditReasoning(text_editor::Action),
    EditContent(text_editor::Action),
    SubmitEdit,
    CancelEdit,

    Delete,
    DeleteComplete,

    ContentUpdate(markdown_viewer::Message),
    ReasoningUpdate(markdown_viewer::Message),
}

impl Hash for Message {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::UpdateMessageDTO(data) => data.id.hash(state),
            Self::ReasoningExpanded(data) => data.hash(state),
            Self::StartEdit => 1.hash(state),
            Self::EditReasoning(data) => {},
            Self::EditContent(data) => {},
            Self::SubmitEdit => 2.hash(state),
            Self::CancelEdit => 3.hash(state),
            Self::Delete => 4.hash(state),
            Self::DeleteComplete => 5.hash(state),
            Self::ContentUpdate(data) => data.hash(state),
            Self::ReasoningUpdate(data) => data.hash(state),
        }
    }
}

#[derive(Debug, Default)]
pub struct SharedState {
    pub(super) editing: Option<MessageID>,
    pub(super) editing_tmp_content: text_editor::Content,
    pub(super) editing_tmp_reasoning: text_editor::Content,
}

#[derive(Debug, Clone)]
pub struct MessageViewer {
    pub(super) message_dto: MessageDTO,

    pub(super) content: MarkdownViewer,

    pub(super) reasoning: MarkdownViewer,
    pub(super) reasoning_expanded: bool,

    pub(super) conversations_service: ConversationsService,
}

impl MessageViewer {
    pub fn new(
        conversation_service: ConversationsService,
        message_dto: MessageDTO,
    ) -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        let initial_content = message_dto.content.clone();
        let initial_reasoning = message_dto.reasoning.clone().unwrap_or_default();

        (
            Self {
                conversations_service: conversation_service,
                message_dto,
                content: take_component(
                    &mut tasks,
                    Message::ContentUpdate,
                    MarkdownViewer::new(&initial_content)
                ),
                reasoning: take_component(
                    &mut tasks,
                    Message::ReasoningUpdate,
                    MarkdownViewer::new(&initial_reasoning)
                ),
                reasoning_expanded: false,
            },
            iced::Task::batch(tasks),
        )
    }

    pub const fn get_id(&self) -> MessageID {
        self.message_dto.id
    }

    pub fn get_dto(&self) -> MessageDTO {
        let origin_dto = self.message_dto.clone();
        let reasoning_trimmed = self.reasoning.get_original().trim(); 

        MessageDTO {
            id: origin_dto.id,
            conversation_id: origin_dto.conversation_id,
            content: self.content.get_original().to_string(),
            reasoning: if reasoning_trimmed.is_empty() {
                None
            } else {
                Some(reasoning_trimmed.to_string())
            },
            timestamp: origin_dto.timestamp,
            role: origin_dto.role,
            summary: origin_dto.summary,
            chunks: origin_dto.chunks.clone(),
        }
    }

    pub fn is_user_message(&self) -> bool {
        self.message_dto.role == RoleType::User
    }

    pub fn is_editing(&self, state: &SharedState) -> bool {
        state.editing == Some(self.message_dto.id)
    }
}
