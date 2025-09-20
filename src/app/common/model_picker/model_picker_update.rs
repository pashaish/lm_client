use super::ModelPicker;
use framework::Context;
use iced::Task;

impl ModelPicker {
    pub fn update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::ModelsLoaded(models) => {
                if let Some(selected) = self.selected_model.clone() {
                    if !models.contains(&selected) {
                        self.set_current_model(ctx, None);
                    }
                }

                self.models = models;
                Task::none()
            }
            super::Message::StartLoadingProviders => {
                let app_settings = ctx.app_settings.clone();
                Task::perform(
                    async move { app_settings.get_providers() },
                    super::Message::ProvidersLoaded,
                )
            }
            super::Message::ProvidersLoaded(providers) => {
                self.providers = providers;
                self.selected_provider = self.get_current_provider(ctx);
                let model = self.get_current_model(ctx);
                if model.is_some() {
                    self.selected_model = model;
                }

                Task::done(super::Message::StartLoadingModels)
            }
            super::Message::SelectModel(model) => {
                self.set_current_model(ctx, Some(model));
                Task::none()
            }
            super::Message::StartLoadingModels => {
                let app_settings = ctx.app_settings.clone();
                let provider = self.get_current_provider(ctx);

                if let Some(provider) = provider {
                    return Task::perform(
                        async move {
                            let models = app_settings.get_models(&provider).await;

                            if let Ok(models) = models {
                                return models;
                            }

                            vec![]
                        },
                        super::Message::ModelsLoaded,
                    );
                }

                Task::none()
            }

            super::Message::SelectProvider(provider) => {
                self.set_current_provider(ctx, Some(provider));
                Task::done(super::Message::StartLoadingModels)
            }
        }
    }
}
