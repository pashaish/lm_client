use super::{
    tree_node::{self, NodeAction, SharedState}, Folders
};
use framework::{types::dto::ConversationNodeID, Context};
use iced::Task;

impl Folders {
    pub fn update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::ReleaseFreeArea => {
                if self.shared_state.dragged.is_some() {
                    let c_id = 0;
                    let c_index = self.root_folder.get_children().len();
                    return self.drop_handle(Some((c_id, c_index)), ctx)
                }
                Task::none()
            }
            super::Message::TreeNode(message) => {
                let mut tasks = vec![];

                tasks.push(self
                    .root_folder
                    .update(&mut self.shared_state, &message, ctx)
                    .map(super::Message::TreeNode));

                tasks.push(Self::tree_node_handle(&mut self.shared_state, &message));

                if let tree_node::Message::NodeAction(id, NodeAction::ReleaseFolder) = message {
                    if self.shared_state.dragged.is_some() {
                        tasks.push(self.drop_handle(Some((id, 0)), ctx));
                    }
                }

                Task::batch(tasks)
            }
            super::Message::CreateFolder => create_folder_task(self.root_folder.get_id()),
            super::Message::CreateChat => create_chat_task(self.root_folder.get_id()),
            super::Message::Drag(id) => {
                self.shared_state.dragged = Some(id);
                Task::none()
            }
            super::Message::Drop(pair) => {
                if self.shared_state.hover_insert_place.is_some() {
                    return self.drop_handle(self.shared_state.hover_insert_place, ctx);
                }

                self.drop_handle(pair, ctx)
            }
            super::Message::Selected(id) => {
                self.shared_state.selected = Some(id);
                Task::none()
            }
        }
    }

    fn drop_handle(
        &mut self,
        pair: Option<(ConversationNodeID, usize)>,
        ctx: &mut Context,
    ) -> Task<super::Message> {
        let mut tasks = vec![];
        if let Some(dragged_id) = self.shared_state.dragged {
            if let Some((parent_id, child_index)) = pair {
                let dragged = self
                    .root_folder
                    .find_child(dragged_id)
                    .expect("Parent should be present in the tree");
                if dragged.find_child(parent_id).is_none() {
                    ctx.conversations_service
                        .move_conversation(dragged_id, parent_id, child_index)
                        .expect("Failed to move conversation");

                    let dragged_parent = self.root_folder.find_parent(dragged_id);

                    if let Some(dragged_parent) = dragged_parent {
                        tasks.push(
                            self.root_folder
                                .actualize_children_task(dragged_parent.get_id())
                                .map(super::Message::TreeNode),
                        );
                    }

                    tasks.push(
                        self.root_folder
                            .actualize_children_task(parent_id)
                            .map(super::Message::TreeNode),
                    );
                }
            }

            tasks.push(
                self.root_folder
                    .update(
                        &mut self.shared_state,
                        &tree_node::Message::NodeAction(dragged_id, tree_node::NodeAction::Drop),
                        ctx,
                    )
                    .map(super::Message::TreeNode),
            );

            self.shared_state.dragged = None;
            self.shared_state.pressed = None;
        }

        Task::batch(tasks)
    }

    fn tree_node_handle(
        state: &mut SharedState,
        message: &tree_node::Message,
    ) -> Task<super::Message> {
        match message.clone() {
            tree_node::Message::NodeAction(id, action) => match action {
                tree_node::NodeAction::Drag => {
                    if state.pressed.is_some() {
                        return Task::done(super::Message::Drag(id));
                    }

                    Task::none()
                }
                tree_node::NodeAction::Hover(hover) => {
                    state.hover = if hover { Some(id) } else { None };

                    Task::none()
                }
                tree_node::NodeAction::Press => {
                    if state.initial_press_point.is_none() {
                        state.initial_press_point = state.current_point;
                    }

                    state.pressed = Some(id);
                    Task::none()
                }
                tree_node::NodeAction::ReleaseChat => {
                    state.pressed = None;
                    if state.dragged.is_none() {
                        state.initial_press_point = None;
                        return Task::done(super::Message::Selected(id));
                    }

                    Task::none()
                }
                tree_node::NodeAction::ReleaseFolder => {
                    state.pressed = None;
                    if state.dragged.is_none() {
                        state.initial_press_point = None;
                    }

                    Task::none()
                }
                tree_node::NodeAction::MouseMoved(point) => {
                    if state.pressed.is_none() {
                        state.initial_press_point = None;
                    }

                    state.current_point = Some(point);

                    if let Some(initial_point) = state.initial_press_point {
                        if Some(id) == state.pressed && point.distance(initial_point) > 2.0 {
                            state.dragged = Some(id);
                        }
                    }

                    Task::none()
                }
                tree_node::NodeAction::HoverInsertPlace(index) => {
                    if let Some(index) = index {
                        state.hover_insert_place = Some((id, index));
                    } else if let Some((cid, _)) = state.hover_insert_place {
                        if cid == id {
                            state.hover_insert_place = None;
                        }
                    }

                    Task::none()
                }
                _ => Task::none(),
            },
        }
    }
}

fn create_folder_task(id: ConversationNodeID) -> Task<super::Message> {
    Task::done(super::Message::TreeNode(tree_node::Message::NodeAction(
        id,
        tree_node::NodeAction::StartFolderCreate,
    )))
}

fn create_chat_task(id: ConversationNodeID) -> Task<super::Message> {
    Task::done(super::Message::TreeNode(tree_node::Message::NodeAction(
        id,
        tree_node::NodeAction::StartConversationCreate,
    )))
}
