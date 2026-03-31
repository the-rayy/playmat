use crate::{message::ServerMessageEnvelope, time::now_utc};

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub enum ServerMessage {
  Empty,
  SignIn(SignInToken),
}

impl ServerMessage {
  pub fn pack(self) -> ServerMessageEnvelope {
    ServerMessageEnvelope { msg: self, timestamp: now_utc().unix_timestamp() }
  }
}

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct SignInToken {
  pub token: String,
}
