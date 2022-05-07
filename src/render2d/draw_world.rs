use sfml::graphics::{
  Color, RenderTarget, RenderWindow, Sprite, Transformable,
};
use sfml::system::Vector2f;

use std::ops::Deref;

use generational_arena::Arena;

use crate::flyout_actions::*;
use crate::shared_traits::DrawableTrait;
use crate::texture_storage::TextureData;
use crate::texture_storage::TextureStorage;
use crate::world::World;

pub fn draw_world(window: &mut RenderWindow, world: &mut World) {
  window.clear(Color::rgb(50, 200, 50));

  draw_background(window, world);
  draw_all(window, &world.texture_storage, &world.minions);
  draw_all(window, &world.texture_storage, &world.towers);
  draw_healthbars(window, world);
  draw_all(window, &world.texture_storage, &world.projectiles);
  draw_flyout_actions(window, &world.texture_storage, &world.active_flyout_actions);
  //draw_ui_elements(window, world);
  window.set_active(true);
  window.display();
}

fn draw_sprite(
  window: &mut RenderWindow,
  texture_data: &TextureData,
  position: &Vector2f,
  scale: &Vector2f,
  frame: usize,
) {
  let mut sprite = Sprite::with_texture(&texture_data.texture);
  //TODO: Animation?
  if !texture_data.sprite_rects.is_empty() {
    sprite.set_texture_rect(&texture_data.sprite_rects[frame]);
  }
  sprite.set_scale(Vector2f::from((
    texture_data.scale * scale.x,
    texture_data.scale * scale.y,
  )));
  sprite.set_position((position.x.floor(), position.y.floor()));
  sprite.set_origin((texture_data.origin.x.floor(), texture_data.origin.y.floor()));

  window.draw(&sprite);
}

fn draw_background(window: &mut RenderWindow, world: &mut World) {
  let texture = &world.texture_storage.get_texture_data("background").texture;
  let sprite = Sprite::with_texture(texture.deref());

  window.draw(&sprite);
}

fn draw_all<T: ?Sized + DrawableTrait>(
  window: &mut RenderWindow,
  texture_storage: &TextureStorage,
  container: &Arena<Box<T>>,
) {
  for (_index, elem) in container.iter() {
    draw_sprite(
      window,
      texture_storage.get_texture_data(elem.sprite_sheet_name()),
      elem.position(),
      elem.scale(),
      elem.current_frame(),
    );
  }
}

fn draw_flyout_actions(
  window: &mut RenderWindow,
  texture_storage: &TextureStorage,
  active_flyout_actions: &Vec<Box<FlyoutAction>>,
) {
  for action in active_flyout_actions {
    draw_sprite(
      window,
      texture_storage.get_texture_data(action.image_name()),
      action.position(),
      action.scale(),
      0,
    );
  }
}

fn draw_healthbars(window: &mut RenderWindow, world: &mut World) {
  let green_texture_data = &world
    .texture_storage
    .get_texture_data("private_sprites/healthbar_green.png");
  let red_texture_data = &world
    .texture_storage
    .get_texture_data("private_sprites/healthbar_red.png");

  //TODO: Factorize
  for (_index, minion) in &world.minions {
    {
      {
        let mut sprite = Sprite::with_texture(red_texture_data.texture.deref());
        sprite.set_scale(Vector2f::from((
          red_texture_data.scale,
          red_texture_data.scale,
        )));
        let mut position = minion.position().clone();
        position.y -= 61.;
        position.x -= 31.;
        sprite.set_position((position.x.floor(), position.y.floor()));
        sprite.set_origin((red_texture_data.origin.x, red_texture_data.origin.y));
        window.draw(&sprite);
      }

      {
        let mut sprite = Sprite::with_texture(green_texture_data.texture.deref());
        let percentage_multiplier = minion.health() as f32 / minion.max_health() as f32;

        sprite.set_scale(Vector2f::from((
          green_texture_data.scale * percentage_multiplier,
          green_texture_data.scale,
        )));
        let mut position = minion.position().clone();
        position.y -= 61.;
        position.x -= 31. + (green_texture_data.default_width as f32 / 2.)
          - green_texture_data.default_width as f32 * (percentage_multiplier / 2.);
        sprite.set_position((position.x, position.y));
        sprite.set_origin((green_texture_data.origin.x, green_texture_data.origin.y));
        window.draw(&sprite);
      }
    }
  }
}
