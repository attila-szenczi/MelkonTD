use amethyst::ecs::prelude::Entity;

pub struct TileMap {
    pub tiles: Vec<u32>,
    pub rows: u32,
    pub columns: u32,
    pub entities: Vec<Entity>,
}