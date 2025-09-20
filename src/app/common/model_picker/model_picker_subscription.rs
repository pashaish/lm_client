use iced::Subscription;
use framework::{utils::event_system::Event, Context};

use super::ModelPicker;

const EVENT_PROVIDERS_UPDATE: Event = Event::ProvidersUpdate(vec![]);

impl ModelPicker {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        subs.push(
            ctx.event_system.subscribe(&EVENT_PROVIDERS_UPDATE, super::Message::ProvidersLoaded)
        );

        Subscription::batch(subs)
    }
}
