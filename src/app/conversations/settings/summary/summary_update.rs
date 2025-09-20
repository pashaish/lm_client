use super::Summary;
use iced::{widget::text_editor, Task};
use framework::Context;

impl Summary {
    pub fn update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::UpdateConversation(conversation) => {
                self.conversation = conversation;
                Task::done(super::Message::StartLoadingSummary)
            }
            super::Message::SummaryModelPicker(model_picker_message) => {
                self.summary_model_picker.update(
                    ctx,
                    model_picker_message,
                ).map(super::Message::SummaryModelPicker)
            }
            super::Message::UpdateHandEditing(enabled) => {
                self.hand_editing = enabled;
                Task::none()
            }
            super::Message::ToggleSummary(enabled) => {
                self.conversation.summary_enabled = enabled;
                ctx.conversations_service.update_conversation(self.conversation.id, &self.conversation)
                    .expect("Failed to update conversation");
                Task::none()
            }
            super::Message::UpdateSummaryContent(action) => {
                self.summary_content.perform(action);
                let conversations_service = ctx.conversations_service.clone();

                let mut last_messages = conversations_service
                    .get_last_messages(self.conversation.id, 0, 1)
                    .expect("Failed to get last messages");

                let last_message = last_messages.first_mut()
                    .expect("No last message found");

                last_message.summary = Some(self.summary_content.text());
                ctx.conversations_service.update_message_dto(last_message).expect("Failed to update message DTO");
                Task::none()
            }
            super::Message::StartLoadingSummary => {
                let last_messages = ctx.conversations_service.get_last_messages(self.conversation.id, 0, 2)
                    .unwrap_or_default();

                for message in last_messages {
                    if let Some(sum) = message.summary {
                        if !sum.is_empty() {
                            self.summary_content = text_editor::Content::with_text(&sum);
                        }
                    }
                }
                Task::none()
            }
        }
    }
}
