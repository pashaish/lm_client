use super::{
    tree_node_state::{Content, FolderDescriptor}, NodeAction, SharedState, TreeNode
};
use framework::{
    Context,
    types::dto::{ConversationNodeDTO, ConversationNodeID},
    utils::notify,
};
use iced::Task;

impl TreeNode {
    pub fn update(
        &mut self,
        state: &mut SharedState,
        message: &super::Message,
        ctx: &mut Context,
    ) -> Task<super::Message> {
        let mut tasks = vec![];

        match message.clone() {
            super::Message::NodeAction(id, action) => {
                if self.id == id {
                    tasks.push(self.update_action_current(ctx, state, &action));
                } else {
                    tasks.push(self.update_action_miss(&action));
                }
            }
        };

        if let Content::Folder(descriptor) = &mut self.content {
            let children = descriptor.children.iter_mut();
            for child in children {
                tasks.push(child.update(state, message, ctx));
            }
        }

        Task::batch(tasks)
    }

    pub fn actualize_children_task(&mut self, id: ConversationNodeID) -> Task<super::Message> {
        self.recusive_actualize_children_task(id).map_or_else(Task::none, |task| task)
    }

    fn recusive_actualize_children_task(&mut self, id: ConversationNodeID) -> Option<Task<super::Message>> {
        if self.id == id {
            return Some(Task::done(super::Message::NodeAction(
                self.id,
                super::NodeAction::StartActualizeChildren,
            )));
        }

        if let Content::Folder(descriptor) = &mut self.content {
            for child in &mut descriptor.children {
                if let Some(task) = child.recusive_actualize_children_task(id) {
                    return Some(task);
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    pub fn actualize_task(&mut self, id: ConversationNodeID) -> Task<super::Message> {
        self.recusive_actualize_task(id).map_or_else(Task::none, |task| task)
    }

    fn recusive_actualize_task(&mut self, id: ConversationNodeID) -> Option<Task<super::Message>> {
        if self.id == id {
            return Some(Task::done(super::Message::NodeAction(
                self.id,
                super::NodeAction::StartActualize,
            )));
        }

        if let Content::Folder(descriptor) = &mut self.content {
            for child in &mut descriptor.children {
                if let Some(task) = child.recusive_actualize_task(id) {
                    return Some(task);
                }
            }
        }

        None
    }

    fn update_action_current(
        &mut self,
        ctx: &Context,
        state: &mut SharedState,
        action: &super::NodeAction,
    ) -> Task<super::Message> {
        match action.clone() {
            super::NodeAction::StartDelete => self.start_delete(ctx),
            super::NodeAction::StartRename => {
                state.renaming_process = Some(self.id);
                state.rename_temp_value = Some(self.name.clone());

                ctx.focus_manager.focus(self.focus_id.clone())
            }
            super::NodeAction::RenameProcess(new_value) => {
                state.rename_temp_value = Some(new_value);
                Task::none()
            }
            super::NodeAction::RenameStartSave => self.rename_start_save(ctx, state),
            super::NodeAction::RenameError(error) => self.rename_error(&error),
            super::NodeAction::RenameCancel => {
                state.renaming_process = None;
                Task::none()
            }
            super::NodeAction::StartActualize => {
                let id = self.id;
                let service = ctx.conversations_service.clone();

                Task::perform(
                    async move {
                        let conversation = service.get_conversation(id);
                        conversation.expect("Failed to get conversation")
                    },
                    move |cnv| super::Message::NodeAction(id, super::NodeAction::Actualized(cnv)),
                )
            }
            super::NodeAction::Actualized(conversation) => {
                self.id = conversation.id;
                self.name = conversation.name;
                Task::none()
            }
            super::NodeAction::ActualizedName(name) => {
                self.name = name;
                Task::none()
            }
            super::NodeAction::StartActualizeChildren => {
                self.content.expect_folder();
                let id = self.id;
                let db = ctx.conversations_service.clone();

                Task::perform(
                    async move { db.get_children(id).expect("Failed to get children") },
                    move |list| {
                        super::Message::NodeAction(id, super::NodeAction::ActualizedChildren(list))
                    },
                )
            }
            super::NodeAction::ActualizedChildren(conversations) => {
                self.actualized_children(&conversations)
            }
            super::NodeAction::ReleaseFolder => {
                if let Content::Folder(_) = &mut self.content {
                    self.toggle_expanded(state);
                }

                Task::none()
            }
            super::NodeAction::HoverInsertPlace(_)
            | super::NodeAction::ReleaseChat
            | super::NodeAction::Press
            | super::NodeAction::Drag
            | super::NodeAction::Drop
            | super::NodeAction::MouseMoved(_)
            | super::NodeAction::Hover(_) => Task::none(),

            super::NodeAction::StartConversationCreate => {
                let db = ctx.conversations_service.clone();
                let id = self.id;
                Task::perform(
                    async move {
                        db.add_chat("New Chat", id)
                            .expect("Failed to create conversation")
                    },
                    move |conversation| {
                        super::Message::NodeAction(
                            id,
                            super::NodeAction::ConversationCreated(conversation),
                        )
                    },
                )
            }
            super::NodeAction::FolderCreated(conversation)
            | super::NodeAction::ConversationCreated(conversation) => {
                self.conversation_created(&conversation)
            }

            super::NodeAction::StartFolderCreate => self.start_folder_create(ctx),
            super::NodeAction::StartLoadingChildren => self.start_loading_children(ctx),
            super::NodeAction::LoadedChildren(conversations) => {
                self.loaded_children(&conversations)
            }
        }
    }

    fn update_action_miss(&self, _action: &super::NodeAction) -> Task<super::Message> {
        Task::none()
    }

    fn rename_error(&self, error: &str) -> Task<super::Message> {
        let id = self.id;
        let error = error.to_string();
        Task::perform(
            async {
                notify::validation(error);
            },
            move |()| super::Message::NodeAction(id, super::NodeAction::RenameCancel),
        )
    }

    fn start_delete(&self, ctx: &Context) -> Task<super::Message> {
        let service = ctx.conversations_service.clone();
        let id = self.id;
        let parent_id = self.parent_id;

        Task::perform(
            async move {
                if notify::confirmation("Are you sure you want to delete this node?").await
                {
                    service
                        .delete_conversation(id)
                        .expect("Failed to delete node");
                }
            },
            move |()| {
                super::Message::NodeAction(parent_id, super::NodeAction::StartActualizeChildren)
            },
        )
    }

    fn rename_start_save(
        &self,
        ctx: &Context,
        state: &mut SharedState,
    ) -> Task<super::Message> {
        let service = ctx.conversations_service.clone();
        let id = self.id;
        let name = state.rename_temp_value.clone().unwrap_or_else(|| self.name.clone());

        if name.trim().is_empty() {
            return Task::done(super::Message::NodeAction(
                id,
                super::NodeAction::RenameError("Name cannot be empty".to_string()),
            ));
        }

        state.renaming_process = None;

        Task::perform(
            async move {
                let mut dto = service
                    .get_conversation(id)
                    .expect("Failed to get conversation");
                dto.name = name;

                service
                    .clone()
                    .update_conversation(id, &dto)
                    .expect("Failed to rename conversation");
            },
            move |()| super::Message::NodeAction(id, super::NodeAction::StartActualize),
        )
    }

    fn actualized_children(
        &mut self,
        conversations: &[ConversationNodeDTO],
    ) -> Task<super::Message> {
        let mut tasks = vec![];

        if let Content::Folder(descriptor) = &mut self.content {
            let mut actual_children = Vec::<(Self, Task<super::Message>)>::new();

            for conversation in conversations {
                actual_children.push(
                    descriptor
                        .children
                        .iter()
                        .find(|child| child.id == conversation.id)
                        .cloned()
                        .map_or_else(
                            || Self::from_conversation_dto(conversation),
                            |child| (child, Task::none()),
                        ),
                );
            }

            descriptor.children.clear();

            for (child, task) in actual_children {
                descriptor.children.push(child);
                tasks.push(task);
            }
        } else {
            panic!("Invalid state: expected Folder content");
        }

        Task::batch(tasks)
    }

    fn conversation_created(&mut self, conversation: &ConversationNodeDTO) -> Task<super::Message> {
        let mut tasks = vec![];

        if let Content::Folder(descriptor) = &mut self.content {
            let (child, task) = Self::from_conversation_dto(conversation);

            tasks.push(task);
            descriptor.children.insert(0, child);
        } else {
            panic!("Invalid state: expected Folder content");
        }

        if conversation.is_chat() {
            tasks.push(
                Task::done(
                    super::Message::NodeAction(
                        conversation.id,
                        NodeAction::ReleaseChat,
                    )
                )
            );
        }

        Task::batch(tasks)
    }

    fn start_folder_create(&self, ctx: &Context) -> Task<super::Message> {
        let service = ctx.conversations_service.clone();
        let id = self.id;
        Task::perform(
            async move {
                service
                    .add_folder("New Folder", id)
                    .expect("Failed to create folder")
            },
            move |conversation| {
                super::Message::NodeAction(id, super::NodeAction::FolderCreated(conversation))
            },
        )
    }

    fn start_loading_children(&mut self, ctx: &Context) -> Task<super::Message> {
        self.content = Content::Loading;

        let id = self.id;
        let service = ctx.conversations_service.clone();

        Task::perform(
            async move { service.get_children(id).expect("Failed to get children") },
            move |list| super::Message::NodeAction(id, super::NodeAction::LoadedChildren(list)),
        )
    }

    fn loaded_children(&mut self, conversations: &[ConversationNodeDTO]) -> Task<super::Message> {
        let mut children = vec![];
        let mut tasks = vec![];

        for conversation in conversations {
            let (child, task) = Self::from_conversation_dto(conversation);

            children.push(child);
            tasks.push(task);
        }

        self.content = Content::Folder(FolderDescriptor { children });

        Task::batch(tasks)
    }
}
