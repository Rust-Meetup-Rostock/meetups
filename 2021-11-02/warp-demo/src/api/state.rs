use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use super::entities::Entity;

#[derive(Clone)]
pub struct State {
    pub store: Arc<RwLock<HashMap<u8, Entity>>>,
}

impl State {
    pub fn _new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let entity = Entity::default();

        let result = Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        };

        result.store.write().unwrap().insert(entity.number, entity);

        result
    }
}
