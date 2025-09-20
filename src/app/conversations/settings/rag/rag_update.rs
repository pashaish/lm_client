use super::Rag;
use framework::{Context, types::common::ProgressStatus, utils::notify};
use iced::Task;

impl Rag {
    pub fn update(&mut self, ctx: &mut Context, message: super::Message) -> Task<super::Message> {
        match message {
            super::Message::ModelPicker(message) => self
                .model_picker
                .update(ctx, message)
                .map(super::Message::ModelPicker),
            super::Message::CancelLoadingFiles => {
                if let Some(aborter) = self.loading_files_aborter.take() {
                    aborter.abort();
                    self.loading_files_aborter = None;
                    ctx.vector_service.cancel_loading_files();
                }

                Task::none()
            }
            super::Message::FilesLoaded(files) => {
                let task = Task::run(
                    ctx.vector_service.load_files(self.conversation.id, files),
                    super::Message::UpdateProgressFilesLoading,
                );

                let (task, task_abort) = task.abortable();
                self.loading_files_aborter = Some(task_abort);

                task
            }
            super::Message::UpdateProgressFilesLoading(progress) => match progress {
                ProgressStatus::Finished => {
                    self.loading_files_aborter = None;
                    Task::none()
                }
                ProgressStatus::Started
                | ProgressStatus::Progress {
                    name: _,
                    range: _,
                    current: _,
                } => Task::none(),
            },
            super::Message::StartLoadingFiles => {
                if self.model_picker.is_defined() {
                    return Task::perform(
                        async move { notify::file_selection("Select files") },
                        super::Message::FilesLoaded,
                    );
                }

                Task::none()
            }
            super::Message::RagFilesListLoaded(rag_files) => {
                self.rag_files = rag_files;
                Task::none()
            }
            super::Message::StartDeletingRagFile(rag_file_id) => {
                let vector_service = ctx.vector_service.clone();
                let conversation_id = self.conversation.id;

                Task::perform(
                    async move {
                        if notify::confirmation("Are you sure you want to delete this file?").await
                        {
                            vector_service
                                .delete_rag_file(conversation_id, rag_file_id)
                                .expect("Failed to delete rag file");
                        }
                    },
                    |()| super::Message::StartLoadingRagFilesLists,
                )
            }

            super::Message::StartLoadingRagFilesLists => {
                let vector_service = ctx.vector_service.clone();

                let conversation_id = self.conversation.id;

                Task::perform(
                    async move { vector_service.get_files(conversation_id) },
                    super::Message::RagFilesListLoaded,
                )
            }
            super::Message::ChangeChunkSize(chunk_size) => self.change_chunk_size(ctx, chunk_size),
            super::Message::ChangeChunksCount(chunks_count) => {
                self.change_chunks_count(ctx, chunks_count)
            }
        }
    }

    fn change_chunk_size(&mut self, ctx: &Context, chunk_size: i32) -> Task<super::Message> {
        #[allow(clippy::cast_sign_loss)]
        let chunk_size = chunk_size as usize;
        let mut conversations_service = ctx.conversations_service.clone();
        let conversation_id = self.conversation.id;
        let mut temp_conversation = ctx
            .conversations_service
            .get_conversation(conversation_id)
            .expect("Failed to get conversation");
        temp_conversation.rag_chunk_size = chunk_size;
        conversations_service
            .update_conversation(conversation_id, &temp_conversation)
            .expect("Failed to update chunk size");

        self.conversation.rag_chunk_size = chunk_size;

        Task::none()
    }

    fn change_chunks_count(&mut self, ctx: &Context, chunks_count: i32) -> Task<super::Message> {
        #[allow(clippy::cast_sign_loss)]
        let chunks_count = chunks_count as usize;
        let mut conversations_service = ctx.conversations_service.clone();
        let conversation_id = self.conversation.id;
        let mut temp_conversation = ctx
            .conversations_service
            .get_conversation(conversation_id)
            .expect("Failed to get conversation");
        temp_conversation.rag_chunks_count = chunks_count;
        conversations_service
            .update_conversation(conversation_id, &temp_conversation)
            .expect("Failed to update chunks count");

        self.conversation.rag_chunks_count = chunks_count;

        Task::none()
    }
}
