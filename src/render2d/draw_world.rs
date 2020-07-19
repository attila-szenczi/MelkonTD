use sfml::graphics::Color;
use sfml::graphics::{
  blend_mode::BlendMode, RenderStates, RenderTarget, RenderWindow, Sprite, Texture, Transformable,
};
use sfml::system::Vector2f;

use std::ops::Deref;

use crate::texture_storage::TextureData;
use crate::world::World;

pub fn draw_world(window: &mut RenderWindow, world: &mut World) {
  window.clear(Color::rgb(50, 200, 50));

  draw_background(window, world);
  draw_minions(window, world);
  draw_towers(window, world);
  //draw_healthbars(window, world);
  draw_projectiles(window, world);
  //draw_ui_elements(window, world);
  window.set_active(true);
  window.display();
}

fn draw_sprite(window: &mut RenderWindow, texture_data: &TextureData, position: &Vector2f) {
  let mut sprite = Sprite::with_texture(&texture_data.texture);
  sprite.set_scale(Vector2f::from((texture_data.scale, texture_data.scale)));
  sprite.set_position((position.x.floor(), position.y.floor()));
  sprite.set_origin((texture_data.origin.x.floor(), texture_data.origin.y.floor()));

  window.draw(&sprite);
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

fn draw_minions(window: &mut RenderWindow, world: &mut World) {
  for minion in &world.minions {
    draw_sprite(
      window,
      world
        .texture_storage
        .get_texture_data(minion.sprite_sheet_name()),
      minion.position(),
    );
  }
}

fn draw_projectiles(window: &mut RenderWindow, world: &mut World) {
  for projectile in &world.projectiles {
    draw_sprite(
      window,
      world
        .texture_storage
        .get_texture_data(projectile.sprite_sheet_name()),
      projectile.position(),
    );
  }
}
