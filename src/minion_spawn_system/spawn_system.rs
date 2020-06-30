use amethyst::{
  assets::AssetStorage,
  core::transform::{Parent, Transform},
  derive::SystemDesc,
  ecs::{
    prelude::*,
    prelude::{Entities, LazyUpdate, Read, System},
    world::Builder,
  },
  renderer::{SpriteRender, SpriteSheet, Texture},
};

use crate::{
  hierarchy_lookup::HierarchyLookup,
  minion::{Minion, TestMinion},
  z_layer::{z_layer_to_coordinate, ZLayer},
};

const SPAWN_POINT_X: f32 = -50.;
const SPAWN_POINT_Y: f32 = 230.;

#[derive(SystemDesc)]
pub struct MinionSpawnSystem {
  sprite_render_minion: SpriteRender,
  sprite_render_healthbar_back: SpriteRender,
  sprite_render_healthbar_front: SpriteRender,
  sprite_render_healthbar_outline: SpriteRender,
  counter: u32,
}

impl MinionSpawnSystem {
  pub fn new(
    sprite_render_minion: SpriteRender,
    sprite_render_healthbar_back: SpriteRender,
    sprite_render_healthbar_front: SpriteRender,
    sprite_render_healthbar_outline: SpriteRender,
  ) -> Self {
    MinionSpawnSystem {
      sprite_render_minion,
      sprite_render_healthbar_back,
      sprite_render_healthbar_front,
      sprite_render_healthbar_outline,
      counter: 0,
    }
  }

  fn insert_child_rect<'a>(
    &self,
    sprite_render: &SpriteRender,
    x_scale: f32,
    y_scale: f32,
    parent: Entity,
    z_layer: ZLayer,
    entities: &Entities<'a>,
    updater: &Read<'a, LazyUpdate>,
    hierarchy_lookup: &mut Write<'a, HierarchyLookup>,
    rect_key: &str,
  ) {
    let mut transform_healthbar_outline = Transform::default();
    transform_healthbar_outline.set_translation_xyz(0., 80., z_layer_to_coordinate(z_layer));
    let scale = transform_healthbar_outline.scale_mut();
    scale.x = x_scale;
    scale.y = y_scale;

    let rect_entity = updater
      .create_entity(&entities)
      .with(sprite_render.clone())
      .with(transform_healthbar_outline)
      .with(Parent { entity: parent })
      .build();

    hierarchy_lookup.insert(parent, rect_key, rect_entity);
  }
}

impl<'a> System<'a> for MinionSpawnSystem {
  type SystemData = (
    Entities<'a>,
    Read<'a, LazyUpdate>,
    Read<'a, AssetStorage<Texture>>,
    Read<'a, AssetStorage<SpriteSheet>>,
    Write<'a, HierarchyLookup>,
  );

  //TODO: Remove asset storage and spritesheet somehow
  fn run(
    &mut self,
    (entities, updater, _asset_storage, _sprite_sheet, mut hierarchy_lookup): Self::SystemData,
  ) {
    self.counter += 1;

    if self.counter == 40 {
      let mut transform = Transform::default();
      transform.set_translation_xyz(
        SPAWN_POINT_X,
        SPAWN_POINT_Y,
        z_layer_to_coordinate(ZLayer::Minion),
      );

      let scale = transform.scale_mut();
      scale.x = 0.5;
      scale.y = 0.5;

      let entity = updater
        .create_entity(&entities)
        .with(self.sprite_render_minion.clone())
        .with(transform)
        .with(Minion::new(Box::new(TestMinion::new())))
        .build();

      self.insert_child_rect(
        &self.sprite_render_healthbar_outline,
        2.1,
        0.65,
        entity,
        ZLayer::HealthBarBack,
        &entities,
        &updater,
        &mut hierarchy_lookup,
        "healthbar_outline",
      );

      self.insert_child_rect(
        &self.sprite_render_healthbar_back,
        2.,
        0.5,
        entity,
        ZLayer::HealthBarMiddle,
        &entities,
        &updater,
        &mut hierarchy_lookup,
        "healthbar_back",
      );

      self.insert_child_rect(
        &self.sprite_render_healthbar_front,
        2.,
        0.5,
        entity,
        ZLayer::HealthBarFront,
        &entities,
        &updater,
        &mut hierarchy_lookup,
        "healthbar_front",
      );

      self.counter = 0;
    }
  }
}
