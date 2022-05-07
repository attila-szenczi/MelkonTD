pub use minion_spawner::MinionSpawner;
pub use minion_trait::MinionTrait;
pub use minion_updater::update_minions;
pub use test_minion::TestMinion;

mod minion_component;
mod minion_death_system;
mod minion_spawner;
mod minion_trait;
mod minion_update_system;
mod minion_updater;
mod test_minion;
