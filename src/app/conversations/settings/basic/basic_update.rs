use super::Basic;
use framework::Context;
use iced::{Task, widget::text_editor};

impl Basic {
    pub fn update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::PresetEdit(action) => {
                self.preset_content.perform(action);

                return self.change_prompt(ctx);
            }
            super::Message::ModelPicker(message) => self
                .model_picker
                .update(ctx, message)
                .map(super::Message::ModelPicker),
            super::Message::ChangeMaxMessages(max_messages) => {
                self.change_max_messages(ctx, max_messages)
            }
            super::Message::InputName(name) => {
                self.temp_name = name;
                Task::none()
            }
            super::Message::SubmitName => self.submit_name(ctx),
            super::Message::RenameComplete(name) => {
                self.save_process = false;
                self.temp_name = name;
                Task::none()
            }
            super::Message::SelectPreset(preset) => {
                let preset_id = if preset.id == 0 {
                    None
                } else {
                    Some(preset.id)
                };

                let conversations_service = ctx.conversations_service.clone();
                let conversation_id = self.conversation.id;

                Task::perform(
                    async move {
                        conversations_service
                            .set_preset(conversation_id, preset_id)
                            .expect("Failed to set preset");

                        conversations_service.get_preset(conversation_id)
                    },
                    super::Message::SelectedPreset,
                )
            }
            super::Message::StartLoadingPresetsList => {
                let presets_service = ctx.presets_service.clone();

                Task::perform(
                    async move {
                        presets_service
                            .get_presets()
                            .expect("Failed to load presets")
                    },
                    super::Message::PresetsLoaded,
                )
            }
            super::Message::PresetsLoaded(presets) => {
                self.presets.clone_from(&presets);

                if let Some(ref selected_preset) = self.selected_preset {
                    self.selected_preset =
                        presets.iter().find(|p| p.id == selected_preset.id).cloned();
                }

                Task::none()
            }
            super::Message::StartLoadPreset => self.start_load_preset(ctx),
            super::Message::SelectedPreset(preset) => {
                self.selected_preset = preset;
                Task::none()
            }
        }
    }

    fn start_load_preset(&self, ctx: &Context) -> Task<super::Message> {
        let conversations_service = ctx.conversations_service.clone();
        let id = self.conversation.id;

        Task::perform(
            async move { conversations_service.get_preset(id) },
            super::Message::SelectedPreset,
        )
    }

    fn submit_name(&mut self, ctx: &mut Context) -> Task<super::Message> {
        self.save_process = true;
        let temp_name = self.temp_name.clone();
        self.temp_name.clear();
        let mut conversations_service = ctx.conversations_service.clone();
        let conversation_id = self.conversation.id;
        let mut temp_conversation = conversations_service
            .get_conversation(conversation_id)
            .expect("Failed to get conversation");
        temp_conversation.name.clone_from(&temp_name);

        Task::perform(
            async move {
                conversations_service
                    .update_conversation(conversation_id, &temp_conversation)
                    .expect("Failed to rename conversation");
            },
            move |()| super::Message::RenameComplete(temp_name.clone()),
        )
    }

    fn change_prompt(&mut self, ctx: &Context) -> Task<super::Message> {
        let mut conversations_service = ctx.conversations_service.clone();
        let conversation_id = self.conversation.id;
        let mut temp_conversation = conversations_service
            .get_conversation(conversation_id)
            .expect("Failed to get conversation");
        temp_conversation.prompt = self.preset_content.text().to_string();

        conversations_service
            .clone()
            .update_conversation(conversation_id, &temp_conversation.clone())
            .expect("Failed to update prompt");

        Task::none()
    }

    fn change_max_messages(&mut self, ctx: &Context, max_messages: i32) -> Task<super::Message> {
        #[allow(clippy::cast_sign_loss)]
        let max_messages = max_messages as usize;
        let mut conversations_service = ctx.conversations_service.clone();
        let conversation_id = self.conversation.id;
        let mut temp_conversation = ctx
            .conversations_service
            .get_conversation(conversation_id)
            .expect("Failed to get conversation");
        temp_conversation.max_messages = max_messages;
        conversations_service
            .update_conversation(conversation_id, &temp_conversation)
            .expect("Failed to update max messages");

        self.conversation.max_messages = max_messages;

        Task::none()
    }
}
