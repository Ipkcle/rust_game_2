use std::mem::discriminant;
use ggez::graphics::Vector2;
use specs::DenseVecStorage;
use utils::State;

#[derive(Component, Debug, Clone, PartialEq)]
#[storage(DenseVecStorage)]
pub struct Action {
    action_type: ActionType,
    action_performed: bool,
}

impl Action {
    pub fn new() -> Action {
        Action {
            action_type: ActionType::NoAction,
            action_performed: true,
        }
    }

    pub fn get_action_type(&self) -> ActionType {
        self.action_type.clone()
    }

    pub fn is_action_preformed(&self) -> bool {
        self.action_performed
    }

    pub fn set_action_preformed(&mut self, action_performed: bool) {
        self.action_performed = action_performed;
    }
}

impl State for Action {
    type Input = Input;

    fn take_input(&mut self, input: Input) {
        let mut next_action = None;
        match &self.action_type {
            &ActionType::NoAction => {
                if let InputType::Action {
                    action,
                    start_or_stop: StartOrStop::Start,
                } = input.input_type {
                    next_action = Some(action);
                    self.action_performed = false;
                }
            }
            current_action => {
                if let InputType::Action {
                    action: input_action,
                    start_or_stop: StartOrStop::Stop,
                } = input.input_type {
                    if discriminant(current_action) == discriminant(&input_action) {
                        next_action = Some(ActionType::NoAction);
                    }
                }
            }
        }
        if let Some(action) = next_action {
            self.action_type = action;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    NoAction,
    Move(Vector2),
    Shoot(Vector2),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Input {
    pub input_type: InputType,
}

impl Input {
    fn new(input_type: InputType) -> Self {
        Self {
            input_type,
        }
    }

    pub fn no_input() -> Self {
        Self::new(InputType::NoInput)
    }

    pub fn action(action_type: ActionType, start_or_stop: StartOrStop) -> Self {
        Self::new(InputType::NoInput)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    NoInput,
    Action {
        action: ActionType,
        start_or_stop: StartOrStop,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StartOrStop {
    Start,
    Stop,
}
