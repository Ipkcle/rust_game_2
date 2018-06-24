use assets::Assets;
use components::{collision::AABB, physics::*, render::*, tags::*, deletion_conditions::*, combat::*, *};
use ggez::Context;
use main_state::debug::DebugTable;
use resources::{Camera, DeltaTime};
use specs::ReadExpect;
use specs::Read;
use specs::WriteExpect;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;
use specs::Entities;
use specs::LazyUpdate;

pub mod input;
pub mod collision;

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

pub struct Render<'c> {
    context: &'c mut Context,
}

impl<'c> Render<'c> {
    pub fn new(context: &'c mut Context) -> Self {
        Self { context }
    }
}

impl<'a, 'c> System<'a> for Render<'c> {
    type SystemData = (
        WriteExpect<'a, DebugTable>,
        ReadExpect<'a, Camera>,
        WriteExpect<'a, Assets>,
        ReadStorage<'a, DrawableComponent>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (mut table, camera, mut assets, drawable, position): Self::SystemData) {
        use specs::Join;

        table.render(self.context);
        for (drawable, position) in (&drawable, &position).join() {
            drawable.render(
                self.context,
                &mut assets,
                position,
                camera.get_draw_parameters(),
            );
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

pub struct UpdateCamera;

impl<'a> System<'a> for UpdateCamera {
    type SystemData = (
        WriteExpect<'a, Camera>,
        ReadStorage<'a, CameraFollows>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (mut camera, follows, position): Self::SystemData) {
        use specs::Join;

        for (_follows, position) in (&follows, &position).join() {
            camera.set_translation(position.get());
        }
    }
}

pub struct DeleteEntities;

impl<'a> System<'a> for DeleteEntities {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        Entities<'a>,
        ReadStorage<'a, MarkedForDeletion>,
        ReadStorage<'a, InteractedWith>,
        ReadStorage<'a, DistanceTraveled>,
        WriteStorage<'a, TimeExisted>,
    );

    fn run(&mut self, (dt, entities, marked, interacted, distance, mut time): Self::SystemData) {
        use specs::Join;

        for (entity, _marked) in (&*entities, &marked).join() {
            let _ = entities.delete(entity);
        }
        for (entity, interacted) in (&*entities, &interacted).join() {
            if interacted.should_delete() {
                let _ = entities.delete(entity);
            }
        }
        for (entity, distance) in (&*entities, &distance).join() {
            if distance.should_delete() {
                let _ = entities.delete(entity);
            }
        }
        for (entity, time) in (&*entities, &mut time).join() {
            time.add(dt.get());
            if time.should_delete() {
                let _ = entities.delete(entity);
            }
        }
    }
}

pub struct ShootBullets;

impl<'a> System<'a> for ShootBullets {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, AABB>,
        WriteStorage<'a, CanShoot>,
    );

    fn run(&mut self, (dt, entities, updater, position, aabb, mut can_shoot): Self::SystemData) {
        use specs::Join;

        for (pos, aabb, can_shoot) in (&position, &aabb, &mut can_shoot).join() {
            can_shoot.update(pos, aabb, dt.get(), &entities, &updater);
        }
    }
}
