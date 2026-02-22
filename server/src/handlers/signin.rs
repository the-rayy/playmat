use std::time::Duration;

use shared::message::{
  client::SignInCredentials,
  server::{ServerMessage, SignInToken},
};
use tokio::time::sleep;

pub async fn handler(_data: SignInCredentials) -> ServerMessage {
  sleep(Duration::from_secs(3)).await;
  ServerMessage::SignIn(SignInToken {
    token: String::from("asd"),
  })
}
