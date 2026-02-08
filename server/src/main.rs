use server::run_server;

#[tokio::main]
async fn main() {
  env_logger::init();
  run_server().await;
}
