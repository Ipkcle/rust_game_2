use specs::VecStorage;
use specs::DenseVecStorage;
use specs::HashMapStorage;
#[macro_use]
pub mod physics;
pub mod render;
pub mod tags;
pub mod collision;
pub mod prefab;
pub mod combat;
pub mod deletion_conditions;

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct Name {
    string: String,
}

impl Name {
    pub fn new(string: String) -> Self {
        Self { string }
    }

    pub fn read(&self) -> &str {
        self.string.as_str()
    }
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub enum AI {
    BasicEnemy,
}
