use types::dto::{ConversationNodeID, MessageDTO, MessageUsedRagChunk, RoleType};

use crate::utils::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct MessagesDatabase {
    connection: DatabaseConnection,
}

impl MessagesDatabase {
    /// # Panics
    pub fn new(connection: DatabaseConnection) -> Self {
        connection
            .lock()
            .unwrap()
            .execute(
                "CREATE TABLE IF NOT EXISTS messages (
                  id INTEGER PRIMARY KEY,
                  conversation_id INTEGER NOT NULL,
                  content TEXT NOT NULL,
                  reasoning TEXT,
                  timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                  role INTEGER NOT NULL,
                  summary TEXT,
                  chunks TEXT
              )",
                [],
            )
            .expect("Failed to create messages table");

        Self { connection }
    }

    /// # Errors
    /// # Panics
    pub fn delete_message(&self, message_id: i64) -> Result<(), rusqlite::Error> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        connection.execute(
            "DELETE FROM messages WHERE id = ?",
            rusqlite::params![message_id],
        )?;

        Ok(())
    }

    /// # Errors
    /// # Panics
    pub fn get_message(&self, message_id: i64) -> Result<MessageDTO, rusqlite::Error> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        let mut stmt = connection.prepare(
            "SELECT id, conversation_id, content, reasoning, timestamp, role, summary
             FROM messages 
             WHERE id = ?",
        )?;

        let row_mapper = |row: &rusqlite::Row| -> Result<MessageDTO, rusqlite::Error> {
            let message = Self::row_to_message(row)?;
            Ok(message)
        };

        let rows = stmt.query_map(rusqlite::params![message_id], row_mapper)?;

        let messages: Result<Vec<MessageDTO>, rusqlite::Error> = rows.collect();

        messages
            .map(|mut v| v.pop())
            .and_then(|v| v.ok_or(rusqlite::Error::QueryReturnedNoRows))
    }

    /// # Errors
    /// # Panics
    pub fn delete_messages(
        &self,
        conversation_id: ConversationNodeID,
    ) -> Result<(), rusqlite::Error> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        connection.execute(
            "DELETE FROM messages WHERE conversation_id = ?",
            rusqlite::params![conversation_id],
        )?;

        Ok(())
    }

    /// # Errors
    /// # Panics
    pub fn insert_message(
        &self,
        conversation_id: ConversationNodeID,
        content: &str,
        reasoning: &str,
        role: &RoleType,
        chunks: &[MessageUsedRagChunk],
    ) -> Result<(), rusqlite::Error> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        connection.execute(
            "INSERT INTO messages (
                conversation_id,
                content,
                reasoning,
                role,
                chunks
            ) VALUES (?, ?, ?, ?, ?)",
            rusqlite::params![
                conversation_id,
                content,
                reasoning,
                Self::message_role_to_int(role),
                serde_json::to_string(&chunks).unwrap_or_default(),
            ],
        )?;

        Ok(())
    }
 

    /// # Errors
    /// # Panics
    pub fn insert_message_dto(&self, message_dto: MessageDTO) -> Result<(), rusqlite::Error> {
        self.insert_message(
            message_dto.conversation_id,
            &message_dto.content,
            &message_dto.reasoning.unwrap_or_default(),
            &message_dto.role,
            &message_dto.chunks,
        )
    }

    /// # Errors
    /// # Panics
    pub fn update_message_dto(&self, message_dto: MessageDTO) -> Result<(), rusqlite::Error> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        connection.execute(
            "UPDATE messages 
             SET content = ?, reasoning = ?, role = ?, summary = ?, chunks = ?
             WHERE id = ?",
            rusqlite::params![
                message_dto.content,
                message_dto.reasoning.unwrap_or_default(),
                Self::message_role_to_int(&message_dto.role),
                message_dto.summary.unwrap_or_default(),
                serde_json::to_string(&message_dto.chunks).unwrap_or_default(),

                message_dto.id,
            ],
        )?;

        Ok(())
    }

    /// # Errors
    /// # Panics
    pub fn get_last_messages(
        &self,
        conversation_id: ConversationNodeID,
        known_id: i64,
        limit: usize,
    ) -> Result<Vec<MessageDTO>, rusqlite::Error> {
        let connection = self.connection.lock().expect("Failed to lock connection");
        let query = if known_id > 0 {
            "SELECT id, conversation_id, content, reasoning, timestamp, role, summary, chunks
             FROM messages 
             WHERE conversation_id = ? AND id < ? 
             ORDER BY id DESC
             LIMIT ?"
        } else {
            "SELECT id, conversation_id, content, reasoning, timestamp, role, summary, chunks
             FROM messages 
             WHERE conversation_id = ? 
             ORDER BY id DESC
             LIMIT ?"
        };

        let mut stmt = connection.prepare(query)?;

        let row_mapper = |row: &rusqlite::Row| -> Result<MessageDTO, rusqlite::Error> {
            let message = Self::row_to_message(row)?;
            Ok(message)
        };

        let query = if known_id > 0 {
            rusqlite::params![conversation_id, known_id, limit]
        } else {
            rusqlite::params![conversation_id, limit]
        };

        let rows = stmt.query_map(query, row_mapper)?;

        let messages: Result<Vec<MessageDTO>, rusqlite::Error> = rows.collect();

        messages.map(|mut v| {
            v.reverse();
            v
        })
    }

    fn message_role_from_int(role: i32) -> RoleType {
        match role {
            0 => RoleType::User,
            1 => RoleType::Assistant,
            2 => RoleType::System,
            _ => panic!("Invalid role type"),
        }
    }

    const fn message_role_to_int(role: &RoleType) -> i32 {
        match role {
            RoleType::User => 0,
            RoleType::Assistant => 1,
            RoleType::System => 2,
        }
    }

    fn row_to_message(row: &rusqlite::Row) -> Result<MessageDTO, rusqlite::Error> {
        Ok(MessageDTO {
            id: row.get(0)?,
            conversation_id: row.get(1)?,
            content: row.get(2)?,
            reasoning: row.get(3)?,
            timestamp: row.get(4)?,
            role: Self::message_role_from_int(row.get(5)?),
            summary: row.get(6).ok(),
            chunks: serde_json::from_str::<Vec<MessageUsedRagChunk>>(row.get::<_, String>(7).unwrap_or("[]".to_string()).as_str())
            .unwrap_or_default(),
        })
    }
}
