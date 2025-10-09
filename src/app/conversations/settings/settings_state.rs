use std::collections::HashMap;

use framework::{types::dto::ConversationNodeDTO, utils::take_component};

use super::basic::Basic;

#[derive(Debug, Clone, Hash)]
pub enum Message {
    Summary(super::summary::Message),
    Rag(super::rag::Message),
    Basic(super::basic::Message),
    
    ClearView,
    ToggleGroup(String),
    UpdateConversation(ConversationNodeDTO),
}

#[derive(Debug)]
pub struct Settings {
    // Components
    pub(super) basic: super::basic::Basic,
    pub(super) rag: super::rag::Rag,
    pub(super) summary: super::summary::Summary,

    // State
    pub(super) conversation: ConversationNodeDTO,
    pub(super) groups_expaned: HashMap<String, bool>,
}

impl Settings {
    pub fn new_full(conversation: ConversationNodeDTO, groups_expaned: HashMap<String, bool>) -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        (
            Self {
                groups_expaned,
                summary: take_component(
                    &mut tasks,
                    super::Message::Summary,
                    super::summary::Summary::new(&conversation),
                ),
                rag: take_component(
                    &mut tasks,
                    super::Message::Rag,
                    super::rag::Rag::new(&conversation),
                ),
                basic: take_component(
                    &mut tasks,
                    super::Message::Basic,
                    Basic::new(&conversation),
                ),
                conversation,
            },
            iced::Task::batch(tasks),
        )
    }

    pub fn get_groups_expaned(&self) -> HashMap<String, bool> {
        self.groups_expaned.clone()
    }
}
