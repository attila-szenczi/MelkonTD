use sfml::graphics::Transform;

//TODO: No Send + Sync after amethyst removal
pub trait MinionTrait: Send + Sync {
  fn hit(&mut self, damage: i32);

  fn health(&self) -> i32;
  fn max_health(&self) -> i32;
  fn dead(&self) -> bool;

  fn sprite_sheet_name(&self) -> &'static str;
  fn transform(&self) -> &Transform;
  fn transform_mut(&mut self) -> &mut Transform;
  //get_animation_state
}
