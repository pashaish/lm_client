use framework::Context;
use iced::Subscription;

use super::Conversations;

impl Conversations {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        subs.push(self.folders.subscription(ctx).map(super::Message::Folders));
        if let Some(ref settings) = self.settings {
            subs.push(settings.subscription(ctx).map(super::Message::Settings));
        }

        for (id, chat) in &self.chats {
            subs.push(
                chat.subscription(ctx)
                    .with(*id)
                    .map(|(id, m)| super::Message::Chat(id, m)),
            );
            subs.push(ctx.conversations_service.subscribe_delete_conversation(
                *id,
                super::Message::DeleteConversation
            ));
        }


        Subscription::batch(subs)
    }

    pub fn selected_subscription(&self, _ctx: &Context) -> Subscription<super::Message> {
        let subs = vec![];

        Subscription::batch(subs)
    }
}
