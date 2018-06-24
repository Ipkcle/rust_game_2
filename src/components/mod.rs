use specs::VecStorage;
#[macro_use]
pub mod physics;
pub mod render;
pub mod tags;
pub mod collision;
pub mod prefab;
pub mod combat;
pub mod deletion_conditions;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[storage(VecStorage)]
pub struct IdentificationNumber(u32);

impl IdentificationNumber {
    pub fn next(current: &mut u32) -> IdentificationNumber {
        *current += 1;
        IdentificationNumber(current.clone())
    }
}

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
