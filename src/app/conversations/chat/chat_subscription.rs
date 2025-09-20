use framework::{types::{common::ProgressStatus, dto::MessageDTO}, utils::event_system::Event, Context};
use iced::Subscription;

use super::Chat;

impl Chat {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        let messages = self.messages.clone();
        for (_, message_viewer) in messages {
            let message_id = message_viewer.get_id();
            subs.push(
                message_viewer
                    .subscription(ctx)
                    .with(message_id)
                    .map(|(id, m)| super::Message::UpdateMessage(id, m)),
            );

            subs.push(ctx.event_system.subscribe(
                &Event::MessageDelete(message_id),
                super::Message::DeleteMessage,
            ));
        }

        if let Some(ref gathering_message) = self.gathering_message {
            subs.push(
                gathering_message
                    .subscription(ctx)
                    .map(super::Message::UpdateGatheringMessage),
            );
        }

        if let Some(ref chat) = self.chat {
            subs.push(ctx.event_system.subscribe(
                &Event::ConversationUpdate(chat.clone()),
                super::Message::ChatUpdate,
            ));

            subs.push(ctx.event_system.subscribe(
                &Event::ConversationReceiveMessage(MessageDTO {
                    conversation_id: chat.id,
                    ..Default::default()
                }),
                |message| super::Message::LoadedBatchMessages(vec![message]),
            ));
            
            subs.push(ctx.conversations_service.subscribe_delete_conversation(
                chat.id,
                super::Message::DeleteConversation,
            ));

        }
        
        subs.push(ctx.event_system.subscribe(
            &Event::LoadingFilesStatus(ProgressStatus::Started),
            super::Message::LoadingFilesStatus
        ));

        Subscription::batch(subs)
    }
}
