use framework::Context;
use iced::Subscription;

use super::Settings;

impl Settings {
    pub fn subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        subs.push(self.providers_settings.subscription(ctx).map(super::Message::UpdateProvidersSettings));

        Subscription::batch(subs)
    }

    pub fn selected_subscription(&self, ctx: &Context) -> Subscription<super::Message> {
        let mut subs = vec![];

        subs.push(self.providers_settings.selected_subscription(ctx).map(super::Message::UpdateProvidersSettings));

        Subscription::batch(subs)
    }
}
