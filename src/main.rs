mod animation;
mod battle_input_states;
mod flyout_actions;
mod game_states;
mod minion;
mod projectile;
mod render2d;
mod shared_traits;
mod texture_storage;
mod tower;
mod world;

fn main() -> Result<(), std::io::Error> {
  let mut game = game_states::Game::new();

  game.run();

  Ok(())
}
