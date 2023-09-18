pub mod database;
pub use database::*;
pub mod encrypt;
pub use encrypt::*;
pub mod profile {
    pub mod history;
    pub mod user;
}
pub mod password {
    pub mod generator;
    pub mod manager;
}
pub use password::generator::*;
pub use password::manager::*;
pub use password::*;
pub use profile::history::*;
pub use profile::user::*;
pub use profile::*;
pub mod sayhi {

    pub use super::*;
    pub fn hi() {
        println!("hey");
    }
}
