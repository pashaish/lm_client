use framework::Context;
use iced::{Subscription, event::listen_raw};

use super::MessageViewer;

impl MessageViewer {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        subs.push(
            self.content
                .subscription(ctx)
                .map(super::Message::ContentUpdate),
        );
        subs.push(
            self.reasoning
                .subscription(ctx)
                .map(super::Message::ReasoningUpdate),
        );

        // subs.push(listen_raw(|_event, _, _| {
        //     Some(super::Message::RequestVisibleBounds)
        // }));

        Subscription::batch(subs)
    }
}
