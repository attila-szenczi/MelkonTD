use amethyst::core::math::Vector3;
use amethyst::ecs::World;
use amethyst::renderer::SpriteRender;

use std::collections::HashMap;

use crate::load_image::load_sprites;

pub struct TextureData {
  sprite_renders: Vec<SpriteRender>,
  // sprite_count: i32,
  width: i32,
  height: i32,
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
    width: i32,
    height: i32,
  ) {
    let sprite_renders = load_sprites(world, filepath_without_extension, sprite_count);
    self.lookup.insert(
      String::from(filepath_without_extension),
      TextureData {
        sprite_renders,
        // sprite_count,
        width,
        height,
      },
    );
  }

  // pub fn get_texture_data(&self, key: &str) -> Option<&TextureData> {
  //     self.lookup.get(&String::from(key))
  // }

  pub fn get_texture(&self, key: &str, index: i32) -> SpriteRender {
    self.lookup.get(&String::from(key)).unwrap().sprite_renders[index as usize].clone()
  }

  pub fn get_texture_with_scale(&self, key: &str, dimension: i32) -> (SpriteRender, Vector3<f32>) {
    let texture_data = self.lookup.get(&String::from(key)).unwrap();
    let sprite_render = texture_data.sprite_renders[0].clone();
    let vec = Vector3::new(
      1. * (dimension as f32 / texture_data.width as f32),
      1. * (dimension as f32 / texture_data.height as f32),
      1.,
    );
    (sprite_render, vec)
  }
}
