use super::{
    Chat,
    message_viewer::{self, MessageViewer},
};
use framework::{
    ComplitationStatus, Context,
    services::MessagingEvent,
    types::{common::ProgressStatus, dto::MessageID},
};
use framework::{
    types::dto::{MessageDTO, RoleType},
    utils::take_component,
};
use iced::{
    Task,
    advanced::widget::{operate, operation},
    widget::text_editor,
};

const BATCH_SIZE: usize = 8;
const INITIAL_BATCH_SIZE: usize = 64;

impl Chat {
    pub fn update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        let task = self.basic_update(ctx, message);

        let mut sorted_messages_ids: Vec<i64> = self.messages.keys().copied().collect();

        sorted_messages_ids.sort_unstable();
        self.sorted_messages_ids.clone_from(&sorted_messages_ids);

        self.is_need_generate = false;
        if let Some(last_message_id) = sorted_messages_ids.last() {
            if let Some(last_message) = self.messages.get(last_message_id) {
                if last_message.get_dto().role == RoleType::User {
                    self.is_need_generate = true;
                }
            }
        }

        task
    }

    fn basic_update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::StopMessageLoading => {
                if let Some(aborter) = self.gathering_message_aborter.take() {
                    aborter.abort();
                }

                self.gathering_message = None;
                self.gathering_message_process = false;
                self.text_editor_content = text_editor::Content::new();
                Task::none()
            }
            super::Message::ToggleSettings(_) => Task::none(),
            super::Message::DeleteConversation(conversation_id) => {
                if conversation_id == self.conversation_id {
                    self.messages.clear();
                    self.last_message_id = 0;
                    self.is_loaded_all_messages = false;

                    if let Some(aborter) = self.gathering_message_aborter.take() {
                        aborter.abort();
                    }
                    self.gathering_message_aborter = None;
                }
                Task::none()
            }
            super::Message::DeleteMessage(message_id) => {
                self.messages.remove(&message_id);
                Task::none()
            }
            super::Message::ChatUpdate(dto) => {
                self.chat = Some(dto);
                Task::none()
            }
            super::Message::UpdateGatheringMessage(message) => {
                if let Some(ref mut gathering_message) = self.gathering_message {
                    return gathering_message
                        .update(message, &mut self.shared_messages_state)
                        .map(super::Message::UpdateGatheringMessage);
                }

                Task::none()
            }
            super::Message::MessagingServiceEvent(event) => match event {
                MessagingEvent::ReceiveMessage(status) => {
                    self.chat_complitation_event(ctx, status)
                }
                MessagingEvent::Error(_err) => {
                    self.gathering_message_process = false;
                    self.gathering_message = None;
                    self.gathering_message_aborter = None;
                    self.text_editor_content = text_editor::Content::new();

                    Task::none()
                }
            },
            super::Message::SendMessage => self.send_message(ctx),
            super::Message::UpdateTextEditor(action) => {
                if self.gathering_message_process {
                    return Task::none();
                }

                self.text_editor_content.perform(action);

                Task::none()
            }
            super::Message::OnScrollMessageList(viewport) => {
                if viewport.absolute_offset_reversed().y > 50.0 {
                    return Task::none();
                }

                let mut tasks = vec![];

                let service = ctx.conversations_service.clone();
                let conversation_id = self.conversation_id;

                let last_message_id = self.last_message_id;
                tasks.push(Task::perform(
                    async move {
                        service
                            .get_last_messages(conversation_id, last_message_id, BATCH_SIZE)
                            .expect("Failed to load messages")
                    },
                    super::Message::LoadedBatchMessages,
                ));

                Task::batch(tasks)
            }
            super::Message::UpdateMessage(message_id, message) => {
                self.update_message(ctx, message_id, &message)
            }
            super::Message::StartLoading => self.start_loading(ctx),
            super::Message::LoadedChat(chat) => {
                self.chat = chat;
                Task::none()
            }
            super::Message::LoadedBatchMessages(new_messages) => {
                self.loaded_batch_messages(ctx, new_messages)
            }
            super::Message::EndLoadingMessages => {
                self.is_loaded_all_messages = false;
                Task::none()
            }
            super::Message::CommitGatheringMessage(new_dto) => {
                let mut tasks = vec![];
                let mut gathering_message = self
                    .gathering_message
                    .take()
                    .expect("Failed to get gathering message");

                self.gathering_message_process = false;

                tasks.push(
                    gathering_message
                        .update(
                            message_viewer::Message::UpdateMessageDTO(new_dto.clone()),
                            &mut self.shared_messages_state,
                        )
                        .map(super::Message::UpdateGatheringMessage),
                );

                self.messages.insert(new_dto.id, gathering_message);

                tasks.push(operate(operation::focusable::focus(
                    iced::advanced::widget::Id::new(self.text_editor_id.clone()),
                )));

                Task::batch(tasks)
            }
            super::Message::LoadingFilesStatus(status) => match status {
                ProgressStatus::Progress {
                    name,
                    range,
                    current,
                } => {
                    if !self.loading_file {
                        return Task::none();
                    }

                    #[allow(clippy::cast_precision_loss)]
                    let (start, end, current) =
                        (range.start as f32, range.end as f32, current as f32);

                    self.loading_progress = Some((name, start..end, current));

                    Task::none()
                }
                ProgressStatus::Finished => {
                    self.loading_progress = None;
                    self.loading_file = false;
                    Task::none()
                }
                ProgressStatus::Started => {
                    self.loading_file = true;
                    Task::none()
                }
            },
            super::Message::StartSummarizing => {
                self.gathering_message_process = true;
                Task::none()
            }
            super::Message::Summarized(_updated_message) => {
                self.gathering_message_process = false;
                Task::none()
            }
        }
    }

    fn send_message(&mut self, ctx: &Context) -> Task<super::Message> {
        if self.gathering_message_process {
            return Task::none();
        }

        if self.text_editor_content.text().trim().is_empty() && !self.is_need_generate {
            return Task::none();
        }

        let messaging_service = ctx.messaging_service.clone();
        let conversation_id = self.conversation_id;
        let message = self.text_editor_content.text();

        let stream = if self.is_need_generate {
            messaging_service
                .generate_message(conversation_id)
                .expect("Failed to generate message")
        } else {
            messaging_service
                .send_message(conversation_id, message)
                .expect("Failed to send message")
        };

        let (task, abort) = Task::run(stream, super::Message::MessagingServiceEvent).abortable();

        self.gathering_message_aborter = Some(abort);

        task
    }

    fn update_message(
        &mut self,
        ctx: &mut Context,
        message_id: MessageID,
        message: &message_viewer::Message,
    ) -> Task<super::Message> {
        if !self.messages.contains_key(&message_id) {
            return Task::none();
        }

        let mut tasks = vec![];

        tasks.push(
            self.messages
                .get_mut(&message_id)
                .expect("Failed to get message")
                .update(message.clone(), &mut self.shared_messages_state)
                .map(move |m| super::Message::UpdateMessage(message_id, m)),
        );

        tasks.push(self.message_handle(ctx, message_id, message));

        Task::batch(tasks)
    }

    fn start_loading(&mut self, ctx: &Context) -> Task<super::Message> {
        let mut tasks = vec![];
        let conversations_service = ctx.conversations_service.clone();
        let conversation_id = self.conversation_id;

        tasks.push(Task::perform(
            async move {
                let conversation = conversations_service
                    .clone()
                    .get_conversation(conversation_id)
                    .expect("Failed to load conversation");

                if conversation.is_chat() {
                    return Some(conversation);
                }

                None
            },
            super::Message::LoadedChat,
        ));

        let conversations_service = ctx.conversations_service.clone();

        tasks.push(Task::perform(
            async move {
                conversations_service
                    .get_last_messages(conversation_id, 0, INITIAL_BATCH_SIZE)
                    .expect("Failed to load messages")
            },
            super::Message::LoadedBatchMessages,
        ));

        Task::batch(tasks)
    }

    fn loaded_batch_messages(
        &mut self,
        ctx: &Context,
        new_messages: Vec<MessageDTO>,
    ) -> Task<super::Message> {
        if new_messages.is_empty() {
            return Task::done(super::Message::EndLoadingMessages);
        }

        let mut tasks = vec![];

        let mut new_messages_viewers = vec![];

        for message_dto in new_messages {
            if message_dto.conversation_id != self.conversation_id {
                continue;
            }
            let message_id = message_dto.id;
            if self.messages.contains_key(&message_id) {
                let old_message_viewer = self
                    .messages
                    .get_mut(&message_id)
                    .expect("Failed to get message");

                tasks.push(
                    old_message_viewer
                        .update(
                            message_viewer::Message::UpdateMessageDTO(message_dto),
                            &mut self.shared_messages_state,
                        )
                        .map(move |m| super::Message::UpdateMessage(message_id, m)),
                );

                continue;
            }

            let message_viewer = take_component(
                &mut tasks,
                move |m| super::Message::UpdateMessage(message_id, m),
                MessageViewer::new(ctx.conversations_service.clone(), message_dto.clone()),
            );

            new_messages_viewers.push(message_viewer);
        }

        for message_viewer in new_messages_viewers {
            let message_id = message_viewer.get_id();
            self.messages
                .insert(message_viewer.get_id(), message_viewer);
            if self.last_message_id == 0 || message_id < self.last_message_id {
                self.last_message_id = message_id;
            }
        }

        if self.messages.is_empty() {
            tasks.push(Task::done(super::Message::EndLoadingMessages));
        }

        Task::batch(tasks)
    }

    fn chat_complitation_event(
        &mut self,
        ctx: &mut Context,
        status: ComplitationStatus,
    ) -> Task<super::Message> {
        match status {
            ComplitationStatus::Start => {
                self.gathering_message_process = true;
                self.gathering_message = Some(
                    MessageViewer::new(
                        ctx.conversations_service.clone(),
                        MessageDTO {
                            content: String::new(),
                            conversation_id: self.conversation_id,
                            id: 0,
                            reasoning: None,
                            role: RoleType::Assistant,
                            timestamp: String::new(),
                            summary: None,
                            chunks: vec![],
                        },
                    )
                    .0,
                );
                Task::none()
            }
            ComplitationStatus::Message(chunk) => {
                if self.gathering_message.is_none() {
                    log::debug!("Failed to get gathering message");
                    return Task::none();
                }

                let gathering_message = self
                    .gathering_message
                    .as_mut()
                    .expect("Failed to get gathering message");

                gathering_message.append_content(&chunk.content);
                gathering_message.append_reasoning(&chunk.reasoning_content);

                Task::none()
            }
            ComplitationStatus::End => {
                let conversation = ctx
                    .conversations_service
                    .get_conversation(self.conversation_id);

                if conversation.is_ok() {
                    let conversation = conversation.expect("Failed to get conversation");

                    let is_summary =
                        conversation.summary_enabled &&
                        conversation.summary_provider.is_some();

                    if conversation.is_chat() && is_summary {
                        let messaging_service = ctx.messaging_service.clone();
                        let conversation_id = self.conversation_id;
                        return self.end_task(ctx)
                            .chain(Task::done(super::Message::StartSummarizing))
                            .chain(Task::perform(
                            async move {
                                let result = messaging_service
                                    .summarize(conversation_id)
                                    .await;

                                if result.is_err() {
                                    log::error!("Failed to summarize: {result:?}");
                                    return MessageDTO::default();
                                }

                                result.expect("Failed to summarize")
                            },
                            super::Message::Summarized,
                        ));
                    }
                    return self.end_task(ctx);
                }

                Task::none()
            }
            ComplitationStatus::Error(_err) => {
                self.gathering_message_process = false;
                Task::none()
            }
        }
    }

    fn end_task(&mut self, ctx: &mut Context) -> iced::Task<super::Message> {
        let gathering_message = self.gathering_message.clone();

        if gathering_message.is_none() {
            log::debug!("Failed to get gathering message");
            return Task::none();
        }

        let gathering_message = gathering_message.expect("Failed to get gathering message");

        let service = ctx.conversations_service.clone();
        let last_message_id = gathering_message.get_id();
        let conversation_id = self.conversation_id;

        self.text_editor_content = text_editor::Content::new();

        let gathering_dto = gathering_message.get_dto();

        self.gathering_message_aborter = None;

        Task::perform(
            async move {
                let message_for_db = gathering_dto.clone();
                service
                    .insert_message_dto(&message_for_db)
                    .expect("Failed to add message");

                service
                    .get_last_messages(conversation_id, last_message_id, 1)
                    .expect("Failed to load messages")
                    .first()
                    .expect("Failed to get message")
                    .clone()
            },
            super::Message::CommitGatheringMessage,
        )
    }

    fn message_handle(
        &mut self,
        ctx: &Context,
        message_id: MessageID,
        message: &message_viewer::Message,
    ) -> Task<super::Message> {
        match message {
            message_viewer::Message::DeleteComplete => {
                if ctx.conversations_service.get_message(message_id).is_err() {
                    self.messages
                        .remove(&message_id)
                        .expect("Failed to remove message");
                }

                Task::none()
            }
            _ => Task::none(),
        }
    }
}
