use framework::Context;
use iced::{Subscription, event::listen_with};

use super::Folders;

impl Folders {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        subs.push(
            self.root_folder
                .subscription(ctx)
                .map(super::Message::TreeNode),
        );

        subs.push(
            listen_with(move |event, _, _| match event {
                iced::Event::Mouse(iced::mouse::Event::ButtonReleased(
                    iced::mouse::Button::Left,
                )) => Some(super::Message::Drop),
                _ => None,
            })
            .map(|message| message(None)),
        );

        Subscription::batch(subs)
    }
}
