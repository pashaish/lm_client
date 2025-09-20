use framework::{types::dto::MessageDTO, utils::event_system::Event, Context};
use iced::Subscription;

use super::MessageViewer;

impl MessageViewer {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        Subscription::batch(subs)
    }
}
