use std::ops::{Add, Sub, AddAssign, SubAssign};
use components::physics::{Position, Velocity};
use components::collision::AABB;
use components::prefab::*;
use ggez::graphics::Vector2;
use specs::DenseVecStorage;
use specs::VecStorage;
use specs::{Entities, LazyUpdate};

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct CanShoot {
    bullet: Prefab,
    bullet_direction: Vector2,
    bullet_speed: f32,
    shoot_cycle: Cycle,
}

impl CanShoot {
    pub fn new(bullet: Prefab, bullet_speed: f32, windup: f32, cooldown: f32) -> Self {
        CanShoot {
            bullet,
            bullet_direction: Vector2::zeros(),
            bullet_speed,
            shoot_cycle: Cycle::new(windup, cooldown),
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
                pos.get(),
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

#[derive(Debug, Clone)]
pub struct Cycle {
    phase: Phase,
    active: bool,
    windup_time: f32,
    cooldown_time: f32,
    time: f32,
}

impl Cycle {
    pub fn new(windup_time: f32, cooldown_time: f32) -> Self {
        Cycle {
            phase: Phase::Inactive,
            active: false,
            windup_time,
            cooldown_time,
            time: 0.0,
        }
    }
    pub fn get_phase_change(&mut self, dt: f32) -> Option<PhaseChange> {
        let phase_change = match self.phase {
            Phase::Inactive => {
                if self.active {
                    self.begin_phase(Phase::Windup);
                    Some(PhaseChange::BeginWindup)
                } else {
                    None
                }
            }
            Phase::Windup => {
                // if this statement is here, you can cancel out of a wind up by being inactive
                /*
                if !self.active {
                    self.begin_phase(Phase::Inactive);
                    Some(PhaseChange::CancelWindup)
                } else 
                */
                if self.time > self.windup_time {
                    self.begin_phase(Phase::Cooldown);
                    Some(PhaseChange::Trigger)
                } else {
                    None
                }
            }
            Phase::Cooldown => {
                if self.time > self.cooldown_time {
                    self.begin_phase(Phase::Inactive);
                    Some(PhaseChange::EndCooldown)
                } else {
                    None
                }
            }
        };
        if self.phase != Phase::Inactive {
            self.time += dt;
        }
        phase_change
    }
    fn begin_phase(&mut self, phase: Phase) {
        self.phase = phase;
        self.time = 0.0;
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    pub fn get_phase(&self) -> Phase {
        self.phase
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Inactive,
    Windup,
    Cooldown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseChange {
    BeginWindup,
    CancelWindup,
    Trigger,
    EndCooldown,
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
    Health, Damage;

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
