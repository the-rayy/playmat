use std::time::Duration;

use protocol::message::{
  client::SignInCredentials,
  server::{ServerMessage, SignInToken},
};
use tokio::time::sleep;

pub async fn handler(data: SignInCredentials) -> ServerMessage {
  sleep(Duration::from_secs(3)).await;
  ServerMessage::SignIn(SignInToken {
    token: data.username,
  })
}
