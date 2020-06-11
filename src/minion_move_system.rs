use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{
        prelude::*,
        prelude::{Read, System},
    },
};

use crate::minion::Minion;

#[derive(SystemDesc)]
pub struct MinionMoveSystem;

impl<'a> System<'a> for MinionMoveSystem {
    type SystemData = (
        ReadStorage<'a, Minion>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (test_minions, mut transforms, time): Self::SystemData) {
        for (_test_minion, transform) in (&test_minions, &mut transforms).join() {
            transform.prepend_translation_y(-50. * time.delta_seconds());
        }
    }
}
