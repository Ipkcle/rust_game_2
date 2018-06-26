use assets::Assets;
use components::{
    collision::AABB, combat::*,
    deletion_conditions::*, physics::*, render::*, tags::*, *,
};
use ggez::{Context};
use main_state::debug::DebugTable;
use resources::{Camera, DeltaTime};
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::WriteExpect;
use specs::WriteStorage;

pub struct UpdatePos;

impl UpdatePos {
    fn update_get_distance(pos: &mut Position, vel: &Velocity, dt: &DeltaTime) -> f32 {
        let mut distance = 0.0;
        let displacement = vel.get() * dt.get();
        pos.add(displacement);
        distance += displacement.norm();
        distance
    }

    fn update(pos: &mut Position, vel: &Velocity, dt: &DeltaTime) {
        pos.add(vel.get() * dt.get());
    }

    fn print_pos(pos: &Position, name: &Name, debug_table: &mut DebugTable) {
        debug_table.load(
            name.read().to_owned(),
            format!("x: {}, y: {}", pos.x() as i32, pos.y() as i32),
        );
    }
}

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        WriteExpect<'a, DebugTable>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, DistanceTraveled>,
    );

    fn run(&mut self, (dt, mut debug_table, vel, mut pos, mut dist): Self::SystemData) {
        use specs::Join;
        for (vel, pos, _) in (&vel, &mut pos, !&dist).join() {
            Self::update(pos, vel, &*dt);
        }
        for (vel, pos, dist) in (&vel, &mut pos, &mut dist).join() {
            dist.add(Self::update_get_distance(pos, vel, &*dt));
        }
    }
}

pub struct UpdateVel;

impl<'a> System<'a> for UpdateVel {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        ReadStorage<'a, Acceleration>,
        ReadStorage<'a, MoveDrag>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (dt, acc, drag, mut vel): Self::SystemData) {
        use specs::Join;
        for (acc, vel) in (&acc, &mut vel).join() {
            vel.add(acc.get() * dt.get());
        }

        for (drag, vel) in (&drag, &mut vel).join() {
            let v_old = vel.get();
            vel.add(-1.0 * drag.get_constant() * v_old * dt.get());
        }
    }
}

pub struct HandleMoveDirection;

impl<'a> System<'a> for HandleMoveDirection {
    type SystemData = (
        ReadStorage<'a, MoveDirection>,
        WriteStorage<'a, Acceleration>,
    );

    fn run(&mut self, (dir, mut acc): Self::SystemData) {
        use specs::Join;
        for (dir, acc) in (&dir, &mut acc).join() {
            let magnitude = dir.get_move_acceleration();
            acc.set(dir.get() * magnitude);
        }
    }
}
