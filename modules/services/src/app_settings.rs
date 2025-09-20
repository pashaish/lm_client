use api::open_ai_api::OpenAiApi;
use database::{
    DatabaseConnection,
    databases::{ProvidersDatabase, StorageDatabase},
};
use types::dto::{ProviderDTO, ProviderID};
use utils::event_system::{Event, EventSystem};

#[derive(Debug, Clone)]
pub struct AppSettings {
    #[allow(dead_code)]
    storage: StorageDatabase,
    providers_db: ProvidersDatabase,
    lm_api: OpenAiApi,
    event_system: EventSystem,
}

impl AppSettings {
    pub fn new(
        connection: DatabaseConnection,
        lm_api: OpenAiApi,
        event_system: EventSystem,
    ) -> Self {
        Self {
            event_system,
            storage: StorageDatabase::new(connection.clone()),
            providers_db: ProvidersDatabase::new(connection),
            lm_api,
        }
    }

    #[must_use] pub fn get_provider(&self, id: ProviderID) -> Option<ProviderDTO> {
        self.providers_db.get_provider(id)
    }

    /// # Errors
    /// # Panics
    pub fn delete_provider(&mut self, id: ProviderID) -> Result<(), String> {
        self.providers_db
            .delete_provider(id)
            .map_err(|e| e.to_string())?;

        self.event_system
            .dispatch(Event::ProvidersUpdate(
                self.providers_db.get_providers(),
            ));

        Ok(())
    }

    #[must_use] pub fn get_providers(&self) -> Vec<ProviderDTO> {
        self.providers_db.get_providers()
    }

    /// # Errors
    /// # Panics
    pub fn add_provider(
        &mut self,
        dto: &ProviderDTO,
    ) -> Result<ProviderID, String> {
        let provider_id = self
            .providers_db
            .add_provider(&dto.name, &dto.url, &dto.api_key, &dto.default_model)
            .map_err(|e| e.to_string())?;

        self.event_system
            .dispatch(Event::ProvidersUpdate(
                self.providers_db.get_providers(),
            ));

        Ok(provider_id)
    }

    /// # Errors
    /// # Panics
    pub fn update_provider(&mut self, dto: &ProviderDTO) -> Result<(), String> {
        self.providers_db
            .update_provider(dto)
            .map_err(|e| e.to_string())?;

        self.event_system
            .dispatch(Event::ProvidersUpdate(
                self.providers_db.get_providers(),
            ));

        Ok(())
    }

    /// # Errors
    /// # Panics
    pub async fn get_models(&self, provider: &ProviderDTO) -> Result<Vec<String>, String> {
        self.lm_api
            .get_models(provider.id)
            .await
    }
}
