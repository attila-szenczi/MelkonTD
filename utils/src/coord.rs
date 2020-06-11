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