mod config;
mod handlers;
mod http;

#[tokio::main]
async fn main() {
  env_logger::init();
  http::run(config::SERVER_IP_PORT)
    .await
    .expect("http server crashed");
}
