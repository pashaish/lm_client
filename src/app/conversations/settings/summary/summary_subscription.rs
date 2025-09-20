use iced::Subscription;
use framework::{types::dto::ConversationNodeDTO, utils::event_system::Event, Context};

use super::Summary;

impl Summary {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        subs.push(self.summary_model_picker.subscription(ctx).map(super::Message::SummaryModelPicker));
        subs.push(ctx.event_system.subscribe(
            &Event::ConversationUpdate(ConversationNodeDTO::empty_with_id(self.conversation.id)),
            super::Message::UpdateConversation
        ));

        Subscription::batch(subs)
    }
}
