pub type RagFileID = i64;

#[derive(Debug, Clone, Hash)]
pub struct RagFileDTO {
    pub file_name: String,
    pub id: RagFileID,
    pub dimension: usize,
    pub embedding_model: String,
}

pub type ChunkRagId = i64;

#[derive(Debug, Clone, Hash)]
pub struct ChunkRagDTO {
    pub id: ChunkRagId,
    pub file_id: RagFileID,
    pub chunk: String,
}
