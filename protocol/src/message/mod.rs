use time::OffsetDateTime;

use crate::message::{client::ClientMessage, server::ServerMessage};

pub mod client;
pub mod server;

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct ClientMessageEnvelope {
  msg: ClientMessage,

  timestamp: i64,
}

impl ClientMessageEnvelope {
  pub fn timestamp(&self) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(self.timestamp)
      .expect("could not read datetime from timestamp")
  }

  pub fn to_bytes(self) -> Vec<u8> {
    bitcode::encode(&self)
  }

  pub fn from_bytes(bin: &[u8]) -> Result<Self, String> {
    bitcode::decode(bin).map_err(|e| format!("{e}"))
  }

  pub fn unpack(self) -> ClientMessage {
    self.msg
  }
}

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct ServerMessageEnvelope {
  msg: ServerMessage,

  timestamp: i64,
}

impl ServerMessageEnvelope {
  pub fn timestamp(&self) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(self.timestamp)
      .expect("could not read datetime from timestamp")
  }

  pub fn to_bytes(self) -> Vec<u8> {
    bitcode::encode(&self)
  }

  pub fn from_bytes(bin: &[u8]) -> Result<Self, String> {
    bitcode::decode(bin).map_err(|e| format!("{e}"))
  }

  pub fn unpack(self) -> ServerMessage {
    self.msg
  }
}
