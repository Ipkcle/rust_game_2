use components::{deletion_conditions::MarkedForDeletion, combat::*, physics::*};
use resources::DeltaTime;
use specs::Entities;
use specs::LazyUpdate;
use specs::Read;
use specs::ReadExpect;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;

pub struct ShootBullets;

impl<'a> System<'a> for ShootBullets {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, CanShoot>,
    );

    fn run(&mut self, (dt, entities, updater, position, mut can_shoot): Self::SystemData) {
        use specs::Join;

        for (entity, pos, can_shoot) in (&*entities, &position, &mut can_shoot).join() {
            can_shoot.update(pos, dt.get(), &entities, &updater);
        }
    }
}

pub struct HandleDeath;

impl<'a> System<'a> for HandleDeath {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Health>,
    );

    fn run(&mut self, (entities, updater, health): Self::SystemData) {
        use specs::Join;

        for (entity, health) in (&*entities, &health).join() {

            if health.get() <= 0 {
                updater.insert(entity, MarkedForDeletion);
            }
        }
    }
}
