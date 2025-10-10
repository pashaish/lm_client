use crate::app::common::markdown_viewer;

use super::{MessageViewer, message_viewer_state::SharedState};
use framework::{Context, types::dto::MessageDTO, utils::notify};
use iced::{
    Task,
    widget::{text_editor},
};

impl MessageViewer {
    pub fn update(
        &mut self,
        message: super::Message,
        state: &mut SharedState,
    ) -> Task<super::Message> {
        match message {
            super::Message::ContentUpdate(msg) => self.content.update(msg).map(super::Message::ContentUpdate),
            super::Message::ReasoningUpdate(msg) => self.reasoning.update(msg).map(super::Message::ReasoningUpdate),
            super::Message::Delete => {
                let conversations_service = self.conversations_service.clone();
                let dto_id = self.message_dto.id;
                Task::perform(
                    async move {
                        if notify::confirmation("Are you sure you want to delete this message?")
                            .await
                        {
                            conversations_service
                                .clone()
                                .delete_message(dto_id)
                                .expect("Failed to delete message DTO");
                        }
                    },
                    |()| super::Message::DeleteComplete,
                )
            }
            super::Message::DeleteComplete => Task::none(),
            super::Message::StartEdit => {
                state.editing = Some(self.message_dto.id);
                state.editing_tmp_content = text_editor::Content::with_text(&self.content.get_original());
                state.editing_tmp_reasoning =
                    text_editor::Content::with_text(&self.reasoning.get_original());
                Task::none()
            }
            super::Message::EditContent(action) => {
                state.editing_tmp_content.perform(action);
                Task::none()
            }
            super::Message::EditReasoning(action) => {
                state.editing_tmp_reasoning.perform(action);
                Task::none()
            }
            super::Message::CancelEdit => {
                state.editing = None;
                Task::none()
            }
            super::Message::SubmitEdit => {
                state.editing = None;
                self.message_dto.content = state.editing_tmp_content.text();
                self.message_dto.reasoning = Some(state.editing_tmp_reasoning.text());

                let dto = MessageDTO {
                    content: state.editing_tmp_content.text(),
                    reasoning: Some(state.editing_tmp_reasoning.text()),
                    conversation_id: self.message_dto.conversation_id,
                    id: self.message_dto.id,
                    timestamp: self.message_dto.timestamp.clone(),
                    role: self.message_dto.role.clone(),
                    summary: self.message_dto.summary.clone(),
                    chunks: self.message_dto.chunks.clone(),
                };

                let service = self.conversations_service.clone();

                Task::perform(
                    async move {
                        service
                            .update_message_dto(&dto)
                            .expect("Failed to upsert message DTO");

                        service
                            .get_message(dto.id)
                            .expect("Failed to get conversation")
                    },
                    super::Message::UpdateMessageDTO,
                )
            }
            super::Message::UpdateMessageDTO(dto) => {
                log::debug!("Update message DTO: {:?}", dto);
                self.message_dto = dto.clone();
                self.content.set_original(dto.content);
                self.reasoning.set_original(dto.reasoning.unwrap_or_default());

                Task::none()
            }
            // super::Message::LinkClicked(_url) => Task::none(),
            super::Message::ReasoningExpanded(expanded) => {
                self.reasoning_expanded = expanded;
                Task::none()
            }
        }
    }

    pub fn append_content(&mut self, new_content: &str) {
        self.content.append(new_content);
    }

    pub fn append_reasoning(&mut self, new_content: &str) {
        self.reasoning.append(new_content);
    }
}
