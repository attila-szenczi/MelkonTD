use std::ops::Deref;

use sfml::graphics::{RenderStates, RenderTarget, RenderWindow, Sprite};
use sfml::window::Event;

use super::game_state_trait::{GameState, Transition};

use crate::world::World;

pub struct BattleState;

impl BattleState {
  pub fn new() -> Self {
    BattleState {}
  }
}

impl<'b> GameState for BattleState {
  fn run(&mut self, window: &mut RenderWindow, world: &mut World) -> Transition {
    // minion_spawn
    // projectile update
    // minion update
    // minion death
    // projectile death
    // simple animation

    render(window, world);

    // State machine
    while let Some(event) = window.poll_event() {
      println!("poll battle");
      // Request closing for the window
      match event {
        Event::Closed => {
          window.close();
          return Transition::Quit;
        }
        //Event::MouseButtonPressed { button, x, y } => println!("Left click"),
        _ => (),
      }
    }

    Transition::KeepState
  }
}

fn render(window: &mut RenderWindow, world: &mut World) {
  window.set_active(true);

  let texture = &world.texture_storage.get_texture_data("background").texture;
  let sprite = Sprite::with_texture(texture.deref());

  window.draw(&sprite);
  //window.draw_sprite(&sprite, RenderStates::default());
  window.display();
}
