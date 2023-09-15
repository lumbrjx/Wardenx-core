use wardenx_core::warden_db;

fn main() {
    let _ = warden_db::connect_to_db();
    println!("done");
}
