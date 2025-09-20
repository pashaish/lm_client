use super::ProvidersSettings;
use framework::{Context, types::dto::ProviderDTO, utils::notify};
use iced::Task;

impl ProvidersSettings {
    pub fn update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::SaveProvider => {
                if self.selected_provider.is_none() {
                    return Task::none();
                }

                let mut dto = self.temp_provider.clone();
                dto.id = self.selected_provider.expect("No provider selected");

                ctx.app_settings
                    .update_provider(&dto)
                    .expect("Failed to update provider");

                Task::done(super::Message::LoadedProviders(vec![dto]))
            }
            super::Message::ResizePane(event) => {
                self.panes.resize(event.split, event.ratio);
                Task::none()
            }
            super::Message::SelectProvider(provider_id) => {
                let is_unsaved_changes = self.is_unsaved_changes();
                let current_provider_id = self.selected_provider;

                Task::perform(
                    async move {
                        if !is_unsaved_changes {
                            return provider_id;
                        }

                        if is_unsaved_changes
                            && notify::confirmation(
                                "You have unsaved changes. Do you want to discard them?",
                            )
                            .await
                        {
                            return provider_id;
                        }

                        current_provider_id
                    },
                    super::Message::SelectedProvider,
                )
            }
            super::Message::SelectedProvider(provider_id) => {
                if provider_id == self.selected_provider {
                    return Task::none();
                }

                self.selected_provider = provider_id;
                if let Some(provider_id) = provider_id {
                    if let Some(provider) = self.providers.get(&provider_id) {
                        self.temp_provider = provider.clone();
                    }
                } else {
                    self.temp_provider = ProviderDTO::default();
                }

                Task::none()
            }
            super::Message::DeleteProvider => {
                if self.selected_provider.is_none() {
                    return Task::none();
                }

                let provider_id = self.selected_provider.expect("No provider selected");

                let mut app_settings = ctx.app_settings.clone();

                Task::perform(
                    async move {
                        if notify::confirmation("Are you sure you want to delete this provider?")
                            .await
                        {
                            app_settings
                                .delete_provider(provider_id)
                                .expect("Failed to delete provider");

                            return Some(provider_id);
                        }

                        None
                    },
                    super::Message::DeleteProviderComplete,
                )
            }
            super::Message::DeleteProviderComplete(provider_id) => {
                if let Some(provider_id) = provider_id {
                    self.selected_provider = None;
                    self.providers.remove(&provider_id);
                }

                Task::none()
            }
            super::Message::AddProvider => {
                let mut app_settings = ctx.app_settings.clone();
                let unsaved_changes = self.is_unsaved_changes();

                Task::perform(
                    async move {
                        if !unsaved_changes || notify::confirmation("Unsaved Changes").await {
                            let provider_id = app_settings
                                .add_provider(&ProviderDTO::default())
                                .expect("Failed to add provider");

                            return app_settings.get_provider(provider_id);
                        }

                        None
                    },
                    super::Message::CreatedProvider,
                )
            }
            super::Message::CreatedProvider(provider) => {
                if let Some(ref provider) = provider {
                    self.try_reset_temp();
                    self.providers.insert(provider.id, provider.clone());
                    return Task::done(super::Message::SelectProvider(Some(provider.id)));
                }

                Task::none()
            }
            super::Message::StartLoading => {
                let app_settings = ctx.app_settings.clone();
                Task::perform(
                    async move { app_settings.get_providers() },
                    super::Message::LoadedProviders,
                )
            }
            super::Message::LoadedProviders(providers) => {
                for provider in providers {
                    self.providers.insert(provider.id, provider);
                }

                Task::none()
            }
            super::Message::UpdateProviderApiKey(api_key) => {
                self.temp_provider.api_key.clone_from(&api_key);
                Task::none()
            }
            super::Message::UpdateProviderDefaultModel(default_model) => {
                self.temp_provider.default_model.clone_from(&default_model);
                Task::none()
            }
            super::Message::UpdateProviderName(name) => {
                self.temp_provider.name.clone_from(&name);
                Task::none()
            }
            super::Message::UpdateProviderUrl(url) => {
                self.temp_provider.url.clone_from(&url);
                Task::none()
            }
        }
    }
}
