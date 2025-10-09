use std::{collections::HashMap, hash::Hash};

use framework::types::dto::{ConversationNodeDTO, ConversationNodeID, ConversationType};
use iced::Point;

#[derive(Clone, Debug)]
pub enum NodeAction {
    StartFolderCreate,
    FolderCreated(ConversationNodeDTO),

    StartConversationCreate,
    ConversationCreated(ConversationNodeDTO),

    StartLoadingChildren,
    LoadedChildren(Vec<ConversationNodeDTO>),

    StartActualize,
    Actualized(ConversationNodeDTO),
    ActualizedName(String),

    StartActualizeChildren,
    ActualizedChildren(Vec<ConversationNodeDTO>),

    Hover(bool),

    #[allow(dead_code)]
    Drag,
    #[allow(dead_code)]
    Drop,

    Press,
    ReleaseChat,
    ReleaseFolder,
    MouseMoved(Point<f32>),

    HoverInsertPlace(Option<usize>),

    StartRename,
    RenameProcess(String),
    RenameStartSave,
    RenameCancel,
    RenameError(String),

    StartDelete,
}

impl Hash for NodeAction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[derive(Debug, Clone, Hash)]
pub enum Message {
    NodeAction(ConversationNodeID, NodeAction),
}

#[derive(Debug, Clone, Hash)]
pub struct FolderDescriptor {
    pub(super) children: Vec<TreeNode>,
}

#[derive(Debug, Clone, Hash)]
pub struct FileDescriptor {}

#[derive(Debug, Clone, Hash)]
pub enum Content {
    Folder(FolderDescriptor),
    #[allow(dead_code)]
    Chat(FileDescriptor),
    Loading,
}

impl Content {
    #[allow(dead_code)]
    pub const fn is_folder(&self) -> bool {
        matches!(self, Self::Folder(_))
    }

    pub fn expect_folder(&self) -> &FolderDescriptor {
        if let Self::Folder(descriptor) = self {
            return descriptor;
        }

        panic!("Expected folder, but found {self:?}");
    }

    #[allow(dead_code)]
    pub fn expect_file(&self) -> &FileDescriptor {
        if let Self::Chat(descriptor) = self {
            return descriptor;
        }

        panic!("Expected file, but found {self:?}");
    }
}

#[derive(Debug, Clone)]
pub struct SharedState {
    pub hover: Option<ConversationNodeID>,
    pub pressed: Option<ConversationNodeID>,
    pub selected: Option<ConversationNodeID>,
    pub dragged: Option<ConversationNodeID>,
    pub hover_insert_place: Option<(ConversationNodeID, usize)>,
    pub initial_press_point: Option<Point<f32>>,
    pub current_point: Option<Point<f32>>,
    pub renaming_process: Option<ConversationNodeID>,
    pub rename_temp_value: Option<String>,

    pub expanded: HashMap<ConversationNodeID, bool>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            hover: None,
            pressed: None,
            selected: None,
            dragged: None,
            hover_insert_place: None,
            initial_press_point: None,
            current_point: None,
            renaming_process: None,
            rename_temp_value: None,

            expanded: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct TreeNode {
    // State
    pub(super) name: String,
    pub(super) id: ConversationNodeID,
    pub(super) content: Content,
    pub(super) parent_id: ConversationNodeID,
    pub(super) focus_id: &'static str,
}

impl TreeNode {
    pub fn new_root(parent_id: ConversationNodeID) -> (Self, iced::Task<Message>) {
        Self::new(
            "root".to_string(),
            ConversationNodeID::default(),
            &ConversationType::Folder,
            parent_id,
        )
    }

    fn new(
        name: String,
        id: ConversationNodeID,
        tp: &ConversationType,
        parent_id: ConversationNodeID,
    ) -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        let mut component = Self {
            parent_id,
            name,
            id,
            content: Content::Loading,
            focus_id: format!("TREE_NODE_{id}").leak(),
        };

        if matches!(tp, ConversationType::Folder) {
            tasks.push(iced::Task::done(Message::NodeAction(
                component.id,
                NodeAction::StartLoadingChildren,
            )));
        } else {
            component.content = Content::Chat(FileDescriptor {});
        }

        (component, iced::Task::batch(tasks))
    }

    #[allow(dead_code)]
    pub const fn is_folder(&self) -> bool {
        matches!(self.content, Content::Folder(_))
    }

    pub const fn is_chat(&self) -> bool {
        matches!(self.content, Content::Chat(_))
    }

    pub fn from_conversation_dto(conversation: &ConversationNodeDTO) -> (Self, iced::Task<Message>) {
        Self::new(
            conversation.name.clone(),
            conversation.id,
            &conversation.tp.clone(),
            conversation.parent_id,
        )
    }

    pub const fn get_id(&self) -> ConversationNodeID {
        self.id
    }

    pub fn is_root(&self) -> bool {
        self.id == ConversationNodeID::default()
    }

    pub fn find_child(&self, id: ConversationNodeID) -> Option<Self> {
        if self.id == id {
            return Some(self.clone());
        }

        if let Content::Folder(descriptor) = &self.content {
            for child in &descriptor.children {
                if let Some(found) = child.find_child(id) {
                    return Some(found);
                }
            }
        }

        None
    }

    pub fn find_parent(&self, id: ConversationNodeID) -> Option<Self> {
        if let Content::Folder(descriptor) = &self.content {
            for child in &descriptor.children {
                if child.id == id {
                    return Some(self.clone());
                }

                if let Some(found) = child.find_parent(id) {
                    return Some(found);
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    pub fn get_children(&self) -> Vec<Self> {
        if let Content::Folder(descriptor) = &self.content {
            return descriptor.children.clone();
        }

        vec![]
    }

    pub fn is_hover(&self, state: &SharedState) -> bool {
        state.hover == Some(self.id)
    }

    pub fn temp_name(&self, state: &SharedState) -> String {
        if let Some(renaming_process) = state.renaming_process {
            if renaming_process == self.id {
                if let Some(rename_temp_value) = &state.rename_temp_value {
                    return rename_temp_value.clone();
                }
            }
        }

        String::new()
    }

    pub fn is_renaming_process(&self, state: &SharedState) -> bool {
        state.renaming_process == Some(self.id)
    }

    pub fn is_pressed(&self, state: &SharedState) -> bool {
        state.pressed == Some(self.id)
    }

    pub fn is_selected(&self, state: &SharedState) -> bool {
        state.selected == Some(self.id)
    }

    #[allow(dead_code)]
    pub fn is_dragged(&self, state: &SharedState) -> bool {
        state.dragged == Some(self.id)
    }

    pub fn is_expanded(&self, state: &SharedState) -> bool {
        state.expanded.get(&self.id).copied().unwrap_or(false)
    }

    pub fn set_expanded(&self, state: &mut SharedState, expanded: bool) {
        state.expanded.insert(self.id, expanded);
    }

    pub fn toggle_expanded(&self, state: &mut SharedState) {
        if state.dragged.is_some() && self.is_expanded(state) {
            return;
        }

        self.set_expanded(state, !self.is_expanded(state));
    }

    pub const fn get_insert_place_index(&self, state: &SharedState) -> Option<usize> {
        if let Some((id, index)) = state.hover_insert_place {
            if id == self.id {
                return Some(index);
            }
        }

        None
    }
}
