pub mod database;
pub use database::*;
pub mod encrypt;
pub use encrypt::*;
pub mod profile {
    pub(crate) mod history;
    pub(crate) mod user;
}
pub mod password {
    pub(crate) mod generator;
    pub(crate) mod manager;
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
