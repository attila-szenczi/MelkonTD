use amethyst::{
  core::bundle::SystemBundle,
  core::transform::Transform,
  core::transform::TransformBundle,
  core::ArcThreadPool,
  ecs::prelude::{Dispatcher, DispatcherBuilder, Entity},
  input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
  input::{InputBundle, InputEvent, StringBindings},
  prelude::*,
  renderer::{Camera, SpriteRender},
  shrev::EventChannel,
  window::ScreenDimensions,
};

use log::info;

use crate::{
  input_system::UserInputSystem,
  load_image::load_sprites,
  minion_death_system::MinionDeathSystem,
  minion_spawn_system::MinionSpawnSystem,
  minion_update_system::MinionUpdateSystem,
  projectile::ProjectileDeathSystem,
  projectile::ProjectileUpdateSystem,
  simple_animation_system::SimpleAnimationSystem,
  texture_lookup::TextureLookup,
  tile_map::{TileMap, TileType},
  tower_update_system::TowerUpdateSystem,
  z_layer::{z_layer_to_coordinate, ZLayer},
};

#[derive(Default)]
pub struct GameState<'a, 'b> {
  dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> GameState<'a, 'b> {
  fn create_dispatcher(&mut self, world: &mut World) {
    let mut dispatcher_builder = DispatcherBuilder::new();
    let input_bundle = InputBundle::<StringBindings>::new();
    input_bundle
      .build(world, &mut dispatcher_builder)
      .expect("Input boundle couldn't be added");
    dispatcher_builder = dispatcher_builder
      .with(
        create_minion_spawn_system(&world),
        "minion_spawn_system",
        &[],
      )
      .with(
        create_user_input_system(&world),
        "user_input_system",
        &["input_system"],
      )
      .with(TowerUpdateSystem, "tower_update_system", &["input_system"])
      .with(
        ProjectileUpdateSystem,
        "projectile_update_system",
        &["input_system"],
      )
      .with(
        MinionUpdateSystem,
        "minion_update_system",
        &["input_system", "projectile_update_system"],
      )
      .with(
        MinionDeathSystem,
        "minion_death_system",
        &["input_system", "projectile_update_system"],
      )
      .with(
        ProjectileDeathSystem,
        "projectile_death_system",
        &["input_system", "projectile_update_system"],
      )
      .with(
        SimpleAnimationSystem,
        "simple_animation_system",
        &["input_system"],
      );
    let transform_bundle = TransformBundle::new();
    transform_bundle
      .build(world, &mut dispatcher_builder)
      .expect("Transform boundle couldn't be added");

    let mut dispatcher = dispatcher_builder
      .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
      .build();
    dispatcher.setup(world);

    self.dispatcher = Some(dispatcher);
  }
}

impl<'a, 'b> SimpleState for GameState<'a, 'b> {
  // On start will run when this state is initialized. For more
  // state lifecycle hooks, see:
  // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    self.create_dispatcher(data.world);

    let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();

    init_camera(data.world, &dimensions);

    let sprites = load_sprites(data.world, "sprites/tiles", 3);
    init_sprites(data.world, &sprites, &dimensions);
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

    // Keep going
    Trans::None
  }

  fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
    if let Some(dispatcher) = self.dispatcher.as_mut() {
      dispatcher.dispatch(&data.world);
    }

    Trans::None
  }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
  // Center the camera in the middle of the screen, and let it cover
  // the entire screen
  let mut transform = Transform::default();
  transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

  world
    .create_entity()
    .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
    .with(transform)
    .build();
}

fn init_sprites(world: &mut World, sprites: &[SpriteRender], _dimensions: &ScreenDimensions) {
  //TODO: Workaround to pass borrow checker. Refactor once i am not a rust novice.
  let mut sprite_data: Vec<(i32, f32, f32)> = Vec::new();
  {
    let map = world.read_resource::<TileMap>();

    let rows = map.rows;
    let columns = map.columns;
    for i in 0..rows {
      for j in 0..columns {
        let index = i * columns + j;
        let sprite_type = map.tiles[index as usize];
        let pos_x = map.map_rect.bottom_left.x + 50 * j + 25;
        let pos_y = map.map_rect.bottom_left.y + 50 * i + 25;

        sprite_data.push((
          tile_type_to_sprite_index(sprite_type),
          pos_x as f32,
          pos_y as f32,
        ));
      }
    }
  }

  let mut tile_entities: Vec<Entity> = Vec::new();

  for (sprite_index, pos_x, pos_y) in sprite_data {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
      pos_x as f32,
      pos_y as f32,
      z_layer_to_coordinate(ZLayer::TileMap),
    );

    tile_entities.push(
      world
        .create_entity()
        .with(sprites[sprite_index as usize].clone())
        .with(transform)
        .build(),
    );
  }
  let mut map = world.write_resource::<TileMap>();
  map.entities = tile_entities;
}

fn tile_type_to_sprite_index(tile_type: TileType) -> i32 {
  match tile_type {
    TileType::Unusable => 0,
    TileType::Slot => 1,
    TileType::Road => 2,
    _ => panic!("Invalid tile type at this point"),
  }
}

fn create_minion_spawn_system(world: &World) -> MinionSpawnSystem {
  let texture_lookup = world.read_resource::<TextureLookup>();

  MinionSpawnSystem::new(
    texture_lookup.get_texture("sprites/minion", 0),
    texture_lookup.get_texture("sprites/healthbar_back", 0),
    texture_lookup.get_texture("sprites/healthbar_front", 0),
    texture_lookup.get_texture("sprites/healthbar_outline", 0),
  )
}

fn create_user_input_system(world: &World) -> UserInputSystem {
  let reader_id = world
    .fetch_mut::<EventChannel<InputEvent<StringBindings>>>()
    .register_reader();
  UserInputSystem::new(reader_id)
}
