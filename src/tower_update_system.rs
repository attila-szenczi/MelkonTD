use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    core::transform::Transform,
    ecs::{
        prelude::*,
        prelude::{Read, System},
    },
};

use crate::tower::Tower;
use crate::minion::Minion;

#[derive(SystemDesc)]
pub struct TowerUpdateSystem;

impl<'a> System<'a> for TowerUpdateSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Tower>,
        WriteStorage<'a, Minion>,
        ReadStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (entities, mut towers, mut minions, transforms, time): Self::SystemData) {
        for (tower, transform) in (&mut towers, &transforms).join() {
            tower.update(&entities, transform, &mut minions, &transforms, time.delta_seconds());
        }
    }
}