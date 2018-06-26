use specs::VecStorage;
#[macro_use]
pub mod physics;
pub mod render;
pub mod tags;
pub mod collision;
pub mod prefab;
pub mod combat;
pub mod deletion_conditions;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
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
