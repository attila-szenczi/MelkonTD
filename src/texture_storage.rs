use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;

use sfml::graphics::{IntRect, Sprite, Texture};
use sfml::system::{SfBox, Vector2f, Vector2u};

// #[derive(Clone)]
// pub struct SpriteRenderWithDefaultScale {
//   pub sprite_render: SpriteRender,
//   pub default_scale: Vector3<f32>,
// }

pub struct TextureData {
  pub texture: SfBox<Texture>,
  pub sprite_rects: Vec<IntRect>, //Only filled in case there are multiple sprites
  pub scale: f32,
  pub origin: Vector2f,
}

pub struct TextureStorage {
  textures: HashMap<String, TextureData>,
  working_dir: PathBuf,
}

impl TextureStorage {
  pub fn new() -> Self {
    let mut working_dir = std::env::current_dir().expect("Working directory not found");
    working_dir.push("resources");
    TextureStorage {
      textures: HashMap::new(),
      working_dir,
    }
  }

  pub fn insert_with_key(&mut self, key: &str, filepath: &str) {
    let mut path = self.working_dir.clone();
    path.push(filepath);
    println!("Path: {}", path.as_path().to_str().unwrap());
    let texture = Texture::from_file(path.to_str().unwrap()).unwrap();

    //TODO: Pass origin optionally
    let sprite_width = texture.size().x as i32;
    let sprite_height = texture.size().y as i32;
    let origin = Vector2f::new((sprite_width / 2) as f32, (sprite_height / 2) as f32);

    self.textures.insert(
      String::from(key),
      TextureData {
        texture: texture,
        sprite_rects: vec![],
        scale: 1., //TODO: If it will be used outside of background it might not be good
        origin,
      },
    );
  }

  pub fn insert(
    &mut self,
    filepath: &str,
    default_width: i32, //Unified scale is expected
  ) {
    let mut path = self.working_dir.clone();
    path.push(filepath);
    let mut texture = Texture::from_file(path.to_str().unwrap()).unwrap();
    texture.set_smooth(true);

    let sprite_width = texture.size().x as i32;
    let sprite_height = texture.size().y as i32;
    let origin = Vector2f::new((sprite_width / 2) as f32, (sprite_height / 2) as f32);

    //TODO: Create sprites
    let scale = default_width as f32 / texture.size().x as f32;
    self.textures.insert(
      String::from(filepath),
      TextureData {
        texture: texture,
        sprite_rects: vec![],
        scale,
        origin,
      },
    );
  }

  pub fn insert_sprite_sheet(
    &mut self,
    filepath: &str,
    default_in_game_sprite_width: i32,
    columns: i32,
    rows: i32,
  ) {
    let mut path = self.working_dir.clone();
    path.push(filepath);
    let texture = Texture::from_file(path.to_str().unwrap()).unwrap();

    let sprite_rects = create_sprite_rects(columns, rows, texture.size());
    let scale = default_in_game_sprite_width as f32 / (texture.size().x as f32 / columns as f32);

    //TODO: Pass origin optionally
    let sprite_width = texture.size().x as i32 / columns;
    let sprite_height = texture.size().y as i32 / rows;
    let origin = Vector2f::new((sprite_width / 2) as f32, (sprite_height / 2) as f32);

    self.textures.insert(
      String::from(filepath),
      TextureData {
        texture: texture,
        sprite_rects,
        scale,
        origin,
      },
    );
  }

  pub fn get_texture_data(&self, key: &str) -> &TextureData {
    //println!("Lookup texture {}", key);
    self.textures.get(&String::from(key)).unwrap()
  }

  // pub fn get_texture(&self, key: &str, index: i32) -> SpriteRender {
  //   self.lookup.get(&String::from(key)).unwrap().sprite_renders[index as usize].clone()
  // }

  // pub fn get_texture_with_default_scale(&self, key: &str) -> SpriteRenderWithDefaultScale {
  //   let texture_data = self.lookup.get(&String::from(key)).unwrap();
  //   let sprite_render = texture_data.sprite_renders[0].clone();
  //   SpriteRenderWithDefaultScale {
  //     sprite_render,
  //     default_scale: texture_data.default_scale,
  //   }
  // }

  // pub fn get_texture_with_scale(&self, key: &str, dimension: i32) -> (SpriteRender, Vector3<f32>) {
  //   let texture_data = self.lookup.get(&String::from(key)).unwrap();
  //   let sprite_render = texture_data.sprite_renders[0].clone();
  //   let vec = Vector3::new(
  //     1. * (dimension as f32 / texture_data.width as f32),
  //     1. * (dimension as f32 / texture_data.height as f32),
  //     1.,
  //   );
  //   (sprite_render, vec)
  // }
}

fn create_sprite_rects(columns: i32, rows: i32, image_dimensions: Vector2u) -> Vec<IntRect> {
  let sprite_width = image_dimensions.x as i32 / columns;
  let sprite_height = image_dimensions.y as i32 / rows;
  let mut sprite_rects = Vec::new();
  sprite_rects.reserve((columns * rows) as usize);

  for row in 0..rows {
    for column in 0..columns {
      let start_x = sprite_width as i32 * column;
      let start_y = sprite_height as i32 * row;
      sprite_rects.push(IntRect::new(start_x, start_y, sprite_width, sprite_height));
    }
  }

  sprite_rects
}
