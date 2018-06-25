use assets::{DrawableAsset};
use components::collision::*;
use components::physics::*;
use components::render::*;
use components::combat::*;
use components::deletion_conditions::*;
use components::tags::IsPlayer;
use components::*;
use ggez::graphics::Vector2;
use specs::world::EntityBuilder;
use specs::{Entities, Entity, LazyUpdate, World};
use std::collections::HashMap;
use std::mem::{discriminant, Discriminant};

macro_rules! make_prefab_components_enum {
    {
        $($ident:ident: $ty:ty),*
    } => {

        as_item! {
            #[derive(Debug, Clone, FromVariants)]
            pub enum PrefabComponent { $($ident($ty),)* }
        }

        impl PrefabComponent {
            pub fn add_to_entity_builder<'a>(&self, entity_builder: EntityBuilder<'a>) ->  EntityBuilder<'a> {
                match self.clone() {
                    $(make_prefab_components_enum!(@make_pattern_branch $ident, c) => entity_builder.with(c),)*
                }
            }
            pub fn add_to_entity<'a>(&self, entity: &Entity, updater: &LazyUpdate) {
                match self.clone() {
                    $(make_prefab_components_enum!(@make_pattern_branch $ident, c) => updater.insert(*entity, c),)*
                }
            }
        }
    };
    (@make_pattern_branch $variant:ident, $inner:ident) => { PrefabComponent::$variant($inner) };
}

macro_rules! Prefab {
    ($($component:expr),*) => {
        Prefab::new()$(.with_component(PrefabComponent::from($component)))*
    }
}

make_prefab_components_enum! {
    Position: Position,
    Velocity: Velocity,
    IsPlayer: IsPlayer,
    MoveDrag: MoveDrag,
    MoveDirection: MoveDirection,
    Acceleration: Acceleration,
    DrawableComponent: DrawableComponent,
    Collisions: Collisions,
    Hitbox: Hitbox,
    AABB: AABB,
    IsBlocked: IsBlocked,
    BlocksMovement: BlocksMovement,
    Damage: Damage,
    Health: Health,
    CanShoot: CanShoot,
    IdentificationNumber: IdentificationNumber,
    InteractedWith: InteractedWith,
    DistanceTraveled: DistanceTraveled,
    TimeExisted: TimeExisted,
    Name: Name
}

#[derive(Debug, Clone)]
pub struct Prefab {
    components: HashMap<Discriminant<PrefabComponent>, PrefabComponent>,
}

impl Prefab {
    pub fn new() -> Self {
        Prefab {
            components: HashMap::new(),
        }
    }

    pub fn with(mut self, prefab: &Prefab) -> Prefab {
        self.add_prefab(prefab);
        self
    }

    pub fn with_component(mut self, prefab_component: PrefabComponent) -> Prefab {
        self.add_component(prefab_component);
        self
    }

    pub fn with_pos(mut self, pos: Position) -> Prefab {
        self.with_component(PrefabComponent::from(pos))
    }

    pub fn with_vel(mut self, vel: Velocity) -> Prefab {
        self.with_component(PrefabComponent::from(vel))
    }

    pub fn add_component(&mut self, prefab_component: PrefabComponent) {
        self.components
            .insert(discriminant(&prefab_component), prefab_component);
    }

    pub fn add_prefab(&mut self, prefab: &Prefab) {
        for component in prefab.components.values() {
            self.add_component(component.clone());
        }
    }

    pub fn in_world(&self, world: &mut World) {
        let mut entity_builder = world.create_entity();
        for component in self.components.values() {
            entity_builder = component.add_to_entity_builder(entity_builder);
        }
        entity_builder.build();
    }

    pub fn in_entities(&self, entities: &Entities, updater: &LazyUpdate) {
        let new_entity = entities.create();
        for component in self.components.values() {
            component.add_to_entity(&new_entity, updater);
        }
    }
}

pub mod prefabs {
    use super::*;

    pub fn player() -> Prefab {
        let accel = 2500.0;
        let max_speed = 250.0;
        let drag_constant = accel / max_speed;
        Prefab!(
            IsPlayer,
            Position::zeros(),
            Velocity::zeros(),
            Acceleration::zeros(),
            MoveDrag::new(drag_constant),
            MoveDirection::new(accel),
            Collisions::new(),
            IsBlocked,
            CanShoot::new(bullet(), 300.0, 0.2, 0.32),
            Name::new("Player".to_owned())
        ).with(&circle(20))
    }

    pub fn wall() -> Prefab {
        Prefab!(
            Position::zeros(),
            Hitbox::Rectangle {
                dimensions: Vector2::new(10 as f32, 10 as f32),
                angle: 0.0,
            },
            BlocksMovement,
            DrawableComponent::new(DrawableAsset::rect(10, 10)),
            Name::new("Block".to_owned())
        )
    }

    pub fn bullet() -> Prefab {
        Prefab!(
            TimeExisted::with_max(0.3),
            Name::new("Bullet".to_owned())
        ).with(&circle(3))
    }

    pub fn rect(w: u32, h: u32) -> Prefab {
        let dimensions = Vector2::new(w as f32, h as f32);
        Prefab!(
            DrawableComponent::new(DrawableAsset::rect(w, h)),
            AABB::new(dimensions),
            Hitbox::Rectangle {
                dimensions,
                angle: 0.0,
            }
        )
    }

    pub fn circle(radius: u32) -> Prefab {
        let diameter = radius as f32 * 2.0;
        Prefab!(
            DrawableComponent::new(DrawableAsset::circle(radius)),
            AABB::new(Vector2::new(diameter, diameter)),
            Hitbox::Circle {
                radius: radius as f32,
            }
        )
    }
}
