extern crate env_logger;
extern crate serde_derive;
extern crate warp;

mod api;

use tokio::task;

#[tokio::main]
async fn main() {
    init_log();

    let _ = task::spawn(api::run()).await;
}

fn init_log() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env().init();
}
