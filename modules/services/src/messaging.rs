use std::pin::Pin;

use api::{ComplitationStatus, open_ai_api::OpenAiApi};
use database::{DatabaseConnection, databases::ProvidersDatabase};
use iced::futures::{Stream, StreamExt};
use types::dto::{ChunkRagDTO, ConversationNodeID, LmModel, MessageDTO, PresetDTO};
use utils::event_system::{Event, EventSystem};

use crate::{ConversationsService, VectorService};

#[derive(Clone)]
pub struct MessagingService {
    conversations_service: ConversationsService,
    lm_api: OpenAiApi,
    vector_service: VectorService,
    providers_db: ProvidersDatabase,
    event_system: EventSystem,
}

#[derive(Debug, Clone, Hash)]
pub enum MessagingEvent {
    ReceiveMessage(ComplitationStatus),
    Error(String),
}

impl MessagingService {
    #[must_use]
    pub fn new(
        conversations_service: ConversationsService,
        lm_api: OpenAiApi,
        vector_service: VectorService,
        connection: DatabaseConnection,
        event_system: EventSystem,
    ) -> Self {
        Self {
            event_system,
            conversations_service,
            lm_api,
            vector_service,
            providers_db: ProvidersDatabase::new(connection),
        }
    }

    /// # Errors
    /// # Panics
    pub fn send_message(
        &self,
        conversation_id: ConversationNodeID,
        message: String,
    ) -> Result<Pin<Box<dyn Stream<Item = MessagingEvent> + Send>>, String> {
        let user_message = MessageDTO {
            id: 0,
            conversation_id,
            content: message,
            reasoning: None,
            role: types::dto::RoleType::User,
            timestamp: String::new(),
            summary: None,
            chunks: Vec::new(),
        };

        let message_for_write = user_message;
        self.conversations_service.write_message(
            conversation_id,
            &message_for_write.content.clone(),
            &message_for_write.reasoning.unwrap_or_default(),
            &message_for_write.role,
        )?;

        self.generate_message(conversation_id)
    }

    /// # Errors
    /// # Panics
    pub async fn summarize(
        &self,
        conversation_id: ConversationNodeID,
    ) -> Result<MessageDTO, String> {
        let conversation = self
            .conversations_service
            .get_conversation(conversation_id)?;

        let summary_model = conversation.summary_model.unwrap_or_default();

        let summary_model = if summary_model.is_empty() {
            let provider = conversation
                .summary_provider
                .and_then(|p| self.providers_db.get_provider(p).map(|p| p.default_model));

            if let Some(provider) = provider {
                provider
            } else {
                return Err("No summary model found".to_string());
            }
        } else {
            summary_model
        };

        if summary_model.is_empty() {
            return Err("No summary model found".to_string());
        }

        if let Some(summary_provider_id) = conversation.summary_provider {
            let messages = self
                .conversations_service
                .get_last_messages(conversation_id, 0, 5)?;

            let preset = PresetDTO {
                temperature: 0.4,
                prompt: "
                        SUMMARIZE LAST SUMMARY WITH NEW MESSAGES, DONT LOSE CONTEXT,
                        BUT SAVE CAPACITY, THIS NEED NOT FOR HUMAN,
                        FORMAT OUTPUT MUST BE CONVINIENT FOR OTHER LLM
                        YOU MUST UPDATE INFO OF LAST SUMMARY, NOT OVERWRITE IT
                    "
                .to_string(),
                ..Default::default()
            };

            let mut history_message_content = String::new();
            for message in &messages {
                let summary = message.clone().summary.unwrap_or_default();

                history_message_content.push_str(&format!(
                    "<{role}>{content}</{role}>\n",
                    role = message.role.to_string(),
                    content = message.content
                ));

                if !summary.is_empty() {
                    history_message_content.push_str(&format!("<SUMMARY>{summary}</SUMMARY>\n"));
                }
            }

            let summary_provider = self.providers_db.get_provider(summary_provider_id);

            let mut stream = self.lm_api.chat_completions(
                LmModel {
                    model_name: summary_model,
                    provider: summary_provider,
                },
                vec![],
                Some(preset),
                MessageDTO {
                    content: history_message_content,
                    role: types::dto::RoleType::User,
                    ..Default::default()
                },
            )?;

            let mut summary = String::new();
            while let Some(completion) = stream.next().await {
                match completion {
                    ComplitationStatus::Start => {}
                    ComplitationStatus::Message(message) => {
                        summary.push_str(&message.content);
                    }

                    ComplitationStatus::End => {
                        let mut last_message = self
                            .conversations_service
                            .get_last_messages(conversation_id, 0, 1)?
                            .last()
                            .expect("Failed to get last message")
                            .clone();

                        last_message.summary = Some(summary.clone());
                        self.conversations_service
                            .update_message_dto(&last_message)?;

                        let new_conversation = self
                            .conversations_service
                            .get_conversation(conversation_id)?;

                        self.event_system
                            .clone()
                            .dispatch(Event::ConversationUpdate(new_conversation));

                        return Ok(last_message);
                    }

                    ComplitationStatus::Error(err) => {
                        log::error!("[SUMMARY]: {err:?}");
                        break;
                    }
                }
            }
        };

        Err("Failed to summarize conversation".to_string())
    }

