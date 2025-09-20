use std::collections::HashMap;

use framework::types::dto::{ProviderDTO, ProviderID};
use iced::{widget::pane_grid, Task};

#[derive(Debug, Clone)]
pub(super) enum Pane {
    ProvidersList,
    ProviderDetails,
}


#[derive(Debug, Clone)]
pub enum Message {
    StartLoading,
    LoadedProviders(Vec<ProviderDTO>),

    UpdateProviderName(String),
    UpdateProviderUrl(String),
    UpdateProviderApiKey(String),
    UpdateProviderDefaultModel(String),

    AddProvider,
    DeleteProvider,
    DeleteProviderComplete(Option<ProviderID>),
    SelectProvider(Option<ProviderID>),
    SelectedProvider(Option<ProviderID>),
    ResizePane(pane_grid::ResizeEvent),
    SaveProvider,
    CreatedProvider(Option<ProviderDTO>),
}

#[derive(Debug, Clone)]
pub struct ProvidersSettings {
        // State
        pub(super) providers: HashMap<ProviderID, ProviderDTO>,
        pub(super) selected_provider: Option<ProviderID>,
        pub(super) panes: pane_grid::State<Pane>,
        pub(super) temp_provider: ProviderDTO,
}

impl ProvidersSettings {
    pub fn new() -> (Self, iced::Task<Message>) {
        let mut tasks = vec![]; 

        tasks.push(Task::done(Message::StartLoading));


        let (mut panes, providers_list) = pane_grid::State::new(Pane::ProvidersList);

        let (_, left_split) = panes
            .split(pane_grid::Axis::Vertical, providers_list, Pane::ProviderDetails)
            .expect("Failed to split pane");

        panes.resize(left_split, 0.2);

        (
            Self {
                temp_provider: ProviderDTO::default(),
                providers: HashMap::new(),
                panes,
                selected_provider: None,
            },
            iced::Task::batch(tasks)
        )
    }

    pub fn is_unsaved_changes(&self) -> bool {
        if self.selected_provider.is_none() {
            return false;
        }

        let selected_provider = self.selected_provider.expect("Selected provider is None");

        if let Some(provider) = self.providers.get(&selected_provider) {
            return *provider != self.temp_provider;
        }

        false
    }

    pub(super) fn get_sorted_providers(&self) -> Vec<ProviderDTO> {
        let mut sorted_providers: Vec<ProviderDTO> = self.providers.values().cloned().collect();
        sorted_providers.sort_by(|a, b| a.id.cmp(&b.id));
        sorted_providers
    }

    pub fn try_reset_temp(&mut self) {
        if let Some(id) = self.selected_provider {
            if let Some(provider) = self.providers.get(&id) {
                self.temp_provider = provider.clone();
            }
        } else {
            self.temp_provider = ProviderDTO::default();
        }
    }
}
