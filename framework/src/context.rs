use api::open_ai_api::OpenAiApi;
use database::create_database_connection;
use services::{
    AppSettings, ConversationsService, MessagingService, PresetsService, VectorService,
};
use utils::{event_system::EventSystem, focus_manager::FocusManager};

pub struct Context {
    pub app_settings: AppSettings,
    pub event_system: EventSystem,
    pub focus_manager: FocusManager,
    pub conversations_service: ConversationsService,
    pub presets_service: PresetsService,
    pub vector_service: VectorService,
    pub messaging_service: MessagingService,
}

impl Context {
    #[must_use] pub fn new() -> Self {
        let folder = Self::get_application_folder();
        let connection = create_database_connection(format!("{folder}/database.db").as_str());

        let event_system = EventSystem::new();
        let lm_api = OpenAiApi::new(connection.clone());

        let vector_service = VectorService::new(
            64,
            connection.clone(),
            lm_api.clone(),
            event_system.clone(),
        );

        let conversations_service = ConversationsService::new(
            &connection,
            event_system.clone(),
            vector_service.clone(),
        );
        let presets_service = PresetsService::new(
            connection.clone(),
            event_system.clone(),
        );

        let app_settings = AppSettings::new(
            connection.clone(),
            lm_api.clone(),
            event_system.clone(),
        );
        let focus_manager = FocusManager::new();

        let messaging_service = MessagingService::new(
            conversations_service.clone(),
            lm_api,
            vector_service.clone(),
            connection,
            event_system.clone(),
        );

        Self {
            app_settings,
            event_system,
            focus_manager,
            conversations_service,
            presets_service,
            vector_service,
            messaging_service,
        }
    }

    /// # Panics
    #[cfg(not(debug_assertions))]
    fn get_application_folder() -> String {
        let mut app_data = dirs::home_dir().unwrap();
        app_data.push(format!(".{}", utils::APP_NAME_SYSTEM));

        if !app_data.exists() {
            std::fs::create_dir_all(&app_data).unwrap();
        }

        app_data.to_str().expect("Failed to convert path to string").to_string()
    }

    #[cfg(debug_assertions)]
    fn get_application_folder() -> String {
        "./default_workspace".to_string()
    }
}
