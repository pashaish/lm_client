use rusqlite::params;
use types::dto::{ProviderDTO, ProviderID};

use crate::utils::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct ProvidersDatabase {
    pub connection: DatabaseConnection,
}

impl ProvidersDatabase {
    /// # Panics
    pub fn new(connection: DatabaseConnection) -> Self {
        connection
            .lock()
            .expect("Failed to lock connection")
            .execute(
                "CREATE TABLE IF NOT EXISTS providers (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    url TEXT NOT NULL,
                    api_key TEXT NOT NULL,
                    default_model TEXT
        )",
                [],
            )
            .expect("Failed to create providers table");

        Self { connection }
    }

    /// # Panics
    #[must_use] pub fn get_provider(&self, id: ProviderID) -> Option<ProviderDTO> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        let mut stmt = connection
            .prepare("SELECT id, name, url, api_key, default_model FROM providers WHERE id = ?")
            .expect("Failed to prepare statement");

        let provider = stmt
            .query_map([id], |row| Ok(Self::row_to_dto(row)))
            .expect("Failed to query provider")
            .next()
            .and_then(std::result::Result::ok);

        provider
    }

    /// # Errors
    /// # Panics
    pub fn add_provider(
        &self,
        name: &str,
        url: &str,
        api_key: &str,
        default_model: &str,
    ) -> rusqlite::Result<ProviderID> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        connection.execute(
            "INSERT INTO providers (name, url, api_key, default_model) VALUES (?, ?, ?, ?)",
            [name, url, api_key, default_model],
        )?;

        let id = connection.last_insert_rowid() as ProviderID;

        Ok(id)
    }

    /// # Errors
    /// # Panics
    pub fn update_provider(&self, dto: &ProviderDTO) -> rusqlite::Result<ProviderID> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        connection.execute(
            "UPDATE providers SET name = ?, url = ?, api_key = ?, default_model = ? WHERE id = ?",
            params![dto.name, dto.url, dto.api_key, dto.default_model, dto.id],
        )?;

        Ok(dto.id)
    }

    /// # Errors
    /// # Panics
    pub fn delete_provider(&self, id: ProviderID) -> rusqlite::Result<ProviderID> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        connection.execute("DELETE FROM providers WHERE id = ?", [id])?;

        Ok(id)
    }

    /// # Panics
    #[must_use] pub fn get_providers(&self) -> Vec<ProviderDTO> {
        let connection = self.connection.lock().expect("Failed to lock connection");

        let mut stmt = connection
            .prepare("SELECT id, name, url, api_key, default_model FROM providers")
            .expect("Failed to prepare statement");

        let providers = stmt
            .query_map([], |row| Ok(Self::row_to_dto(row)))
            .expect("Failed to query providers")
            .filter_map(std::result::Result::ok)
            .collect();

        providers
    }

    fn row_to_dto(row: &rusqlite::Row) -> ProviderDTO {
        ProviderDTO {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            url: row.get(2).unwrap(),
            api_key: row.get(3).unwrap(),
            default_model: row.get(4).unwrap_or_default(),
        }
    }
}
