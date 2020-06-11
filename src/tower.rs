use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    ecs::{
        prelude::{Entity, ReadStorage, WriteStorage, Join, Entities},
        Component, DenseVecStorage,
    },
};

use if_chain::if_chain;

use crate::minion::Minion;

pub struct Tower {
    //TODO Boxed dyn trait object to handle firing, evolution options etc
    pub target: Option<Entity>,
    pub damage: i32,
    pub firing_timer: f32,
    pub range: f32,
}

impl Component for Tower {
    type Storage = DenseVecStorage<Self>;
}

impl Tower {
    pub fn new() -> Self {
        Tower {
            target: None,
            damage: 10,
            firing_timer: 1.,
            range: 70.,
        }
    }

    pub fn update<'a>(
        &mut self,
        entities: &Entities<'a>,
        tower_transform: &Transform,
        minions: &mut WriteStorage<'a, Minion>, //TODO: read storage once projectile's will exist
        transforms: &ReadStorage<'a, Transform>,
        elapsed: f32,
    ) {
        if self.update_timer(elapsed) {
            if_chain! {
                if let Some(entity) = self.target;
                if let Some(target_transform) = transforms.get(entity);
                if self.is_in_range(tower_transform.translation(), target_transform.translation());
                if let Some(target_minion) = minions.get_mut(entity);
                then {
                    target_minion.hit(self.damage);
                    //fire
                } else {
                    //TODO: Lookup instead of entities join?
                    for (entity, minion, transform) in (entities, minions, transforms).join() {
                        if self.is_in_range(tower_transform.translation(), transform.translation()) {
                            self.target = Some(entity);
                            minion.hit(self.damage);
                        }
                    }
                }
            }
        }
    }

    fn update_timer(&mut self, elapsed: f32) -> bool {
        self.firing_timer -= elapsed;
        if self.firing_timer < 0. {
            self.firing_timer += 1.;
            true
        } else {
            false
        }
    }

    fn is_in_range(&self, lhs: &Vector3<f32>, rhs: &Vector3<f32>) -> bool {
        let y_diff = lhs.y - rhs.y;
        let x_diff = lhs.x - rhs.x;
        let square_sum = y_diff * y_diff + x_diff * x_diff;
        square_sum.sqrt() < self.range
    }
}
