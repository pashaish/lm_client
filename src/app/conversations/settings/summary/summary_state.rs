use std::hash::Hash;

use framework::{types::dto::ConversationNodeDTO, utils::take_component};
use iced::widget::text_editor;

use crate::app::common::model_picker;


#[derive(Debug, Clone)]
pub enum Message {
    SummaryModelPicker(model_picker::Message),

    ToggleSummary(bool),
    UpdateSummaryContent(text_editor::Action),
    UpdateHandEditing(bool),
    UpdateConversation(ConversationNodeDTO),
    StartLoadingSummary,
}

impl Hash for Message {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[derive(Debug)]
pub struct Summary {
    // Components
    pub(super) summary_model_picker: model_picker::ModelPicker,

    // State
    pub(super) summary_content: text_editor::Content,
    pub(super) conversation: ConversationNodeDTO,
    pub(super) hand_editing: bool,
}

impl Summary {
    pub fn new(
        conversation: &ConversationNodeDTO,
    ) -> (Self, iced::Task<Message>) {
        let mut tasks = vec![]; 

        tasks.push(iced::Task::done(Message::StartLoadingSummary));

        (
            Self {
                summary_model_picker: take_component(
                    &mut tasks,
                    super::Message::SummaryModelPicker,
                    model_picker::ModelPicker::new(model_picker::ModelType::Summary(conversation.id)),
                ),
                hand_editing: false,
                summary_content: text_editor::Content::new(),
                conversation: conversation.clone(),
            },
            iced::Task::batch(tasks)
        )
    }
}
