use ggez::graphics::Vector2;
use specs::DenseVecStorage;
use specs::VecStorage;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Position {
    vec: Vector2,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Velocity {
    vec: Vector2,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Knockback {
    pub velocity: Velocity,
    pub from: Position
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Acceleration {
    vec: Vector2,
}

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct MoveDrag {
    drag_constant: f32,
}

impl MoveDrag {
    pub fn new(drag_constant: f32) -> Self {
        Self { drag_constant }
    }

    pub fn get_constant(&self) -> f32 {
        self.drag_constant
    }
}

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct MoveDirection {
    direction: Vector2,
    move_acceleration: f32,
}

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

        pub fn set(&mut self, vec: Vector2) {
            self.vec = vec;
        }

        pub fn get(&self) -> Vector2 {
            self.vec.clone()
        }

        pub fn x(&self) -> f32 {
            self.vec.x
        }

        pub fn y(&self) -> f32 {
            self.vec.y
        }
    }

    impl Add {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            let vec = self.vec + other.vec;
            Self::new(vec.x, vec.y)
        }
    }

    impl AddAssign {
        fn add_assign(&mut self, other: Self) {
            self.vec += other.vec;
        }
    }

    impl Sub {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            let vec = self.vec - other.vec;
            Self::new(vec.x, vec.y)
        }
    }

    impl SubAssign {
        fn sub_assign(&mut self, other: Self) {
            self.vec -= other.vec;
        }
    }
}

for_impl! {
    Position, Velocity, Acceleration;

    impl {
        pub fn add(&mut self, vec: Vector2) {
            self.vec += vec;
        }
        pub fn plus(&self, vec: Vector2) -> Self {
            let vec = self.clone().vec + vec;
            Self::new(vec.x, vec.y)
        }
    }
}
