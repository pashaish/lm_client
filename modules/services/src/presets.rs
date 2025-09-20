use database::{DatabaseConnection, databases::PresetsDatabase};
use types::dto::PresetDTO;
use utils::event_system::{Event, EventSystem};

#[derive(Debug, Clone)]
pub struct PresetsService {
    presets_db: PresetsDatabase,
    event_system: EventSystem,
}

impl PresetsService {
    pub fn new(connection: DatabaseConnection, event_system: EventSystem) -> Self {
        Self {
            presets_db: PresetsDatabase::new(connection),
            event_system,
        }
    }

    /// # Errors
    pub fn get_presets(&self) -> Result<Vec<PresetDTO>, String> {
        self.presets_db.get_all_presets()
    }

    /// # Errors
    pub fn add_preset(&self, dto: &PresetDTO) -> Result<PresetDTO, String> {
        let result = self.presets_db
            .add_preset(&dto.name, &dto.prompt, dto.temperature, dto.max_tokens);
        
        self.event_system.clone().dispatch(Event::UpdatePresets(
            self.presets_db.get_all_presets()?,
        ));

        result
    }

    /// # Errors
    pub fn delete_preset(&self, id: i64) -> Result<(), String> {
        let result = self.presets_db.delete_preset(id);

        self.event_system.clone().dispatch(Event::UpdatePresets(
            self.presets_db.get_all_presets()?,
        ));

        result
    }

    /// # Errors
    pub fn update_preset(&self, dto: &PresetDTO) -> Result<PresetDTO, String> {
        let result = self.presets_db.update_preset(dto);
        
        self.event_system.clone().dispatch(Event::UpdatePresets(
            self.presets_db.get_all_presets()?,
        ));

        result
    }

    /// # Errors
    pub fn get_preset(&self, id: i64) -> Result<PresetDTO, String> {
        self.presets_db.get_preset(id)
    }
}
