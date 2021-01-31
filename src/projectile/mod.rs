pub use self::projectile_trait::ProjectileTrait;
pub use self::projectile_updater::update_projectiles;
pub use self::pulsing_electric_ball::PulsingElectricBall;

mod projectile_component;
mod projectile_death_system;
mod projectile_trait;
mod projectile_update_system;
mod projectile_updater;
mod pulsing_electric_ball;
