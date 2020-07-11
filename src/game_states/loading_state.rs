use sfml::window::Event;

use super::battle_state::BattleState;
use super::game_state_trait::{GameState, Transition};
use crate::world::World;
use sfml::graphics::RenderWindow;

pub struct LoadingState;

impl LoadingState {
  pub fn new() -> Self {
    LoadingState {}
  }
}

impl<'b> GameState for LoadingState {
  fn run(&mut self, window: &mut RenderWindow, world: &mut World) -> Transition {
    //Load assets
    world
      .texture_storage
      .insert_with_key("background", "private_sprites/game_background_1.png");

    world
      .texture_storage
      .insert("private_sprites/5_enemies_1_attack_018.png");
    world.texture_storage.insert("sprites/healthbar_back.png");
    world.texture_storage.insert("sprites/healthbar_front.png");
    world
      .texture_storage
      .insert("sprites/healthbar_outline.png");
    world
      .texture_storage
      .insert("private_sprites/electric_tower.png");
    world.texture_storage.insert("sprites/projectile.png");
    world
      .texture_storage
      .insert("private_sprites/pulsing_electric_ball.png");
    world
      .texture_storage
      .insert("sprites/electric_mage_tower_icon.png");
    world.texture_storage.insert("sprites/locked_icon.png");
    world
      .texture_storage
      .insert("private_sprites/game_background_1.png");

    // Event processing
    while let Some(event) = window.poll_event() {
      // Request closing for the window
      if event == Event::Closed {
        window.close();
      }
    }
    return Transition::PushState(Box::new(BattleState::new()));
  }
}
