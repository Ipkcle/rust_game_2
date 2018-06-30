use assets::Assets;
use components::{
    collision::AABB, combat::*,
    deletion_conditions::*, physics::*, render::*, tags::*, *,
};
use ggez::{graphics::Vector2, Context};
use main_state::debug::DebugTable;
use utils::cycle::Phase;
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
        Entities<'a>,
        ReadStorage<'a, AABB>,
        WriteExpect<'a, Camera>,
        ReadStorage<'a, CameraFollows>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (entities, aabb, mut camera, follows, position): Self::SystemData) {
        use specs::Join;

        for (entity, position, _) in (&*entities, &position, &follows).join() {
            let center = if let Some(aabb) = aabb.get(entity) {
                position.get() + aabb.get_center()
            } else {
                position.get()
            };
            camera.set_center(center);
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

pub struct HandleNPC;

impl<'a> System<'a> for HandleNPC {
    type SystemData = (ReadStorage<'a, IsPlayer>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, AI>,
                       WriteStorage<'a, MoveDirection>,
                       WriteStorage<'a, ShootData>,);

    fn run(&mut self, (is_player, position, ai, mut move_direction, mut shoots): Self::SystemData) {
        use specs::Join;
        let mut player_position = Position::zeros();
        for (_is_player, position) in (&is_player, &position).join() {
            player_position = position.clone();
        }
        for (_ai, position, move_direction, shoots) in (&ai, &position, &mut move_direction, &mut shoots).join() {
            let displacement = player_position - *position;
            let distance = displacement.get().norm();
            if distance < 130.0 {
                move_direction.set(Vector2::zeros());
                shoots.set_direction(displacement.get());
                shoots.set_shooting(true);
            } else {
                if shoots.get_phase() == Phase::Inactive {
                    move_direction.set(displacement.get());
                } else {
                    move_direction.set(Vector2::zeros());
                }
                shoots.set_shooting(false);
            }
        }
        /*
            shoots.set_shooting(self.shoot_stack.is_active());
            shoots.set_direction(self.shoot_stack.get_direction_recent());
            if shoots.get_phase() == Phase::Inactive {
                move_direction.set(self.move_stack.get_direction_recent());
            } else {
                move_direction.set(Vector2::zeros());
            }
        */
    }
}
