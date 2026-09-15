use engine::rendering::texture::Id;

use framework::{
  self, Event,
  gui::{self, Label},
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

fn grey() -> math::Color {
  math::Color::new(0.5, 0.5, 0.5, 1.0)
}

pub struct Game {
  initialized: bool,
  grid: Vec<Cell>,
  tex_white: Option<Id>,
  tex_circle: Option<Id>,
  tex_cross: Option<Id>,
  tex_font: Option<Id>,
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
      initialized: false,
      grid,
      tex_white: None,
      tex_circle: None,
      tex_cross: None,
      tex_font: None,
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

  const fn tex_for_state(&self, state: CellState) -> Id {
    match state {
      CellState::Open => self.tex_white.expect("white texture not loaded"),
      CellState::Player1 => self.tex_cross.expect("cross texture not loaded"),
      CellState::Player2 => self.tex_circle.expect("circle texture not loaded"),
    }
  }

  fn play_move(&mut self, ctx: &mut framework::Context, id: &String, state: CellState) {
    self.set_cell_state(id, state);
    let tex_key = self.tex_for_state(state);
    ctx.gui.get_mut_button(id).texture_key = tex_key;

    if check_winner(&self.grid) == Some(CellState::Player1) {
      let label_quads = Label::new(
        String::from("P1 won"),
        math::Rect::new(-0.3, -0.7, 0.10, 0.15),
        white(),
        self.tex_font.expect("font not loaded"),
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
        self.tex_font.expect("font not loaded"),
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

  fn handle_click(&mut self, ctx: &mut framework::Context, id: &String) {
    if self.is_game_over() {
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

  fn setup_ui(&mut self, ctx: &mut framework::Context) {
    let tex_white = ctx
      .assets
      .load_texture(include_bytes!("assets/tx_white.png"))
      .expect("could not load white texture");
    let tex_circle = ctx
      .assets
      .load_texture(include_bytes!("assets/tx_circle.png"))
      .expect("could not load circle texture");
    let tex_cross = ctx
      .assets
      .load_texture(include_bytes!("assets/tx_cross.png"))
      .expect("could not load cross texture");
    let tex_font = ctx
      .assets
      .load_font(include_bytes!("assets/Kenney Rocket.ttf"))
      .expect("could not load font");

    for &pos in CELL_POSITIONS.iter() {
      let btn = framework::gui::Button::new(pos.rect(), grey(), tex_white);
      ctx.gui.add_button(pos.id().to_string(), btn);
    }

    self.tex_white = Some(tex_white);
    self.tex_circle = Some(tex_circle);
    self.tex_cross = Some(tex_cross);
    self.tex_font = Some(tex_font);
  }
}

impl framework::Game for Game {
  fn update(&mut self, ctx: &mut framework::Context) {
    if !self.initialized {
      self.setup_ui(ctx);
      self.initialized = true;
      return;
    }

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
