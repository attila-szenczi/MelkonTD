use sfml::system::Vector2f;

//TODO: No Send + Sync after amethyst removal
pub trait MinionTrait: Send + Sync {
  fn hit(&mut self, damage: i32);

  fn health(&self) -> i32;
  fn max_health(&self) -> i32;
  fn dead(&self) -> bool;

  fn sprite_sheet_name(&self) -> &'static str;
  fn position(&self) -> &Vector2f;
  fn position_mut(&mut self) -> &mut Vector2f;
  //get_animation_state
}
