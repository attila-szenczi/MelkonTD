pub enum ZLayer {
    TileMap,
    Minion,
}

pub fn z_layer_to_coordinate(layer: ZLayer) -> f32 {
    match layer {
        ZLayer::TileMap => 0.,
        ZLayer::Minion => 0.01,
    }
}
