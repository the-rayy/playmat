use crate::framework::{self, Event, gui};

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
  [CellPosition::TopLeft, CellPosition::TopMiddle, CellPosition::TopRight],
  [CellPosition::CenterLeft, CellPosition::CenterMiddle, CellPosition::CenterRight],
  [CellPosition::BottomLeft, CellPosition::BottomMiddle, CellPosition::BottomRight],
  // Columns
  [CellPosition::TopLeft, CellPosition::CenterLeft, CellPosition::BottomLeft],
  [CellPosition::TopMiddle, CellPosition::CenterMiddle, CellPosition::BottomMiddle],
  [CellPosition::TopRight, CellPosition::CenterRight, CellPosition::BottomRight],
  // Diagonals
  [CellPosition::TopLeft, CellPosition::CenterMiddle, CellPosition::BottomRight],
  [CellPosition::TopRight, CellPosition::CenterMiddle, CellPosition::BottomLeft],
];

fn grey() -> crate::math::Color {
  crate::math::Color::new(0.5, 0.5, 0.5, 1.0)
}
fn green() -> crate::math::Color {
  crate::math::Color::new(0.3, 0.8, 0.3, 1.0)
}
fn red() -> crate::math::Color {
  crate::math::Color::new(0.8, 0.3, 0.3, 1.0)
}

fn winner_button_id() -> String {
  String::from("winner")
}

pub struct Game {
  initialized: bool,
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
      initialized: false,
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

  fn play_move(&mut self, ctx: &mut framework::Context, id: &String, state: CellState) {
    self.set_cell_state(id, state);
    ctx.gui.get_mut_button(id).color = state.color();

    if let Some(winner) = check_winner(&self.grid) {
      ctx.gui.get_mut_button(&winner_button_id()).color = winner.color();
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

  fn setup_ui(&self, ctx: &mut framework::Context) {
    for &pos in CELL_POSITIONS.iter() {
      let btn = framework::gui::Button::new(pos.rect(), grey());
      ctx.gui.add_button(pos.id().to_string(), btn);
    }

    let winner_rect = crate::math::Rect::new(-0.4, -0.7, 0.8, 0.15);
    let btn = framework::gui::Button::new(winner_rect, grey());
    ctx.gui.add_button(winner_button_id(), btn);
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
  fn id(self) -> &'static str {
    match self {
      CellPosition::TopLeft => "tl",
      CellPosition::TopMiddle => "tm",
      CellPosition::TopRight => "tr",
      CellPosition::CenterLeft => "cl",
      CellPosition::CenterMiddle => "cm",
      CellPosition::CenterRight => "cr",
      CellPosition::BottomLeft => "bl",
      CellPosition::BottomMiddle => "bm",
      CellPosition::BottomRight => "br",
    }
  }

  /// Screen rect for this cell's button.
  fn rect(self) -> crate::math::Rect {
    let (x, y) = match self {
      CellPosition::TopLeft => (-0.35, 0.15),
      CellPosition::TopMiddle => (-0.1, 0.15),
      CellPosition::TopRight => (0.15, 0.15),
      CellPosition::CenterLeft => (-0.35, -0.1),
      CellPosition::CenterMiddle => (-0.1, -0.1),
      CellPosition::CenterRight => (0.15, -0.1),
      CellPosition::BottomLeft => (-0.35, -0.35),
      CellPosition::BottomMiddle => (-0.1, -0.35),
      CellPosition::BottomRight => (0.15, -0.35),
    };
    crate::math::Rect::new(x, y, 0.2, 0.2)
  }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum CellState {
  Open,
  Player1,
  Player2,
}

impl CellState {
  fn color(self) -> crate::math::Color {
    match self {
      CellState::Open => grey(),
      CellState::Player1 => green(),
      CellState::Player2 => red(),
    }
  }
}

fn check_winner(grid: &[Cell]) -> Option<CellState> {
  let get = |pos: CellPosition| -> CellState {
    grid.iter().find(|c| c.pos == pos).unwrap().state
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
