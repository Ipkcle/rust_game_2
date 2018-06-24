use specs::VecStorage;
use specs::DenseVecStorage;
use ggez::graphics::Vector2;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Position{vec: Vector2}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Velocity{vec: Vector2}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Acceleration{vec: Vector2}

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct MoveDrag {
    drag_constant: f32
}

impl MoveDrag {
    pub fn new(drag_constant: f32) -> Self {
        Self {
            drag_constant,
        }
    }

    pub fn get_constant(&self) -> f32 {
        self.drag_constant
    }
}

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct MoveDirection{direction: Vector2, move_acceleration: f32}

impl MoveDirection {
    pub fn new(move_acceleration: f32) -> Self {
        Self {
            direction: Vector2::zeros(),
            move_acceleration,
        }
    }
    pub fn get(&self) -> Vector2 {
        if self.direction != Vector2::zeros() {
            self.direction.clone().normalize()
        } else {
            self.direction.clone()
        }
    }
    pub fn get_move_acceleration(&self) -> f32 {
        self.move_acceleration
    }
    pub fn set(&mut self, vec: Vector2) {
        self.direction = vec;
    }
    pub fn x(&self) -> f32 {
        self.direction.x
    }
    pub fn y(&self) -> f32 {
        self.direction.y
    }
    pub fn add(&mut self, vec: Vector2) {
        self.direction += vec;
    }
}

for_impl! {
    Position, Velocity, Acceleration;

    impl {
        pub fn new(x: f32, y: f32) -> Self {
            Self {
                vec: Vector2::new(x, y),
            }
        }
        pub fn zeros() -> Self {
            Self {
                vec: Vector2::zeros(),
            }
        }
    }
}

for_impl! {
    Position, Velocity, Acceleration;

    impl {
        pub fn set(&mut self, vec: Vector2) {
            self.vec = vec;
        }
    }
}

for_impl! {
    Position, Velocity, Acceleration;

    impl {
        pub fn x(&self) -> f32 {
            self.vec.x
        }
        pub fn y(&self) -> f32 {
            self.vec.y
        }
        pub fn get(&self) -> Vector2 {
            self.vec.clone()
        }
        pub fn add(&mut self, vec: Vector2) {
            self.vec += vec;
        }
    }
}
