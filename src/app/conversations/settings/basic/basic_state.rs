use std::hash::Hash;

use framework::{types::dto::{ConversationNodeDTO, PresetDTO}, utils::take_component};
use iced::widget::text_editor;

use crate::app::common::model_picker::{self, ModelPicker, ModelType};


#[derive(Debug, Clone)]
pub enum Message {
    ModelPicker(model_picker::Message),

    ChangeMaxMessages(i32),
    InputName(String),
    SubmitName,
    RenameComplete(String),
    SelectPreset(PresetDTO),
    SelectedPreset(Option<PresetDTO>),
    StartLoadPreset,
    StartLoadingPresetsList,
    PresetsLoaded(Vec<PresetDTO>),
    PresetEdit(text_editor::Action),
}

impl Hash for Message {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[derive(Debug)]
pub struct Basic {
    // Components
    pub(super) model_picker: ModelPicker,

    // State
    pub(super) temp_name: String,
    pub(super) save_process: bool,
    pub(super) presets: Vec<PresetDTO>,
    pub(super) selected_preset: Option<PresetDTO>,
    pub(super) conversation: ConversationNodeDTO,
    pub(super) preset_content: text_editor::Content,
}

impl Basic {
    pub fn new(conversation: &ConversationNodeDTO) -> (Self, iced::Task<Message>) {
        let mut tasks = vec![]; 

        tasks.push(iced::Task::done(super::Message::StartLoadingPresetsList));
        tasks.push(iced::Task::done(super::Message::StartLoadPreset));

        (
            Self {
                preset_content: text_editor::Content::with_text(&conversation.prompt),
                model_picker: take_component(
                    &mut tasks,
                    super::Message::ModelPicker,
                    ModelPicker::new(ModelType::Basic(conversation.id))
                ),
                selected_preset: None,
                presets: vec![],
                save_process: false,
                conversation: conversation.clone(),
                temp_name: conversation.name.clone(),
            },
            iced::Task::batch(tasks)
        )
    }

    pub fn clear_view(&mut self) {
        self.selected_preset = None;
        self.temp_name.clear();
        self.save_process = false;
    }
}
