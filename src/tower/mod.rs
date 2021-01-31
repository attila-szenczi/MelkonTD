pub use self::electric_mage_tower::ElectricMageTower;
pub use self::tower_trait::TowerTrait;
pub use self::tower_type::TowerType;
pub use self::tower_updater::update_towers;

mod electric_mage_tower;
mod tower_component;
mod tower_trait;
mod tower_type;
mod tower_update_system;
mod tower_updater;
