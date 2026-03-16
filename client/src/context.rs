#[derive(PartialEq, Eq, Hash)]
pub enum Scene {
  Login,
  Rooms,
}

pub struct Context {
  pub debug: String,
  pub timestamp: Option<time::OffsetDateTime>,
  pub token: Option<String>,
  pub scene: Scene,
}

impl Default for Context {
  fn default() -> Self {
      Self {
        debug: Default::default(),
        timestamp: Default::default(),
        token: Default::default(),
        scene: Scene::Login,
    }
  }
}
