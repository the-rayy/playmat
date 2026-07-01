#[derive(Default)]
pub struct Input {
  cursor_pos: CursorPos
}

impl Input {
  pub fn set_cursor_pos(&mut self, x: i32, y: i32) {
    self.cursor_pos.x = x;
    self.cursor_pos.y = y;
  }
}

#[derive(Default, Debug)]
pub struct CursorPos { pub x: i32, pub y: i32 }
