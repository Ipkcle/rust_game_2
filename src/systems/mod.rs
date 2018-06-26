use assets::Assets;
use components::{
    collision::AABB, combat::*,
    deletion_conditions::*, physics::*, render::*, tags::*, *,
};
use ggez::{graphics::Vector2, Context};
use main_state::debug::DebugTable;
use resources::{Camera, DeltaTime};
use specs::Entities;
use specs::LazyUpdate;
use specs::Read;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::WriteExpect;
use specs::WriteStorage;
use utils::State;

pub mod collision;
pub mod input;
pub mod combat;
pub mod physics;


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
        ReadStorage<'a, AABB>,
    );

    fn run(&mut self, (mut table, camera, mut assets, drawable, position, aabb): Self::SystemData) {
        use specs::Join;

        table.render(self.context);
        for (drawable, position, _) in (&drawable, &position, !&aabb).join() {
            drawable.render(
                self.context,
                &mut assets,
                position,
                camera.get_draw_parameters(),
            );
        }
        for (drawable, position, aabb) in (&drawable, &position, &aabb).join() {
            drawable.render(
                self.context,
                &mut assets,
                &position.plus(-1.0 * aabb.get_center()),
                camera.get_draw_parameters(),
            );
        }
    }
}
