use std::ops::{Add, Sub, AddAssign, SubAssign};
use utils::cycle::*;
use components::physics::{Position, Velocity};
use components::collision::AABB;
use components::prefab::*;
use ggez::graphics::Vector2;
use specs::DenseVecStorage;
use specs::VecStorage;
use specs::NullStorage;
use specs::{Entity, Entities, LazyUpdate};

#[derive(Default, Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct RecievesCollideEffects;

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct CollideEffects {
    prefab: Prefab
}

impl CollideEffects {
    pub fn from_prefab(prefab: Prefab) -> Self {
        Self {
            prefab, 
        }
    }

    pub fn apply(&self, from: Entity, to: Entity, updater: &LazyUpdate) {
        self.prefab.apply_effects_to_entity(from, to, updater);
    }
}

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct DodgeData {
    speed_added: f32,
    direction: Vector2,
    dodge_cycle: Cycle,
}

impl DodgeData {
    pub fn new_with_cooldown(speed_added: f32, windup: Option<f32>, cooldown: Option<f32>) -> Self {
        DodgeData {
            speed_added,
            direction: Vector2::new(1.0, 0.0),
            dodge_cycle: Cycle::new(windup, cooldown),
        }
    }
    pub fn new(speed_added: f32, windup: Option<f32>, drag_constant: f32) -> Self {
        let cooldown = (0.01 as f32).ln() / (-1.0 * drag_constant);
        DodgeData {
            speed_added,
            direction: Vector2::new(1.0, 0.0),
            dodge_cycle: Cycle::new(windup, Some(cooldown)),
        }
    }
    pub fn update(
        &mut self,
        dt: f32,
        entitity: Entity,
        updater: &LazyUpdate,
    ) -> Option<PhaseChange> {
        let phase_change = self.dodge_cycle.get_phase_change(dt);
        if let Some(PhaseChange::Trigger) = phase_change {
            self.dodge(
                entitity,
                updater,
            );
        }
        phase_change
    }
    fn dodge(&self, entity: Entity, update: &LazyUpdate) {
        PrefabComponent::from(Velocity::from_vector2(self.direction.normalize() * self.speed_added))
            .merge_with_entity(entity, update);
    }

    pub fn set_dodging(&mut self, dodging: bool) {
        self.dodge_cycle.set_active(dodging);
    }

    pub fn set_direction(&mut self, direction: Vector2) {
        if direction != Vector2::zeros() {
            self.direction = direction.normalize();
        }
    }

    pub fn get_phase(&self) -> Phase {
        self.dodge_cycle.get_phase()
    }
}

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct ShootData {
    bullet: Prefab,
    bullet_direction: Vector2,
    bullet_speed: f32,
    shoot_cycle: Cycle,
    shoot_from_distance: f32,
}

impl ShootData {
    pub fn new(bullet: Prefab, bullet_speed: f32, windup: Option<f32>, cooldown: Option<f32>, shoot_from_distance: f32) -> Self {
        ShootData {
            bullet,
            bullet_direction: Vector2::zeros(),
            bullet_speed,
            shoot_cycle: Cycle::new(windup, cooldown),
            shoot_from_distance,
        }
    }
    pub fn update(
        &mut self,
        pos: &Position,
        dt: f32,
        entities: &Entities,
        updater: &LazyUpdate,
    ) -> Option<PhaseChange> {
        let phase_change = self.shoot_cycle.get_phase_change(dt);
        if let Some(PhaseChange::Trigger) = phase_change {
            self.shoot(
                pos.get() + (self.bullet_direction * self.shoot_from_distance),
                self.bullet_direction * self.bullet_speed,
                entities,
                updater,
            );
        }
        phase_change
    }
    fn shoot(&self, pos: Vector2, vel: Vector2, entities: &Entities, updater: &LazyUpdate) {
        self.bullet
            .clone()
            .with_pos(Position::new(pos.x, pos.y))
            .with_vel(Velocity::new(vel.x, vel.y))
            .in_entities(entities, updater);
    }
    pub fn set_shooting(&mut self, shooting: bool) {
        self.shoot_cycle.set_active(shooting);
    }
    pub fn set_direction(&mut self, direction: Vector2) {
        if direction != Vector2::zeros() {
            self.bullet_direction = direction.normalize();
        }
    }
    pub fn get_phase(&self) -> Phase {
        self.shoot_cycle.get_phase()
    }
}

#[derive(Component, Debug, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub struct Stamina {
    value: i32
}

#[derive(Component, Debug, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub struct Health {
    value: i32
}

#[derive(Component, Debug, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub struct Damage {
    value: i32
}

for_impl! {
    Health, Damage, Stamina;

    impl {
        pub fn new(val: i32) -> Self {
            Self {
                value: val,
            }
        }

        pub fn get(&self) -> i32 {
            self.value
        }

        pub fn set(&mut self, val: i32) {
            self.value = val;
        }
    }

    impl AddAssign {
        fn add_assign(&mut self, other: Self) {
            self.value += other.value
        }
    }

    impl Add {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self::new(self.value + other.value)
        }
    }

    impl SubAssign {
        fn sub_assign(&mut self, other: Self) {
            self.value -= other.value
        }
    }

    impl Sub {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self::new(self.value - other.value)
        }
    }
}
