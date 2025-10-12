use framework::{
    services::ConversationsService,
    types::dto::{MessageDTO, MessageID, RoleType}, utils::take_component,
};
use iced::{Rectangle, widget::text_editor};

use crate::{app::common::markdown_viewer::{self, MarkdownViewer, MarkdownViewerConfig}, theme::dark_theme::dark_theme_pallete};

#[derive(Debug, Clone)]
pub enum Message {
    UpdateMessageDTO(MessageDTO),

    // LinkClicked(iced::widget::markdown::Url),
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

    RequestVisibleBounds,
    VisibleBounds(Option<Rectangle>),
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

    pub(super) id: iced::widget::container::Id,

    pub(super) visible: bool,
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
                visible: false,
                id: iced::widget::container::Id::unique(),

                conversations_service: conversation_service,
                message_dto: message_dto.clone(),
                content: take_component(
                    &mut tasks,
                    Message::ContentUpdate,
                    MarkdownViewer::new(&initial_content, md_style_config(&message_dto))
                ),
                reasoning: take_component(
                    &mut tasks,
                    Message::ReasoningUpdate,
                    MarkdownViewer::new(&initial_reasoning, md_style_config(&message_dto))
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

fn md_style_config(dto: &MessageDTO) -> MarkdownViewerConfig {
    MarkdownViewerConfig {
        heading_color: if dto.role == RoleType::User {
            dark_theme_pallete().text
        } else {
            dark_theme_pallete().primary
        }
    }
}