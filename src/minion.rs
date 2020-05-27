use amethyst::ecs::{Component, DenseVecStorage};

pub struct TestMinion {
}

impl Component for TestMinion {
    type Storage = DenseVecStorage<Self>;
}