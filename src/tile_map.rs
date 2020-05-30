use amethyst::ecs::prelude::Entity;

use utils::coord::Coord;
use utils::rect::Rect;

//TODO: Tiles: enum
pub struct TileMap {
    pub tiles: Vec<i32>,
    pub rows: i32,
    pub columns: i32,
    pub map_rect : Rect,
    pub tile_width: i32,
    pub tile_height: i32,
    pub entities: Vec<Entity>,
}

impl TileMap {
    pub fn new(
        tiles : Vec<i32>,
        top_left: Coord,
        rows: i32,
        columns: i32,
        tile_width: i32,
        tile_height: i32,
    ) -> Self {

        TileMap {
            tiles,
            rows,
            columns,
            map_rect : Rect::new(top_left, columns * tile_width, rows * tile_height),
            tile_width,
            tile_height,
            entities: Vec::new(),
        }
    }

    pub fn find_tile(self, position_x: i32, position_y: i32) -> Option<Entity> {
        if position_x < self.map_rect.top_left.x
            || position_x > self.map_rect.top_left.x + self.map_rect.width
            || position_y < self.map_rect.top_left.y
            || position_y > self.map_rect.top_left.y + self.map_rect.height
        {
            None
        } else {
            let offset_x = position_x - self.map_rect.top_left.x;
            let offset_y = position_y - self.map_rect.top_left.y;
            let column = offset_x / self.tile_width;
            let row = offset_y / self.tile_height;
            let index = self.columns * row + column;
            Some(*self.entities.get(index as usize).unwrap())
        }
    }
}
