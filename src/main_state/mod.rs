use assets::Assets;
use components::collision::*;
use components::combat::*;
use components::deletion_conditions::*;
use components::physics::*;
use components::prefab::prefabs::*;
use components::render::*;
use components::tags::*;
use components::Name;
use components::*;
use ggez::{
    event::*, graphics, graphics::{Point2, Vector2}, timer, Context, GameResult,
};
use resources::{Camera, DeltaTime};
use specs::{RunNow, World};
use systems::{
    collision::{ResolveCollisions, UpdatePenetrations}, combat::{HandleDeath, UpdateActions},
    input::{Axis, DirectionInputScalar, Player},
    physics::{HandleMoveDirection, UpdatePos, UpdateVel}, DeleteEntities, HandleNPC, Render,
    UpdateCamera,
};

pub mod debug;

use std::time::{Duration, Instant};

struct Stopwatch {
    last_mark: Instant,
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            last_mark: Instant::now(),
        }
    }

    pub fn mark(&mut self) -> Duration {
        let duration = self.since_mark();
        self.last_mark = Instant::now();
        duration
    }

    pub fn since_mark(&self) -> Duration {
        Instant::now().duration_since(self.last_mark)
    }
}

pub struct GameSystems {
    update_pos: UpdatePos,
    update_camera: UpdateCamera,
    update_vel: UpdateVel,
    handle_npc: HandleNPC,
    handle_move_direction: HandleMoveDirection,
    handle_death: HandleDeath,
    update_penetrations: UpdatePenetrations,
    resolve_collisions: ResolveCollisions,
    delete_entities: DeleteEntities,
    update_actions: UpdateActions,
}

impl GameSystems {
    pub fn new() -> Self {
        Self {
            update_pos: UpdatePos,
            update_camera: UpdateCamera,
            update_vel: UpdateVel,
            handle_npc: HandleNPC,
            handle_move_direction: HandleMoveDirection,
            handle_death: HandleDeath,
            update_penetrations: UpdatePenetrations,
            resolve_collisions: ResolveCollisions,
            delete_entities: DeleteEntities,
            update_actions: UpdateActions,
        }
    }

    pub fn update(&mut self, world: &mut World) {
        world.maintain();
        self.delete_entities.run_now(&world.res);
        self.update_pos.run_now(&world.res);
        self.update_vel.run_now(&world.res);
        self.handle_npc.run_now(&world.res);
        self.handle_move_direction.run_now(&world.res);
        self.update_actions.run_now(&world.res);
        self.update_penetrations.run_now(&world.res);
        self.resolve_collisions.run_now(&world.res);
        self.handle_death.run_now(&world.res);
        self.update_camera.run_now(&world.res);
    }

