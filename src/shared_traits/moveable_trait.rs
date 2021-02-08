use sfml::system::Vector2f;

//TODO: No Send + Sync after amethyst removal
pub trait MoveableTrait: Send + Sync {
  fn position_mut(&mut self) -> &mut Vector2f;
  fn scale_mut(&mut self) -> &mut Vector2f;
}
