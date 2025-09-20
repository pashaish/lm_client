
use framework::{
    types::dto::ConversationNodeID,
    utils::take_component,
};


use super::tree_node::{SharedState, TreeNode};

#[derive(Debug, Clone)]
pub struct Folders {
    // Components
    pub(super) root_folder: TreeNode,

    // State
    pub(super) shared_state: SharedState,
}

#[derive(Debug, Clone)]
pub enum Message {
    TreeNode(super::tree_node::Message),
    CreateFolder,
    CreateChat,
    Drag(ConversationNodeID),
    Drop(Option<(ConversationNodeID, usize)>),
    Selected(ConversationNodeID),
    ReleaseFreeArea,
}

impl Folders {
    pub fn new() -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        (
            Self {
                root_folder: take_component(
                    &mut tasks,
                    Message::TreeNode,
                    TreeNode::new_root(ConversationNodeID::default()),
                ),
                shared_state: SharedState::new(),
            },
            iced::Task::batch(tasks),
        )
    }
}
