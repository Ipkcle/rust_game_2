use specs::Entity;
//use specs::VecStorage;
use specs::NullStorage;
use specs::HashMapStorage;

#[derive(Component, Default, Debug, Clone)]
#[storage(NullStorage)]
pub struct MarkedForDeletion;

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct InteractedWith {
    max: Option<u32>,
    interacted_with: Vec<Entity>,
}

impl InteractedWith {
    pub fn new() -> InteractedWith {
        InteractedWith {
            max: None,
            interacted_with: Vec::new(),
        }
    }

    pub fn with_max(max: u32) -> InteractedWith {
        InteractedWith {
            max: Some(max),
            interacted_with: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.interacted_with.push(entity);
    }

    pub fn get_entities(&self) -> Vec<Entity> {
        self.interacted_with.clone()
    }

    pub fn has_interacted_with(&self, entity: Entity) -> bool {
        self.interacted_with.contains(&entity)
    }

    pub fn should_delete(&self) -> bool {
        if let Some(max) = self.max {
            self.interacted_with.len() >= max as usize
        } else {
            false
        }
    }
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct DistanceTraveled {
    max: Option<f32>,
    current: f32,
}

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct TimeExisted {
    max: Option<f32>,
    current: f32,
}

for_impl! { DistanceTraveled, TimeExisted;

    impl {
        pub fn with_max(max: f32) -> Self {
            Self {
                max: Some(max),
                current: 0.0,
            }
        }

        pub fn new() -> Self {
            Self {
                max: None,
                current: 0.0,
            }
        }

        pub fn add(&mut self, distance: f32) {
            self.current += distance;
        }

        pub fn get(&self) -> f32 {
            self.current
        }

        pub fn should_delete(&self) -> bool {
            if let Some(max) = self.max {
                 self.current > max
            } else {
                false
            }
        }
    }
}
