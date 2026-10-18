use playmat_engine::{
  self,
  event::Event,
  framework::Context,
  gui::{self, Button, Label},
  math::{self, Rect},
  rendering::texture::TextureKey,
};

const CELL_POSITIONS: [CellPosition; 9] = [
  CellPosition::TopLeft,
  CellPosition::TopMiddle,
  CellPosition::TopRight,
  CellPosition::CenterLeft,
  CellPosition::CenterMiddle,
  CellPosition::CenterRight,
  CellPosition::BottomLeft,
  CellPosition::BottomMiddle,
  CellPosition::BottomRight,
];

const WIN_LINES: [[CellPosition; 3]; 8] = [
  // Rows
  [
    CellPosition::TopLeft,
    CellPosition::TopMiddle,
    CellPosition::TopRight,
  ],
  [
    CellPosition::CenterLeft,
    CellPosition::CenterMiddle,
    CellPosition::CenterRight,
  ],
  [
    CellPosition::BottomLeft,
    CellPosition::BottomMiddle,
    CellPosition::BottomRight,
  ],
  // Columns
  [
    CellPosition::TopLeft,
    CellPosition::CenterLeft,
    CellPosition::BottomLeft,
  ],
  [
    CellPosition::TopMiddle,
    CellPosition::CenterMiddle,
    CellPosition::BottomMiddle,
  ],
  [
    CellPosition::TopRight,
    CellPosition::CenterRight,
    CellPosition::BottomRight,
  ],
  // Diagonals
  [
    CellPosition::TopLeft,
    CellPosition::CenterMiddle,
    CellPosition::BottomRight,
  ],
  [
    CellPosition::TopRight,
    CellPosition::CenterMiddle,
    CellPosition::BottomLeft,
  ],
];

fn white() -> math::Color {
  math::Color::new(1.0, 1.0, 1.0, 1.0)
}

fn black() -> math::Color {
  math::Color::new(0.0, 0.0, 0.0, 1.0)
}

fn grey() -> math::Color {
  math::Color::new(0.5, 0.5, 0.5, 1.0)
}

pub struct Game {
  grid: Vec<Cell>,
}

impl Game {
  pub fn new() -> Self {
    let grid = CELL_POSITIONS
      .iter()
      .map(|&pos| Cell {
        id: pos.id().to_string(),
        pos,
        state: CellState::Open,
      })
      .collect();

    Self {
      grid,
    }
  }

  fn set_cell_state(&mut self, id: &String, state: CellState) {
    self
      .grid
      .iter_mut()
      .find(|c| &c.id == id)
      .expect("unknown cell id")
      .state = state;
  }

  fn first_open_cell_id(&self) -> Option<String> {
    self
      .grid
      .iter()
      .find(|c| c.state == CellState::Open)
      .map(|c| c.id.clone())
  }

  fn play_move(&mut self, ctx: &mut Context, id: &String, state: CellState) {
    self.set_cell_state(id, state);
    ctx.gui.get_mut_button(id).texture_key = state.tex();

    if check_winner(&self.grid) == Some(CellState::Player1) {
      let label_quads = Label::new(
        String::from("P1 won"),
        math::Rect::new(-0.3, -0.7, 0.10, 0.15),
        white(),
        Self::tex_key_font(),
      )
      .quads(ctx.assets.get_fonts());
      for (i, q) in label_quads.into_iter().enumerate() {
        ctx.gui.add_quad(format!("label_{}", i), q);
      }
    }

    if check_winner(&self.grid) == Some(CellState::Player2) {
      let label_quads = Label::new(
        String::from("P2 won"),
        math::Rect::new(-0.3, -0.7, 0.10, 0.15),
        white(),
        Self::tex_key_font(),
      )
      .quads(ctx.assets.get_fonts());
      for (i, q) in label_quads.into_iter().enumerate() {
        ctx.gui.add_quad(format!("label_{}", i), q);
      }
    }
  }

  fn is_game_over(&self) -> bool {
    check_winner(&self.grid).is_some()
  }

  fn handle_click(&mut self, ctx: &mut Context, id: &String) {
    if self.is_game_over() {
      return;
    }

    if id == "start" {
      ctx.gui.reset();
      self.setup_ui(ctx);
      return;
    }

    self.play_move(ctx, id, CellState::Player1);

    if self.is_game_over() {
      return;
    }

    if let Some(open_id) = self.first_open_cell_id() {
      self.play_move(ctx, &open_id, CellState::Player2);
    }
  }

  fn tex_key_white() -> TextureKey {
    TextureKey("white".to_string())
  }
  fn tex_key_circle() -> TextureKey {
    TextureKey("circle".to_string())
  }
  fn tex_key_cross() -> TextureKey {
    TextureKey("cross".to_string())
  }
  fn tex_key_font() -> TextureKey {
    TextureKey("font".to_string())
  }

