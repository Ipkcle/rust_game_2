use specs::prelude::Resources;
use assets::Assets;
use components::{collision::*, physics::*, render::*, tags::*, *};
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Vector2;
use ggez::Context;
use main_state::debug::DebugTable;
use resources::{Camera, DeltaTime};
use specs::ReadExpect;
use specs::WriteExpect;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;

pub mod input;
pub mod collision;

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        WriteExpect<'a, DebugTable>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (dt, mut debug_table, name, vel, mut pos): Self::SystemData) {
        use specs::Join;
        for (name, vel, pos) in (&name, &vel, &mut pos).join() {
            pos.add(vel.get() * dt.get());
            debug_table.load(
                name.read().to_owned(),
                format!("x: {}, y: {}", pos.x() as i32, pos.y() as i32),
            );
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

pub struct UpdateCamera {}

impl<'a> System<'a> for UpdateCamera {
    type SystemData = (
        WriteExpect<'a, Camera>,
        ReadStorage<'a, CameraFollows>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (mut camera, follows, position): Self::SystemData) {
        use specs::Join;

        for (follows, position) in (&follows, &position).join() {
            camera.set_translation(position.get());
        }
    }
}
