#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct Envelope {
  pub data: Data,
}

impl Envelope {
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
