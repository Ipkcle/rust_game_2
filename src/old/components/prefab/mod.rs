use std::mem::{discriminant, Discriminant};
use std::collections::HashMap;
use std::any::Any;
use specs::{RunNow, World, Component, Entity, Entities, LazyUpdate};
use specs::world::EntityBuilder;
use components::Name;
use components::physics::*;
use components::render::*;
use components::collision::*;
use components::tags::TakesInput;
use components;

macro_rules! as_item {
    ($i:item) => { $i };
}


macro_rules! as_ident {
    ($i:ident) => { $i };
}
macro_rules! match_pattern {
    ($i:ident) => { PrefabComponent::$i(c) };
}
macro_rules! i_hate_myself {
    {
        $($ident:ident: $ty:ty),*;
    } => {

        as_item! {
            enum PrefabComponent { $($ident($ty),)* }
        }

        impl PrefabComponent {
            pub fn add_to_entity_builder<'a>(&self, entity_builder: EntityBuilder<'a>) ->  EntityBuilder<'a> {
                match self.clone() {
                    $(match_pattern!($ident) => entity_builder.with(c),)*
                }
            }
            pub fn add_to_entity<'a>(&self, entity: &Entity, updater: &LazyUpdate) {
                match self.clone() {
                    $(match_pattern!($ident) => updater.insert(*entity, c),)*
                }
            }
        }
    }
}

i_hate_myself! {
    Pos: Position;
}
//  #[derive(Debug, Clone, FromVariants)]
//  pub enum PrefabComponent {
//      Position(Position),
//      Velocity(Velocity),
//      TakesInput(TakesInput),
//  }

//  impl PrefabComponent {
//      pub fn add_to_entity_builder<'a>(&self, entity_builder: EntityBuilder<'a>) ->  EntityBuilder<'a> {
//          match self.clone() {
//              PrefabComponent::Velocity(c) => {
//                  entity_builder.with(c)
//              },
//              PrefabComponent::Position(c) => {
//                  entity_builder.with(c)
//              },
//              PrefabComponent::TakesInput(c) => {
//                  entity_builder.with(c)
//              },
//          }
//      }
//      pub fn add_to_entity<'a>(&self, entity: &Entity, updater: &LazyUpdate) {
//          match self.clone() {
//              PrefabComponent::Velocity(c) => {
//                  updater.insert(*entity, c)
//              },
//              PrefabComponent::Position(c) => {
//                  updater.insert(*entity, c)
//              },
//              PrefabComponent::TakesInput(c) => {
//                  updater.insert(*entity, c)
//              },
//          }
//      }
//  }

#[derive(Clone)]
pub struct Prefab {
    components: HashMap<Discriminant<PrefabComponent>, PrefabComponent>,
}

impl Prefab {
    pub fn add_component(&mut self, prefab_component: PrefabComponent) {
        self.components.insert(discriminant(&prefab_component), prefab_component);
    }

    pub fn generate_in_world(&self, world: &mut World) {
        let mut entity_builder = world.create_entity();
        for component in self.components.values() {
            entity_builder = component.add_to_entity_builder(entity_builder);
        }
        entity_builder.build();
    }

    pub fn lazy_generate_in_entities(&self, entities: &Entities, updater: &LazyUpdate) {
        let new_entity = entities.create();
        for component in self.components.values() {
            component.add_to_entity(&new_entity, updater);
        }
    }
}
