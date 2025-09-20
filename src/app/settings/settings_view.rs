use framework::Context;
use iced::{
    Element,
    widget::Container,
};


use super::Settings;

impl Settings {
    pub fn view(&self, ctx: &Context) -> Element<super::Message> {
        Container::new(
            self.providers_settings.view(ctx).map(super::Message::UpdateProvidersSettings)
        ).into()
    }
}
