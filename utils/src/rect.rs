use crate::coord::Coord;

pub struct Rect {
  pub bottom_left: Coord,
  pub width: i32,
  pub height: i32,
}

impl Rect {
  pub fn new(bottom_left: Coord, width: i32, height: i32) -> Self {
    Rect {
      bottom_left,
      width,
      height,
    }
  }

  pub fn left(&self) -> i32 {
    self.bottom_left.x
  }

  pub fn right(&self) -> i32 {
    self.bottom_left.x + self.width
  }

  pub fn bottom(&self) -> i32 {
    self.bottom_left.y
  }
  
  pub fn top(&self) -> i32 {
    self.bottom_left.y + self.height
  }


  //TODO: make it templated
  pub fn is_in(&self, x: f32, y: f32) -> bool {
    x as i32 > self.left()
      && (x as i32) < self.right()
      && y as i32 > self.bottom()
      && (y as i32) < self.top()
  }
}
