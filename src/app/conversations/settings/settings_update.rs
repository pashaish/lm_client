use super::Settings;
use framework::Context;
use iced::Task;

impl Settings {
    pub fn update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::Basic(message) => {
                self
                    .basic
                    .update(ctx, message)
                    .map(super::Message::Basic)
            }
            super::Message::Rag(message) => {
                self
                    .rag
                    .update(ctx, message)
                    .map(super::Message::Rag)
            }
            super::Message::Summary(message) => {
                self
                    .summary
                    .update(ctx, message)
                    .map(super::Message::Summary)
            }

            super::Message::UpdateConversation(conversation) => {
                self.conversation = conversation;
                Task::none()
            }
            super::Message::ToggleGroup(group) => {
                let expanded = self.groups_expaned.entry(group).or_insert(false);
                *expanded = !*expanded;
                Task::none()
            }
            super::Message::ClearView => self.clear_view(),
        }
    }

    fn clear_view(&mut self) -> Task<super::Message> {
        self.basic.clear_view();
        self.rag.clear_view();
        Task::none()
    }
}
