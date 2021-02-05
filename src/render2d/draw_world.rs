use sfml::graphics::{
  blend_mode::BlendMode, Color, RectangleShape, RenderStates, RenderTarget, RenderWindow, Shape,
  Sprite, Texture, Transformable,
};
use sfml::system::Vector2f;

use std::ops::Deref;

use generational_arena::Arena;

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

fn draw_healthbars(window: &mut RenderWindow, world: &mut World) {
  //Temp, will be replaced with textures
  for (_index, minion) in &world.minions {
    {
      let mut rectangle = RectangleShape::new();

      let mut position = minion.position().clone();
      position.y -= 61.;
      position.x -= 31.;
      position.x = position.x.floor();
      position.y = position.y.floor();
      rectangle.set_size(Vector2f::new(62., 17.));
      rectangle.set_fill_color(Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
      });
      rectangle.set_position(position);
      window.draw(&rectangle);
    }

    {
      let mut rectangle = RectangleShape::new();

      let mut position = minion.position().clone();
      position.y -= 60.;
      position.x -= 30.;
      position.x = position.x.floor();
      position.y = position.y.floor();
      rectangle.set_size(Vector2f::new(60., 15.));
      rectangle.set_fill_color(Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
      });
      rectangle.set_position(position);
      window.draw(&rectangle);
    }

    let mut rectangle = RectangleShape::new();

    let mut position = minion.position().clone();
    position.y -= 60.;
    position.x -= 30.;
    position.x = position.x.floor();
    position.y = position.y.floor();
    rectangle.set_size(Vector2f::new(50., 15.));
    rectangle.set_fill_color(Color {
      r: 0,
      g: 255,
      b: 0,
      a: 255,
    });
    rectangle.set_position(position);

    window.draw(&rectangle);
  }
}
