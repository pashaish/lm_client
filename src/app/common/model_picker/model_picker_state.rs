use framework::{
    Context,
    types::dto::{ConversationNodeID, ProviderDTO},
};
use iced::Task;

#[derive(Debug, Clone)]
pub enum ModelType {
    Basic(ConversationNodeID),
    Embedding(ConversationNodeID),
    Summary(ConversationNodeID),
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectProvider(ProviderDTO),
    SelectModel(String),
    StartLoadingModels,
    StartLoadingProviders,
    ProvidersLoaded(Vec<ProviderDTO>),
    ModelsLoaded(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct ModelPicker {
    pub(super) models: Vec<String>,
    pub(super) providers: Vec<ProviderDTO>,
    pub(super) selected_model: Option<String>,
    pub(super) selected_provider: Option<ProviderDTO>,
    pub(super) model_type: ModelType,
}

impl ModelPicker {
    pub fn new(model_type: ModelType) -> (Self, iced::Task<Message>) {
        let mut tasks = vec![];

        tasks.push(Task::done(Message::StartLoadingProviders));

        (
            Self {
                model_type,
                models: vec![],
                selected_model: None,
                selected_provider: None,
                providers: vec![],
            },
            iced::Task::batch(tasks),
        )
    }

    pub(super) fn set_current_model(&mut self, ctx: &mut Context, model: Option<String>) {
        match self.model_type {
            ModelType::Basic(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .expect("Failed to get conversation");

                let mut dto = chat;
                dto.model.clone_from(&model);
                ctx.conversations_service
                    .update_conversation(dto.id, &dto)
                    .expect("Failed to update conversation");
            }
            ModelType::Embedding(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .expect("Failed to get conversation");

                let mut dto = chat;
                dto.embedding_model.clone_from(&model);
                ctx.conversations_service
                    .update_conversation(dto.id, &dto)
                    .expect("Failed to update conversation");
            }
            ModelType::Summary(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .expect("Failed to get conversation");

                let mut dto = chat;
                dto.summary_model.clone_from(&model);
                ctx.conversations_service
                    .update_conversation(dto.id, &dto)
                    .expect("Failed to update conversation");
            }
        }

        self.selected_model = model;
    }

    pub(super) fn get_current_model(&self, ctx: &Context) -> Option<String> {
        match self.model_type {
            ModelType::Basic(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .ok();

                chat.and_then(|chat| chat.model)
            }
            ModelType::Embedding(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .ok();

                chat.and_then(|chat| chat.embedding_model)
            }
            ModelType::Summary(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .ok();

                chat.and_then(|chat| chat.summary_model)
            }
        }
    }

    pub const fn is_defined(&self) -> bool {
        match self.model_type {
            ModelType::Summary(_) |
            ModelType::Embedding(_) |
            ModelType::Basic(_) => self.selected_model.is_some(),
        }
    }

    pub(super) fn get_current_provider(&self, ctx: &Context) -> Option<ProviderDTO> {
        match self.model_type {
            ModelType::Summary(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .ok();

                chat.and_then(|chat| chat.summary_provider)
                    .and_then(|provider_id| ctx.app_settings.get_provider(provider_id))
            }
            ModelType::Basic(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .ok();

                chat.and_then(|chat| chat.provider)
                    .and_then(|provider_id| ctx.app_settings.get_provider(provider_id))
            }
            ModelType::Embedding(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .ok();

                chat.and_then(|chat| chat.embedding_provider)
                    .and_then(|provider_id| ctx.app_settings.get_provider(provider_id))
            }
        }
    }

    pub(super) fn set_current_provider(
        &mut self,
        ctx: &mut Context,
        provider: Option<ProviderDTO>,
    ) {
        match self.model_type {
            ModelType::Basic(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .expect("Failed to get conversation");

                let mut dto = chat;
                dto.provider = provider.clone().map(|p| p.id);
                ctx.conversations_service
                    .update_conversation(dto.id, &dto)
                    .expect("Failed to update conversation");
            }
            ModelType::Embedding(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .expect("Failed to get conversation");

                let mut dto = chat;
                dto.embedding_provider = provider.clone().map(|p| p.id);
                ctx.conversations_service
                    .update_conversation(dto.id, &dto)
                    .expect("Failed to update conversation");
            }
            ModelType::Summary(conversation_id) => {
                let chat = ctx
                    .conversations_service
                    .get_conversation(conversation_id)
                    .expect("Failed to get conversation");

                let mut dto = chat;
                dto.summary_provider = provider.clone().map(|p| p.id);
                ctx.conversations_service
                    .update_conversation(dto.id, &dto)
                    .expect("Failed to update conversation");
            }
        }

        self.selected_provider = provider;
    }
}
