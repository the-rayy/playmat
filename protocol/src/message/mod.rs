use time::OffsetDateTime;

use crate::message::{client::ClientMessage, server::ServerMessage};

pub mod client;
pub mod server;

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct ClientMessageEnvelope {
  pub msg: ClientMessage,

  timestamp: i64,
}

impl ClientMessageEnvelope {
  pub fn new(msg: ClientMessage) -> Self {
    Self {
      msg,
      timestamp: crate::time::now_utc().unix_timestamp(),
    }
  }

  pub fn timestamp(&self) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(self.timestamp).unwrap()
  }

  pub fn to_bytes(self) -> Vec<u8> {
    bitcode::encode(&self)
  }

  pub fn from_bytes(bin: &[u8]) -> Result<Self, String> {
    bitcode::decode(bin).map_err(|e| format!("{e}"))
  }
}

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct ServerMessageEnvelope {
  pub msg: ServerMessage,

  timestamp: i64,
}

impl ServerMessageEnvelope {
  pub fn new(msg: ServerMessage) -> Self {
    Self {
      msg,
      timestamp: crate::time::now_utc().unix_timestamp(),
    }
  }

  pub fn timestamp(&self) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(self.timestamp).unwrap()
  }

  pub fn to_bytes(self) -> Vec<u8> {
    bitcode::encode(&self)
  }

  pub fn from_bytes(bin: &[u8]) -> Result<Self, String> {
    bitcode::decode(bin).map_err(|e| format!("{e}"))
  }
}
