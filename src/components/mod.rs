use specs::VecStorage;
#[macro_use]
pub mod physics;
pub mod render;
pub mod tags;
pub mod collision;

#[derive(Component, Debug)]
#[component(VecStorage)]
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
