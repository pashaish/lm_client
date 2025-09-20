
use framework::utils::take_component;

use super::providers_settings;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateProvidersSettings(providers_settings::Message),
}

#[derive(Debug, Clone)]
pub struct Settings {
    // Components
    pub(super) providers_settings: providers_settings::ProvidersSettings,
}

impl Settings {
    pub fn new() -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];


        (
            Self {
                providers_settings: take_component(
                    &mut tasks,
                    super::Message::UpdateProvidersSettings,
                    providers_settings::ProvidersSettings::new()
                ),
            },
            iced::Task::batch(tasks),
        )
    }

    pub fn is_unsaved_changes(&self) -> bool {
        self.providers_settings.is_unsaved_changes()
    }

    pub fn try_reset_temp(&mut self) {
        self.providers_settings.try_reset_temp();
    }
}
