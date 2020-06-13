use amethyst::{
    core::math::Vector3,
    core::transform::Transform,
    ecs::{prelude::*, Component, DenseVecStorage},
    renderer::SpriteRender,
};

use if_chain::if_chain;

use crate::minion::Minion;
use crate::projectile::Projectile;
use crate::z_layer::{z_layer_to_coordinate, ZLayer};

pub struct Tower {
    //TODO Boxed dyn trait object to handle firing, evolution options etc
    pub target: Option<Entity>,
    pub damage: i32,
    pub firing_timer: f32,
    pub range: f32,
    sprite_render: SpriteRender,
}

impl Component for Tower {
    type Storage = DenseVecStorage<Self>;
}

impl Tower {
    pub fn new(sprite_render: SpriteRender) -> Self {
        Tower {
            target: None,
            damage: 10,
            firing_timer: 1.,
            range: 70.,
            sprite_render,
        }
    }

    pub fn update<'a>(
        &mut self,
        entities: &Entities<'a>,
        tower_transform: &Transform,
        minions: &ReadStorage<'a, Minion>,
        transforms: &ReadStorage<'a, Transform>,
        updater: &Read<'a, LazyUpdate>,
        elapsed: f32,
    ) {
        if self.update_timer(elapsed) {
            if_chain! {
                if let Some(entity) = self.target;
                if let Some(target_transform) = transforms.get(entity);
                if self.is_in_range(tower_transform.translation(), target_transform.translation());
                then {
                    self.fire(entities, updater, tower_transform.translation());
                } else {
                    //TODO: Lookup instead of entities join?
                    for (entity, _minion, transform) in (entities, minions, transforms).join() {
                        let tower_translation = tower_transform.translation();
                        if self.is_in_range(tower_translation, transform.translation()) {
                            self.target = Some(entity);
                            self.fire(entities, updater, tower_translation);

                            break;
                        }
                    }
                }
            }
        }
    }

    fn fire<'a>(
        &mut self,
        entities: &Entities<'a>,
        updater: &Read<'a, LazyUpdate>,
        tower_translation: &Vector3<f32>,
    ) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            tower_translation.x as f32,
            tower_translation.y as f32,
            z_layer_to_coordinate(ZLayer::Projectile),
        );
        updater
            .create_entity(&entities)
            .with(self.sprite_render.clone())
            .with(transform)
            .with(Projectile::new(self.target.unwrap(), 10, 5., 150.))
            .build();
        self.reset_timer();
    }

    fn update_timer(&mut self, elapsed: f32) -> bool {
        if self.firing_timer > 0. {
            self.firing_timer -= elapsed;
        } else {
            self.firing_timer = 0.;
        }
        self.firing_timer <= 0.
    }

    fn reset_timer(&mut self) {
        self.firing_timer += 1.;
    }

    fn is_in_range(&self, lhs: &Vector3<f32>, rhs: &Vector3<f32>) -> bool {
        let y_diff = lhs.y - rhs.y;
        let x_diff = lhs.x - rhs.x;
        let square_sum = y_diff * y_diff + x_diff * x_diff;
        square_sum.sqrt() < self.range
    }
}
