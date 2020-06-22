use amethyst::core::math::Vector3;
use amethyst::ecs::World;
use amethyst::renderer::SpriteRender;

use std::collections::HashMap;

use crate::load_image::load_sprites;
use crate::texture_lookup::SpriteRenderWithDefaultScale;
use crate::tile_map::TileType;
use crate::tower::TowerType;
use utils::rect::Rect;

pub type FlyoutFuncType = Box<dyn Fn(&mut World, i32, &Rect) + Send + Sync>;

#[derive(PartialEq, Eq, Hash)]
pub enum EntityType {
  Tile(TileType),
  //Tower(TowerType),
}

pub struct FlyoutAction {
  pub icon: SpriteRenderWithDefaultScale,
  //clicked_tile_index: i32, clicked_tile_rect: Rect
  pub action: FlyoutFuncType,
}

impl FlyoutAction {
  pub fn new(icon: SpriteRenderWithDefaultScale, action: FlyoutFuncType) -> Self {
    FlyoutAction { icon, action }
  }
}

pub enum FlyoutOption {
  //Locked(SpriteRender), //Sign the user that there is an option which is not yet available.
  Action(FlyoutAction),
}

#[derive(Default)]
pub struct FlyoutActionStorage {
  actions: HashMap<EntityType, Vec<FlyoutOption>>,
}

impl FlyoutActionStorage {
  pub fn insert(&mut self, entity_type: EntityType, action: FlyoutOption) {
    let options = self.actions.entry(entity_type).or_default();
    options.push(action);
    assert!(
      options.len() <= 4,
      "Max 4 flyout option is supported, but a 5th was provided."
    );
  }

  //Clear before each map as each map has a distinct set of available towers and upgrades.
  //Unused as maps are not really implemented yet.
  #[allow(dead_code)]
  pub fn clear(&mut self) {
    self.actions.clear();
  }

  pub fn get_actions(&self, entity_type: EntityType) -> &Vec<FlyoutOption> {
    self.actions.get(&entity_type).unwrap()
  }
}
