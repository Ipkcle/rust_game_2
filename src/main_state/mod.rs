use assets::{Assets, DrawableAsset};
use components::collision::*;
use components::physics::*;
use components::prefab::prefabs::*;
use components::render::*;
use components::combat::*;
use components::deletion_conditions::*;
use components::tags::TakesInput;
use components::Name;
use components::*;
use ggez::{
    event::*, graphics, graphics::{Point2, Vector2}, timer, Context, GameResult,
};
use resources::{Camera, DeltaTime};
use specs::world::EntityBuilder;
use specs::VecStorage;
use specs::{Component, Entities, LazyUpdate, RunNow, World};
use std::boxed::Box;
use systems::collision::*;
use systems::input::{Axis, DirectionInputScalar, Input};
use systems::*;

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
    update_vel: UpdateVel,
    handle_move_direction: HandleMoveDirection,
    update_penetrations: UpdatePenetrations,
    resolve_collisions: ResolveCollisions,
    deletion: Deletion,
}

impl GameSystems {
    pub fn new() -> Self {
        Self {
            update_pos: UpdatePos,
            update_vel: UpdateVel,
            handle_move_direction: HandleMoveDirection,
            update_penetrations: UpdatePenetrations,
            resolve_collisions: ResolveCollisions,
            deletion: Deletion,
        }
    }

    pub fn update(&mut self, world: &mut World) {
        self.update_pos.run_now(&world.res);
        self.update_vel.run_now(&world.res);
        self.handle_move_direction.run_now(&world.res);
        self.update_penetrations.run_now(&world.res);
        self.resolve_collisions.run_now(&world.res);
        self.deletion.run_now(&world.res);
    }

    pub fn draw(&mut self, ctx: &mut Context, world: &mut World) {
        Render::new(ctx).run_now(&world.res);
    }
}
pub struct MainState {
    world: World,
    game_systems: GameSystems,
    stopwatch: Stopwatch,
    input: Input,
}
impl MainState {
    pub fn new(ctx: &mut Context, _width: u32, _height: u32) -> GameResult<MainState> {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Acceleration>();
        world.register::<Name>();
        world.register::<IdentificationNumber>();
        world.register::<MoveDrag>();
        world.register::<MoveDirection>();
        world.register::<TakesInput>();
        world.register::<DrawableComponent>();
        world.register::<Collisions>();
        world.register::<Hitbox>();
        world.register::<BlocksMovement>();
        world.register::<IsBlocked>();
        world.register::<Health>();
        world.register::<Damage>();
        world.register::<MarkedForDeletion>();
        world.register::<InteractedWith>();
        world.register::<DistanceTraveled>();
        world.register::<TimeExisted>();
        world.add_resource(Assets::new(ctx));
        world.add_resource(DeltaTime::new(0.0));
        world.add_resource(Camera::new_with(
            Point2::new(100.0, 100.0),
            Point2::new(1.0, 1.0),
        ));
        world.add_resource(debug::DebugTable::new(ctx, Point2::new(0.0, 0.0)));
        player().with(&circle(50)).in_world(&mut world);
        wall().with(&rect(100, 100)).with_pos(0.0, 150.0).in_world(&mut world);
        wall().with(&circle(50)).with_pos(190.0, 150.0).in_world(&mut world);
        Ok(MainState {
            world,
            game_systems: GameSystems::new(),
            stopwatch: Stopwatch::new(),
            input: Input::new(),
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
            self.input.run_now(&mut self.world.res);
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
            Keycode::W => {
                self.input
                    .move_stack
                    .activate_direction(DirectionInputScalar::Negative, Axis::Y);
            }
            Keycode::S => {
                self.input
                    .move_stack
                    .activate_direction(DirectionInputScalar::Positive, Axis::Y);
            }
            Keycode::A => {
                self.input
                    .move_stack
                    .activate_direction(DirectionInputScalar::Negative, Axis::X);
            }
            Keycode::D => {
                self.input
                    .move_stack
                    .activate_direction(DirectionInputScalar::Positive, Axis::X);
            }
            Keycode::Up => {
                self.input
                    .shoot_stack
                    .activate_direction(DirectionInputScalar::Positive, Axis::Y);
            }
            _ => (), // Do nothing
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::W => {
                self.input
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Negative, Axis::Y);
            }
            Keycode::S => {
                self.input
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Positive, Axis::Y);
            }
            Keycode::A => {
                self.input
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Negative, Axis::X);
            }
            Keycode::D => {
                self.input
                    .move_stack
                    .deactivate_direction(DirectionInputScalar::Positive, Axis::X);
            }
            _ => (), // Do nothing
        }
    }
}
