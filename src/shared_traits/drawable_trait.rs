use sfml::system::Vector2f;

//TODO: No Send + Sync after amethyst removal
pub trait DrawableTrait: Send + Sync {
  fn sprite_sheet_name(&self) -> &'static str;
  fn position(&self) -> &Vector2f;
  fn position_mut(&mut self) -> &mut Vector2f;
  fn scale(&self) -> &Vector2f;
  fn scale_mut(&mut self) -> &mut Vector2f;
  fn current_frame(&self) -> usize;
  //get_animation_state
}
