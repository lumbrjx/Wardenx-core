pub use rusqlite::{Connection, Result};

pub use crate::database::*;
use cuid2::cuid;
// Manager
pub mod manager {
    use rusqlite::params;

    use crate::{database::warden_db::db_connect, profile::history::history::add_record};

    use super::*;

    #[derive(Debug)]
    pub struct Secret {
        pub label: String,
        pub password: String,
    }
    impl Secret {
        pub fn create_secret(&self) -> Result<()> {
            let conn = db_connect()?;
            conn.execute(
                "INSERT INTO passwords (id, label, password) VALUES (?, ?, ?)",
                (cuid(), &self.label, &self.password),
            )?;
            let _ = add_record("new passowrd is created: ".to_string() + &self.label);
            Ok(())
        }
    }
    pub fn delete_secret(label: String) -> Result<()> {
        let conn = db_connect()?;
        conn.execute("DELETE FROM passwords WHERE label = ?1", [label.clone()])?;
        let _ = add_record("a passowrd is deleted: ".to_string() + &label);
        Ok(())
    }
    pub fn edit_secret_password(label: &String, new_password: String) -> Result<()> {
        let conn = db_connect()?;
        conn.execute(
            "UPDATE passwords SET password = ?1 WHERE label = ?2",
            [new_password, label.to_string()],
        )?;
        let _ = add_record("a passowrd is edited: ".to_string() + &label);

        Ok(())
    }
    pub fn edit_secret_label(label: &String, new_label: String) -> Result<()> {
        let conn = db_connect()?;
        conn.execute(
            "UPDATE passwords SET label = ?1 WHERE label = ?2",
            [new_label, label.to_string()],
        )?;
        let _ = add_record("a passowrd is edited: ".to_string() + &label);

        Ok(())
    }
    pub fn get_password(label: String) -> Result<Option<Secret>> {
        let conn = db_connect()?;
        let mut stmt = conn.prepare("SELECT * FROM passwords WHERE label = ?1")?;
        let mut rows = stmt.query(params![label])?;

        if let Some(row) = rows.next()? {
            let secret = Secret {
                label: row.get(1)?,
                password: row.get(2)?,
            };

            Ok(Some(secret))
        } else {
            Ok(None)
        }
    }
    pub fn get_all_passwords() -> Result<Vec<Secret>> {
        let conn = db_connect()?;
        let mut stmt = conn.prepare("SELECT * FROM passwords")?;
        let rows = stmt.query_map(params![], |row| {
            Ok(Secret {
                label: row.get(1)?,
                password: row.get(2)?,
            })
        })?;

        let mut passwords = Vec::new();
        for password in rows {
            passwords.push(password?);
        }

        Ok(passwords)
    }
}
