use std::sync::{Arc, Mutex};

use rusqlite::ffi::sqlite3_auto_extension;
use sqlite_vec::sqlite3_vec_init;

pub type DatabaseConnection = Arc<Mutex<rusqlite::Connection>>;

/// # Panics
#[must_use] pub fn create_database_connection(db_path: &str) -> DatabaseConnection {
    unsafe {
        #[allow(clippy::missing_transmute_annotations)]
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    }

    let connection = rusqlite::Connection::open(db_path).expect("Failed to open database");

    Arc::new(Mutex::new(connection))
}
