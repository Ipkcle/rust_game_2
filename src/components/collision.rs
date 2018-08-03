use std::mem::swap;
use specs::VecStorage;
use specs::NullStorage;
use ggez::graphics::Vector2;

pub fn get_circle_center(position: Vector2, radius: f32) -> Vector2 {
    Vector2::new(position.x + radius, position.y + radius)
}

pub fn get_rectangle_center(position: Vector2, dimensions: Vector2) -> Vector2 {
    position + (dimensions / 2.0)
}


#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct AABB(Vector2);

impl AABB {
    pub fn new(vec: Vector2) -> AABB {
        AABB(vec)
    }

    pub fn get(&self) -> Vector2 {
        self.0.clone()
    }

    pub fn get_center(&self) -> Vector2 {
        self.get() * 0.5
    }
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub enum Hitbox {
    Circle { radius: f32 },
    LineSegment { length: f32, angle: f32 },
    Rectangle { dimensions: Vector2, angle: f32 },
}

impl Hitbox {
    pub fn center(&self) ->  Vector2 {
        match self {
            Hitbox::Circle {
                radius
            } => {
                Vector2::new(*radius, *radius)
            },
            Hitbox::LineSegment {
                ..
            } => {
                Vector2::zeros()
            }
            Hitbox::Rectangle {
                dimensions,
                ..
            } => {
                dimensions / 2.0
            }
        }
    }
    pub fn center_from(&self, position: Vector2) -> Vector2 {
        position + self.center()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CollisionType {
    Stop,
}

#[derive(Debug, Clone)]
pub struct Collision {
    penetration: Vector2,
    collision_type: CollisionType,
}

impl Collision {
    pub fn new(penetration: Vector2, collision_type: CollisionType) -> Self {
        Collision {
            penetration,
            collision_type,
        }
    }

    pub fn is_type(&self, collision_type: CollisionType) -> bool {
        self.collision_type == collision_type
    }

    pub fn get_penetration(&self) -> Vector2 {
        self.penetration
    }
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Collisions {
    collisions: Vec<Collision>,   
}

impl Collisions {
    pub fn new() -> Self {
        Collisions {
            collisions: Vec::new(),
        }
    }

    pub fn recieve_collision(&mut self, collision: Collision) {
        self.collisions.push(collision);
    }

    pub fn get_net_vector(&self, collision_type: CollisionType) -> Vector2 {
        let mut return_vector = Vector2::zeros();
        for collision in self.collisions.iter() {
            if collision.is_type(collision_type) {
                return_vector += collision.get_penetration();
            }
        }
        return_vector
    }

    pub fn clear(&mut self) {
        self.collisions = Vec::new();
    }

    pub fn clear_return(&mut self) -> Vec<Collision> {
        let mut new_vec = Vec::new();
        swap(&mut new_vec, &mut self.collisions);
        new_vec
    }
}

#[derive(Default, Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct BlocksMovement;

#[derive(Default, Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct IsBlocked;