    pub fn draw(&mut self, ctx: &mut Context, world: &mut World) {
        Render::new(ctx).run_now(&world.res);
    }
}
pub struct MainState {
    world: World,
    game_systems: GameSystems,
    stopwatch: Stopwatch,
    player: Player,
}
impl MainState {
    pub fn new(ctx: &mut Context, _width: u32, _height: u32) -> GameResult<MainState> {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Acceleration>();
        world.register::<Name>();
        world.register::<MoveDrag>();
        world.register::<MoveDirection>();
        world.register::<IsPlayer>();
        world.register::<CameraFollows>();
        world.register::<DrawableComponent>();
        world.register::<Collisions>();
        world.register::<Hitbox>();
        world.register::<AABB>();
        world.register::<BlocksMovement>();
        world.register::<IsBlocked>();
        world.register::<Stamina>();
        world.register::<Health>();
        world.register::<Damage>();
        world.register::<MarkedForDeletion>();
        world.register::<InteractedWith>();
        world.register::<DistanceTraveled>();
        world.register::<TimeExisted>();
        world.register::<ShootData>();
        world.register::<DodgeData>();
        world.register::<CollideEffects>();
        world.register::<RecievesCollideEffects>();
        world.register::<Knockback>();
        world.register::<Push>();
        world.register::<AI>();
        world.add_resource(Assets::new(ctx));
        world.add_resource(DeltaTime::new(0.0));
        world.add_resource(Camera::new_with(
            Vector2::new(850.0, 450.0),
            Point2::new(100.0, 100.0),
            Point2::new(1.0, 1.0),
        ));
        world.add_resource(debug::DebugTable::new(ctx, Point2::new(0.0, 0.0)));
        player().in_world(&mut world);
        dummy()
            .with_pos(Position::new(150.0, 70.0))
            .in_world(&mut world);
        enemy()
            .with_pos(Position::new(50.0, 70.0))
            .in_world(&mut world);
        wall()
            .with(&rect(100, 100))
            .with_pos(Position::new(150.0, 150.0))
            .in_world(&mut world);
        wall()
            .with(&circle(50))
            .with_pos(Position::new(175.0, 150.0))
            .in_world(&mut world);
        Ok(MainState {
            world,
            game_systems: GameSystems::new(),
            stopwatch: Stopwatch::new(),
            player: Player::new(),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            {
                //update the delta time
                let mut delta = self.world.write_resource::<DeltaTime>();
                delta.set(timer::duration_to_f64(self.stopwatch.mark()) as f32);
            }
            self.player.run_now(&mut self.world.res);
            self.game_systems.update(&mut self.world);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //clear the contex
        graphics::clear(ctx);

        //draw game objects
        self.game_systems.draw(ctx, &mut self.world);

        //show context on screen
        graphics::present(ctx);

        //yeild cpu when not active
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            Keycode::Space => {
                self.player.dodge = true;
            }
            Keycode::W => {
                self.player
                    .move_stack
                    .activate_direction(DirectionInputScalar::Negative, Axis::Y);
            }
            Keycode::S => {
                self.player
                    .move_stack
                    .activate_direction(DirectionInputScalar::Positive, Axis::Y);
            }
            Keycode::A => {
                self.player
                    .move_stack
                    .activate_direction(DirectionInputScalar::Negative, Axis::X);
            }
            Keycode::D => {
                self.player
                    .move_stack
                    .activate_direction(DirectionInputScalar::Positive, Axis::X);
            }
            Keycode::Up => {
                self.player
                    .shoot_stack
                    .activate_direction(DirectionInputScalar::Negative, Axis::Y);
            }
            Keycode::Down => {
                self.player
                    .shoot_stack
                    .activate_direction(DirectionInputScalar::Positive, Axis::Y);
            }
            Keycode::Left => {
                self.player
                    .shoot_stack
                    .activate_direction(DirectionInputScalar::Negative, Axis::X);
            }
            Keycode::Right => {
                self.player
                    .shoot_stack
                    .activate_direction(DirectionInputScalar::Positive, Axis::X);
            }
            _ => (), // Do nothing
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::W => {
                self.player
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Negative, Axis::Y);
            }
            Keycode::S => {
                self.player
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Positive, Axis::Y);
            }
            Keycode::A => {
                self.player
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Negative, Axis::X);
            }
            Keycode::D => {
                self.player
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Positive, Axis::X);
            }
            Keycode::Up => {
                self.player
                    .shoot_stack
                    .deactivate_direction(DirectionInputScalar::Negative, Axis::Y);
            }
            Keycode::Down => {
                self.player
                    .shoot_stack
                    .deactivate_direction(DirectionInputScalar::Positive, Axis::Y);
            }
            Keycode::Left => {
                self.player
                    .shoot_stack
                    .deactivate_direction(DirectionInputScalar::Negative, Axis::X);
            }
            Keycode::Right => {
                self.player
                    .shoot_stack
                    .deactivate_direction(DirectionInputScalar::Positive, Axis::X);
            }
            _ => (), // Do nothing
        }
    }
}
