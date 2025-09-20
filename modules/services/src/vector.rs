use std::{fmt::Debug, fs, ops::Range, path::PathBuf, pin::Pin};

use api::open_ai_api::OpenAiApi;
use database::{databases::{ConversationDatabase, ProvidersDatabase, VectorDatabase}, DatabaseConnection};
use iced::{futures::{channel::mpsc, SinkExt, Stream, StreamExt}, Subscription};
use text_splitter::{ChunkConfig, MarkdownSplitter, TextSplitter};
use tiktoken_rs::{o200k_base, CoreBPE};
use types::{common::ProgressStatus, dto::{ChunkRagDTO, ChunkRagId, ConversationNodeID, LmModel, MessageUsedRagChunk, RagFileDTO, RagFileID}};
use utils::event_system::{Event, EventSystem};

const EMBEDDING_CHUNKS_BATCH: usize = 1000;

#[derive(Debug, Clone)]
pub struct VectorService {
    vector_db: VectorDatabase,
    conversation_db: ConversationDatabase,
    providers_db: ProvidersDatabase,
    lm_api: OpenAiApi,
    event_system: EventSystem,
}

impl VectorService {
    pub fn new(
        _max_chunk_size: usize,
        connection: DatabaseConnection,
        lm_api: OpenAiApi,
        event_system: EventSystem,
    ) -> Self {
        Self {
            vector_db: VectorDatabase::new(connection.clone()),
            conversation_db: ConversationDatabase::new(connection.clone()),
            providers_db: ProvidersDatabase::new(connection),
            lm_api,
            event_system,
        }
    }

