pub use self::minion_component::Minion;
pub use self::minion_death_system::MinionDeathSystem;
pub use self::minion_update_system::MinionUpdateSystem;
pub use self::test_minion::TestMinion;

mod minion_component;
mod minion_death_system;
mod minion_trait;
mod minion_update_system;
mod test_minion;
