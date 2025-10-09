use std::{collections::HashMap, hash::Hash};

use framework::{types::dto::ConversationNodeID, utils::take_component};
use iced::widget::pane_grid::{self, ResizeEvent};

use super::{chat::Chat, folders::Folders};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Folders,
    Chat,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    Folders(super::folders::Message),
    Settings(super::settings::Message),
    Chat(ConversationNodeID, super::chat::Message),

    Resize(ResizeEvent),
    DeleteConversation(ConversationNodeID),

    HideSettingsPane,
    ShowSettingsPane,
}
impl Hash for Message {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        if let Self::Chat(id, _) = self {
            id.hash(state);
        }
    }
}

#[derive(Debug)]
pub struct Conversations {
    // Components
    pub(super) folders: Folders,
    pub(super) chats: HashMap<ConversationNodeID, Chat>,
    pub(super) settings: Option<super::settings::Settings>,

    // State
    pub(super) current_chat_id: Option<ConversationNodeID>,
    pub(super) panes: pane_grid::State<Pane>,
    pub(super) settings_pane: Option<pane_grid::Pane>,
    pub(super) chat_pane: pane_grid::Pane,
    pub(super) settings_pane_ratio: f32,
    pub(super) settings_pane_split: Option<pane_grid::Split>,
}

impl Conversations {
    pub fn new() -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        let (mut panes, folders_pane) = pane_grid::State::new(Pane::Folders);

        let (chat_pane, folders_split) = panes
            .split(pane_grid::Axis::Vertical, folders_pane, Pane::Chat)
            .expect("Failed to split pane");

        panes.resize(folders_split, 0.2);

        (
            Self {
                settings_pane_split: None,
                settings_pane_ratio: 0.8,
                chat_pane,
                settings_pane: None,
                panes,
                folders: take_component(&mut tasks, Message::Folders, Folders::new()),
                settings: None,
                chats: HashMap::new(),
                current_chat_id: None,
            },
            iced::Task::batch(tasks),
        )
    }

    pub fn get_chat(&self) -> Option<&Chat> {
        self.current_chat_id.and_then(|chat_id| self.chats.get(&chat_id))
    }

    #[allow(dead_code)]
    pub fn get_chat_mut(&mut self) -> Option<&mut Chat> {
        if let Some(chat_id) = self.current_chat_id {
            self.chats.get_mut(&chat_id)
        } else {
            None
        }
    }

    pub(super) const fn settings_expanded(&self) -> bool {
        self.settings_pane.is_some()
    }
}
