use sfml::graphics::RenderWindow;
use sfml::system::Clock;
use sfml::window::Event;

use super::game_state_trait::{GameState, Transition};

use crate::minion::{update_minions, MinionSpawner};
use crate::projectile::update_projectiles;
use crate::render2d::draw_world;
use crate::tower::update_towers;
use crate::world::World;

pub struct BattleState {
  minion_spawner: MinionSpawner,
  clock: Clock,
}

impl BattleState {
  pub fn new() -> Self {
    BattleState {
      minion_spawner: MinionSpawner::new(),
      clock: Clock::start(), //TODO: Check std clock or something
    }
  }
}

impl<'b> GameState for BattleState {
  fn run(&mut self, window: &mut RenderWindow, world: &mut World) -> Transition {
    let elapsed = self.clock.restart().as_seconds();
    self.clock.restart();
    self.minion_spawner.update(world);
    update_towers(world, elapsed);
    update_projectiles(world, elapsed);
    update_minions(&mut world.minions, elapsed);
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