  fn setup_startscreen(&self, ctx: &mut Context) {
    ctx
      .assets
      .load_font(
        Self::tex_key_font(),
        include_bytes!("assets/Kenney Rocket.ttf"),
      )
      .expect("could not load font");

    let pos = Rect::new(-0.3, -0.1, 0.6, 0.2);
    let btn = Button::new(pos, grey(), Self::tex_key_white());
    ctx.gui.add_button("start".to_string(), btn);
    let label_quads = Label::new(
      String::from("Start"),
      math::Rect::new(-0.25, -0.05, 0.07, 0.1),
      black(),
      Self::tex_key_font(),
    )
    .quads(ctx.assets.get_fonts());
    for (i, q) in label_quads.into_iter().enumerate() {
      ctx.gui.add_quad(format!("startgame_{}", i), q);
    }

    let label_quads = Label::new(
      String::from("tictactoe"),
      math::Rect::new(-0.35, 0.4, 0.07, 0.1),
      white(),
      Self::tex_key_font(),
    )
    .quads(ctx.assets.get_fonts());
    for (i, q) in label_quads.into_iter().enumerate() {
      ctx.gui.add_quad(format!("ttt_{}", i), q);
    }
  }

  fn setup_ui(&self, ctx: &mut Context) {
    ctx
      .assets
      .load_texture(Self::tex_key_white(), include_bytes!("assets/tx_white.png"))
      .expect("could not load white texture");
    ctx
      .assets
      .load_texture(
        Self::tex_key_circle(),
        include_bytes!("assets/tx_circle.png"),
      )
      .expect("could not load circle texture");
    ctx
      .assets
      .load_texture(Self::tex_key_cross(), include_bytes!("assets/tx_cross.png"))
      .expect("could not load cross texture");
    ctx
      .assets
      .load_font(
        Self::tex_key_font(),
        include_bytes!("assets/Kenney Rocket.ttf"),
      )
      .expect("could not load font");

    for &pos in CELL_POSITIONS.iter() {
      let btn = Button::new(pos.rect(), grey(), Self::tex_key_white());
      ctx.gui.add_button(pos.id().to_string(), btn);
    }
  }
}

impl playmat_engine::framework::Game for Game {
  fn init(&mut self, ctx: &mut Context) {
    self.setup_startscreen(ctx);
  }

  fn update(&mut self, ctx: &mut Context) {
    for ev in ctx.events() {
      match ev {
        Event::Gui(gui::Event::ButtonClicked { id }) => {
          self.handle_click(ctx, &id);
        }
      }
    }
  }
}

struct Cell {
  id: String,
  pos: CellPosition,
  state: CellState,
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum CellPosition {
  TopLeft,
  TopMiddle,
  TopRight,
  CenterLeft,
  CenterMiddle,
  CenterRight,
  BottomLeft,
  BottomMiddle,
  BottomRight,
}

impl CellPosition {
  /// Short id used both as the grid `Cell::id` and the gui button id.
  const fn id(self) -> &'static str {
    match self {
      Self::TopLeft => "tl",
      Self::TopMiddle => "tm",
      Self::TopRight => "tr",
      Self::CenterLeft => "cl",
      Self::CenterMiddle => "cm",
      Self::CenterRight => "cr",
      Self::BottomLeft => "bl",
      Self::BottomMiddle => "bm",
      Self::BottomRight => "br",
    }
  }

  /// Screen rect for this cell's button.
  const fn rect(self) -> math::Rect {
    let (x, y) = match self {
      Self::TopLeft => (-0.35, 0.15),
      Self::TopMiddle => (-0.1, 0.15),
      Self::TopRight => (0.15, 0.15),
      Self::CenterLeft => (-0.35, -0.1),
      Self::CenterMiddle => (-0.1, -0.1),
      Self::CenterRight => (0.15, -0.1),
      Self::BottomLeft => (-0.35, -0.35),
      Self::BottomMiddle => (-0.1, -0.35),
      Self::BottomRight => (0.15, -0.35),
    };
    math::Rect::new(x, y, 0.2, 0.2)
  }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum CellState {
  Open,
  Player1,
  Player2,
}

impl CellState {
  fn tex(self) -> TextureKey {
    match self {
      Self::Open => Game::tex_key_white(),
      Self::Player1 => Game::tex_key_cross(),
      Self::Player2 => Game::tex_key_circle(),
    }
  }
}

fn check_winner(grid: &[Cell]) -> Option<CellState> {
  let get = |pos: CellPosition| -> CellState {
    grid
      .iter()
      .find(|c| c.pos == pos)
      .expect("no more empty cells")
      .state
  };

  WIN_LINES.iter().find_map(|line| {
    let [a, b, c] = line.map(get);
    if a != CellState::Open && a == b && b == c {
      Some(a)
    } else {
      None
    }
  })
}
