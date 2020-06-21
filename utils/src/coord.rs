pub struct Coord {
  pub x: i32,
  pub y: i32,
}

impl Coord {
  pub fn new(x: i32, y: i32) -> Self {
    Coord { x, y }
  }

  pub fn distance_from(&self, other: &Coord) -> f32 {
    let y_diff = self.y - other.y;
    let x_diff = self.x - other.x;
    let square_sum = (y_diff * y_diff + x_diff * x_diff) as f32;
    square_sum.sqrt()
  }
}

#[derive(Clone, Copy)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

impl Vector2 {
  pub fn new(x: f32, y: f32) -> Self {
    Vector2 { x, y }
  }

  pub fn distance_from(&self, other: &Vector2) -> f32 {
    let y_diff = self.y - other.y;
    let x_diff = self.x - other.x;
    let square_sum = (y_diff * y_diff + x_diff * x_diff) as f32;
    square_sum.sqrt()
  }

  pub fn normalize(&mut self) {
    let distance = (self.x * self.x + self.y * self.y).sqrt();
    self.x /= distance;
    self.y /= distance;
  }

  pub fn is_equal(&self, rhs: &Vector2) -> bool {
    if (self.x - rhs.x).abs() < 0.001 && (self.y - rhs.y).abs() < 0.001 {
      return true;
    }

    false
  }
}

impl PartialEq for Vector2 {
  fn eq(&self, other: &Self) -> bool {
    if (self.x - other.x).abs() < 0.001 && (self.y - other.y).abs() < 0.001 {
      return true;
    }

    false
  }
}
