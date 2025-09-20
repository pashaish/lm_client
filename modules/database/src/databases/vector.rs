
use rusqlite::params;
use types::dto::{ChunkRagDTO, ChunkRagId, ConversationNodeID, RagFileDTO, RagFileID};
use zerocopy::IntoBytes;

use crate::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct VectorDatabase {
    connection: DatabaseConnection,
}

impl VectorDatabase {
    /// # Errors
    /// # Panics
    pub fn new(sqlite_connection: DatabaseConnection) -> Self {
        sqlite_connection
            .lock()
            .expect("Failed to lock connection")
            .execute(
                "CREATE TABLE IF NOT EXISTS vectors_files (
                  id INTEGER PRIMARY KEY,
                  conversation_id INTEGER NOT NULL,
                  file_hash TEXT NOT NULL,
                  file_name TEXT NOT NULL,
                  dimensions INTEGER NOT NULL,
                  embedding_model TEXT NOT NULL,
                  UNIQUE(conversation_id, file_hash, dimensions, embedding_model)
                )",
                [],
            )
            .expect("Failed to create table");

        Self {
            connection: sqlite_connection,
        }
    }

    pub fn get_chunk_by_id(
        &self,
        conversation_id: ConversationNodeID,
        chunk_id: ChunkRagId,
        dimensions: usize,
        embedding_model: &str,
    ) -> Result<Option<ChunkRagDTO>, String> {
        let connection = self
            .connection
            .lock()
            .expect("Failed to lock connection");

        let table_name = self.get_vectors_table_name(conversation_id, dimensions, embedding_model);

        let mut stmt = connection
            .prepare(&format!("SELECT id, file_id, chunk FROM {table_name} WHERE id = ?"))
            .map_err(|e| format!("Failed to prepare statement: {e}"))?;

        let mut rows = stmt
            .query(params![chunk_id])
            .map_err(|e| format!("Failed to query: {e}"))?;

        rows.next().expect("Failed to get next row").map_or(Ok(None), |row| {
            let id: ChunkRagId = row.get(0).expect("Failed to get chunk id");
            let file_id: RagFileID = row.get(1).expect("Failed to get file id");
            let chunk: String = row.get(2).expect("Failed to get chunk");

            Ok(Some(ChunkRagDTO { id, file_id, chunk }))
        })
    }

    /// # Errors
    /// # Panics
    pub fn search(
        &self,
        conversation_id: ConversationNodeID,
        query_vector: &[f32],
        count: usize,
        embedding_model: &str,
    ) -> Result<Vec<(f32, ChunkRagId)>, String> {
        let connection = self
            .connection
            .lock()
            .expect("Failed to lock connection");

        let dimensions = query_vector.len();
        let vectors_table_name = self.get_vectors_table_name(conversation_id, dimensions, embedding_model);

        let stmt = connection
            .prepare(&format!(
                "SELECT distance, id FROM {vectors_table_name} WHERE embedding MATCH ? ORDER BY distance LIMIT ?"
            ));

        if stmt.is_err() {
            log::error!("Failed to prepare statement: {:?}", stmt.err());
            return Ok(vec![]);
        }

        let mut stmt = stmt
            .expect("Failed to prepare statement");

        let mut rows = stmt
            .query(params![query_vector.as_bytes(), count])
            .expect("Failed to query");

        let mut chunks = vec![];

        while let Some(row) = rows.next().expect("Failed to get next row") {
            let chunk: (f32, ChunkRagId) = (row.get(0).expect("Failed to get distance"), row.get(1).expect("Failed to get chunk"));
            chunks.push(chunk);
        }

        Ok(chunks)
    }

    /// # Errors
    /// # Panics
    pub fn delete_rag_file(
        &self,
        conversation_id: ConversationNodeID,
        rag_file_id: RagFileID,
    ) -> Result<(), String> {
        let connection = self
            .connection
            .lock()
            .expect("Failed to lock connection");

        let mut table_names = vec![];

        let mut stmt = connection
            .prepare("SELECT dimensions, embedding_model FROM vectors_files WHERE conversation_id = ? AND id = ?")
            .expect("Failed to prepare statement");

        let mut rows = stmt
            .query(params![conversation_id, rag_file_id])
            .expect("Failed to query");

        while let Some(row) = rows.next().expect("Failed to get next row") {
            let dimensions: usize = row.get(0).expect("Failed to get dimension");
            let embedding_model: String = row.get(1).expect("Failed to get embedding model");

            let vectors_table_name = self.get_vectors_table_name(conversation_id, dimensions, &embedding_model);
            table_names.push(vectors_table_name);
        }

        for table_name in table_names {
            let mut stmt = connection
                .prepare(&format!("DELETE FROM {table_name} WHERE file_id = ?"))
                .expect("Failed to prepare statement");

            stmt.execute(params![rag_file_id])
                .expect("Failed to execute statement");

            let mut check_stmt = connection
                .prepare(&format!("SELECT COUNT(*) FROM {table_name}"))
                .expect("Failed to prepare statement");

            let mut check_rows = check_stmt
                .query([])
                .expect("Failed to query");

            if let Some(row) = check_rows.next().expect("Failed to get next row") {
                let count: i64 = row.get(0).expect("Failed to get count");
                if count == 0 {
                    drop(check_rows);
                    let mut drop_stmt = connection
                        .prepare(&format!("DROP TABLE IF EXISTS {table_name}"))
                        .expect("Failed to prepare statement");

                    drop_stmt.execute([])
                        .expect("Failed to execute statement");

                    // VACUUM;
                    connection.execute("VACUUM", [])
                        .expect("Failed to execute VACUUM");
                }
            }
        }

        let mut stmt = connection
            .prepare("DELETE FROM vectors_files WHERE conversation_id = ? AND id = ?")
            .expect("Failed to prepare statement");

        stmt.execute(params![conversation_id, rag_file_id])
            .expect("Failed to execute statement");

        Ok(())
    }

    /// # Errors
    /// # Panics
    pub fn delete_all_files_in_conversation(
        &self,
        conversation_id: ConversationNodeID,
    ) -> Result<(), String> {
        let connection = self
            .connection
            .lock()
            .expect("Failed to lock connection");

        let mut table_names = vec![];
        let mut stmt = connection
            .prepare("SELECT dimensions, embedding_model FROM vectors_files WHERE conversation_id = ?")
            .expect("Failed to prepare statement");

        let mut rows = stmt
            .query(params![conversation_id])
            .expect("Failed to query");

        while let Some(row) = rows.next().expect("Failed to get next row") {
            let dimensions: usize = row.get(0).expect("Failed to get dimension");
            let embedding_model: String = row.get(1).expect("Failed to get embedding model");

            let vectors_table_name = self.get_vectors_table_name(conversation_id, dimensions, &embedding_model);
            table_names.push(vectors_table_name);
        }

        for table_name in table_names {
            let mut stmt = connection
                .prepare(&format!("DROP TABLE IF EXISTS {table_name}"))
                .expect("Failed to prepare statement");

            stmt.execute([])
                .expect("Failed to execute statement");

            connection.execute("VACUUM", [])
                .expect("Failed to execute VACUUM");
        }
            
        let mut stmt = connection
            .prepare("DELETE FROM vectors_files WHERE conversation_id = ?")
            .expect("Failed to prepare statement");

        stmt.execute(params![conversation_id])
            .expect("Failed to execute statement");

        Ok(())
    }

    /// # Errors
    /// # Panics
    pub fn get_files(
        &self,
        conversation_id: ConversationNodeID,
    ) -> Result<Vec<RagFileDTO>, String> {
        let connection = self
            .connection
            .lock()
            .expect("Failed to lock connection");

        let mut stmt = connection
            .prepare("SELECT file_name, id, dimensions, embedding_model FROM vectors_files WHERE conversation_id = ?")
            .expect("Failed to prepare statement");

        let mut rows = stmt
            .query(params![conversation_id])
            .expect("Failed to query");

        let mut files = vec![];

        while let Some(row) = rows.next().expect("Failed to get next row") {
            let file_name: String = row.get(0).expect("Failed to get file name");
            let id: RagFileID = row.get(1).expect("Failed to get file id");
            let dimension: usize = row.get(2).expect("Failed to get dimension");
            let embedding_model: String = row.get(3).expect("Failed to get embedding model");

            files.push(RagFileDTO { file_name, id, dimension, embedding_model });
        }

        Ok(files)
    }

    #[must_use] pub fn get_file(
        &self,
        conversation_id: ConversationNodeID,
        file_id: RagFileID,
    ) -> Option<RagFileDTO> {
        let connection = self
            .connection
            .lock()
            .expect("Failed to lock connection");

        let mut stmt = connection
            .prepare("SELECT file_name, id, dimensions, embedding_model FROM vectors_files WHERE conversation_id = ? AND id = ?")
            .expect("Failed to prepare statement");

        let mut rows = stmt
            .query(params![conversation_id, file_id])
            .expect("Failed to query");

        rows.next().expect("Failed to get next row").map(|row| {
            let file_name: String = row.get(0).expect("Failed to get file name");
            let id: RagFileID = row.get(1).expect("Failed to get file id");
            let dimension: usize = row.get(2).expect("Failed to get dimension");
            let embedding_model: String = row.get(3).expect("Failed to get embedding model");

            RagFileDTO { file_name, id, dimension, embedding_model }
        })
    }

    /// # Panics
    pub fn insert_records(
        &self,
        conversation_id: ConversationNodeID,
        file_hash: &str,
        file_name: &str,
        chunks: &[String],
        vectors: &[Vec<f32>],
        embedding_model: &str,
    ) {
        let connection = self
            .connection
            .lock()
            .expect("Failed to lock connection");

        let dimensions = vectors.first()
            .expect("Failed to get first vector")
            .len();

        let vectors_table_name = self.get_vectors_table_name(conversation_id, dimensions, embedding_model);

        connection
            .execute(
            &format!("CREATE VIRTUAL TABLE IF NOT EXISTS {vectors_table_name} USING vec0(
                    id INTEGER PRIMARY KEY,
                    embedding float[{dimensions}],
                    file_id INTEGER NOT NULL,
                    chunk TEXT NOT NULL
                )
            "),
            params![],
        )
        .expect("Failed to create virtual table");

        let mut stmt = connection
            .prepare("INSERT INTO vectors_files (conversation_id, file_hash, file_name, dimensions, embedding_model) 
                  VALUES (?, ?, ?, ?, ?) 
                  ON CONFLICT(conversation_id, file_hash, dimensions, embedding_model) 
                  DO UPDATE SET file_name = excluded.file_name")
            .expect("Failed to prepare statement");

        stmt.execute(params![conversation_id, file_hash, file_name, dimensions, embedding_model])
            .expect("Failed to execute statement");

        let file_id: i32 = connection
            .prepare("SELECT id FROM vectors_files WHERE conversation_id = ? AND file_hash = ? AND dimensions = ? AND embedding_model = ?")
            .expect("Failed to prepare statement")
            .query_row(params![conversation_id, file_hash, dimensions, embedding_model], |row| {
                row.get(0)
            })
            .expect("Failed to get file id");

        let mut stmt = connection
            .prepare(&format!("INSERT INTO {vectors_table_name} (embedding, file_id, chunk) VALUES (?, ?, ?)"))
            .expect("Failed to prepare statement");

        for (chunk, vector) in chunks.iter().zip(vectors.iter()) {
            let mut check_stmt = connection
                .prepare(&format!("SELECT COUNT(*) FROM {vectors_table_name} WHERE file_id = ? AND chunk = ?"))
                .expect("Failed to prepare statement");

            let mut check_rows = check_stmt
                .query(params![file_id, chunk])
                .expect("Failed to query");

            let count: i64 = check_rows.next().expect("Failed to get next row").map_or(0, |row| row.get(0).expect("Failed to get count"));

            if count > 0 {
                continue;
            }

            stmt.execute(params![vector.as_bytes(), file_id, chunk])
                .expect("Failed to execute statement");
        }
    }

    /// # Panics
    #[must_use] pub fn check_by_file_hash(&self, conversation_id: ConversationNodeID, file_hash: &str, embedding_model: &str, dimensions: usize) -> bool {
        let connection = self
            .connection
            .lock()
            .expect("Failed to lock connection");

        let mut stmt = connection
            .prepare("SELECT COUNT(*) FROM vectors_files WHERE conversation_id = ? AND file_hash = ? AND dimensions = ? AND embedding_model = ?")
            .expect("Failed to prepare statement");

        let mut rows = stmt
            .query(params![conversation_id, file_hash, dimensions, embedding_model])
            .expect("Failed to query");

        rows.next().expect("Failed to get next row").is_some_and(|row| {
            let count: i64 = row.get(0).expect("Failed to get count");
            count > 0
        })
    }

    fn get_vectors_table_name(&self, conversation_id: ConversationNodeID, dimensions: usize, embedding_model: &str) -> String {
        format!("vectors_conversation_{conversation_id}_{embedding_model}_{dimensions}").replace('-', "_").replace(':', "_")
    }
}
