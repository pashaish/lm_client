use framework::{
    services::ConversationsService,
    types::dto::{MessageDTO, MessageID, RoleType},
};
use iced::widget::{markdown, text_editor};

#[derive(Debug, Clone)]
pub enum Message {
    UpdateMessageDTO(MessageDTO),

    LinkClicked(iced::widget::markdown::Url),
    ReasoningExpanded(bool),

    StartEdit,
    EditReasoning(text_editor::Action),
    EditContent(text_editor::Action),
    SubmitEdit,
    CancelEdit,

    Delete,
    DeleteComplete,
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

    pub(super) content_string: String,
    pub(super) content: Vec<markdown::Item>,

    pub(super) reasoning_string: String,
    pub(super) reasoning: Vec<markdown::Item>,
    pub(super) reasoning_expanded: bool,

    pub(super) conversations_service: ConversationsService,
}

impl MessageViewer {
    pub fn new(
        conversation_service: ConversationsService,
        message_dto: MessageDTO,
    ) -> (Self, iced::Task<Message>) {
        let tasks = vec![];

        let initial_content = message_dto.content.clone();
        let initial_reasoning = message_dto.reasoning.clone().unwrap_or_default();

        (
            Self {
                conversations_service: conversation_service,
                message_dto,
                content_string: initial_content.clone(),
                content: markdown::parse(&initial_content).collect(),
                reasoning_string: initial_reasoning.clone(),
                reasoning: markdown::parse(&initial_reasoning).collect(),
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

        MessageDTO {
            id: origin_dto.id,
            conversation_id: origin_dto.conversation_id,
            content: self.content_string.clone(),
            reasoning: if self.reasoning_string.trim().is_empty() {
                None
            } else {
                Some(self.reasoning_string.trim().to_string())
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
