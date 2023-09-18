pub use rusqlite::{Connection, Result};

pub use crate::database::*;
use cuid2::cuid;
// Profile
pub mod profile {
    use rusqlite::params;

    use crate::{database::warden_db::db_connect, profile::history::history::add_record};

    use super::*;
    #[derive(Debug)]
    pub struct User {
        pub username: String,
        pub master_password: String,
        pub recovery_key: String,
    }
    // CRUD

    impl User {
        // Create User
        pub fn create_user(&self) -> Result<()> {
            let conn = db_connect()?;
            conn.execute(
                "INSERT INTO profile (id, username, master_password, recovery_key) VALUES (?, ?, ?, ?)",
                (
                    cuid(),
                    &self.username,
                    &self.master_password,
                    &self.recovery_key,
                ),
            )?;
            let _ = add_record("new user is created".to_string());
            Ok(())
        }
    }
    pub fn get_profile_table_length() -> Result<usize> {
        let conn = db_connect()?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM profile")?;
        let count = stmt.query_row([], |row| row.get(0))?;
        Ok(count)
    }
    pub fn get_user() -> Result<Option<User>> {
        let conn = db_connect()?;
        let mut stmt = conn.prepare("SELECT * FROM profile")?;
        let mut rows = stmt.query(params![])?;

        if let Some(row) = rows.next()? {
            let secret = User {
                username: row.get(1)?,
                master_password: row.get(2)?,
                recovery_key: row.get(3)?,
            };

            Ok(Some(secret))
        } else {
            Ok(None)
        }
    }
    pub fn edit_master_password(label: &String, new_password: String) -> Result<()> {
        let conn = db_connect()?;
        conn.execute(
            "UPDATE profile SET master_password = ?1 WHERE username = ?2",
            [new_password, label.to_string()],
        )?;
        let _ = add_record("the matser passowrd is edited: ".to_string() + &label);

        Ok(())
    }
    pub fn edit_recovery_key(label: &String, new_password: String) -> Result<()> {
        let conn = db_connect()?;
        conn.execute(
            "UPDATE profile SET recovery_key = ?1 WHERE username = ?2",
            [new_password, label.to_string()],
        )?;
        let _ = add_record("the recovery_key is edited: ".to_string() + &label);

        Ok(())
    }
    pub fn edit_username(label: &String, new_label: String) -> Result<()> {
        let conn = db_connect()?;
        conn.execute(
            "UPDATE profile SET username = ?1 WHERE username = ?2",
            [new_label, label.to_string()],
        )?;
        let _ = add_record("the username is edited: ".to_string() + &label);

        Ok(())
    }
}
