use crate::{entities::Entity, global::Direction};

pub struct Board {
    entities: Vec<Box<dyn Entity>>,
}

impl Board {
    pub fn new(entities: Vec<Box<dyn Entity>>) -> Self {
        Self { entities }
    }
    pub fn player_input(&mut self, input: Direction) {
        for entity in self.entities.iter_mut() {
            entity.player_input(input);
        }
    }
}
