#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub enum ClientMessage {
  SignIn(SignInCredentials),
}

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct SignInCredentials {
  pub email: String,
  pub password: String,
}
