
use super::{
    Conversations,
    chat::{self, Chat},
    folders, settings,
};
use framework::{Context, types::dto::ConversationNodeID, utils::take_component};
use iced::{Task, widget::pane_grid};

impl Conversations {
    pub fn update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::HideSettingsPane => {
                self.settings_pane_opened = false;
                self.panes.resize(self.settings_pane_split, 1.0);
                Task::none()
            }
            super::Message::ShowSettingsPane => {
                self.settings_pane_opened = true;
                self.panes.resize(self.settings_pane_split, self.settings_pane_ratio);
                Task::none()
            }
            super::Message::DeleteConversation(id) => {
                self.chats.remove(&id);
                if self.current_chat_id == Some(id) {
                    self.current_chat_id = None;
                    return Task::done(super::Message::HideSettingsPane);
                }
                Task::none()
            }
            super::Message::Settings(message) => {
                if let Some(ref mut settings) = self.settings {
                    return settings.update(ctx, message).map(super::Message::Settings);
                }

                Task::none()
            }
            super::Message::Folders(folders::Message::Selected(selected_id)) => {
                self.catch_selected_conversation(ctx, selected_id)
            }
            super::Message::Folders(message) => self
                .folders
                .update(ctx, message)
                .map(super::Message::Folders),
            super::Message::Chat(target_chat_id, message) => {
                let mut tasks = vec![];

                if let Some((_, chat_for_update)) =
                    self.chats.iter_mut().find(|(id, _)| **id == target_chat_id)
                {
                    tasks.push(Self::process_chat_message(
                        ctx,
                        target_chat_id,
                        chat_for_update,
                        &message,
                    ));
                }

                Task::batch(tasks)
            }
            super::Message::Resize(event) => {
                self.settings_pane_ratio = event.ratio;

                self.panes.resize(event.split, event.ratio);
                Task::none()
            }
        }
    }

    fn process_chat_message(
        ctx: &mut Context,
        chat_id: ConversationNodeID,
        chat: &mut Chat,
        message: &chat::Message,
    ) -> Task<super::Message> {
        let mut tasks = vec![];

        tasks.push(
            chat.update(ctx, message.clone())
                .map(move |m| super::Message::Chat(chat_id, m)),
        );

        match message {
            chat::Message::ToggleSettings(true) => {
                tasks.push(Task::done(super::Message::ShowSettingsPane));
            }
            chat::Message::ToggleSettings(false) => {
                tasks.push(Task::done(super::Message::HideSettingsPane));
            }
            _ => {}
        }

        Task::batch(tasks)
    }

    fn catch_selected_conversation(
        &mut self,
        ctx: &mut Context,
        selected_id: ConversationNodeID,
    ) -> Task<super::Message> {
        let mut tasks = vec![];

        tasks.push(
            self.folders
                .update(ctx, folders::Message::Selected(selected_id))
                .map(super::Message::Folders),
        );

        self.current_chat_id = Some(selected_id);

        if let std::collections::hash_map::Entry::Vacant(e) = self.chats.entry(selected_id) {
            e.insert(take_component(
                &mut tasks,
                move |m| super::Message::Chat(selected_id, m),
                Chat::new(selected_id),
            ));
        }

        if let Ok(conversation) = ctx.conversations_service.get_conversation(selected_id) {
            let groups_expaned = self.settings
                .as_ref()
                .map(super::settings::settings_state::Settings::get_groups_expaned)
                .unwrap_or_default();

            self.settings = Some(take_component(
                &mut tasks,
                super::Message::Settings,
                settings::Settings::new_full(
                    conversation,
                    groups_expaned,
                ),
            ));
        } else {
            self.settings = None;
        }

        Task::batch(tasks)
    }
}
