pub use rusqlite::{Connection, Result};

// Database
pub mod warden_db {

    use super::*;

    #[derive(Debug)]
    pub struct Profile {
        id: i32,
        username: String,
        master_password: String,
    }

    // Create a local database if it doesn't exist *path modified later*
    // Create profile, passwords, history tables
    pub fn connect_to_db() -> Result<()> {
        let path = "./dev1b.db3";
        let conn = Connection::open(path)?;

        // Profile table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS profile (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                master_password TEXT NOT NULL,
                reset_question1 TEXT NOT NULL,
                reset_question2 TEXT NOT NULL
            )",
            [],
        )?;

        // Passwords table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS passwords (
                id INTEGER PRIMARY KEY,
                label TEXT NOT NULL,
                password TEXT NOT NULL
            )",
            [],
        )?;

        // History table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY,
                operation TEXT NOT NULL,
                date_time TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }
}
