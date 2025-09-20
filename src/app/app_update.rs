use framework::utils::notify;
use iced::Task;

use super::App;

impl App {
    pub fn update(&mut self, message: super::Message) -> iced::Task<super::Message> {
        let mut tasks = vec![];

        self.context.event_system.pre_update();

        // IMPORTANT: This function should only be called in response to specific user actions (ACTION).
        // If it's being called repeatedly or continuously, that's incorrect and must be fixed.
        // Correct usage: occasional calls triggered by actions (CALL/s).
        // Incorrect usage: continuous or frequent calls — this is a BUG.
        log::info!(target: "APP_EVENT", "{message:?}");

        match message {
            super::Message::FocusManager(message) => tasks.push(
                self.context
                    .focus_manager
                    .root_update(message)
                    .map(super::Message::FocusManager),
            ),

            // Components Updates
            super::Message::Conversations(message) => tasks.push(
                self.conversations
                    .update(&mut self.context, message)
                    .map(super::Message::Conversations),
            ),

            super::Message::Settings(message) => tasks.push(
                self.settings
                    .update(&mut self.context, message)
                    .map(super::Message::Settings),
            ),
            super::Message::Presets(message) => tasks.push(
                self.presets
                    .update(&self.context, message)
                    .map(super::Message::Presets),
            ),

            // Updates
            super::Message::StartChangeView(view) => {
                let is_unsaved_changes = self.presets.is_unsaved_changes() || self.settings.is_unsaved_changes();
                tasks.push(Task::perform(
                    async move {
                        if is_unsaved_changes {
                            if notify::confirmation("Unsaved Changes").await {
                                return Some(view);
                            }

                            return None;
                        }
                        Some(view)
                    },
                    super::Message::CompleteChangeView,
                ));
            }
            crate::app::Message::CompleteChangeView(view) => {
                if let Some(view) = view {
                    self.presets.try_reset_temp();
                    self.settings.try_reset_temp();
                    self.current_view = view;
                }
            }
        }

        Task::batch(tasks)
    }
}
