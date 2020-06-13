use amethyst::shrev::EventChannel;
use amethyst::{
    assets::AssetStorage,
    core::math::{Point3},
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    window::ScreenDimensions,
    ecs::{
        prelude::World,
        prelude::*,
        prelude::{Entities, LazyUpdate, Read, System},
    },
    input::{InputEvent, InputHandler, StringBindings},
    renderer::{Camera, SpriteRender, SpriteSheet, Texture},
    winit::MouseButton,
};

use crate::load_image::load_sprites;
use crate::tile_map::{TileMap, TileType};
use crate::z_layer::{z_layer_to_coordinate, ZLayer};
use crate::tower::Tower;

type EventType = InputEvent<StringBindings>;

#[derive(SystemDesc)]
pub struct UserInputSystem {
    reader_id: ReaderId<EventType>,
    tower_sprite_render: SpriteRender,
    projectile_sprite_render: SpriteRender,
}

impl UserInputSystem {
    pub fn new(reader_id: ReaderId<EventType>, tower_sprite_render: SpriteRender, projectile_sprite_render: SpriteRender) -> Self {
        UserInputSystem {
            reader_id,
            tower_sprite_render,
            projectile_sprite_render
        }
    }
}

impl<'a> System<'a> for UserInputSystem {
    type SystemData = (
        Read<'a, EventChannel<EventType>>,
        Read<'a, InputHandler<StringBindings>>,
        WriteExpect<'a, TileMap>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, AssetStorage<Texture>>,
        Read<'a, AssetStorage<SpriteSheet>>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
        ReadExpect<'a, ScreenDimensions>,
    );

    fn run(
        &mut self,
        (
            channel,
            input_handler,
            mut tile_map,
            entities,
            updater,
            _texture_storage,
            _sprite_sheet_storage,
            camera_storage,
            transform_storage,
            screen_dimensions,
        ): Self::SystemData,
    ) {
        for event in channel.read(&mut self.reader_id) {
            match event {
                EventType::MouseButtonPressed(MouseButton::Left) => {
                    if let Some((x, y)) = input_handler.mouse_position() {

                        if let Some((camera, transform)) = (&camera_storage, &transform_storage).join().next() {
                            let center_screen = Point3::new(x, y, 0.0);

                            let world_point = camera.projection().screen_to_world_point(
                                center_screen,
                                screen_dimensions.diagonal(),
                                &transform,
                            );

                            match tile_map.find_tile(world_point.x as i32, world_point.y as i32) {
                                Some((index, TileType::Slot, rect)) => {
                                    let mut transform = Transform::default();
                                    transform.set_translation_xyz(
                                        rect.bottom_left.x as f32,
                                        rect.bottom_left.y as f32,
                                        z_layer_to_coordinate(ZLayer::Minion),
                                    );

                                    println!("Spawn tower\n");
                                    let entity = updater
                                        .create_entity(&entities)
                                        .with(self.tower_sprite_render.clone())
                                        .with(transform)
                                        .with(Tower::new(self.projectile_sprite_render.clone()))
                                        .build();
                                    tile_map.occupy_slot(index, entity);
                                }
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

pub struct UserInputSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, UserInputSystem> for UserInputSystemDesc {
    fn build(self, world: &mut World) -> UserInputSystem {
        <UserInputSystem as System<'_>>::SystemData::setup(world);

        let reader_id = world
            .fetch_mut::<EventChannel<EventType>>()
            .register_reader();
        let mut tower_sprite_renders = load_sprites(world, "sprites/tower", 1);
        let mut projectile_sprite_renders = load_sprites(world, "sprites/projectile", 1);
        UserInputSystem::new(reader_id, tower_sprite_renders.pop().unwrap(), projectile_sprite_renders.pop().unwrap())
    }
}
