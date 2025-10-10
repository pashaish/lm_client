use std::path::PathBuf;

use framework::{types::{common::ProgressStatus, dto::{ConversationNodeDTO, RagFileDTO, RagFileID}}, utils::take_component};

use crate::app::common::model_picker;


#[derive(Debug, Clone)]
pub enum Message {
    ModelPicker(model_picker::Message),

    CancelLoadingFiles,
    FilesLoaded(Vec<PathBuf>),
    UpdateProgressFilesLoading(ProgressStatus),
    StartLoadingFiles,
    RagFilesListLoaded(Vec<RagFileDTO>),
    StartLoadingRagFilesLists,
    StartDeletingRagFile(RagFileID),    
    ChangeChunkSize(i32),
    ChangeChunksCount(i32),
}

#[derive(Debug, Clone)]
pub struct Rag {
    // Components
    pub(super) model_picker: model_picker::ModelPicker,

    // State
    pub(super) rag_files: Vec<RagFileDTO>,
    pub(super) loading_files_aborter: Option<iced::task::Handle>,
    pub(super) conversation: ConversationNodeDTO,
}

impl Rag {
    pub fn new(
        conversation: &ConversationNodeDTO,
    ) -> (Self, iced::Task<Message>) {
        let mut tasks = vec![]; 

        tasks.push(iced::Task::done(super::Message::StartLoadingRagFilesLists));

        (
            Self {
                conversation: conversation.clone(),
                model_picker: take_component(
                    &mut tasks,
                    Message::ModelPicker,
                    model_picker::ModelPicker::new(model_picker::ModelType::Embedding(conversation.clone().id))
                ),
                loading_files_aborter: None,
                rag_files: vec![],
            },
            iced::Task::batch(tasks)
        )
    }

    pub fn clear_view(&mut self) {
        self.rag_files.clear();
        self.loading_files_aborter = None;
    }
}
