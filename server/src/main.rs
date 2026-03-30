mod config;
mod http;
mod handlers;

#[tokio::main]
async fn main() {
  env_logger::init();
  if let Err(e) = http::run(config::SERVER_IP_PORT).await {
    panic!("{}", e);
  }
}