    #[must_use] pub fn get_chunk(
        &self,
        conversation_id: ConversationNodeID,
        chunk_id: ChunkRagId,
        dimenstion: usize,
        embedding_model: &str
    ) -> Result<Option<ChunkRagDTO>, String> {
        self.vector_db
            .get_chunk_by_id(conversation_id, chunk_id, dimenstion, embedding_model)
    }

    pub fn cancel_loading_files(&self) {
        self.event_system
            .clone()
            .dispatch(Event::LoadingFilesStatus(ProgressStatus::Finished));
    }

    /// # Panics
    /// # Errors
    pub async fn search(
        &self,
        embedding_lm_model: LmModel,
        query: String,
        conversation_id: ConversationNodeID,
    ) -> Vec<MessageUsedRagChunk> {
        let conversation = self.conversation_db.get_conversation(conversation_id)
            .expect("Failed to get conversation");

        let query_embedding = self
            .lm_api.clone()
            .embeddings(embedding_lm_model.clone(), vec![query])
            .await
            .expect("Failed to get embeddings");

        let embedding_lm_model = embedding_lm_model.model_name.clone();

        let mut founded = vec![];
        let count = conversation.rag_chunks_count;

        for embedding in &query_embedding.data {
            founded.extend(self.vector_db
                .search(
                    conversation_id,
                    &embedding.embedding,
                    count, 
                    &embedding_lm_model
                ).expect("Failed to search vector database")
            );
        }

        founded.sort_by(|(distance_a, _a), (distance_b, _b)| {
            distance_a.partial_cmp(distance_b).unwrap()
        });

        founded
            .iter()
            .filter(|(distance, _chunk_id)| {
                distance < &1.05f32
            })
            .map(|(_distance, chunk_id)| *chunk_id)
            .map(|chunk_id| {
                MessageUsedRagChunk {
                    chunk_id,
                    dimension: query_embedding.data.first().unwrap().embedding.len(),
                    embedding_model: embedding_lm_model.to_string(),
                }
            })
            .collect()
    }

    /// # Panics
    /// # Errors
    #[must_use] pub fn load_files(
        &self,
        conversation_id: ConversationNodeID,
        loading_files: Vec<PathBuf>,
    ) -> Pin<Box<dyn Stream<Item = ProgressStatus> + Send>> {
        let providers_db = self.providers_db.clone();
        let chat = self
            .conversation_db
            .get_conversation(conversation_id)
            .expect("Failed to get conversation");

        let embedding_lm_model = LmModel {
            model_name: chat.embedding_model.expect("Failed to get model"),
            provider: chat.embedding_provider
                .and_then(move |p_id| providers_db.get_provider(p_id)),
        };

        let self_cp = self.clone();

        let stream = async_fn_stream::fn_stream(async move |output| {
            let mut output = output;
            output.emit(ProgressStatus::Started).await;

            for file_path in loading_files {
                let file_content = fs::read_to_string(&file_path.clone()).expect("Failed to read file");

                output.emit(ProgressStatus::Progress {
                    name: format!("{}", file_path.to_string_lossy()),
                    range: 0..file_content.len(),
                    current: 0,
                }).await;

                let file_hash = seahash::hash(file_content.as_bytes()).to_string();

                let file_name = file_path.file_name().map(|f| f.to_os_string());
                let file_name_for_chunking = file_name.clone();

                let mut current_len = 0;
                let mut buffer_chunks: Vec<String> = Vec::new();
                let mut batches_chunks: Vec<(usize, Vec<String>)> = Vec::new();

                output.emit(ProgressStatus::Progress {
                    name: format!("Chunking {} ...", file_path.to_string_lossy()),
                    range: 0..1,
                    current: 0
                }).await;

                let (mut sender, mut reciever) = mpsc::channel::<Option<(Range<usize>, usize)>>(0);

                let path = file_path.to_string_lossy().to_string().clone();
                let chunking_sender_thread = async move {
                    while let Some(Some((range, current))) = reciever.next().await {
                        output.emit(ProgressStatus::Progress {
                            name: format!("Chunking {path} ..."),
                            range,
                            current,
                        }).await;
                    }

                    output
                };
                
                let file_content_for_chunking = file_content.clone();
                let chunking_parsing_thread = tokio::spawn(async move {
                    for chunk in Self::create_splitter(
                        &file_name_for_chunking.clone().unwrap_or_default().to_string_lossy(),
                        &file_content_for_chunking,
                        &MarkdownSplitter::new(chat.rag_chunk_size),
                        &text_splitter::TextSplitter::new(
                            ChunkConfig::new(chat.rag_chunk_size)
                                .with_overlap(16)
                                .expect("Failed to create chunk config")
                                .with_trim(true)
                                .with_sizer(o200k_base().expect("Failed to get tokenizer"))
                        )
                    ) {
                        current_len += chunk.len();
    
                        buffer_chunks.push(chunk.to_string());
    
                        if buffer_chunks.len() >= EMBEDDING_CHUNKS_BATCH {
                            batches_chunks.push((current_len, buffer_chunks.clone()));
                            buffer_chunks.clear();
                        }
    
                        let file_content_for_chunking = file_content_for_chunking.clone();
                        let mut sender_cp = sender.clone();
                        let _ = sender_cp.try_send(Some((0..file_content_for_chunking.len(), current_len)));
                    }

                    if !buffer_chunks.is_empty() {
                        batches_chunks.push((current_len, buffer_chunks.clone()));
                        buffer_chunks.clear();
                    }

                    sender.send(None).await.expect("Failed to send to channel");

                    batches_chunks
                });

                let threads_result = tokio::join!(chunking_sender_thread, chunking_parsing_thread);
                output = threads_result.0;
                batches_chunks = threads_result.1.expect("Failed to join thread");

                for (len, batch) in batches_chunks {
                    let embeddings = self_cp.lm_api.embeddings(embedding_lm_model.clone(), batch.clone()).await;
                    let embeddings = embeddings.expect("Failed to get embeddings");

                    let vectors = embeddings.data.iter().map(|e| e.embedding.clone());
                    let vectors = vectors.collect::<Vec<Vec<f32>>>();

                    let file_name = file_name
                        .clone()
                        .expect("Failed to get file name")
                        .to_string_lossy()
                        .to_string()
                        .clone();

                    let model_name = embedding_lm_model.model_name.clone();

                    self_cp.vector_db.insert_records(
                        conversation_id,
                        &file_hash,
                        &file_name,
                        &batch,
                        &vectors,
                        &model_name,
                    );

                    output.emit(ProgressStatus::Progress {
                        name: format!("Loading {} ...", file_path.to_string_lossy()),
                        range: 0..file_content.len(),
                        current: len,
                    }).await;
                }
            }

            let files = self_cp
                .vector_db
                .get_files(conversation_id)
                .expect("Failed to get files");

            output.emit(ProgressStatus::Finished).await;

            self_cp.event_system
                .clone()
                .dispatch(utils::event_system::Event::RagFilesUpdated {
                    conversation_id,
                    files,
                });
        });

        let mut event_system = self.event_system.clone();
        let stream = stream.map(move |status| {
            event_system.dispatch(Event::LoadingFilesStatus(status.clone()));
            status
        });

        (Box::pin(stream)) as _
    }

    fn create_splitter<'a>(
        filename: &'a str,
        text: &'a str,
        markdown_splitter: &'a MarkdownSplitter<text_splitter::Characters>,
        text_splitter: &'a TextSplitter<CoreBPE>,
    ) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        let file_extension = filename
            .split('.')
            .last()
            .unwrap_or("txt");

        if file_extension == "md" {
            Box::new(markdown_splitter.chunks(text))
        } else {
            Box::new(text_splitter.chunks(text))
        }
    }

    /// # Panics
    /// # Errors
    pub fn delete_all_files(&self, conversation_id: ConversationNodeID) -> Result<(), String> {
        self.vector_db
            .delete_all_files_in_conversation(conversation_id)
            .expect("Failed to delete all files");

        let files = self
            .vector_db
            .get_files(conversation_id)
            .expect("Failed to get files");

        self.event_system
            .clone()
            .dispatch(utils::event_system::Event::RagFilesUpdated {
                conversation_id,
                files,
            });

        Ok(())
    }

    /// # Panics
    /// # Errors
    pub fn delete_rag_file(
        &self,
        conversation_id: ConversationNodeID,
        rag_file_id: RagFileID,
    ) -> Result<(), String> {
        self.vector_db
            .delete_rag_file(conversation_id, rag_file_id)
            .expect("Failed to delete rag file");

        let files = self
            .vector_db
            .get_files(conversation_id)
            .expect("Failed to get files");

        self.event_system
            .clone()
            .dispatch(utils::event_system::Event::RagFilesUpdated {
                conversation_id,
                files,
            });

        Ok(())
    }

    /// # Panics
    /// # Errors
    #[must_use]
    pub fn get_files(&self, conversation_id: ConversationNodeID) -> Vec<RagFileDTO> {
        self.vector_db
            .get_files(conversation_id)
            .expect("Failed to get files")
    }

    #[must_use] pub fn get_file(
        &self,
        conversation_id: ConversationNodeID,
        rag_file_id: RagFileID,
    ) -> Option<RagFileDTO> {
        self.vector_db
            .get_file(conversation_id, rag_file_id)
    }

    pub fn subscribe_files_update<T>(
        &self,
        conversation_id: ConversationNodeID,
        converter: impl Fn((ConversationNodeID, Vec<RagFileDTO>)) -> T + 'static,
    ) -> Subscription<T>
    where
        T: Debug + Send + 'static,
    {
        self.event_system.subscribe(
            &utils::event_system::Event::RagFilesUpdated {
                conversation_id,
                files: vec![],
            },
            move |(conversation_id, files)| converter((conversation_id, files)),
        )
    }
}
