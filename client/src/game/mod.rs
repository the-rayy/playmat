
use crate::framework::{self, Event, gui};

pub struct Game {
  initialized: bool,

  grid: Vec<Cell>,
}

impl Game {
  pub fn new() -> Self {
    let grid = vec![
      Cell {
        id: "tl".to_string(),
        pos: CellPosition::TopLeft,
        state: CellState::Open,
      },
      Cell {
        id: "tm".to_string(),
        pos: CellPosition::TopMiddle,
        state: CellState::Open,
      },
      Cell {
        id: "tr".to_string(),
        pos: CellPosition::TopRight,
        state: CellState::Open,
      },
      Cell {
        id: "cl".to_string(),
        pos: CellPosition::CenterLeft,
        state: CellState::Open,
      },
      Cell {
        id: "cm".to_string(),
        pos: CellPosition::CenterMiddle,
        state: CellState::Open,
      },
      Cell {
        id: "cr".to_string(),
        pos: CellPosition::CenterRight,
        state: CellState::Open,
      },
      Cell {
        id: "bl".to_string(),
        pos: CellPosition::BottomLeft,
        state: CellState::Open,
      },
      Cell {
        id: "bm".to_string(),
        pos: CellPosition::BottomMiddle,
        state: CellState::Open,
      },
      Cell {
        id: "br".to_string(),
        pos: CellPosition::BottomRight,
        state: CellState::Open,
      },
    ];
    Self {
      initialized: false,
      grid,
    }
  }
}

impl framework::Game for Game {
  fn update(&mut self, ctx: &mut framework::Context) {
    if self.initialized {
      for ev in ctx.events() {
        match ev {
          Event::Gui(gui::Event::ButtonClicked { id }) => {
            self
              .grid
              .iter_mut()
              .find(|c| c.id == id)
              .as_mut()
              .unwrap()
              .state = CellState::Player1;
            let btn = ctx.gui.get_mut_button(&id);
            btn.color = crate::math::Color::new(0.3, 0.8, 0.3, 1.0);

            if let Some(winner) = check_winner(&self.grid) {
              let winner_btn_id = String::from("winner");
              let btn = ctx.gui.get_mut_button(&winner_btn_id);
              btn.color = match winner {
                CellState::Open => crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
                CellState::Player1 => crate::math::Color::new(0.3, 0.8, 0.3, 1.0),
                CellState::Player2 => crate::math::Color::new(0.8, 0.3, 0.3, 1.0),
              }
            } else {
              let id = self
                .grid
                .iter().find(|c| c.state == CellState::Open)
                .unwrap()
                .id
                .clone();
              self
                .grid
                .iter_mut()
                .find(|c| c.id == id)
                .as_mut()
                .unwrap()
                .state = CellState::Player2;
              let btn = ctx.gui.get_mut_button(&id);
              btn.color = crate::math::Color::new(0.8, 0.3, 0.3, 1.0);
              if let Some(winner) = check_winner(&self.grid) {
                let winner_btn_id = String::from("winner");
                let btn = ctx.gui.get_mut_button(&winner_btn_id);
                btn.color = match winner {
                  CellState::Open => crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
                  CellState::Player1 => crate::math::Color::new(0.3, 0.8, 0.3, 1.0),
                  CellState::Player2 => crate::math::Color::new(0.8, 0.3, 0.3, 1.0),
                }
              }
            }
          }
        }
      }

      return;
    }

    let btn = framework::gui::Button::new(
      crate::math::Rect::new(-0.1, -0.1, 0.2, 0.2),
      crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
    );
    ctx.gui.add_button(String::from("cm"), btn);

    let btn = framework::gui::Button::new(
      crate::math::Rect::new(-0.1, 0.15, 0.2, 0.2),
      crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
    );
    ctx.gui.add_button(String::from("tm"), btn);

    let btn = framework::gui::Button::new(
      crate::math::Rect::new(-0.1, -0.35, 0.2, 0.2),
      crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
    );
    ctx.gui.add_button(String::from("bm"), btn);

    let btn = framework::gui::Button::new(
      crate::math::Rect::new(0.15, -0.1, 0.2, 0.2),
      crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
    );
    ctx.gui.add_button(String::from("cr"), btn);

    let btn = framework::gui::Button::new(
      crate::math::Rect::new(0.15, 0.15, 0.2, 0.2),
      crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
    );
    ctx.gui.add_button(String::from("tr"), btn);

    let btn = framework::gui::Button::new(
      crate::math::Rect::new(0.15, -0.35, 0.2, 0.2),
      crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
    );
    ctx.gui.add_button(String::from("br"), btn);

    let btn = framework::gui::Button::new(
      crate::math::Rect::new(-0.35, -0.1, 0.2, 0.2),
      crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
    );
    ctx.gui.add_button(String::from("cl"), btn);

    let btn = framework::gui::Button::new(
      crate::math::Rect::new(-0.35, 0.15, 0.2, 0.2),
      crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
    );
    ctx.gui.add_button(String::from("tl"), btn);

    let btn = framework::gui::Button::new(
      crate::math::Rect::new(-0.35, -0.35, 0.2, 0.2),
      crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
    );
    ctx.gui.add_button(String::from("bl"), btn);

    let btn = framework::gui::Button::new(
      crate::math::Rect::new(-0.4, -0.7, 0.8, 0.15),
      crate::math::Color::new(0.5, 0.5, 0.5, 1.0),
    );
    ctx.gui.add_button(String::from("winner"), btn);

    self.initialized = true;
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

#[derive(Eq, PartialEq, Debug)]
enum CellState {
  Open,
  Player1,
  Player2,
}

fn check_winner(grid: &Vec<Cell>) -> Option<CellState> {
  // Helper to get cell state by CellPosition
  let get =
    |pos: CellPosition| -> &CellState { &grid.iter().find(|c| c.pos == pos).unwrap().state };

  let lines: [[CellPosition; 3]; 8] = [
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

  for line in lines.iter() {
    let a = get(line[0]);
    let b = get(line[1]);
    let c = get(line[2]);

    if a == b && b == c {
      match a {
        CellState::Player1 => return Some(CellState::Player1),
        CellState::Player2 => return Some(CellState::Player2),
        CellState::Open => {}
      }
    }
  }

  None
}
