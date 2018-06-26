use components::{combat::*, physics::*};
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
            /*
            // if the entity has an action component
            if let Some(action) = action.get_mut(entity) {
                // if the current action is a shoot action
                if let ActionType::Shoot(direction) = action.get_action_type() {
                    // if the shoot action has yet to be performed
                    if !action.is_action_preformed() {
                        can_shoot.set_shooting(true);
                        can_shoot.set_direction(direction);
                        action.set_action_preformed(true);
                    }
                    // if the cooldown on the shoot ends
                    let phase_change = can_shoot.update(pos, dt.get(), &entities, &updater);
                    if let Some(PhaseChange::EndCooldown) = phase_change {
                        // give the action component an input to stop shooting
                        action.take_input(Input::action(
                            ActionType::Shoot(Vector2::zeros()),
                            StartOrStop::Stop,
                        ));
                    }
                }
            } else {
                // just update can_shoot
            }
            */
            can_shoot.update(pos, dt.get(), &entities, &updater);
        }
    }
}
