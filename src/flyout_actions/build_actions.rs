use amethyst::ecs::prelude::World;

use super::flyout_action_storage::FlyoutAction;
use crate::texture_lookup::TextureLookup;
use utils::rect::Rect;

pub fn build_locked_action<'a>(texture_lookup: &TextureLookup) -> FlyoutAction {
  let action = |_world: &mut World, _tile_index: i32, _tile_rect: Rect| -> () {};

  let icon = texture_lookup.get_texture_with_default_scale("sprites/locked_icon");
  FlyoutAction::new(icon, action)
}

pub fn build_electric_mage_tower_action<'a>(texture_lookup: &TextureLookup) -> FlyoutAction {
  let action = |_world: &mut World, _tile_index: i32, _tile_rect: Rect| -> () {
    // let mut transform = Transform::default();
    // transform.set_translation_xyz(
    //   tile_rect.left() as f32 + 25.,
    //   tile_rect.bottom() as f32 + 25.,
    //   z_layer_to_coordinate(ZLayer::Tower),
    // );

    // let texture = {
    //   let inner_texture_lookup = world.read_resource::<TextureLookup>();
    //   inner_texture_lookup.get_texture("private_sprites/electric_tower", 0)
    // };

    // let projectile_sprite_render_with_scale = {
    //   let inner_texture_lookup = world.read_resource::<TextureLookup>();
    //   inner_texture_lookup.get_texture_with_default_scale("private_sprites/pulsing_electric_ball")
    // };

    // let entity = {
    //   world
    //     .create_entity()
    //     .with(texture)
    //     .with(transform)
    //     .with(Tower::new(
    //       Box::new(ElectricMageTower::new(
    //         projectile_sprite_render_with_scale.sprite_render,
    //         projectile_sprite_render_with_scale.default_scale,
    //       )),
    //       TowerType::ElectricMageTower,
    //     ))
    //     .build()
    // };

    // let mut tile_map = world.write_resource::<TileMap>();
    // tile_map.occupy_slot(tile_index, entity);
  };

  let icon = texture_lookup.get_texture_with_default_scale("sprites/electric_mage_tower_icon");
  FlyoutAction::new(icon, action)
}
