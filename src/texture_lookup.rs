use amethyst::ecs::World;
use amethyst::renderer::SpriteRender;

use std::collections::HashMap;

use crate::load_image::load_sprites;

pub struct TextureData {
    sprite_renders: Vec<SpriteRender>,
    // sprite_count: i32,
    // width: i32,
    // height: i32,
}

#[derive(Default)]
pub struct TextureLookup {
    lookup: HashMap<String, TextureData>,
}

impl TextureLookup {
    pub fn insert(
        &mut self,
        world: &mut World,
        filepath_without_extension: &str,
        sprite_count: i32,
        _width: i32,
        _height: i32,
    ) {
        let sprite_renders = load_sprites(world, filepath_without_extension, sprite_count);
        self.lookup.insert(
            String::from(filepath_without_extension),
            TextureData {
                sprite_renders,
                // sprite_count,
                // width,
                // height,
            },
        );
    }

    // pub fn get_texture_data(&self, key: &str) -> Option<&TextureData> {
    //     self.lookup.get(&String::from(key))
    // }

    pub fn get_texture(&self, key: &str, index: i32) -> SpriteRender {
        self.lookup.get(&String::from(key)).unwrap().sprite_renders[index as usize].clone()
    }
}
