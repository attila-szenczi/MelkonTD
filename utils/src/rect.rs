use crate::coord::Coord;

pub struct Rect {
    pub top_left: Coord,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(top_left: Coord, width: i32, height: i32) -> Self {
        Rect {
            top_left,
            width,
            height,
        }
    }
}
