use cuid2::cuid;

use crate::profile::history::history::add_record;

pub fn generate_random_string() -> String {
    let rand = cuid();
    let _ = add_record("random string is generated".to_string());
    rand
}
