use sfml::graphics::{RenderStates, RenderTarget, RenderWindow, Sprite};
use sfml::window::Event;

use super::game_state_trait::{GameState, Transition};

use crate::render2d::draw_world;
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

    draw_world(window, world);

    // State machine
    while let Some(event) = window.poll_event() {
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
