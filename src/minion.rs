use amethyst::ecs::{Component, DenseVecStorage};

pub struct Minion {
    //TODO: box dyn trait
    pub health : i32,
}

impl Minion {
    pub fn new() -> Self{
        Minion{health : 20}
    }

    pub fn hit(&mut self, damage: i32) {
        self.health -= damage;
        println!("Minion got hit, remaining health: {:?}", self.health);
    }
}

impl Component for Minion {
    type Storage = DenseVecStorage<Self>;
}
