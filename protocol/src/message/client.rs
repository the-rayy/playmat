use crate::{message::ClientMessageEnvelope, time::now_utc};

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub enum ClientMessage {
  SignIn(SignInCredentials),
}

impl ClientMessage {
  pub fn pack(self) -> ClientMessageEnvelope {
    ClientMessageEnvelope { msg: self, timestamp: now_utc().unix_timestamp() }
  }
}

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct SignInCredentials {
  pub username: String,
}
