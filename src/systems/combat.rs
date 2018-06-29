use components::{deletion_conditions::MarkedForDeletion, combat::*, physics::*, tags::IsPlayer};
use main_state::debug::DebugTable;
use resources::DeltaTime;
use specs::Entities;
use specs::LazyUpdate;
use specs::Read;
use specs::ReadExpect;
use specs::WriteExpect;
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
        WriteExpect<'a, DebugTable>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, IsPlayer>,
        ReadStorage<'a, Health>,
    );

    fn run(&mut self, (entities, mut table, updater, is_player, health): Self::SystemData) {
        use specs::Join;

        for (entity, health) in (&*entities, &health).join() {
            if let Some(IsPlayer) = is_player.get(entity) {
                table.load("player health".to_string(), health.get().to_string());
            }
            if health.get() <= 0 {
                updater.insert(entity, MarkedForDeletion);
            }
        }
    }
}
