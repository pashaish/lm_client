use super::Presets;
use framework::{types::dto::{PresetDTO, PresetId}, utils::notify, Context};
use iced::{Task, widget::text_editor};

impl Presets {
    pub fn update(&mut self, ctx: &Context, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::SavedChanges(dto) => {
                self.presets.insert(dto.id, dto);
                self.reset_temp();
                Task::none()
            }
            super::Message::CommitChanges => {
                if self.selected_preset.is_none() {
                    return Task::none();
                }

                let mut dto = self.temp_dto.clone();
                let presets_service = ctx.presets_service.clone();
                let temp_prompt = self.temp_prompt.text();
                dto.prompt = temp_prompt;
                Task::perform(
                    async move {
                        let dto = dto.clone();
                        presets_service
                            .update_preset(&dto)
                            .expect("Failed to update preset")
                    },
                    super::Message::SavedChanges,
                )
            }
            super::Message::DeletePreset => {
                if self.selected_preset.is_none() {
                    return Task::none();
                }

                self.delete_preset(ctx)
            }
            super::Message::DeletePresetComplete(preset_id) => match preset_id {
                Some(preset_id) => {
                    self.selected_preset = None;
                    self.presets.remove(&preset_id);
                    Task::none()
                }
                None => Task::none(),
            },
            super::Message::MaxTokensEdit(new_max_tokens) => {
                self.temp_dto.max_tokens = new_max_tokens;
                Task::none()
            }
            super::Message::PromptEdit(new_prompt) => {
                self.temp_prompt.perform(new_prompt);
                Task::none()
            }
            super::Message::TemperatureEdit(new_temperature) => {
                self.temp_dto.temperature = new_temperature;
                Task::none()
            }
            super::Message::NameEdit(new_name) => {
                self.temp_dto.name = new_name;
                Task::none()
            }
            super::Message::SelectPreset(preset_id) => {
                self.select_preset(preset_id)
            }
            super::Message::SelectPresetComplete(preset_id) => {
                self.select_preset_complete(preset_id)
            }
            super::Message::PaneResize(event) => {
                self.panes.resize(event.split, event.ratio);
                Task::none()
            }
            super::Message::AddPreset => {
                let service = ctx.presets_service.clone();
                let unsaved_changes = self.is_unsaved_changes();

                Task::perform(
                    async move {
                        if !unsaved_changes || notify::confirmation("Unsaved Changes").await {
                            return Some(service
                                .add_preset(&PresetDTO::default())
                                .expect("Failed to add preset"));
                        }
                        None
                    },
                    super::Message::CreatedPresetLoaded,
                )
            }
            super::Message::CreatedPresetLoaded(dto) => {
                if let Some(ref dto) = dto {
                    self.try_reset_temp();
                    self.presets.insert(dto.id, dto.clone());
                    return Task::done(super::Message::SelectPreset(Some(dto.id)));
                }

                Task::none()
            }
            super::Message::StartPresetsLoading => {
                let service = ctx.presets_service.clone();

                Task::perform(
                    async move {
                        let presets = service.get_presets();

                        presets.expect("Failed to load presets")
                    },
                    super::Message::PresetsLoaded,
                )
            }
            super::Message::PresetsLoaded(presets) => {
                for preset in presets {
                    self.presets.insert(preset.id, preset);
                }
                Task::none()
            }
        }
    }

    pub fn reset_temp(&mut self) {
        self.temp_dto = self.get_current_preset();
        self.temp_prompt = text_editor::Content::with_text(&self.temp_dto.prompt);
    }

    pub fn try_reset_temp(&mut self) {
        if let Some(preset) = self.try_get_current_preset() {
            self.temp_dto = preset;
            self.temp_prompt = text_editor::Content::with_text(&self.temp_dto.prompt);
        } else {
            self.temp_dto = PresetDTO::default();
            self.temp_prompt = text_editor::Content::new();
        }
    }

    fn select_preset(&self, preset_id: Option<PresetId>) -> Task<super::Message> {
        let is_unsaved_changes = self.is_unsaved_changes();
        let current_preset_id = self.try_get_current_preset().map(|p| p.id);
        Task::perform(
            async move {
                if let Some(preset_id) = preset_id {
                    if is_unsaved_changes {
                        if notify::confirmation("Unsaved Changes").await {
                            return Some(preset_id);
                        }
                        return current_preset_id;
                    }
                    return Some(preset_id);
                }

                preset_id
            },
            super::Message::SelectPresetComplete,
        )
    }

    fn select_preset_complete(
        &mut self,
        preset_id: Option<PresetId>,
    ) -> Task<super::Message> {
        let initial_preset_id = self.selected_preset;
        self.selected_preset = preset_id;
        if let Some(preset_id) = preset_id {
            if self
                .try_get_current_preset()
                .is_some_and(move |p| Some(p.id) == initial_preset_id)
            {
                return Task::none();
            }

            self.temp_dto = self
                .presets
                .get(&preset_id)
                .expect("Preset not found")
                .clone();
            self.temp_prompt = text_editor::Content::with_text(&self.temp_dto.prompt);
        }
        Task::none()
    }

    fn delete_preset(&mut self, ctx: &Context) -> Task<super::Message> {
        let presets_service = ctx.presets_service.clone();
        let selected_preset = self.selected_preset;

        Task::perform(
            async move {
                if let Some(preset_id) = selected_preset {
                    if notify::confirmation("Delete Preset").await {
                        presets_service
                            .delete_preset(preset_id)
                            .expect("Failed to delete preset");
                        return Some(preset_id);
                    }
                    None
                } else {
                    panic!("No preset selected");
                }
            },
            super::Message::DeletePresetComplete,
        )
    }
}
