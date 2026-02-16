#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct Envelope {
  pub data: Data,

  timestamp: i64,
}

impl Default for Envelope {
  fn default() -> Self {
    Self::new()
  }
}

impl Envelope {
  pub fn new() -> Envelope {
    let t = time::OffsetDateTime::now_utc();
    Envelope {
      data: Data::Empty,
      timestamp: t.unix_timestamp(),
    }
  }

  pub fn timestamp(&self) -> time::OffsetDateTime {
    time::OffsetDateTime::from_unix_timestamp(self.timestamp).unwrap()
  }

  pub fn to_bytes(self) -> Vec<u8> {
    bitcode::encode(&self)
  }

  pub fn from_bytes(bin: &[u8]) -> Result<Envelope, String> {
    bitcode::decode(bin).map_err(|e| format!("{e}"))
  }
}

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub enum Data {
  Empty,
}
