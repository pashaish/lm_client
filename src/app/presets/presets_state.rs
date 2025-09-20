use std::collections::HashMap;

use framework::types::dto::{PresetDTO, PresetId};
use iced::{
    widget::{pane_grid, text_editor}, Task
};

#[derive(Debug, Clone)]
pub(super) enum Pane {
    PresetsList,
    PresetDetails,
}

#[derive(Debug)]
pub struct Presets {
    pub(super) presets: HashMap<PresetId, PresetDTO>,
    pub(super) panes: pane_grid::State<Pane>,
    pub(super) selected_preset: Option<PresetId>,
    pub(super) temp_dto: PresetDTO,
    pub(super) temp_prompt: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartPresetsLoading,
    CreatedPresetLoaded(Option<PresetDTO>),
    PresetsLoaded(Vec<PresetDTO>),
    AddPreset,
    SelectPreset(Option<PresetId>),
    SelectPresetComplete(Option<PresetId>),
    PaneResize(iced::widget::pane_grid::ResizeEvent),

    NameEdit(String),
    TemperatureEdit(f32),
    PromptEdit(text_editor::Action),
    MaxTokensEdit(u32),

    CommitChanges,
    SavedChanges(PresetDTO),
    DeletePreset,
    DeletePresetComplete(Option<PresetId>),
}

impl Presets {
    pub fn new() -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        tasks.push(Task::done(super::Message::StartPresetsLoading));

        let (mut panes, presets_list) = pane_grid::State::new(Pane::PresetsList);

        let (_, left_split) = panes
            .split(pane_grid::Axis::Vertical, presets_list, Pane::PresetDetails)
            .expect("Failed to split pane");

        panes.resize(left_split, 0.2);

        (
            Self {
                presets: HashMap::new(),
                panes,
                selected_preset: None,
                temp_dto: PresetDTO::default(),
                temp_prompt: text_editor::Content::new(),
            },
            iced::Task::batch(tasks),
        )
    }

    pub fn is_unsaved_changes(&self) -> bool {
        self.is_unsaved_changed_dto() || self.is_unsaved_changed_prompt()
    }

    /// # Panics
    pub fn get_current_preset(&self) -> PresetDTO {
        if let Some(selected_preset) = self.selected_preset {
            if let Some(preset) = self.presets.get(&selected_preset) {
                return preset.clone();
            }
        }

        panic!("No preset selected");
    }

    pub fn try_get_current_preset(&self) -> Option<PresetDTO> {
        if let Some(selected_preset) = self.selected_preset {
            if let Some(preset) = self.presets.get(&selected_preset) {
                return Some(preset.clone());
            }
        }
        None
    }

    pub(super) fn get_sorted_presets(&self) -> Vec<PresetDTO> {
        let mut sorted_presets: Vec<_> = self.presets.values().cloned().collect();
        sorted_presets.sort_by(|a, b| a.id.cmp(&b.id));
        sorted_presets
    }

    fn is_unsaved_changed_dto(&self) -> bool {
        if let Some(selected_preset) = self.selected_preset {
            if let Some(preset) = self.presets.get(&selected_preset) {
                return preset != &self.temp_dto;
            }
        }
        false
    }

    fn is_unsaved_changed_prompt(&self) -> bool {
        if let Some(selected_preset) = self.selected_preset {
            if let Some(preset) = self.presets.get(&selected_preset) {
                return preset.prompt.trim() != self.temp_prompt.text().trim();
            }
        }
        false
    }
}
