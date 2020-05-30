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
}
