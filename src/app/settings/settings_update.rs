use super::Settings;
use framework::Context;
use iced::Task;

impl Settings {
    pub fn update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::UpdateProvidersSettings(message) => {
                let mut tasks = vec![];

                tasks.push(self.providers_settings.update(ctx, message).map(super::Message::UpdateProvidersSettings));

                Task::batch(tasks)
            }
        }
    }
}