    /// # Errors
    /// # Panics
    pub fn generate_message(
        &self,
        conversation_id: ConversationNodeID,
    ) -> Result<Pin<Box<dyn Stream<Item = MessagingEvent> + Send>>, String> {
        let self_cp = self.clone();
        let stream = Box::pin(async_fn_stream::fn_stream(async move |output| {
            let mut messages = self_cp
                .get_messages(conversation_id)
                .expect("Failed to get messages");

            let mut user_message = messages.last().expect("Failed to get last message").clone();
            messages.remove(messages.len() - 1);            

            assert!(
                !(user_message.role != types::dto::RoleType::User),
                "Last message is not a user message"
            );

            let last_summary = self_cp
                .conversations_service
                .get_last_summary(conversation_id)
                .unwrap_or_default();

            if let Some(last_summary) = last_summary {
                let last_summary = last_summary.summary.unwrap_or_default();
                messages.push(MessageDTO {
                    content: format!("<LAST_SUMMARY>{last_summary}</LAST_SUMMARY>"),
                    role: types::dto::RoleType::System,
                    ..Default::default()
                });
            }


            self_cp
                .rag_process(conversation_id, &mut user_message, &mut messages)
                .await;

            let conversation = self_cp
                .conversations_service
                .get_conversation(conversation_id)
                .expect("Failed to get conversation");

            let conversation_prompt = conversation.prompt.clone();
            if !conversation_prompt.is_empty() {
                messages.push(MessageDTO {
                    content: conversation_prompt,
                    role: types::dto::RoleType::System,
                    ..Default::default()
                });
            }

            if let Some(provider_id) = conversation.provider {
                let provider = self_cp.providers_db.get_provider(provider_id);

                let model = conversation.model.unwrap_or_default();
                let model = if model.is_empty() {
                    provider.map(|p| p.default_model).unwrap_or_default()
                } else {
                    model
                };

                if !model.is_empty() {
                    let lm_model = LmModel {
                        model_name: model,
                        provider: self_cp.providers_db.get_provider(provider_id),
                    };

                    let preset = self_cp.conversations_service.get_preset(conversation_id);

                    if let Ok(ref mut completions) =
                        self_cp.lm_api.chat_completions(lm_model, messages, preset, user_message)
                    {
                        while let Some(completion) = completions.next().await {
                            output
                                .emit(MessagingEvent::ReceiveMessage(completion))
                                .await;
                        }
                    } else {
                        output
                            .emit(MessagingEvent::Error(
                                "Failed to get completions".to_string(),
                            ))
                            .await;
                    }

                    return;
                }
            }

            output
                .emit(MessagingEvent::Error(
                    "No provider or model found".to_string(),
                ))
                .await;
        }));

        Ok(stream)
    }

    fn get_messages(&self, conversation_id: ConversationNodeID) -> Result<Vec<MessageDTO>, String> {
        let conversation = self
            .conversations_service
            .get_conversation(conversation_id)?;

        let max_messages = conversation.max_messages;
        if max_messages > 0 {
            self.conversations_service
                .get_last_messages(conversation_id, 0, max_messages)
        } else {
            Ok(vec![])
        }
    }

    async fn rag_process(
        &self,
        conversation_id: ConversationNodeID,
        user_message: &mut MessageDTO,
        messages: &mut Vec<MessageDTO>,
    ) {
        let conversation = self
            .conversations_service
            .get_conversation(conversation_id)
            .expect("Failed to get conversation");

        let embedding_model = LmModel {
            model_name: conversation.embedding_model.unwrap_or_default(),
            provider: conversation
                .embedding_provider
                .and_then(|p| self.providers_db.get_provider(p)),
        };

        if embedding_model.provider.is_none() || embedding_model.model_name.is_empty() {
            return;
        }

        let user_message_for_search = user_message.clone();
        let chunks = self
            .vector_service
            .search(
                embedding_model,
                user_message_for_search.content,
                conversation_id,
            )
            .await;

        user_message.chunks = chunks;
        self.conversations_service.update_message_dto(user_message).expect("Failed to update message");

        let chunks: Vec<Option<ChunkRagDTO>> = user_message.chunks.iter().map(|chunk| {
            self.vector_service.get_chunk(conversation_id, chunk.chunk_id, chunk.dimension, &chunk.embedding_model)
                .expect("Failed to get chunk")
        }).collect();

        let chunks = chunks.iter().filter(|chunk| {
            chunk.is_some()
        });

        let chunks: String = chunks.map(|chunk| {
            let chunk = chunk.as_ref().unwrap();
            format!("<chunk>{}</chunk>", chunk.chunk)
        }).collect::<Vec<_>>().join("\n");

        messages.push(MessageDTO {
            content: format!("<retrieved_context>{}</retrieved_context>", chunks),
            role: types::dto::RoleType::System,
            ..Default::default()
        });
    }
}
