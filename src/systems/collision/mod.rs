use components::{collision::*, physics::*};
use ggez::graphics::Vector2;
use specs::ReadStorage;
use specs::System;
use specs::WriteStorage;

pub struct UpdatePenetrations;

impl<'a> System<'a> for UpdatePenetrations {
    type SystemData = (
        ReadStorage<'a, Hitbox>,
        ReadStorage<'a, BlocksMovement>,
        ReadStorage<'a, IsBlocked>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Collisions>,
    );

    fn run(
        &mut self,
        (hitbox, blocks_movement, is_blocked, position, mut collisions): Self::SystemData,
    ) {
        use specs::Join;

        for (hitbox_1, position_1, collisions, _) in
            (&hitbox, &position, &mut collisions, &is_blocked).join()
        {
            for (hitbox_2, position_2, _) in (&hitbox, &position, &blocks_movement).join() {
                collisions.recieve_collision(Collision::new(
                    find_penetration(
                        hitbox_1,
                        hitbox_2,
                        position_1.get(),
                        position_2.get(),
                    ),
                    CollisionType::Stop,
                ));
            }
        }
    }
}

pub struct ResolveCollisions;

impl<'a> System<'a> for ResolveCollisions {
    type SystemData = (
        WriteStorage<'a, Collisions>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut collisions, mut position, mut velocity): Self::SystemData) {
        use specs::Join;

        for (collisions, position, _) in (&mut collisions, &mut position, !&velocity).join() {
            position.add(-1.0 * collisions.get_net_vector(CollisionType::Stop));
            collisions.clear();
        }
        for (collisions, position, _velocity) in
            (&mut collisions, &mut position, &mut velocity).join()
        {
            let net_vector = collisions.get_net_vector(CollisionType::Stop);
            position.add(-1.0 * net_vector);
            //TODO use Projective2 in order to cancel the component of the velocity paralell to the
            //net penetration.
            collisions.clear();
        }
    }
}

// invert of vector object 1 would have to be displaced by in order to exit object 2 with lowest
// travel  distance
pub fn find_penetration(
    hitbox_1: &Hitbox,
    hitbox_2: &Hitbox,
    position_1: Vector2,
    position_2: Vector2,
) -> Vector2 {
    match (hitbox_1, hitbox_2) {
        (
            Hitbox::Rectangle {
                dimensions: dimensions_1,
                angle: angle_1,
            },
            Hitbox::Rectangle {
                dimensions: dimensions_2,
                angle: angle_2,
            },
        ) => rect_rect_penetration(
            position_1,
            position_2,
            dimensions_1,
            dimensions_2,
            angle_1,
            angle_2,
        ),
        (
            Hitbox::Rectangle {
                dimensions,
                angle,
            },
            Hitbox::Circle { radius },
        ) => -1.0 * circle_rect_penetration(position_2, position_1, radius, dimensions, angle),
        (
            Hitbox::Circle { radius },
            Hitbox::Rectangle {
                dimensions,
                angle,
            },
        ) => circle_rect_penetration(position_1, position_2, radius, dimensions, angle),
        (Hitbox::Circle { radius: radius_1 }, Hitbox::Circle { radius: radius_2 }) => {
            circle_circle_penetration(position_1, position_2, radius_1, radius_2)
        }
        (_, _) => Vector2::zeros(),
    }
}

