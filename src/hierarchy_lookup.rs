use amethyst::ecs::prelude::Entity;

use std::collections::BTreeMap;

#[derive(Default)]
pub struct HierarchyLookup {
    lookup: BTreeMap<Entity, BTreeMap<String, Entity>>,
}

impl HierarchyLookup {
    pub fn insert(&mut self, parent: Entity, key: &str, entity: Entity) {
        let tree = self.lookup.entry(parent).or_insert(BTreeMap::new());
        *tree.entry(String::from(key)).or_insert(entity) = entity;
    }

    pub fn get_child(&self, parent: Entity, key: &str) -> Option<&Entity> {
        if let Some(tree) = self.lookup.get(&parent) {
            return tree.get(&String::from(key));
        }
        
        None
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.lookup.remove(&entity);
    }
}
