use std::str::FromStr;

use crate::utils::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct StorageDatabase {
    pub connection: DatabaseConnection,
}

impl StorageDatabase {
    /// # Errors
    /// # Panics
    pub fn new(connection: DatabaseConnection) -> Self {
        connection
            .lock()
            .expect("Failed to lock connection")
            .execute(
                "CREATE TABLE IF NOT EXISTS storage (
          id INTEGER PRIMARY KEY,
          key TEXT NOT NULL UNIQUE,
          value TEXT NOT NULL
        )",
                [],
            )
            .expect("Failed to create storage table");

        Self { connection }
    }

    /// # Errors
    /// # Panics
    pub fn write<T>(&self, key: &str, value: T) -> Result<(), rusqlite::Error>
    where
        T: Into<String>,
    {
        let connection = self.connection.lock().expect("Failed to lock connection");

        connection.execute(
            "INSERT OR REPLACE INTO storage (key, value) VALUES (?, ?)",
            rusqlite::params![key, value.into()],
        )?;

        Ok(())
    }

    /// # Errors
    /// # Panics
    pub fn read<T>(&self, key: &str) -> Result<T, rusqlite::Error>
    where
        T: FromStr,
    {
        let connection = self.connection.lock().expect("Failed to lock connection");

        let mut stmt = connection.prepare("SELECT value FROM storage WHERE key = ?")?;
        let value: String = stmt.query_row(rusqlite::params![key], |row| row.get(0))?;

        value
            .parse::<T>()
            .map_err(|_| rusqlite::Error::InvalidQuery)
    }
}
