use types::dto::{ConversationNodeDTO, ConversationNodeID, ConversationType, PresetId};

use crate::utils::DatabaseConnection;

pub type ConversationTypeRaw = i32;

#[derive(Debug, Clone)]
pub struct ConversationDatabase {
    connection: DatabaseConnection,
}

impl ConversationDatabase {
    /// # Panics
    pub fn new(connection: DatabaseConnection) -> Self {
        connection
            .lock()
            .unwrap()
            .execute(
                "CREATE TABLE IF NOT EXISTS conversations (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    parent_id INTEGER,
                    type INTEGER NOT NULL,
                    ordr INTEGER DEFAULT 0,
                    preset_id INTEGER,
                    max_messages INTEGER NOT NULL,
                    embedding_provider INTEGER,
                    embedding_model TEXT,
                    rag_chunk_size INTEGER NOT NULL,
                    rag_chunks_count INTEGER NOT NULL,
                    summary_enabled INTEGER DEFAULT 0,
                    summary_model TEXT,
                    summary_provider INTEGER,
                    provider INTEGER,
                    model TEXT,
                    prompt TEXT
            )",
                [],
            )
            .expect("Failed to create conversations table");

        Self { connection }
    }

    /// # Errors
    /// # Panics
    pub fn get_conversation(
        &self,
        id: ConversationNodeID,
    ) -> Result<ConversationNodeDTO, rusqlite::Error> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        let mut stmt = connection.prepare(
            "SELECT 
                id,
                name,
                parent_id,
                type,
                ordr,
                preset_id,
                max_messages,
                embedding_provider,
                embedding_model,
                rag_chunk_size,
                rag_chunks_count,
                summary_enabled,
                summary_model,
                summary_provider,
                provider,
                model,
                prompt
            FROM conversations WHERE id = ?",
        )?;

        let mut conversation_iter = stmt.query_map(rusqlite::params![id], |row| {
            Ok(ConversationNodeDTO {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
                tp: Self::conversation_type_from_int(row.get(3)?),
                order: row.get(4)?,
                preset_id: row.get(5)?,
                max_messages: row.get(6)?,
                embedding_provider: row.get(7)?,
                embedding_model: row.get(8)?,
                rag_chunk_size: row.get(9).unwrap_or(512),
                rag_chunks_count: row.get(10).unwrap_or(2),
                summary_enabled: row.get(11).unwrap_or_default(),
                summary_model: row.get(12).unwrap_or_default(),
                summary_provider: row.get(13).unwrap_or_default(),
                provider: row.get(14).unwrap_or_default(),
                model: row.get(15).unwrap_or_default(),
                prompt: row.get(16).unwrap_or_default(),
            })
        })?;

        let conversation = conversation_iter
            .next()
            .ok_or(rusqlite::Error::QueryReturnedNoRows)??;

        Ok(conversation)
    }

    /// # Errors
    /// # Panics
    pub fn add_folder(
        &self,
        name: &str,
        parent_id: ConversationNodeID,
        max_messages: usize,
    ) -> Result<ConversationNodeDTO, rusqlite::Error> {
        self.get_conversation(self.add_conversation(
            name,
            parent_id,
            &ConversationType::Folder,
            max_messages,
        )?)
    }

    /// # Errors
    /// # Panics
    pub fn add_chat(
        &self,
        name: &str,
        parent_id: ConversationNodeID,
        max_messages: usize,
    ) -> Result<ConversationNodeDTO, rusqlite::Error> {
        self.get_conversation(self.add_conversation(
            name,
            parent_id,
            &ConversationType::Chat,
            max_messages,
        )?)
    }

    /// # Errors
    /// # Panics
    pub fn get_children(
        &self,
        parent_id: ConversationNodeID,
    ) -> Result<Vec<ConversationNodeDTO>, rusqlite::Error> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        let mut stmt = connection
            .prepare("SELECT
                id,
                name,
                parent_id,
                type,
                ordr,
                preset_id,
                max_messages,
                embedding_provider,
                embedding_model,
                rag_chunk_size,
                rag_chunks_count,
                summary_enabled,
                summary_model,
                summary_provider,
                provider,
                model,
                prompt
            FROM conversations WHERE parent_id = ? ORDER BY ordr")?;

        let conversation_iter = stmt.query_map(rusqlite::params![parent_id], |row| {
            Ok(ConversationNodeDTO {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
                tp: Self::conversation_type_from_int(row.get(3)?),
                order: row.get(4)?,
                preset_id: row.get(5)?,
                max_messages: row.get(6)?,
                embedding_provider: row.get(7)?,
                embedding_model: row.get(8)?,
                rag_chunk_size: row.get(9)?,
                rag_chunks_count: row.get(10)?,
                summary_enabled: row.get(11).unwrap_or_default(),
                summary_model: row.get(12).unwrap_or_default(),
                summary_provider: row.get(13).unwrap_or_default(),
                provider: row.get(14).unwrap_or_default(),
                model: row.get(15).unwrap_or_default(),
                prompt: row.get(16).unwrap_or_default(),
            })
        })?;

        let conversations: Vec<ConversationNodeDTO> =
            conversation_iter.collect::<Result<_, _>>()?;

        Ok(conversations)
    }

    /// # Errors
    /// # Panics
    pub fn get_all_children_recursively(
        &self,
        parent_id: ConversationNodeID,
    ) -> Result<Vec<ConversationNodeDTO>, rusqlite::Error> {
        let mut conversations = self.get_children(parent_id)?;
        
        let mut all_children = Vec::new();
        for conversation in &conversations {
            let children = self.get_all_children_recursively(conversation.id)?;
            all_children.extend(children);
        }
        conversations.extend(all_children);

        Ok(conversations)
    }

    /// # Errors
    /// # Panics
    pub fn move_conversation(
        &self,
        moving: ConversationNodeID,
        new_parent: ConversationNodeID,
        new_index: usize,
    ) -> Result<(), rusqlite::Error> {
        self.normalize_order(new_parent)?;
        self.recusive_move_conversation(moving, new_parent, new_index)?;
        self.normalize_order(new_parent)?;
        Ok(())
    }

    /// # Errors
    /// # Panics
    pub fn set_preset(
        &self,
        id: ConversationNodeID,
        preset_id: Option<PresetId>,
    ) -> Result<(), rusqlite::Error> {
        let connection = self.connection.lock().expect("Failed to lock connection");
        connection
            .execute(
                "UPDATE conversations SET preset_id = ? WHERE id = ?",
                rusqlite::params![preset_id, id],
            )
            .expect("Failed to set preset");

        Ok(())
    }

    /// # Errors
    /// # Panics
    pub fn update(
        &self,
        id: ConversationNodeID,
        new_dto: &ConversationNodeDTO,
    ) -> Result<(), rusqlite::Error> {
        let connection = self.connection.lock().expect("Failed to lock connection");
        connection
            .execute(
                "UPDATE conversations SET
                    name = ?,
                    parent_id = ?,
                    type = ?,
                    max_messages = ?,
                    embedding_provider = ?,
                    embedding_model = ?,
                    rag_chunk_size = ?,
                    rag_chunks_count = ?,
                    summary_enabled = ?,
                    summary_model = ?,
                    summary_provider = ?,
                    provider = ?,
                    model = ?,
                    prompt = ?
                WHERE id = ?",
                rusqlite::params![
                    new_dto.name,
                    new_dto.parent_id,
                    Self::conversation_type_to_int(&new_dto.tp),
                    new_dto.max_messages,
                    new_dto.embedding_provider,
                    new_dto.embedding_model,
                    new_dto.rag_chunk_size,
                    new_dto.rag_chunks_count,
                    new_dto.summary_enabled,
                    new_dto.summary_model,
                    new_dto.summary_provider,
                    new_dto.provider,
                    new_dto.model,
                    new_dto.prompt,

                    id
                ],
            )
            .expect("Failed to rename conversation");

        Ok(())
    }

    /// # Errors
    /// # Panics
    pub fn delete(&self, id: ConversationNodeID) -> Result<(), rusqlite::Error> {
        let connection = self.connection.lock().expect("Failed to lock connection");
        connection
            .execute(
                "DELETE FROM conversations WHERE id = ?",
                rusqlite::params![id],
            )
            .expect("Failed to delete conversation");

        Ok(())
    }

    fn recusive_move_conversation(
        &self,
        moving: ConversationNodeID,
        new_parent: ConversationNodeID,
        new_index: usize,
    ) -> Result<(), rusqlite::Error> {
        let mut connection = self.connection.lock().expect("Failed to lock connection");
        let transaction = connection
            .transaction()
            .expect("Failed to create transaction");

        transaction
            .prepare("UPDATE conversations SET ordr = ordr + 1 WHERE parent_id = ? AND ordr >= ?")?
            .execute(rusqlite::params![new_parent, new_index])?;

        transaction
            .prepare("UPDATE conversations SET ordr = ordr - 1 WHERE parent_id = ? AND ordr < ?")?
            .execute(rusqlite::params![new_parent, new_index])?;

        transaction
            .prepare("UPDATE conversations SET parent_id = ?, ordr = ? WHERE id = ?")?
            .execute(rusqlite::params![new_parent, new_index, moving])?;

        transaction.commit().expect("Failed to commit transaction");

        Ok(())
    }

    fn normalize_order(&self, parent_id: ConversationNodeID) -> Result<(), rusqlite::Error> {
        let conversations = self.get_children(parent_id)?;

        let mut connection = self.connection.lock().expect("Failed to lock connection");
        let transaction = connection
            .transaction()
            .expect("Failed to create transaction");

        for (index, conversation) in conversations.iter().enumerate() {
            transaction
                .prepare("UPDATE conversations SET ordr = ? WHERE id = ?")?
                .execute(rusqlite::params![index, conversation.id])?;
        }

        transaction.commit().expect("Failed to commit transaction");

        Ok(())
    }

    const fn conversation_type_to_int(tp: &ConversationType) -> ConversationTypeRaw {
        match tp {
            ConversationType::Chat => 0,
            ConversationType::Folder => 1,
        }
    }

    fn conversation_type_from_int(tp: ConversationTypeRaw) -> ConversationType {
        match tp {
            0 => ConversationType::Chat,
            1 => ConversationType::Folder,
            _ => panic!("Invalid conversation type"),
        }
    }

    fn add_conversation(
        &self,
        name: &str,
        parent_id: ConversationNodeID,
        conversation_type: &ConversationType,
        max_messages: usize,
    ) -> Result<ConversationNodeID, rusqlite::Error> {
        let conversation_type = Self::conversation_type_to_int(conversation_type);

        let mut connection = self.connection.lock().expect("Failed to lock connection");
        let transaction = connection
            .transaction()
            .expect("Failed to create transaction");

        transaction
            .prepare("UPDATE conversations SET ordr = ordr + 1 WHERE parent_id = ?")?
            .execute(rusqlite::params![parent_id])?;

        transaction
            .prepare("INSERT INTO conversations (
                name,
                parent_id,
                type,
                max_messages,
                rag_chunk_size,
                rag_chunks_count
            ) VALUES (?, ?, ?, ?, ?, ?)")?
            .execute(rusqlite::params![
                name,
                parent_id,
                conversation_type,
                max_messages,
                512,
                2
            ])?;

        transaction.commit().expect("Failed to commit transaction");

        Ok(connection.last_insert_rowid() as ConversationNodeID)
    }
}
