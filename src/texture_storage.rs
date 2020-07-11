use std::collections::HashMap;
use std::path::PathBuf;

use sfml::graphics::Texture;
use sfml::system::SfBox;

// #[derive(Clone)]
// pub struct SpriteRenderWithDefaultScale {
//   pub sprite_render: SpriteRender,
//   pub default_scale: Vector3<f32>,
// }

pub struct TextureData {
  pub texture: SfBox<Texture>, //TODO sprites?
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
    self
      .textures
      .insert(String::from(key), TextureData { texture: texture });
  }

  pub fn insert(
    &mut self,
    //world: &mut World,
    filepath: &str,
    // sprite_count: i32,
    // width: i32,
    // height: i32,
    // default_in_game_width: i32,
    // default_in_game_height: i32,
    // z_coordinate: f32,
  ) {
    let mut path = self.working_dir.clone();
    path.push(filepath);
    let texture = Texture::from_file(path.to_str().unwrap()).unwrap();
    // let sprite_renders = load_sprites(world, filepath_without_extension, sprite_count);

    // let default_scale = Vector3::new(
    //   default_in_game_width as f32 / width as f32,
    //   default_in_game_height as f32 / height as f32,
    //   z_coordinate,
    // );
    self.textures.insert(
      String::from(filepath),
      TextureData {
        texture: texture,
        // sprite_count,
        // width,
        // height,
        //default_scale,
      },
    );
    // );
  }

  pub fn get_texture_data(&self, key: &str) -> &TextureData {
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
