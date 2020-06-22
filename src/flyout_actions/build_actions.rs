use amethyst::core::Transform;
use amethyst::ecs::prelude::{Entities, LazyUpdate, World};
use amethyst::ecs::Read;
use amethyst::prelude::*;
use amethyst::renderer::SpriteRender;

use super::flyout_action_storage::FlyoutAction;
use crate::texture_lookup::TextureLookup;
use crate::tile_map::TileMap;
use crate::tower::{ElectricMageTower, Tower, TowerType};
use crate::z_layer::{z_layer_to_coordinate, ZLayer};
use utils::rect::Rect;

pub fn build_electric_mage_tower_action<'a>(texture_lookup: &TextureLookup) -> FlyoutAction {
  let action = Box::new(
    |world: &mut World, tile_index: i32, tile_rect: &Rect| -> () {
      let mut transform = Transform::default();
      transform.set_translation_xyz(
        tile_rect.left() as f32 + 25.,
        tile_rect.bottom() as f32 + 25.,
        z_layer_to_coordinate(ZLayer::Tower),
      );
      let inner_texture_lookup = world.read_resource::<TextureLookup>();
      let projectile_sprite_render_with_scale = inner_texture_lookup
        .get_texture_with_default_scale("private_sprites/pulsing_electric_ball");
      let updater = world.read_resource::<LazyUpdate>();
      let entities = world.read_resource::<Entities>();
      let entity = updater
        .create_entity(&entities)
        .with(inner_texture_lookup.get_texture("sprites/tower", 0))
        .with(transform)
        .with(Tower::new(
          Box::new(ElectricMageTower::new(
            projectile_sprite_render_with_scale.sprite_render,
            projectile_sprite_render_with_scale.default_scale,
          )),
          TowerType::ElectricMageTower,
        ))
        .build();

      let mut tile_map = world.write_resource::<TileMap>();
      tile_map.occupy_slot(tile_index, entity);
    },
  );

  let icon = texture_lookup.get_texture_with_default_scale("sprites/electric_mage_tower_icon");
  FlyoutAction::new(icon, action)
}
