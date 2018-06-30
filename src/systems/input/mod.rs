use utils::cycle::*;
use components::combat::{DodgeData, ShootData,  Stamina};
use components::physics::{MoveDirection, MoveDrag, Position, Velocity};
use components::tags::IsPlayer;
use resources::DeltaTime;
use ggez::graphics::Vector2;
use specs::Entities;
use specs::Entity;
use specs::LazyUpdate;
use specs::ReadStorage;
use specs::Read;
use specs::ReadExpect;
use specs::System;
use specs::WriteStorage;
use components::prefab::PrefabComponent;
#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DirectionInputScalar {
    Positive,
    Negative,
}

impl DirectionInputScalar {
    pub fn get_value(&self) -> f32 {
        match *self {
            DirectionInputScalar::Positive => 1.0,
            DirectionInputScalar::Negative => -1.0,
        }
    }
}

// DirectionInputStack struct
pub struct DirectionInputStack {
    x_input_stack: Vec<DirectionInputScalar>,
    y_input_stack: Vec<DirectionInputScalar>,
}

impl DirectionInputStack {
    pub fn new() -> Self {
        Self {
            x_input_stack: Vec::new(),
            y_input_stack: Vec::new(),
        }
    }

    fn get_input_stack(&mut self, axis: Axis) -> &mut Vec<DirectionInputScalar> {
        match axis {
            Axis::X => &mut self.x_input_stack,
            Axis::Y => &mut self.y_input_stack,
        }
    }

    pub fn get_direction_old(&self) -> Vector2 {
        let mut x_vec = Vector2::zeros();
        let mut y_vec = Vector2::zeros();
        if let Some(x_magnitude) = self.x_input_stack.first() {
            x_vec = Vector2::new(x_magnitude.get_value(), 0.0);
        }
        if let Some(y_magnitude) = self.y_input_stack.first() {
            y_vec = Vector2::new(0.0, y_magnitude.get_value());
        }
        (x_vec + y_vec)
    }

    pub fn get_direction_recent(&self) -> Vector2 {
        let mut x_vec = Vector2::zeros();
        let mut y_vec = Vector2::zeros();
        if let Some(x_magnitude) = self.x_input_stack.last() {
            x_vec = Vector2::new(x_magnitude.get_value(), 0.0);
        }
        if let Some(y_magnitude) = self.y_input_stack.last() {
            y_vec = Vector2::new(0.0, y_magnitude.get_value());
        }
        (x_vec + y_vec)
    }

    pub fn is_active(&self) -> bool {
        !(self.x_input_stack.is_empty() && self.y_input_stack.is_empty())
    }

    pub fn deactivate_direction(&mut self, direction: DirectionInputScalar, axis: Axis) {
        self.get_input_stack(axis)
            .retain(|element| *element != direction);
    }

    pub fn activate_direction(&mut self, direction: DirectionInputScalar, axis: Axis) {
        if !self.get_input_stack(axis).contains(&direction) {
            self.get_input_stack(axis).push(direction);
        }
    }
}

enum Action {
    None,
    Dodge,
    Shoot,
}

//Player struct
pub struct Player {
    pub move_stack: DirectionInputStack,
    pub shoot_stack: DirectionInputStack,
    action: Action,
    pub dodge: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            move_stack: DirectionInputStack::new(),
            shoot_stack: DirectionInputStack::new(),
            action: Action::None,
            dodge: false,
        }
    }
}

impl<'a> System<'a> for Player {
    type SystemData = (
        ReadStorage<'a, IsPlayer>,
        WriteStorage<'a, Stamina>,
        WriteStorage<'a, MoveDirection>,
        WriteStorage<'a, ShootData>,
        WriteStorage<'a, DodgeData>,
    );

    fn run(&mut self, (takes_input, mut stamina, mut move_direction, mut shoot_data, mut dodge_data): Self::SystemData) {
        use specs::Join;
        for (_takes_input, stamina, move_direction, shoot_data, dodge_data) in
            (&takes_input, &mut stamina, &mut move_direction, &mut shoot_data, &mut dodge_data).join()
        {
            let shoot = self.shoot_stack.is_active();
            match self.action {
                Action::Dodge => {
                    if dodge_data.get_phase() == Phase::Inactive {
                        self.action = Action::None;
                    }
                }
                Action::Shoot => {
                    if shoot_data.get_phase() == Phase::Inactive {
                        self.action = Action::None;
                    }
                }
                _ => {}
            }
            if let Action::None = self.action {
                if self.dodge {
                    self.dodge = false;
                    let direction = self.move_stack.get_direction_recent();
                    if direction != Vector2::zeros() {
                        self.action = Action::Dodge;
                        dodge_data.set_direction(direction);
                        dodge_data.set_dodging(true);
                        shoot_data.set_shooting(false);
                        move_direction.set(Vector2::zeros());
                    }
                } else if shoot {
                    self.action = Action::Shoot;
                    shoot_data.set_direction(self.shoot_stack.get_direction_recent());
                    shoot_data.set_shooting(true);
                    dodge_data.set_dodging(false);
                    move_direction.set(Vector2::zeros());
                } else {
                    shoot_data.set_shooting(false);
                    dodge_data.set_dodging(false);
                    move_direction.set(self.move_stack.get_direction_recent());
                }
            }
        }
    }
}
