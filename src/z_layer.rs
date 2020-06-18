pub enum ZLayer {
  TileMap = 0,
  Minion,
  Tower,
  Projectile,
  HealthBarBack,
  HealthBarFront,
  UiFlyout,
}

pub fn z_layer_to_coordinate(layer: ZLayer) -> f32 {
  layer as i32 as f32 / 100.
}
