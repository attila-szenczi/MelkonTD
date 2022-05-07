use sfml::system::Vector2f;

pub trait MoveableTrait {
  fn position_mut(&mut self) -> &mut Vector2f;
  fn scale_mut(&mut self) -> &mut Vector2f;
}
