use framework::{types::dto::ConversationNodeDTO, utils::event_system::Event, Context};
use iced::Subscription;

use super::Settings;

// const PRESETS_UPDATED_EVENT: &Event = &Event::UpdatePresets(vec![]);

impl Settings {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        subs.push(self.basic.subscription(ctx).map(super::Message::Basic));
        subs.push(self.rag.subscription(ctx).map(super::Message::Rag));
        subs.push(self.summary.subscription(ctx).map(super::Message::Summary));

        subs.push(
            ctx.conversations_service
                .subscribe_delete_conversation(self.conversation.id, |_| super::Message::ClearView),
        );

        subs.push(
            ctx.event_system.subscribe(
                &Event::ConversationUpdate(ConversationNodeDTO::empty_with_id(self.conversation.id)),
                super::Message::UpdateConversation,
            )
        );

        Subscription::batch(subs)
    }
}
