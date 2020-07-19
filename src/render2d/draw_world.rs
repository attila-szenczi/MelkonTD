use sfml::graphics::{
  blend_mode::BlendMode, RenderStates, RenderTarget, RenderWindow, Sprite, Texture, Transformable,
};
use sfml::system::Vector2f;

use std::ops::Deref;

use crate::texture_storage::TextureData;
use crate::world::World;

//Rendering order:
// pub enum ZLayer {
//   TileMap = 0, //Currently invisible, will be completely removed
//   Background,
//   Minion,
//   Tower,
//   HealthBarBack,
//   HealthBarMiddle,
//   HealthBarFront,
//   Projectile,
//   UiFlyout,
// }

pub fn draw_world(window: &mut RenderWindow, world: &mut World) {
  window.set_active(true);

  draw_background(window, world);
  //draw_minions(window, world);
  draw_towers(window, world);
  //draw_healthbars(window, world);
  //draw_projectiles(window, world);
  //draw_ui_elements(window, world);
  window.display();
}

fn draw_background(window: &mut RenderWindow, world: &mut World) {
  let texture = &world.texture_storage.get_texture_data("background").texture;
  let sprite = Sprite::with_texture(texture.deref());

  window.draw(&sprite);
}

fn draw_towers(window: &mut RenderWindow, world: &mut World) {
  for tower in &world.towers {
    draw_sprite(
      window,
      world
        .texture_storage
        .get_texture_data(tower.sprite_sheet_name()),
      tower.position(),
    );
  }
}

fn draw_sprite(window: &mut RenderWindow, texture_data: &TextureData, position: &Vector2f) {
  let mut sprite = Sprite::with_texture(&texture_data.texture);
  sprite.set_scale(Vector2f::from((texture_data.scale, texture_data.scale)));
  sprite.set_position((position.x, position.y));
  sprite.set_origin((texture_data.origin.x, texture_data.origin.y));

  window.draw(&sprite);
}
