pub enum ZLayer {
  TileMap = 0, //Currently invisible, will be completely removed
  Background,
  Minion,
  Tower,
  HealthBarBack,
  HealthBarMiddle,
  HealthBarFront,
  Projectile,
  UiFlyout,
}

pub fn z_layer_to_coordinate(layer: ZLayer) -> f32 {
  layer as i32 as f32 / 10.
}
