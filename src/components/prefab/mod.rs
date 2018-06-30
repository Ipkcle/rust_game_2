use assets::DrawableAsset;
use components::collision::*;
use components::combat::*;
use components::deletion_conditions::*;
use components::physics::*;
use components::render::*;
use components::tags::*;
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
            pub fn add_to_entity(&self, entity: Entity, updater: &LazyUpdate) {
                match self.clone() {
                    $(make_prefab_components_enum!(@make_pattern_branch $ident, c) => updater.insert(entity, c),)*
                }
            }
        }
    };
    (@make_pattern_branch $variant:ident, $inner:ident) => { PrefabComponent::$variant($inner) };
}

impl PrefabComponent {
    pub fn merge_with_entity(&self, entity: Entity, updater: &LazyUpdate) {
        match self.clone() {
            PrefabComponent::Position(val) => updater.exec(move |world| {
                if let Some(old) = world.write_storage::<Position>().get_mut(entity) {
                    *old += val;
                }
            }),
            PrefabComponent::Velocity(val) => updater.exec(move |world| {
                if let Some(old) = world.write_storage::<Velocity>().get_mut(entity) {
                    *old += val;
                }
            }),
            PrefabComponent::Acceleration(val) => updater.exec(move |world| {
                if let Some(old) = world.write_storage::<Acceleration>().get_mut(entity) {
                    *old += val;
                }
            }),
            _ => {}
        }
    }
    pub fn apply_effects_to_entity(&self, from: Entity, to: Entity, updater: &LazyUpdate) {
        match self.clone() {
            PrefabComponent::Damage(val) => updater.exec(move |world| {
                if let Some(old) = world.write_storage::<Health>().get_mut(to) {
                    *old -= Health::new(val.get());
                }
            }),
            PrefabComponent::Push(push) => updater.exec(move |world| {
                let mut direction = Vector2::zeros();
                if let Some(from_velocity) = world.read_storage::<Velocity>().get(from) {
                    if from_velocity.get() != Vector2::zeros() {
                        direction = from_velocity.get().normalize();
                    }
                }
                if let Some(target_velocity) = world.write_storage::<Velocity>().get_mut(to) {
                    *target_velocity += Velocity::from_vector2(direction * push.get_magnitude());
                }
            }),
            PrefabComponent::Knockback(knockback) => updater.exec(move |world| {
                if let Some(velocity) = world.write_storage::<Velocity>().get_mut(to) {
                    if let Some(target_position) = world.read_storage::<Position>().get(to) {
                        if let Some(from_position) =
                            world.read_storage::<Position>().get(from)
                        {
                            let direction = (target_position.get() - from_position.get()).normalize();
                            *velocity += Velocity::from_vector2(direction * knockback.get_magnitude());
                        }
                    }
                }
            }),
            _ => {
                self.merge_with_entity(to, updater);
            }
        }
    }
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
    CameraFollows: CameraFollows,
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
    Stamina: Stamina,
    ShootData: ShootData,
    DodgeData: DodgeData,
    InteractedWith: InteractedWith,
    DistanceTraveled: DistanceTraveled,
    TimeExisted: TimeExisted,
    CollideEffects: CollideEffects,
    RecievesCollideEffects: RecievesCollideEffects,
    Knockback: Knockback,
    Push: Push,
    AI: AI,
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

    pub fn with_pos(self, pos: Position) -> Prefab {
        self.with_component(PrefabComponent::from(pos))
    }

    pub fn with_vel(self, vel: Velocity) -> Prefab {
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
            component.add_to_entity(new_entity, updater);
        }
    }

    pub fn add_to_entity(&self, entity: Entity, updater: &LazyUpdate) {
        for component in self.components.values() {
            component.add_to_entity(entity, updater);
        }
    }

    pub fn apply_effects_to_entity(&self, from: Entity, to: Entity, updater: &LazyUpdate) {
        for component in self.components.values() {
            component.apply_effects_to_entity(from, to, updater);
        }
    }
}

pub mod prefabs {
    use super::*;

    pub fn player() -> Prefab {
        let accel = 2500.0;
        let max_speed = 250.0;
        let drag_constant = accel / max_speed;
        let radius = 15;
        Prefab!(
            IsPlayer,
            Position::zeros(),
            Velocity::zeros(),
            Acceleration::zeros(),
            MoveDrag::new(drag_constant),
            MoveDirection::new(accel),
            Health::new(3),
            Stamina::new(100),
            Collisions::new(),
            IsBlocked,
            CameraFollows,
            RecievesCollideEffects,
            ShootData::new(bullet(), 300.0, Some(0.2), Some(0.32), radius as f32),
            DodgeData::new_with_cooldown(1200.0, None, Some(0.5)),
            Name::new("Player".to_owned())
        ).with(&circle(radius))
    }

    pub fn enemy() -> Prefab {
        Prefab!(
            AI::BasicEnemy,
            Position::zeros(),
            Velocity::zeros(),
            Acceleration::zeros(),
            MoveDrag::new(10.0),
            MoveDirection::new(2500.0),
            Health::new(4),
            BlocksMovement,
            IsBlocked,
            Collisions::new(),
            RecievesCollideEffects,
            ShootData::new(bullet(), 300.0, Some(0.2), Some(0.52), 20.0),
            Name::new("Enemy".to_owned())
        ).with(&circle(20))
    }

    pub fn dummy() -> Prefab {
        Prefab!(
            Position::zeros(),
            Velocity::zeros(),
            MoveDrag::new(10.0),
            Health::new(2),
            //BlocksMovement,
            IsBlocked,
            Collisions::new(),
            RecievesCollideEffects,
            Name::new("Dummy".to_owned())
        ).with(&circle(10))
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
            InteractedWith::with_max(1),
            CollideEffects::from_prefab(bullet_effects()),
            Name::new("Bullet".to_owned())
        ).with(&circle(3))
    }

    pub fn bullet_effects() -> Prefab {
        Prefab!(
            Damage::new(1),
            Push::new(300.0)
        )
    }

    pub fn kick() -> Prefab {
        Prefab!(
            Push::new(500.0)
        )
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
