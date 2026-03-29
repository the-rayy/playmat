#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub enum ServerMessage {
  Empty,
  SignIn(SignInToken),
}

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct SignInToken {
  pub token: String,
}
