use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{
        prelude::World,
        prelude::*,
        prelude::{Entities, LazyUpdate, Read, System, Write},
        world::Builder,
    },
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use std::ops::DerefMut;

use crate::load_image;
use crate::minion;

const _SPAWN_POINT_X: f32 = 325.;
const _SPAWN_POINT_Y: f32 = 600.;

#[derive(SystemDesc)]
pub struct MinionSpawnSystem {
    pub sprite_render: SpriteRender,
    pub counter: u32,
}

impl MinionSpawnSystem {
    pub fn new(sprite_render: SpriteRender) -> MinionSpawnSystem {
        MinionSpawnSystem {
            sprite_render,
            counter: 0,
        }
    }
}

impl<'a> System<'a> for MinionSpawnSystem {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>, Read<'a, AssetStorage<Texture>>, Read<'a, AssetStorage<SpriteSheet>>);

    fn run(&mut self, (_entities, _updater, _asset_storage, _sprite_sheet): Self::SystemData) {
        self.counter += 1;

        print!("{0}\n", self.counter);
        if self.counter == 50 {
            print!("Spawn\n");
            // let mut transform = Transform::default();
            // transform.set_translation_xyz(SPAWN_POINT_X, SPAWN_POINT_Y, 1.);

            //let mut storage = world.read_resource::<AssetStorage<Texture>>();
            //TODO: Attach sprite
            // let sprite_render = load_sprites(world.deref_mut());
            // updater.create_entity(&entities).with(sprite_render.clone()).with(transform).with(minion::TestMinion{}).build();

            self.counter = 0;
        }
    }
}

pub struct MinionSpawnSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, MinionSpawnSystem> for MinionSpawnSystemDesc {
    fn build(self, world: &mut World) -> MinionSpawnSystem {
        <MinionSpawnSystem as System<'_>>::SystemData::setup(world);

        let mut sprite_renders = load_image::load_sprites(world, "sprites/minion", 1);

        MinionSpawnSystem::new(sprite_renders.pop().unwrap())
    }
}
