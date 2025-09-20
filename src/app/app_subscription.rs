use super::App;

impl App {
    pub fn subscription(&self) -> iced::Subscription<super::Message> {
        let mut subs = vec![];

        match self.current_view {
            super::app_state::View::Conversations => {
                subs.push(self.conversations.selected_subscription(&self.context).map(super::Message::Conversations));
            }
            super::app_state::View::Presets => {
                subs.push(self.presets.selected_subscription(&self.context).map(super::Message::Presets));
            }
            super::app_state::View::Settings => {
                subs.push(self.settings.selected_subscription(&self.context).map(super::Message::Settings));
            }
        }

        subs.push(
            self.conversations
                .subscription(&self.context)
                .map(super::Message::Conversations),
        );

        subs.push(
            self.settings
                .subscription(&self.context)
                .map(super::Message::Settings),
        );
        subs.push(
            self.presets
                .subscription(&self.context)
                .map(super::Message::Presets),
        );

        subs.push(
            self.context
                .focus_manager
                .root_subscription()
                .map(super::Message::FocusManager),
        );

        self.context.event_system.post_subscribe();

        iced::Subscription::batch(subs)
    }
}
