use crate::animation::LinearScalePositionTransition;

use crate::world::World;

use sfml::graphics::IntRect;
use sfml::system::Vector2f;

use crate::tower::ElectricMageTower;

pub struct FlyoutAction {
    transition: LinearScalePositionTransition,
    image_name: String,
    full_width: i32,
    pub rect: IntRect,
    full_height: i32,
    source_index: i32, // Which entity type was clicked
    action_index: i32, // nth action
}

impl FlyoutAction {
    pub fn new(
        transition: LinearScalePositionTransition,
        image_name: String,
        full_width: i32,
        full_height: i32,
        source_index: i32,
        action_index: i32,
    ) -> Self {
        FlyoutAction {
            transition,
            image_name,
            full_width,
            full_height,
            rect: IntRect::new(0, 0, 0, 0),
            source_index,
            action_index,
        }
    }

    pub fn execute(&mut self, world: &mut World) {
        //TODO: Do this properly based on the clicked action and remove the clickable slot
        let mage_tower = Box::new(ElectricMageTower::new(Vector2f::new(562., 950.)));
        world.towers.insert(mage_tower);
    }

    pub fn update(&mut self, elapsed: f32) {
        if self.transition.update(elapsed) {
            //Note: Can be 2 px off cause of rounding
            let percentage_multiplier = self.transition.transition_percentage_multiplier();
            self.rect.left = (self.transition.position_current.x
                - self.full_width as f32 * percentage_multiplier / 2.)
                as i32;
            self.rect.width = (self.full_width as f32 * percentage_multiplier) as i32;

            self.rect.top = (self.transition.position_current.y
                - self.full_height as f32 * percentage_multiplier / 2.)
                as i32;
            self.rect.height = (self.full_height as f32 * percentage_multiplier) as i32;
        }
    }

    pub fn image_name(&self) -> &str {
        &self.image_name
    }

    pub fn position(&self) -> &Vector2f {
        &self.transition.position_current
    }

    pub fn scale(&self) -> &Vector2f {
        &self.transition.scale_current
    }
}
