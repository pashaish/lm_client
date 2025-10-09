use iced::Subscription;
use framework::Context;

use super::MarkdownViewer;

impl MarkdownViewer {
    pub fn subscription(&self, _ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        Subscription::batch(subs)
    }
}
