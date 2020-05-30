use amethyst::{
    core::transform::Transform,
    ecs::prelude::Entity,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, SpriteRender},
    window::ScreenDimensions,
};

use log::info;

use crate::{
    load_image::load_sprites,
    tile_map::TileMap,
    z_layer::{z_layer_to_coordinate, ZLayer},
};

pub struct GameState;

impl SimpleState for GameState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let sprites = load_sprites(world, "sprites/tiles", 2);
        init_sprites(world, &sprites, &dimensions);
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
                let sprite_index = map.tiles[index as usize];
                let pos_x = map.map_rect.top_left.x + 50 * j + 25;
                let pos_y = map.map_rect.top_left.y + 50 * i + 25;

                sprite_data.push((sprite_index, pos_x as f32, pos_y as f32));
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
