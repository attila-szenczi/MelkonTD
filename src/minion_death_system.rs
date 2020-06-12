use amethyst::{
    derive::SystemDesc,
    ecs::{
        prelude::*,
    },
};

use crate::minion::Minion;

#[derive(SystemDesc)]
pub struct MinionDeathSystem;

impl<'a> System<'a> for MinionDeathSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Minion>,
    );

    fn run(&mut self, (entities, minions): Self::SystemData) {
        for (entity, minion) in (&entities, &minions).join() {
            if minion.health <= 0 {
                match entities.delete(entity) {
                    Err(e) => println!("error during entity deletion: {:?}", e),
                    Ok(_v) => ()
                }
            }
        }
    }
}