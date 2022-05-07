use sfml::system::Vector2f;

pub trait DrawableTrait {
  fn sprite_sheet_name(&self) -> &'static str;
  fn position(&self) -> &Vector2f;
  fn scale(&self) -> &Vector2f;
  fn current_frame(&self) -> usize;
}
