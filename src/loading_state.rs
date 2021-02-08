use amethyst::{
  input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
  prelude::*,
};

use log::info;

use crate::flyout_actions::FlyoutActionStorage;
use crate::z_layer::{z_layer_to_coordinate, ZLayer};
use crate::{game_state::GameState, texture_lookup::TextureLookup, tile_map::TileMap};
use utils::coord::Coord;

pub struct LoadingState;

impl SimpleState for LoadingState {
  // On start will run when this state is initialized. For more
  // state lifecycle hooks, see:
  // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let tile_map = TileMap::new(
      vec![
        0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 1, 2, 1, 0, 0, 0, 2, 0, 0, 0, 1, 2, 1, 0, 0, 0, 2, 0, 0,
        0, 1, 2, 1, 0, 0, 0, 2, 0, 0, 0, 1, 2, 1, 0, 0, 0, 2, 0, 0,
      ],
      Coord::new(300, 50),
      10,
      5,
      50,
      50,
    );

    data.world.insert(tile_map);

    init_texture_lookup(data.world);
    fill_flyout_actions(data.world);
  }

  fn handle_event(
    &mut self,
    mut _data: StateData<'_, GameData<'_, '_>>,
    event: StateEvent,
  ) -> SimpleTrans {
    if let StateEvent::Window(event) = &event {
      // Check if the window should be closed
      if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
        return Trans::Quit;
      }

      // Listen to any key events
      if let Some(event) = get_key(&event) {
        info!("handling key event: {:?}", event);
      }

      // If you're looking for a more sophisticated event handling solution,
      // including key bindings and gamepad support, please have a look at
      // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
    }

    return Trans::Push(Box::new(GameState::default()));
  }
}

fn init_texture_lookup(world: &mut World) {
  let mut texture_lookup = TextureLookup::default();

  texture_lookup.insert(
    world,
    "private_sprites/5_enemies_1_attack_018",
    1,
    292,
    248,
    146,
    124,
    z_layer_to_coordinate(ZLayer::Minion),
  );
  texture_lookup.insert(
    world,
    "sprites/healthbar_back",
    1,
    32,
    32,
    32,
    32,
    z_layer_to_coordinate(ZLayer::HealthBarBack),
  );
  texture_lookup.insert(
    world,
    "sprites/healthbar_front",
    1,
    32,
    32,
    32,
    32,
    z_layer_to_coordinate(ZLayer::HealthBarFront),
  );
  texture_lookup.insert(
    world,
    "sprites/healthbar_outline",
    1,
    32,
    32,
    32,
    32,
    z_layer_to_coordinate(ZLayer::HealthBarBack),
  );
  texture_lookup.insert(
    world,
    "private_sprites/electric_tower",
    1,
    177,
    180,
    177,
    180,
    z_layer_to_coordinate(ZLayer::Tower),
  );
  texture_lookup.insert(
    world,
    "private_sprites/pulsing_electric_ball",
    8,
    512,
    512,
    64,
    64,
    z_layer_to_coordinate(ZLayer::Projectile),
  );
  texture_lookup.insert(
    world,
    "sprites/electric_mage_tower_icon",
    1,
    512,
    512,
    24,
    24,
    z_layer_to_coordinate(ZLayer::UiFlyout),
  );
  texture_lookup.insert(
    world,
    "sprites/locked_icon",
    1,
    64,
    64,
    24,
    24,
    z_layer_to_coordinate(ZLayer::UiFlyout),
  );
  texture_lookup.insert(
    world,
    "private_sprites/game_background_1",
    1,
    1920,
    1080,
    1920,
    1080,
    z_layer_to_coordinate(ZLayer::Background),
  );
  //texture_lookup.insert(data.world, "sprites/tiles", 3, 50, 50);
  world.insert(texture_lookup);
}

fn fill_flyout_actions(world: &mut World) {
  let action_storage = FlyoutActionStorage::default();
  {
    // let texture_lookup = world.read_resource::<TextureLookup>();
    // action_storage.insert(
    //   EntityType::Tile(TileType::Slot),
    //   build_electric_mage_tower_action(&texture_lookup),
    // );
    // action_storage.insert(
    //   EntityType::Tile(TileType::Slot),
    //   build_locked_action(&texture_lookup),
    // );
    // action_storage.insert(
    //   EntityType::Tile(TileType::Slot),
    //   build_locked_action(&texture_lookup),
    // );
    // action_storage.insert(
    //   EntityType::Tile(TileType::Slot),
    //   build_locked_action(&texture_lookup),
    // );
  }
  world.insert(action_storage);
}
