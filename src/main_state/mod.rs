use std::boxed::Box;
use components::Name;
use components::physics::*;
use components::render::*;
use components::collision::*;
use components::tags::TakesInput;
use ggez::{graphics, timer, Context, GameResult, event::*, graphics::{Point2, Vector2}};
use resources::{DeltaTime, Camera};
use specs::{RunNow, World, Component, Entities, LazyUpdate};
use specs::VecStorage;
use specs::world::EntityBuilder;
use systems::input::{Axis, DirectionInputScalar, Input};
use systems::*;
use systems::collision::*;
use assets::{Assets, DrawableAsset};

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
}

impl GameSystems {
    pub fn new() -> Self {
        Self {
            update_pos: UpdatePos,
            update_vel: UpdateVel,
            handle_move_direction: HandleMoveDirection,
            update_penetrations: UpdatePenetrations,
            resolve_collisions: ResolveCollisions,
        }
    }

    pub fn update(&mut self, world: &mut World) {
        self.update_pos.run_now(&world.res);
        self.update_vel.run_now(&world.res);
        self.handle_move_direction.run_now(&world.res);
        self.update_penetrations.run_now(&world.res);
        self.resolve_collisions.run_now(&world.res);
    }

    pub fn draw(&mut self, ctx: &mut Context, world: &mut World) {
        Render::new(ctx).run_now(&world.res);
    }
}

pub fn player_circle(world: &mut World) {
    let accel = 2500.0;
    let max_speed = 250.0;
    let drag_constant = accel / max_speed;
    world
        .create_entity()
        .with(TakesInput)
        .with(MoveDrag::new(drag_constant))
        .with(MoveDirection::new(accel))
        .with(Position::zeros())
        .with(Velocity::zeros())
        .with(Acceleration::new(0.0, 0.0))
        .with(DrawableComponent::new(DrawableAsset::Player))
        .with(Collisions::new())
        .with(Hitbox::Circle {
            radius: 10.0,
        })
        .with(IsBlocked)
        .with(Name::new("player".to_owned()))
        .build();
}
pub fn player_circle_big(world: &mut World, radius: u32) {
    let accel = 2500.0;
    let max_speed = 250.0;
    let drag_constant = accel / max_speed;
    world
        .create_entity()
        .with(TakesInput)
        .with(MoveDrag::new(drag_constant))
        .with(MoveDirection::new(accel))
        .with(Position::zeros())
        .with(Velocity::zeros())
        .with(Acceleration::new(0.0, 0.0))
        .with(DrawableComponent::new(DrawableAsset::circle(radius)))
        .with(Collisions::new())
        .with(Hitbox::Circle {
            radius: radius as f32,
        })
        .with(IsBlocked)
        .with(Name::new("player".to_owned()))
        .build();
}
pub fn player_square(world: &mut World) {
    let accel = 2500.0;
    let max_speed = 250.0;
    let drag_constant = accel / max_speed;
    world
        .create_entity()
        .with(TakesInput)
        .with(MoveDrag::new(drag_constant))
        .with(MoveDirection::new(accel))
        .with(Position::zeros())
        .with(Velocity::zeros())
        .with(Acceleration::new(0.0, 0.0))
        .with(DrawableComponent::new(DrawableAsset::Block))
        .with(Collisions::new())
        .with(Hitbox::Rectangle {
            dimensions: Vector2::new(20.0, 20.0),
            angle: 0.0,
        })
        .with(IsBlocked)
        .with(Name::new("player".to_owned()))
        .build();
}
pub fn ball(world: &mut World, x: f32, y: f32, radius: u32) {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(DrawableComponent::new(DrawableAsset::circle(radius)))
        .with(Hitbox::Circle {
            radius: radius as f32,
        })
        .with(BlocksMovement)
        .with(Name::new("Block".to_owned()))
        .build();
}
pub fn wall(world: &mut World, x: f32, y: f32, w: u32, h: u32) {
    world
        .create_entity()
        .with(Position::new(x, y))
        .with(DrawableComponent::new(DrawableAsset::rect(w, h)))
        .with(Hitbox::Rectangle {
            dimensions: Vector2::new(w as f32, h as f32),
            angle: 0.0,
        })
        .with(BlocksMovement)
        .with(Name::new("Block".to_owned()))
        .build();
}

pub struct MainState { world: World, game_systems: GameSystems, stopwatch: Stopwatch, input: Input, } impl MainState {
    pub fn new(ctx: &mut Context, _width: u32, _height: u32) -> GameResult<MainState> {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Acceleration>();
        world.register::<Name>();
        world.register::<MoveDrag>();
        world.register::<MoveDirection>();
        world.register::<TakesInput>();
        world.register::<DrawableComponent>();
        world.register::<Collisions>();
        world.register::<Hitbox>();
        world.register::<BlocksMovement>();
        world.register::<IsBlocked>();
        world.add_resource(Assets::new(ctx));
        world.add_resource(DeltaTime::new(0.0));
        world.add_resource(Camera::new_with(Point2::new(100.0, 100.0), Point2::new(1.0, 1.0)));
        world.add_resource(debug::DebugTable::new(ctx, Point2::new(0.0, 0.0)));
        player_circle_big(&mut world, 50);
        wall(&mut world, 100.0, 100.0, 100, 100);
        ball(&mut world, 290.0, 100.0, 50);
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

const max_prefab_components: u32 = 1000;

pub struct Prefab {
    data: String,
    number_of_components: u32,
}

impl Prefab {
    fn create_component(&self, number: u32) -> Option<impl Component> {
        if number > self.number_of_components {
            return None
        }
        None
    }

    pub fn generate_in_world(&self, world: &mut World) {
        let entity_builder = world.create_entity();
        let i = 0;
        while let Some(component) = self.create_component(i) {
            if i >= max_prefab_components {
                break
            }
            entity_builder.with(component);
            i += 1;
        };
        entity_builder.build();
    }

    pub fn lazy_generate_in_entities(&self, entities: &Entities, updater: &LazyUpdate) {
        let new_entity = entities.create();
        let i = 0;
        while let Some(component) = self.create_component(i) {
            if i >= max_prefab_components {
                break
            }
            updater.insert(new_entity, component);
            i += 1;
        };
    }
}
