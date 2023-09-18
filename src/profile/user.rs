pub use rusqlite::{Connection, Result};

pub use crate::database::*;
use cuid2::cuid;
// Profile
pub mod profile {
    use crate::{database::warden_db::db_connect, profile::history::history::add_record};

    use super::*;

    pub struct User {
        pub username: String,
        pub master_password: String,
    }
    // CRUD

    impl User {
        // Create User
        pub fn create_user(&self) -> Result<()> {
            let conn = db_connect()?;
            conn.execute(
                "INSERT INTO profile (id, username, master_password) VALUES (?, ?, ?)",
                (cuid(), &self.username, &self.master_password),
            )?;
            let _ = add_record("new user is created".to_string());
            Ok(())
        }
    }
}
