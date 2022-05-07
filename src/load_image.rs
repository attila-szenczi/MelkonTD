// use amethyst::{
//   assets::{AssetStorage, Loader},
//   prelude::*,
//   renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
// };

// pub fn load_sprites(
//   world: &mut World,
//   filepath_without_extension: &str,
//   sprite_count: i32,
// ) -> Vec<SpriteRender> {
//   let texture_handle = {
//     let loader = world.read_resource::<Loader>();
//     let texture_storage = world.read_resource::<AssetStorage<Texture>>();

//     let image_path = format!("{}{}", filepath_without_extension, ".png");

//     loader.load(image_path, ImageFormat::default(), (), &texture_storage)
//   };

//   let sheet_handle = {
//     let loader = world.read_resource::<Loader>();
//     let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

//     let ron_path = format!("{}{}", filepath_without_extension, ".ron");

//     loader.load(
//       ron_path,
//       SpriteSheetFormat(texture_handle),
//       (),
//       &sheet_storage,
//     )
//   };

//   (0..sprite_count)
//     .map(|i| SpriteRender {
//       sprite_sheet: sheet_handle.clone(),
//       sprite_number: i as usize,
//     })
//     .collect()
// }
