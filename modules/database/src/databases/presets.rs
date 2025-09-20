use rusqlite::params;
use types::dto::{PresetDTO, PresetId};

use crate::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct PresetsDatabase {
    connection: DatabaseConnection,
}

impl PresetsDatabase {
    /// # Panics
    pub fn new(connection: DatabaseConnection) -> Self {
        connection
            .lock()
            .expect("Failed to lock connection")
            .execute(
                "CREATE TABLE IF NOT EXISTS presets (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    prompt TEXT NOT NULL,
                    temperature REAL NOT NULL,
                    max_tokens INTEGER NOT NULL
                )",
                [],
            )
            .expect("Failed to create presets table");

        Self { connection }
    }

    /// # Errors
    /// # Panics
    pub fn add_preset(
        &self,
        name: &str,
        prompt: &str,
        temperature: f32,
        max_tokens: u32,
    ) -> Result<PresetDTO, String> {
        let connection = self.connection.lock().unwrap();
        let mut stmt = connection
            .prepare(
                "INSERT INTO presets (name, prompt, temperature, max_tokens) VALUES (?, ?, ?, ?)",
            )
            .map_err(|e| e.to_string())?;

        stmt.execute((name, prompt, temperature, max_tokens))
            .map_err(|e| e.to_string())?;

        let id = connection.last_insert_rowid();
        Ok(PresetDTO {
            id,
            name: name.to_string(),
            prompt: prompt.to_string(),
            temperature,
            max_tokens,
        })
    }

    /// # Errors
    /// # Panics
    pub fn get_preset(&self, id: PresetId) -> Result<PresetDTO, String> {
        let connection = self.connection.lock().expect("Failed to lock connection");
        let mut stmt = connection
            .prepare("SELECT * FROM presets WHERE id = ?")
            .map_err(|e| e.to_string())?;

        let preset = stmt
            .query_row((id,), |row| {
                Ok(PresetDTO {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    prompt: row.get(2)?,
                    temperature: row.get(3)?,
                    max_tokens: row.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?;

        Ok(preset)
    }

    /// # Errors
    /// # Panics
    pub fn get_all_presets(&self) -> Result<Vec<PresetDTO>, String> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        let mut stmt = connection
            .prepare("SELECT * FROM presets")
            .map_err(|e| e.to_string())?;

        let converter = |row: &rusqlite::Row| -> Result<PresetDTO, rusqlite::Error> {
            Ok(PresetDTO {
                id: row.get(0)?,
                name: row.get(1)?,
                prompt: row.get(2)?,
                temperature: row.get(3)?,
                max_tokens: row.get(4)?,
            })
        };

        let query = params![];

        let presets = stmt.query_map(query, converter);

        let pres: Result<Vec<PresetDTO>, rusqlite::Error> =
            presets.map_err(|e| e.to_string())?.collect();

        pres.map_err(|e| e.to_string())
    }

    /// # Errors
    /// # Panics
    pub fn update_preset(&self, dto: &PresetDTO) -> Result<PresetDTO, String> {
        let connection = self.connection.lock().expect("Failed to lock connection");
        let mut stmt = connection
            .prepare(
                "UPDATE presets SET name = ?, prompt = ?, temperature = ?, max_tokens = ? WHERE id = ?",
            )
            .map_err(|e| e.to_string())?;

        stmt.execute((
            dto.name.clone(),
            dto.prompt.clone(),
            dto.temperature,
            dto.max_tokens,
            dto.id,
        ))
        .map_err(|e| e.to_string())?;

        Ok(dto.clone())
    }

    /// # Errors
    /// # Panics
    pub fn delete_preset(&self, id: PresetId) -> Result<(), String> {
        let connection = self.connection.lock().expect("Failed to lock connection");
        let mut stmt = connection
            .prepare("DELETE FROM presets WHERE id = ?")
            .map_err(|e| e.to_string())?;

        stmt.execute((id,)).map_err(|e| e.to_string())?;
        Ok(())
    }
}
