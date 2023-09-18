pub use rusqlite::{Connection, Result};

// Database
pub mod warden_db {

    use super::*;
    pub fn db_connect() -> Result<Connection> {
        let path = "./dev44.db3";

        let conn = Connection::open(path)?;
        Ok(conn)
    }
    // Create a local database if it doesn't exist *path modified later*
    // Create profile, passwords, history tables
    pub fn create_db_tables() -> Result<Connection> {
        let conn = db_connect()?;

        // Profile table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS profile (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                master_password TEXT NOT NULL,
                recovery_key TEXT NOT NULL
            )",
            [],
        )?;

        // Passwords table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS passwords (
                id TEXT PRIMARY KEY,
                label TEXT NOT NULL,
                password TEXT NOT NULL
            )",
            [],
        )?;

        // History table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id TEXT PRIMARY KEY,
                operation TEXT NOT NULL,
                date_time TEXT NOT NULL
            )",
            [],
        )?;
        Ok(conn)
    }
}
