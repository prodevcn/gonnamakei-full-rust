pub mod crypto;
pub mod file;
pub mod sync;

pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    pretty_env_logger::init();
}

pub fn try_init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    pretty_env_logger::try_init().unwrap_or_else(|_| {
        println!("Cannot init logger");
    });
}