fn circle_rect_penetration(
    circle_position: Vector2,
    rectangle_position: Vector2,
    radius: &f32,
    dimensions: &Vector2,
    angle: &f32,
) -> Vector2 {
    let circle_center = get_circle_center(circle_position, *radius);
    let rectangle_center = get_rectangle_center(rectangle_position, *dimensions);
    let displacement = circle_center - rectangle_center;
    let distance = Vector2::new(displacement.x.abs(), displacement.y.abs());
    let h = rectangle_position + dimensions;

    if (distance.x > ((dimensions.x / 2.0) + radius))
        || (distance.y > ((dimensions.y / 2.0) + radius))
    {
        //green zone (not intersecting), can be thought of as an aabb test.
        return Vector2::zeros()
    } else if (distance.x <= (dimensions.x / 2.0))
        || (distance.y <= (dimensions.y / 2.0))
    {
        //yellow zone (intersecting at side), does basically what the rect_rect test does.
        //shitty algorithm
        /*
        let h_displacement = h - circle_center;
        let l_displacement = rectangle_position - circle_center;
        let px = match l_displacement.x.abs() > h_displacement.x.abs() {
            true => -1.0 * l_displacement.x,
            false => h_displacement.x,
        };
        let py = match l_displacement.y.abs() > h_displacement.y.abs() {
            true => -1.0 * l_displacement.x,
            false => h_displacement.x,
        };
        match px.abs() > py.abs() {
            true => Vector2::new(0.0, py),
            false => Vector2::new(px, 0.0),
        }
        */
        let circle_dimensions = Vector2::new(2.0 * *radius, 2.0 * *radius);
        let h1 = circle_position + circle_dimensions;
        let h2 = rectangle_position + dimensions;

        let d1 = h2 - circle_position;
        let d2 = h1 - rectangle_position;

        if (d1.x > 0.0) & (d2.x > 0.0) & (d1.y > 0.0) & (d2.y > 0.0) {
            let px = match d1.x.abs() < d2.x.abs() {
                true => -1.0 * d1.x,
                false => d2.x,
            };
            let py = match d1.y.abs() < d2.y.abs() {
                true => -1.0 * d1.y,
                false => d2.y,
            };
            return match px.abs() > py.abs() {
                true => Vector2::new(0.0, py),
                false => Vector2::new(px, 0.0),
            }
        } else {
            return Vector2::zeros()
        }
    } else {
        //corners

        //find which corner
        let corner_location = match (displacement.x > 0.0, displacement.y > 0.0) {
            //top-right corner
            (true, true) => Vector2::new(h.x, h.y),
            //bottom-right corner
            (true, false) => Vector2::new(h.x, rectangle_position.y),
            //bottom-left corner
            (false, false) => Vector2::new(rectangle_position.x, rectangle_position.y),
            //top-left corner
            (false, true) => Vector2::new(rectangle_position.x, h.y),
        };

        //get penetration
        circle_point_penetration(circle_position, radius, corner_location)
    }
}

fn circle_point_penetration(
    circle_position: Vector2,
    radius: &f32,
    point_position: Vector2,
) -> Vector2 {
    let circle_center = get_circle_center(circle_position, *radius);
    let displacement = circle_center - point_position;
    if displacement.norm().abs() >= *radius {
        Vector2::zeros()
    } else {
        displacement - (*radius * displacement.normalize())
    }
}

fn circle_circle_penetration(
    position_1: Vector2,
    position_2: Vector2,
    radius_1: &f32,
    radius_2: &f32,
) -> Vector2 {
    let center_1 = get_circle_center(position_1, *radius_1);
    let center_2 = get_circle_center(position_2, *radius_2);
    let displacement = center_1 - center_2;
    if displacement.norm().abs() >= (*radius_1 + *radius_2) {
        Vector2::zeros()
    } else {
        1.0 * (displacement
            - ((radius_1 / displacement.norm()) * displacement)
            - ((radius_2 / displacement.norm()) * displacement))
    }
}

fn rect_rect_penetration(
    position_1: Vector2,
    position_2: Vector2,
    dimensions_1: &Vector2,
    dimensions_2: &Vector2,
    angle_1: &f32,
    angle_2: &f32,
) -> Vector2 {
    let h1 = position_1 + dimensions_1;
    let h2 = position_2 + dimensions_2;

    let d1 = h2 - position_1;
    let d2 = h1 - position_2;

    if (d1.x > 0.0) & (d2.x > 0.0) & (d1.y > 0.0) & (d2.y > 0.0) {
        let px = match d1.x.abs() < d2.x.abs() {
            true => -1.0 * d1.x,
            false => d2.x,
        };
        let py = match d1.y.abs() < d2.y.abs() {
            true => -1.0 * d1.y,
            false => d2.y,
        };
        match px.abs() > py.abs() {
            true => Vector2::new(0.0, py),
            false => Vector2::new(px, 0.0),
        }
    } else {
        Vector2::zeros()
    }
}
