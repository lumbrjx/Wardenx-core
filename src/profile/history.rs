pub use rusqlite::{Connection, Result};
extern crate chrono;
use cuid2::cuid;

pub use crate::database::*;
use chrono::Local;
pub mod history {
    use rusqlite::params;

    use super::*;
    use crate::database::warden_db::db_connect;
    // create new history record
    pub fn add_record(record: String) -> Result<()> {
        let current_datetime = Local::now();
        let formatted_datetime = current_datetime.format("%d/%m/%y/%H:%M").to_string();
        let conn = db_connect()?;
        conn.execute(
            "INSERT INTO history (id, operation, date_time) VALUES (?, ?, ?)",
            (cuid(), record, formatted_datetime),
        )?;
        Ok(())
    }
    pub fn delete_all_records() -> Result<()> {
        let conn = db_connect()?;
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }
    pub struct Record {
        pub record: String,
        pub date_time: String,
    }
    pub fn get_history() -> Result<Vec<Record>> {
        let conn = db_connect()?;
        let mut stmt = conn.prepare("SELECT * FROM history")?;
        let rows = stmt.query_map(params![], |row| {
            Ok(Record {
                record: row.get(1)?,
                date_time: row.get(2)?,
            })
        })?;

        let mut records = Vec::new();
        for record in rows {
            records.push(record?);
        }

        Ok(records)
    }
}
