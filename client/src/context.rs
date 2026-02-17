#[derive(Default)]
pub struct Context {
  pub debug: String,
  pub timestamp: Option<time::OffsetDateTime>,
  pub token: Option<String>,
}
