use amethyst::ecs::prelude::Entity;

use utils::coord::Coord;
use utils::rect::Rect;

#[derive(Copy, Clone)]
pub enum TileType {
  Unusable,
  Slot,
  Road,
  Tower(Entity),
}

pub struct TileMap {
  pub tiles: Vec<TileType>,
  pub rows: i32,
  pub columns: i32,
  pub map_rect: Rect,
  pub tile_width: i32,
  pub tile_height: i32,
  pub entities: Vec<Entity>,
}

impl TileMap {
  pub fn new(
    tiles: Vec<i32>,
    bottom_left: Coord,
    rows: i32,
    columns: i32,
    tile_width: i32,
    tile_height: i32,
  ) -> Self {
    let convert = |i: &i32| -> TileType {
      match i {
        0 => TileType::Unusable,
        1 => TileType::Slot,
        2 => TileType::Road,
        _ => TileType::Unusable,
      }
    };
    let tiles_with_type: Vec<TileType> = tiles.iter().map(convert).collect();

    TileMap {
      tiles: tiles_with_type,
      rows,
      columns,
      map_rect: Rect::new(bottom_left, columns * tile_width, rows * tile_height),
      tile_width,
      tile_height,
      entities: Vec::new(),
    }
  }

  pub fn occupy_slot(&mut self, index: i32, entity: Entity) {
    match self.tiles.get(index as usize).unwrap() {
      TileType::Slot => self.tiles[index as usize] = TileType::Tower(entity),
      _ => panic!("Selected tile is not a slot"),
    }
  }

  //TODO: Rethink this struct
  pub fn find_tile(&self, position_x: i32, position_y: i32) -> Option<(i32, TileType, Rect)> {
    if position_x < self.map_rect.bottom_left.x
      || position_x > self.map_rect.bottom_left.x + self.map_rect.width
      || position_y < self.map_rect.bottom_left.y
      || position_y > self.map_rect.bottom_left.y + self.map_rect.height
    {
      None
    } else {
      println!(
        "rect {:?} {:?} mouse pos {:?} {:?}",
        self.map_rect.bottom_left.x, self.map_rect.bottom_left.y, position_x, position_y
      );
      let offset_x = position_x - self.map_rect.bottom_left.x;
      let offset_y = position_y - self.map_rect.bottom_left.y;
      let column = offset_x / self.tile_width;
      let row = offset_y / self.tile_height;
      let index = self.columns * row + column;
      Some((
        index,
        *self.tiles.get(index as usize).unwrap(),
        Rect {
          bottom_left: Coord {
            x: self.map_rect.bottom_left.x + column * self.tile_width,
            y: self.map_rect.bottom_left.y + row * self.tile_height,
          },
          width: self.tile_width,
          height: self.tile_height,
        },
      ))
    }
  }

  // pub fn find_tile_entity(&self, position_x: i32, position_y: i32) -> Option<Entity> {
  //     if position_x < self.map_rect.bottom_left.x
  //         || position_x > self.map_rect.bottom_left.x + self.map_rect.width
  //         || position_y < self.map_rect.bottom_left.y
  //         || position_y > self.map_rect.bottom_left.y + self.map_rect.height
  //     {
  //         None
  //     } else {
  //         let offset_x = position_x - self.map_rect.bottom_left.x;
  //         let offset_y = position_y - self.map_rect.bottom_left.y;
  //         let column = offset_x / self.tile_width;
  //         let row = offset_y / self.tile_height;
  //         let index = self.columns * row + column;
  //         Some(*self.entities.get(index as usize).unwrap())
  //     }
  // }
}
