#[derive(Copy, Clone)]
pub struct Coord {
  pub x: i32,
  pub y: i32,
}

//TODO: Change everything to f32 or make it a generic
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

  pub fn new_unit_vector_up() -> Self {
    Vector2 { x: 0., y: 1. }
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

  pub fn rotate_ccw(&mut self, angle_in_degree: f32) {
    let theta = angle_in_degree.to_radians();

    let cs = f32::cos(theta);
    let sn = f32::sin(theta);

    let px = self.x * cs - self.y * sn;
    self.y = self.x * sn + self.y * cs;
    self.x = px;
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
