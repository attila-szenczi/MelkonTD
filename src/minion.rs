use amethyst::ecs::{Component, DenseVecStorage};

pub struct Minion {
    //TODO: box dyn trait
    pub max_health: i32,
    pub health: i32,
}

impl Minion {
    pub fn new() -> Self {
        Minion {
            max_health: 20,
            health: 20,
        }
    }

    pub fn hit(&mut self, damage: i32) {
        self.health -= damage;
        println!("Minion got hit, remaining health: {:?}", self.health);
    }
}

impl Component for Minion {
    type Storage = DenseVecStorage<Self>;
}
