mod database;
use password::manager::manager::{get_all_passwords, get_password};
use profile::history::history::{add_record, delete_all_records};
pub use rusqlite::{Connection, Result};
mod encrypt;

use database::warden_db;
mod profile {
    pub(crate) mod history;
    pub(crate) mod user;
}
mod password {
    pub(crate) mod manager;
}

use crate::password::manager::manager::{
    self, delete_secret, edit_secret_label, edit_secret_password, Secret,
};
use crate::profile::history;
use crate::profile::user::profile as user_profile;
fn main() {
    let _ = warden_db::create_db_tables();
    let user = {
        user_profile::User {
            username: "ahmed".to_string(),
            master_password: "qsdfq55446".to_string(),
        }
    };
    let _ = user.create_user();

    let secret = {
        manager::Secret {
            label: "facebook".to_string(),
            password: encrypt::encrypt_pass("qdsklfjsqdf".to_string()),
        }
    };
    let _ = secret.create_secret();
    // let _ = delete_secret(secret.label);
    // let _ = edit_secret_password(&secret.label.to_string(), "newpass".to_string());
    let _ = edit_secret_label(&secret.label, "instagram".to_string());
    // let _ = delete_all_records();
    let password = get_password("instagram".to_string());
    let passwords = get_all_passwords();
    let _ = println!("done {:?}", password);
    let _ = println!("done {:?}", passwords);
}
