mod api;

#[tokio::main]
async fn main() {
    init_log();
    api::run().await;
}

fn init_log() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env().init();
}
