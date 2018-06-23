#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate from_variants;
extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event::*;
use ggez::ContextBuilder;

#[macro_use]
mod macros {



    macro_rules! for_impl({
                          $ ( $ type : ty ) , * ; impl {
                          $ ( $ function : tt ) * } $ (
                          impl $ trait : ty { $ ( $ trait_function : tt ) * }
                          ) * } => {
                          for_impl ! (
                          @ call_tuple $ ( $ type ) , * | (
                          $ ( $ function ) , * ) ) ; for_impl ! (
                          @ call_tuple_trait $ ( $ trait ) , * | (
                          $ ( $ type ) , * ) | (
                          $ ( ( $ ( $ trait_function ) , * ) ) , * ) ) ; } ; {
                          $ ( $ type : ty ) , * ; $ (
                          impl $ trait : ty { $ ( $ trait_function : tt ) * }
                          ) * } => {
                          for_impl ! (
                          @ call_tuple_trait $ ( $ trait ) , * | (
                          $ ( $ type ) , * ) | (
                          $ ( ( $ ( $ trait_function ) , * ) ) , * ) ) ; } ; (
                          @ call_tuple $ ( $ type : ty ) , * | $
                          function_tuple : tt ) => {
                          $ (
                          for_impl ! ( @ call $ type | $ function_tuple ) ; )
                          * } ; (
                          @ call $ type : ty | ( $ ( $ function : tt ) , * ) )
                          => { impl $ type { $ ( $ function ) * } } ; (
                          @ call_tuple_trait $ ( $ trait : ty ) , * | $
                          type_tuple : tt | ( $ ( $ function_tuple : tt ) , *
                          ) ) => {
                          $ (
                          for_impl ! (
                          @ call_trait $ trait | $ type_tuple | $
                          function_tuple ) ; ) * } ; (
                          @ call_trait $ trait : ty | ( $ ( $ type : ty ) , *
                          ) | $ function_tuple : tt ) => {
                          $ (
                          for_impl ! (
                          @ call_trait_final $ trait | $ type | $
                          function_tuple ) ; ) * } ; (
                          @ call_trait_final $ trait : ty | $ type : ty | (
                          $ ( $ function : tt ) , * ) ) => {
                          impl $ trait for $ type { $ ( $ function ) * } } ;);
}
mod assets {
    use std::collections::HashMap;
    use ggez::nalgebra::core::Vector2;
    use ggez::nalgebra::core::dimension::U2;
    use ggez::graphics::{DrawMode, Drawable, Mesh, MeshBuilder};
    use ggez::graphics::{Point2};
    use ggez::Context;
    #[rustc_copy_clone_marker]
    pub enum DrawableAsset {
        Circle {
            radius: u32,
        },
        Rectangle {
            dimensions: Vector2<u32>,
        },
        Player,
        Block,
        Wallh,
        Wallv,
        Bullet,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for DrawableAsset {
        #[inline]
        fn clone(&self) -> DrawableAsset {
            {
                let _: ::std::clone::AssertParamIsClone<u32>;
                let _: ::std::clone::AssertParamIsClone<Vector2<u32>>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for DrawableAsset { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for DrawableAsset {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&DrawableAsset::Circle { radius: ref __self_0 },) => {
                    let mut debug_trait_builder = f.debug_struct("Circle");
                    let _ =
                        debug_trait_builder.field("radius", &&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&DrawableAsset::Rectangle { dimensions: ref __self_0 },) => {
                    let mut debug_trait_builder = f.debug_struct("Rectangle");
                    let _ =
                        debug_trait_builder.field("dimensions",
                                                  &&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&DrawableAsset::Player,) => {
                    let mut debug_trait_builder = f.debug_tuple("Player");
                    debug_trait_builder.finish()
                }
                (&DrawableAsset::Block,) => {
                    let mut debug_trait_builder = f.debug_tuple("Block");
                    debug_trait_builder.finish()
                }
                (&DrawableAsset::Wallh,) => {
                    let mut debug_trait_builder = f.debug_tuple("Wallh");
                    debug_trait_builder.finish()
                }
                (&DrawableAsset::Wallv,) => {
                    let mut debug_trait_builder = f.debug_tuple("Wallv");
                    debug_trait_builder.finish()
                }
                (&DrawableAsset::Bullet,) => {
                    let mut debug_trait_builder = f.debug_tuple("Bullet");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl DrawableAsset {
        pub fn rect(x: u32, y: u32) -> Self {
            DrawableAsset::Rectangle{dimensions: Vector2::new(x, y),}
        }
        pub fn circle(radius: u32) -> Self { DrawableAsset::Circle{radius,} }
    }
    pub struct Assets {
        drawable: DrawableAssets,
    }
    impl Assets {
        pub fn get_drawable(&mut self, ctx: &mut Context,
                            drawable_asset: DrawableAsset) -> &Drawable {
            self.drawable.get_drawable(ctx, drawable_asset)
        }
        pub fn new(ctx: &mut Context) -> Self {
            Self{drawable: DrawableAssets::new(ctx),}
        }
    }
    struct DrawableAssets {
        player: Mesh,
        block: Mesh,
        wallh: Mesh,
        wallv: Mesh,
        bullet: Mesh,
        rectangles: HashMap<Vector2<u32>, Mesh>,
        circles: HashMap<u32, Mesh>,
    }
    impl DrawableAssets {
        pub fn new(ctx: &mut Context) -> DrawableAssets {
            DrawableAssets{player: Self::circle(ctx, 10.0),
                           block: Self::rect(ctx, 20.0, 20.0),
                           wallh: Self::rect(ctx, 400.0, 20.0),
                           wallv: Self::rect(ctx, 20.0, 400.0),
                           bullet: Self::rect(ctx, 2.0, 2.0),
                           rectangles: HashMap::new(),
                           circles: HashMap::new(),}
        }
        fn circle(ctx: &mut Context, radius: f32) -> Mesh {
            let mut builder = MeshBuilder::new();
            builder.circle(DrawMode::Fill,
                           Point2::new(1.0 * radius, 1.0 * radius), radius,
                           radius / 200.0);
            builder.build(ctx).unwrap()
        }
        fn rect(ctx: &mut Context, x: f32, y: f32) -> Mesh {
            let mut builder = MeshBuilder::new();
            builder.polygon(DrawMode::Fill,
                            &[Point2::new(0.0, 0.0), Point2::new(x, 0.0),
                              Point2::new(x, 1.0 * y),
                              Point2::new(0.0, 1.0 * y)]);
            builder.build(ctx).unwrap()
        }
        pub fn get_drawable(&mut self, ctx: &mut Context,
                            drawable_asset: DrawableAsset) -> &Drawable {
            match drawable_asset {
                DrawableAsset::Player => &self.player,
                DrawableAsset::Block => &self.block,
                DrawableAsset::Wallh => &self.wallh,
                DrawableAsset::Wallv => &self.wallv,
                DrawableAsset::Bullet => &self.bullet,
                DrawableAsset::Rectangle { dimensions } => {
                    if let None = self.rectangles.get(&dimensions) {
                        let new_rectangle =
                            Self::rect(ctx, dimensions.x as f32,
                                       dimensions.y as f32);
                        let key = Vector2::new(dimensions.x, dimensions.y);
                        self.rectangles.insert(key, new_rectangle);
                        self.rectangles.get(&key).unwrap()
                    } else { self.rectangles.get(&dimensions).unwrap() }
                }
                DrawableAsset::Circle { radius } => {
                    if let None = self.circles.get(&radius) {
                        let new_circle = Self::circle(ctx, radius as f32);
                        self.circles.insert(radius, new_circle);
                        self.circles.get(&radius).unwrap()
                    } else { self.circles.get(&radius).unwrap() }
                }
            }
        }
    }
}
mod components {
    use specs::VecStorage;
    #[macro_use]
    pub mod physics {
        use specs::VecStorage;
        use specs::DenseVecStorage;
        use ggez::graphics::Vector2;
        #[storage(VecStorage)]
        pub struct Position {
            vec: Vector2,
        }
        impl ::specs::world::Component for Position {
            type
            Storage
            =
            VecStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Position {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Position { vec: ref __self_0_0 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("Position");
                        let _ =
                            debug_trait_builder.field("vec", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Position {
            #[inline]
            fn clone(&self) -> Position {
                match *self {
                    Position { vec: ref __self_0_0 } =>
                    Position{vec:
                                 ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[storage(VecStorage)]
        pub struct Velocity {
            vec: Vector2,
        }
        impl ::specs::world::Component for Velocity {
            type
            Storage
            =
            VecStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Velocity {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Velocity { vec: ref __self_0_0 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("Velocity");
                        let _ =
                            debug_trait_builder.field("vec", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Velocity {
            #[inline]
            fn clone(&self) -> Velocity {
                match *self {
                    Velocity { vec: ref __self_0_0 } =>
                    Velocity{vec:
                                 ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[storage(VecStorage)]
        pub struct Acceleration {
            vec: Vector2,
        }
        impl ::specs::world::Component for Acceleration {
            type
            Storage
            =
            VecStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Acceleration {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Acceleration { vec: ref __self_0_0 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("Acceleration");
                        let _ =
                            debug_trait_builder.field("vec", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Acceleration {
            #[inline]
            fn clone(&self) -> Acceleration {
                match *self {
                    Acceleration { vec: ref __self_0_0 } =>
                    Acceleration{vec:
                                     ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[storage(DenseVecStorage)]
        pub struct MoveDrag {
            drag_constant: f32,
        }
        impl ::specs::world::Component for MoveDrag {
            type
            Storage
            =
            DenseVecStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for MoveDrag {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    MoveDrag { drag_constant: ref __self_0_0 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("MoveDrag");
                        let _ =
                            debug_trait_builder.field("drag_constant",
                                                      &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for MoveDrag {
            #[inline]
            fn clone(&self) -> MoveDrag {
                match *self {
                    MoveDrag { drag_constant: ref __self_0_0 } =>
                    MoveDrag{drag_constant:
                                 ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        impl MoveDrag {
            pub fn new(drag_constant: f32) -> Self { Self{drag_constant,} }
            pub fn get_constant(&self) -> f32 { self.drag_constant }
        }
        #[storage(DenseVecStorage)]
        pub struct MoveDirection {
            direction: Vector2,
            move_acceleration: f32,
        }
        impl ::specs::world::Component for MoveDirection {
            type
            Storage
            =
            DenseVecStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for MoveDirection {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    MoveDirection {
                    direction: ref __self_0_0,
                    move_acceleration: ref __self_0_1 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("MoveDirection");
                        let _ =
                            debug_trait_builder.field("direction",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("move_acceleration",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl MoveDirection {
            pub fn get(&self) -> Vector2 {
                if self.direction != Vector2::zeros() {
                    self.direction.clone().normalize()
                } else { self.direction.clone() }
            }
            pub fn get_move_acceleration(&self) -> f32 {
                self.move_acceleration
            }
            pub fn set(&mut self, vec: Vector2) { self.direction = vec; }
            pub fn new(move_acceleration: f32) -> Self {
                Self{direction: Vector2::zeros(), move_acceleration,}
            }
            pub fn x(&self) -> f32 { self.direction.x }
            pub fn y(&self) -> f32 { self.direction.y }
            pub fn add(&mut self, vec: Vector2) { self.direction += vec; }
        }
        impl Position {
            pub fn new(x: f32, y: f32) -> Self {
                Self{vec: Vector2::new(x, y),}
            }
            pub fn zeros() -> Self { Self{vec: Vector2::zeros(),} }
        }
        impl Velocity {
            pub fn new(x: f32, y: f32) -> Self {
                Self{vec: Vector2::new(x, y),}
            }
            pub fn zeros() -> Self { Self{vec: Vector2::zeros(),} }
        }
        impl Acceleration {
            pub fn new(x: f32, y: f32) -> Self {
                Self{vec: Vector2::new(x, y),}
            }
            pub fn zeros() -> Self { Self{vec: Vector2::zeros(),} }
        }
        impl Position {
            pub fn set(&mut self, vec: Vector2) { self.vec = vec; }
        }
        impl Velocity {
            pub fn set(&mut self, vec: Vector2) { self.vec = vec; }
        }
        impl Acceleration {
            pub fn set(&mut self, vec: Vector2) { self.vec = vec; }
        }
        impl Position {
            pub fn x(&self) -> f32 { self.vec.x }
            pub fn y(&self) -> f32 { self.vec.y }
            pub fn get(&self) -> Vector2 { self.vec.clone() }
            pub fn add(&mut self, vec: Vector2) { self.vec += vec; }
        }
        impl Velocity {
            pub fn x(&self) -> f32 { self.vec.x }
            pub fn y(&self) -> f32 { self.vec.y }
            pub fn get(&self) -> Vector2 { self.vec.clone() }
            pub fn add(&mut self, vec: Vector2) { self.vec += vec; }
        }
        impl Acceleration {
            pub fn x(&self) -> f32 { self.vec.x }
            pub fn y(&self) -> f32 { self.vec.y }
            pub fn get(&self) -> Vector2 { self.vec.clone() }
            pub fn add(&mut self, vec: Vector2) { self.vec += vec; }
        }
    }
    pub mod render {
        use ggez::graphics;
        use ggez::graphics::Drawable;
        use ggez::graphics::Image;
        use ggez::graphics::DrawParam;
        use ggez::graphics::Color;
        use ggez::graphics::{Point2, Vector2};
        use ggez::Context;
        use specs::VecStorage;
        use specs::Component;
        use resources::Camera;
        use assets::{Assets, DrawableAsset};
        use components::physics::Position;
        fn add_point2_vec2(first: Point2, second: Vector2) -> Point2 {
            Point2::new(first.x + second.x, first.y + second.y)
        }
        fn add_point2(first: Point2, second: Point2) -> Point2 {
            Point2::new(first.x + second.x, first.y + second.y)
        }
        fn mul_point2(first: Point2, second: Point2) -> Point2 {
            Point2::new(first.x * second.x, first.y * second.y)
        }
        pub trait HasDrawable {
            fn get_drawable<'a>(&self, ctx: &mut Context,
                                assets: &'a mut Assets)
            -> &'a Drawable;
            fn get_draw_parameters(&self)
            -> DrawParam;
            fn render(&self, ctx: &mut Context, assets: &mut Assets,
                      position: &Position, draw_param: DrawParam) {
                let drawable = self.get_drawable(ctx, assets);
                graphics::draw_ex(ctx, drawable,
                                  graphics::DrawParam{src:
                                                          self.get_draw_parameters().src,
                                                      dest:
                                                          add_point2_vec2(add_point2(draw_param.dest,
                                                                                     self.get_draw_parameters().dest),
                                                                          position.get()),
                                                      rotation:
                                                          draw_param.rotation
                                                              +
                                                              self.get_draw_parameters().rotation,
                                                      scale:
                                                          mul_point2(draw_param.scale,
                                                                     self.get_draw_parameters().scale),
                                                      offset:
                                                          self.get_draw_parameters().offset,
                                                      shear:
                                                          mul_point2(draw_param.shear,
                                                                     self.get_draw_parameters().shear),
                                                      color:
                                                          self.get_draw_parameters().color,}).unwrap();
            }
        }
        #[storage(VecStorage)]
        pub struct DrawableComponent {
            asset: DrawableAsset,
            draw_parameters: graphics::DrawParam,
        }
        impl ::specs::world::Component for DrawableComponent {
            type
            Storage
            =
            VecStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for DrawableComponent {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    DrawableComponent {
                    asset: ref __self_0_0, draw_parameters: ref __self_0_1 }
                    => {
                        let mut debug_trait_builder =
                            f.debug_struct("DrawableComponent");
                        let _ =
                            debug_trait_builder.field("asset",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("draw_parameters",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl DrawableComponent {
            pub fn new(asset: DrawableAsset) -> DrawableComponent {
                DrawableComponent{asset,
                                  draw_parameters:
                                      graphics::DrawParam{..Default::default()},}
            }
            pub fn new_with_color(asset: DrawableAsset, color: Color)
             -> DrawableComponent {
                DrawableComponent{asset,
                                  draw_parameters:
                                      graphics::DrawParam{color:
                                                              Some(color),
                                                                             ..Default::default()},}
            }
        }
        impl HasDrawable for DrawableComponent {
            fn get_drawable<'a>(&self, ctx: &mut Context,
                                assets: &'a mut Assets) -> &'a Drawable {
                assets.get_drawable(ctx, self.asset)
            }
            fn get_draw_parameters(&self) -> DrawParam {
                self.draw_parameters
            }
        }
    }
    pub mod tags {
        use specs::NullStorage;
        #[storage(NullStorage)]
        pub struct TakesInput;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::default::Default for TakesInput {
            #[inline]
            fn default() -> TakesInput { TakesInput }
        }
        impl ::specs::world::Component for TakesInput {
            type
            Storage
            =
            NullStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for TakesInput {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    TakesInput => {
                        let mut debug_trait_builder =
                            f.debug_tuple("TakesInput");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for TakesInput {
            #[inline]
            fn clone(&self) -> TakesInput {
                match *self { TakesInput => TakesInput, }
            }
        }
        #[storage(NullStorage)]
        pub struct CameraFollows;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::default::Default for CameraFollows {
            #[inline]
            fn default() -> CameraFollows { CameraFollows }
        }
        impl ::specs::world::Component for CameraFollows {
            type
            Storage
            =
            NullStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for CameraFollows {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    CameraFollows => {
                        let mut debug_trait_builder =
                            f.debug_tuple("CameraFollows");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for CameraFollows {
            #[inline]
            fn clone(&self) -> CameraFollows {
                match *self { CameraFollows => CameraFollows, }
            }
        }
    }
    pub mod collision {
        use std::mem::swap;
        use specs::VecStorage;
        use specs::NullStorage;
        use ggez::graphics::Vector2;
        pub fn get_circle_center(position: Vector2, radius: f32) -> Vector2 {
            Vector2::new(position.x + radius, position.y + radius)
        }
        pub fn get_rectangle_center(position: Vector2, dimensions: Vector2)
         -> Vector2 {
            position + (dimensions / 2.0)
        }
        #[storage(VecStorage)]
        pub enum Hitbox {
            Circle {
                radius: f32,
            },
            LineSegment {
                length: f32,
                angle: f32,
            },
            Rectangle {
                dimensions: Vector2,
                angle: f32,
            },
        }
        impl ::specs::world::Component for Hitbox {
            type
            Storage
            =
            VecStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Hitbox {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&Hitbox::Circle { radius: ref __self_0 },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("Circle");
                        let _ =
                            debug_trait_builder.field("radius",
                                                      &&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Hitbox::LineSegment {
                     length: ref __self_0, angle: ref __self_1 },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("LineSegment");
                        let _ =
                            debug_trait_builder.field("length",
                                                      &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("angle", &&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&Hitbox::Rectangle {
                     dimensions: ref __self_0, angle: ref __self_1 },) => {
                        let mut debug_trait_builder =
                            f.debug_struct("Rectangle");
                        let _ =
                            debug_trait_builder.field("dimensions",
                                                      &&(*__self_0));
                        let _ =
                            debug_trait_builder.field("angle", &&(*__self_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Hitbox {
            #[inline]
            fn clone(&self) -> Hitbox {
                match (&*self,) {
                    (&Hitbox::Circle { radius: ref __self_0 },) =>
                    Hitbox::Circle{radius:
                                       ::std::clone::Clone::clone(&(*__self_0)),},
                    (&Hitbox::LineSegment {
                     length: ref __self_0, angle: ref __self_1 },) =>
                    Hitbox::LineSegment{length:
                                            ::std::clone::Clone::clone(&(*__self_0)),
                                        angle:
                                            ::std::clone::Clone::clone(&(*__self_1)),},
                    (&Hitbox::Rectangle {
                     dimensions: ref __self_0, angle: ref __self_1 },) =>
                    Hitbox::Rectangle{dimensions:
                                          ::std::clone::Clone::clone(&(*__self_0)),
                                      angle:
                                          ::std::clone::Clone::clone(&(*__self_1)),},
                }
            }
        }
        impl Hitbox {
            pub fn center(&self) -> Vector2 {
                match self {
                    Hitbox::Circle { radius } => {
                        Vector2::new(*radius, *radius)
                    }
                    Hitbox::LineSegment { .. } => { Vector2::zeros() }
                    Hitbox::Rectangle { dimensions, .. } => {
                        dimensions / 2.0
                    }
                }
            }
            pub fn center_from(&self, position: Vector2) -> Vector2 {
                position + self.center()
            }
        }
        #[rustc_copy_clone_marker]
        pub enum CollisionType { Stop, }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for CollisionType {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&CollisionType::Stop,) => {
                        let mut debug_trait_builder = f.debug_tuple("Stop");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for CollisionType {
            #[inline]
            fn eq(&self, other: &CollisionType) -> bool {
                match (&*self, &*other) { _ => true, }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for CollisionType {
            #[inline]
            fn clone(&self) -> CollisionType { { *self } }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::marker::Copy for CollisionType { }
        pub struct Collision {
            penetration: Vector2,
            collision_type: CollisionType,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Collision {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Collision {
                    penetration: ref __self_0_0,
                    collision_type: ref __self_0_1 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("Collision");
                        let _ =
                            debug_trait_builder.field("penetration",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("collision_type",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Collision {
            #[inline]
            fn clone(&self) -> Collision {
                match *self {
                    Collision {
                    penetration: ref __self_0_0,
                    collision_type: ref __self_0_1 } =>
                    Collision{penetration:
                                  ::std::clone::Clone::clone(&(*__self_0_0)),
                              collision_type:
                                  ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        impl Collision {
            pub fn new(penetration: Vector2, collision_type: CollisionType)
             -> Self {
                Collision{penetration, collision_type,}
            }
            pub fn is_type(&self, collision_type: CollisionType) -> bool {
                self.collision_type == collision_type
            }
            pub fn get_penetration(&self) -> Vector2 { self.penetration }
        }
        #[storage(VecStorage)]
        pub struct Collisions {
            collisions: Vec<Collision>,
        }
        impl ::specs::world::Component for Collisions {
            type
            Storage
            =
            VecStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Collisions {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Collisions { collisions: ref __self_0_0 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("Collisions");
                        let _ =
                            debug_trait_builder.field("collisions",
                                                      &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Collisions {
            #[inline]
            fn clone(&self) -> Collisions {
                match *self {
                    Collisions { collisions: ref __self_0_0 } =>
                    Collisions{collisions:
                                   ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        impl Collisions {
            pub fn new() -> Self { Collisions{collisions: Vec::new(),} }
            pub fn recieve_collision(&mut self, collision: Collision) {
                self.collisions.push(collision);
            }
            pub fn get_net_vector(&self, collision_type: CollisionType)
             -> Vector2 {
                let mut return_vector = Vector2::zeros();
                for collision in self.collisions.iter() {
                    if collision.is_type(collision_type) {
                        return_vector += collision.get_penetration();
                    }
                }
                return_vector
            }
            pub fn clear(&mut self) { self.collisions = Vec::new(); }
            pub fn clear_return(&mut self) -> Vec<Collision> {
                let mut new_vec = Vec::new();
                swap(&mut new_vec, &mut self.collisions);
                new_vec
            }
        }
        #[storage(NullStorage)]
        pub struct BlocksMovement;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::default::Default for BlocksMovement {
            #[inline]
            fn default() -> BlocksMovement { BlocksMovement }
        }
        impl ::specs::world::Component for BlocksMovement {
            type
            Storage
            =
            NullStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for BlocksMovement {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    BlocksMovement => {
                        let mut debug_trait_builder =
                            f.debug_tuple("BlocksMovement");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for BlocksMovement {
            #[inline]
            fn clone(&self) -> BlocksMovement {
                match *self { BlocksMovement => BlocksMovement, }
            }
        }
        #[storage(NullStorage)]
        pub struct IsBlocked;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::default::Default for IsBlocked {
            #[inline]
            fn default() -> IsBlocked { IsBlocked }
        }
        impl ::specs::world::Component for IsBlocked {
            type
            Storage
            =
            NullStorage<Self>;
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for IsBlocked {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    IsBlocked => {
                        let mut debug_trait_builder =
                            f.debug_tuple("IsBlocked");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for IsBlocked {
            #[inline]
            fn clone(&self) -> IsBlocked {
                match *self { IsBlocked => IsBlocked, }
            }
        }
    }
    pub mod prefab {
        use std::mem::{discriminant, Discriminant};
        use std::collections::HashMap;
        use std::any::Any;
        use specs::{RunNow, World, Component, Entity, Entities, LazyUpdate};
        use specs::world::EntityBuilder;
        use components::Name;
        use components::physics::*;
        use components::render::*;
        use components::collision::*;
        use components::tags::TakesInput;
        use components;
        macro_rules! as_item(( $ i : item ) => { $ i } ;);
        macro_rules! as_ident(( $ i : ident ) => { $ i } ;);
        macro_rules! match_pattern(( $ i : ident ) => {
                                   PrefabComponent :: $ i ( c ) } ;);
        macro_rules! i_hate_myself({ $ ( $ ident : ident : $ ty : ty ) , * ; }
                                   => {
                                   as_item ! {
                                   enum PrefabComponent {
                                   $ ( $ ident ( $ ty ) , ) * } } impl
                                   PrefabComponent {
                                   pub fn add_to_entity_builder < 'a > (
                                   & self , entity_builder : EntityBuilder <
                                   'a > ) -> EntityBuilder < 'a > {
                                   match self . clone (  ) {
                                   $ (
                                   match_pattern ! ( $ ident ) => {
                                   entity_builder . with ( c ) } , ) * } } pub
                                   fn add_to_entity < 'a > (
                                   & self , entity : & Entity , updater : &
                                   LazyUpdate ) {
                                   match self . clone (  ) {
                                   $ (
                                   match_pattern ! ( $ ident ) => {
                                   updater . insert ( * entity , c ) } , ) * }
                                   } } });
        enum PrefabComponent { Pos(Position), }
        impl PrefabComponent {
            pub fn add_to_entity_builder<'a>(&self,
                                             entity_builder:
                                                 EntityBuilder<'a>)
             -> EntityBuilder<'a> {
                match self.clone() {
                    PrefabComponent::Pos(c) => { entity_builder.with(c) }
                }
            }
            pub fn add_to_entity<'a>(&self, entity: &Entity,
                                     updater: &LazyUpdate) {
                match self.clone() {
                    PrefabComponent::Pos(c) => { updater.insert(*entity, c) }
                }
            }
        }
        pub struct Prefab {
            components: HashMap<Discriminant<PrefabComponent>,
                                PrefabComponent>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Prefab {
            #[inline]
            fn clone(&self) -> Prefab {
                match *self {
                    Prefab { components: ref __self_0_0 } =>
                    Prefab{components:
                               ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        impl Prefab {
            pub fn add_component(&mut self,
                                 prefab_component: PrefabComponent) {
                self.components.insert(discriminant(&prefab_component),
                                       prefab_component);
            }
            pub fn generate_in_world(&self, world: &mut World) {
                let mut entity_builder = world.create_entity();
                for component in self.components.values() {
                    entity_builder =
                        component.add_to_entity_builder(entity_builder);
                }
                entity_builder.build();
            }
            pub fn lazy_generate_in_entities(&self, entities: &Entities,
                                             updater: &LazyUpdate) {
                let new_entity = entities.create();
                for component in self.components.values() {
                    component.add_to_entity(&new_entity, updater);
                }
            }
        }
    }
    #[storage(VecStorage)]
    pub struct Name {
        string: String,
    }
    impl ::specs::world::Component for Name {
        type
        Storage
        =
        VecStorage<Self>;
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Name {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Name { string: ref __self_0_0 } => {
                    let mut debug_trait_builder = f.debug_struct("Name");
                    let _ =
                        debug_trait_builder.field("string", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl Name {
        pub fn new(string: String) -> Self { Self{string,} }
        pub fn read(&self) -> &str { self.string.as_str() }
    }
}
mod main_state {
    use std::boxed::Box;
    use components::Name;
    use components::physics::*;
    use components::render::*;
    use components::collision::*;
    use components::tags::TakesInput;
    use ggez::{graphics, timer, Context, GameResult, event::*,
               graphics::{Point2, Vector2}};
    use resources::{DeltaTime, Camera};
    use specs::{RunNow, World, Component, Entities, LazyUpdate};
    use specs::VecStorage;
    use specs::world::EntityBuilder;
    use systems::input::{Axis, DirectionInputScalar, Input};
    use systems::*;
    use systems::collision::*;
    use assets::{Assets, DrawableAsset};
    pub mod debug {
        use std::collections::HashMap;
        use ggez::graphics::Point2;
        use ggez::Context;
        use ggez::graphics;
        use ggez::graphics::Image;
        const SEPERATOR: &str = ":";
        pub struct DebugTable {
            position: Point2,
            data: HashMap<String, String>,
            images: HashMap<String, Image>,
        }
        impl DebugTable {
            pub fn new(ctx: &mut Context, position: Point2) -> DebugTable {
                let mut t =
                    DebugTable{position,
                               data: HashMap::new(),
                               images: HashMap::new(),};
                let colon_image = DebugTable::make_image_from(ctx, SEPERATOR);
                t.images.insert(SEPERATOR.to_owned(), colon_image);
                t
            }
            pub fn load(&mut self, label: String, data: String) {
                self.data.insert(label, data);
            }
            pub fn render(&mut self, ctx: &mut Context) {
                let mut cursor = self.position.clone();
                let mut lables = Vec::new();
                for label in self.data.keys() { lables.push(label.clone()); }
                for label in lables {
                    self.draw_text(ctx, &label[..], &mut cursor, false);
                    self.draw_text(ctx, SEPERATOR, &mut cursor, false);
                    let value = &self.data.get(&label).unwrap().clone()[..];
                    self.draw_text(ctx, value, &mut cursor, true);
                    cursor.x = self.position.x;
                }
            }
            fn draw_text(&mut self, ctx: &mut Context, string: &str,
                         cursor: &mut Point2, new_line: bool) {
                if let Some(image) = self.images.get(string) {
                    DebugTable::draw(ctx, image, cursor, new_line);
                    return
                }
                self.images.insert(String::from(string),
                                   DebugTable::make_image_from(ctx, string));
                if let Some(image) = self.images.get(string) {
                    DebugTable::draw(ctx, image, cursor, new_line);
                }
            }
            fn draw(ctx: &mut Context, image: &Image, cursor: &mut Point2,
                    new_line: bool) {
                graphics::draw_ex(ctx, image,
                                  graphics::DrawParam{dest:
                                                          *cursor,
                                                                     ..Default::default()}).unwrap();
                cursor.x += image.width() as f32;
                if new_line { cursor.y += image.height() as f32; }
            }
            fn make_image_from(ctx: &mut Context, string: &str) -> Image {
                graphics::Text::new(ctx, string,
                                    &graphics::Font::default_font().unwrap()).unwrap().into_inner()
            }
        }
    }
    use std::time::{Duration, Instant};
    struct Stopwatch {
        last_mark: Instant,
    }
    impl Stopwatch {
        pub fn new() -> Stopwatch { Stopwatch{last_mark: Instant::now(),} }
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
            Self{update_pos: UpdatePos,
                 update_vel: UpdateVel,
                 handle_move_direction: HandleMoveDirection,
                 update_penetrations: UpdatePenetrations,
                 resolve_collisions: ResolveCollisions,}
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
        world.create_entity().with(TakesInput).with(MoveDrag::new(drag_constant)).with(MoveDirection::new(accel)).with(Position::zeros()).with(Velocity::zeros()).with(Acceleration::new(0.0,
                                                                                                                                                                                         0.0)).with(DrawableComponent::new(DrawableAsset::Player)).with(Collisions::new()).with(Hitbox::Circle{radius:
                                                                                                                                                                                                                                                                                                   10.0,}).with(IsBlocked).with(Name::new("player".to_owned())).build();
    }
    pub fn player_circle_big(world: &mut World, radius: u32) {
        let accel = 2500.0;
        let max_speed = 250.0;
        let drag_constant = accel / max_speed;
        world.create_entity().with(TakesInput).with(MoveDrag::new(drag_constant)).with(MoveDirection::new(accel)).with(Position::zeros()).with(Velocity::zeros()).with(Acceleration::new(0.0,
                                                                                                                                                                                         0.0)).with(DrawableComponent::new(DrawableAsset::circle(radius))).with(Collisions::new()).with(Hitbox::Circle{radius:
                                                                                                                                                                                                                                                                                                           radius
                                                                                                                                                                                                                                                                                                               as
                                                                                                                                                                                                                                                                                                               f32,}).with(IsBlocked).with(Name::new("player".to_owned())).build();
    }
    pub fn player_square(world: &mut World) {
        let accel = 2500.0;
        let max_speed = 250.0;
        let drag_constant = accel / max_speed;
        world.create_entity().with(TakesInput).with(MoveDrag::new(drag_constant)).with(MoveDirection::new(accel)).with(Position::zeros()).with(Velocity::zeros()).with(Acceleration::new(0.0,
                                                                                                                                                                                         0.0)).with(DrawableComponent::new(DrawableAsset::Block)).with(Collisions::new()).with(Hitbox::Rectangle{dimensions:
                                                                                                                                                                                                                                                                                                     Vector2::new(20.0,
                                                                                                                                                                                                                                                                                                                  20.0),
                                                                                                                                                                                                                                                                                                 angle:
                                                                                                                                                                                                                                                                                                     0.0,}).with(IsBlocked).with(Name::new("player".to_owned())).build();
    }
    pub fn ball(world: &mut World, x: f32, y: f32, radius: u32) {
        world.create_entity().with(Position::new(x,
                                                 y)).with(DrawableComponent::new(DrawableAsset::circle(radius))).with(Hitbox::Circle{radius:
                                                                                                                                         radius
                                                                                                                                             as
                                                                                                                                             f32,}).with(BlocksMovement).with(Name::new("Block".to_owned())).build();
    }
    pub fn wall(world: &mut World, x: f32, y: f32, w: u32, h: u32) {
        world.create_entity().with(Position::new(x,
                                                 y)).with(DrawableComponent::new(DrawableAsset::rect(w,
                                                                                                     h))).with(Hitbox::Rectangle{dimensions:
                                                                                                                                     Vector2::new(w
                                                                                                                                                      as
                                                                                                                                                      f32,
                                                                                                                                                  h
                                                                                                                                                      as
                                                                                                                                                      f32),
                                                                                                                                 angle:
                                                                                                                                     0.0,}).with(BlocksMovement).with(Name::new("Block".to_owned())).build();
    }
    pub struct MainState {
        world: World,
        game_systems: GameSystems,
        stopwatch: Stopwatch,
        input: Input,
    }
    impl MainState {
        pub fn new(ctx: &mut Context, _width: u32, _height: u32)
         -> GameResult<MainState> {
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
            world.add_resource(Camera::new_with(Point2::new(100.0, 100.0),
                                                Point2::new(1.0, 1.0)));
            world.add_resource(debug::DebugTable::new(ctx,
                                                      Point2::new(0.0, 0.0)));
            player_circle_big(&mut world, 50);
            wall(&mut world, 100.0, 100.0, 100, 100);
            ball(&mut world, 290.0, 100.0, 50);
            Ok(MainState{world,
                         game_systems: GameSystems::new(),
                         stopwatch: Stopwatch::new(),
                         input: Input::new(),})
        }
    }
    impl EventHandler for MainState {
        fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
            const DESIRED_FPS: u32 = 60;
            while timer::check_update_time(ctx, DESIRED_FPS) {
                {
                    let mut delta = self.world.write_resource::<DeltaTime>();
                    delta.set(timer::duration_to_f64(self.stopwatch.mark()) as
                                  f32);
                }
                self.input.run_now(&mut self.world.res);
                self.game_systems.update(&mut self.world);
            }
            Ok(())
        }
        fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
            graphics::clear(ctx);
            self.game_systems.draw(ctx, &mut self.world);
            graphics::present(ctx);
            timer::yield_now();
            Ok(())
        }
        fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode,
                          _keymod: Mod, _repeat: bool) {
            match keycode {
                Keycode::Escape => ctx.quit().unwrap(),
                Keycode::W => {
                    self.input.move_stack.activate_direction(DirectionInputScalar::Negative,
                                                             Axis::Y);
                }
                Keycode::S => {
                    self.input.move_stack.activate_direction(DirectionInputScalar::Positive,
                                                             Axis::Y);
                }
                Keycode::A => {
                    self.input.move_stack.activate_direction(DirectionInputScalar::Negative,
                                                             Axis::X);
                }
                Keycode::D => {
                    self.input.move_stack.activate_direction(DirectionInputScalar::Positive,
                                                             Axis::X);
                }
                Keycode::Up => {
                    self.input.shoot_stack.activate_direction(DirectionInputScalar::Positive,
                                                              Axis::Y);
                }
                _ => (),
            }
        }
        fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode,
                        _keymod: Mod, _repeat: bool) {
            match keycode {
                Keycode::W => {
                    self.input.move_stack.deactivate_direction(DirectionInputScalar::Negative,
                                                               Axis::Y);
                }
                Keycode::S => {
                    self.input.move_stack.deactivate_direction(DirectionInputScalar::Positive,
                                                               Axis::Y);
                }
                Keycode::A => {
                    self.input.move_stack.deactivate_direction(DirectionInputScalar::Negative,
                                                               Axis::X);
                }
                Keycode::D => {
                    self.input.move_stack.deactivate_direction(DirectionInputScalar::Positive,
                                                               Axis::X);
                }
                _ => (),
            }
        }
    }
}
mod resources {
    use ggez::graphics::DrawParam;
    use ggez::graphics::{Point2, Vector2};
    pub struct DeltaTime(f32);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::default::Default for DeltaTime {
        #[inline]
        fn default() -> DeltaTime {
            DeltaTime(::std::default::Default::default())
        }
    }
    impl DeltaTime {
        pub fn new(seconds: f32) -> Self { DeltaTime(seconds) }
        pub fn get(&self) -> f32 { self.0 }
        pub fn set(&mut self, val: f32) { self.0 = val; }
    }
    pub struct Camera {
        translation: Point2,
        scale: Point2,
        rotation: f32,
        shear: Point2,
    }
    impl Default for Camera {
        fn default() -> Camera {
            Camera{translation: Point2::new(0.0, 0.0),
                   scale: Point2::new(0.0, 0.0),
                   rotation: 0.0,
                   shear: Point2::new(0.0, 0.0),}
        }
    }
    impl Camera {
        pub fn new() -> Self {
            Self{translation: Point2::new(0.0, 0.0),
                 scale: Point2::new(1.0, 1.0),
                 rotation: 0.0,
                 shear: Point2::new(0.0, 0.0),}
        }
        pub fn new_with(translation: Point2, scale: Point2) -> Self {
            Self{translation,
                 scale,
                 rotation: 0.0,
                 shear: Point2::new(0.0, 0.0),}
        }
        pub fn get_scale(&self) -> Point2 { self.scale }
        pub fn get_translation(&self) -> Point2 { self.translation }
        pub fn set_scale(&mut self, scale: Vector2) {
            self.scale = Point2::new(scale.x, scale.y);
        }
        pub fn set_translation(&mut self, translation: Vector2) {
            self.translation = Point2::new(translation.x, translation.y);
        }
        pub fn get_draw_parameters(&self) -> DrawParam {
            DrawParam{dest: self.get_translation(),
                      scale: self.get_scale(),
                      rotation: self.rotation,
                      shear: self.shear, ..Default::default()}
        }
    }
}
mod systems {
    use specs::prelude::Resources;
    use assets::Assets;
    use components::{collision::*, physics::*, render::*, tags::*, *};
    use ggez::graphics;
    use ggez::graphics::DrawParam;
    use ggez::graphics::Vector2;
    use ggez::Context;
    use main_state::debug::DebugTable;
    use resources::{Camera, DeltaTime};
    use specs::ReadExpect;
    use specs::WriteExpect;
    use specs::ReadStorage;
    use specs::System;
    use specs::WriteStorage;
    pub mod input {
        use specs::System;
        use specs::ReadStorage;
        use specs::WriteStorage;
        use ggez::graphics::{Vector2};
        use components::tags::TakesInput;
        use components::physics::{MoveDirection};
        enum Action { Item, None, }
        #[rustc_copy_clone_marker]
        pub enum Axis { X, Y, }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Axis {
            #[inline]
            fn clone(&self) -> Axis { { *self } }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::marker::Copy for Axis { }
        #[structural_match]
        #[rustc_copy_clone_marker]
        pub enum DirectionInputScalar { Positive, Negative, }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for DirectionInputScalar {
            #[inline]
            fn clone(&self) -> DirectionInputScalar { { *self } }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::marker::Copy for DirectionInputScalar { }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::Eq for DirectionInputScalar {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () { { } }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for DirectionInputScalar {
            #[inline]
            fn eq(&self, other: &DirectionInputScalar) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*other)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) { _ => true, }
                    } else { false }
                }
            }
        }
        impl DirectionInputScalar {
            pub fn get_value(&self) -> f32 {
                match *self {
                    DirectionInputScalar::Positive => 1.0,
                    DirectionInputScalar::Negative => -1.0,
                }
            }
        }
        pub struct DirectionInputStack {
            x_input_stack: Vec<DirectionInputScalar>,
            y_input_stack: Vec<DirectionInputScalar>,
        }
        impl DirectionInputStack {
            pub fn new() -> Self {
                Self{x_input_stack: Vec::new(), y_input_stack: Vec::new(),}
            }
            fn get_input_stack(&mut self, axis: Axis)
             -> &mut Vec<DirectionInputScalar> {
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
                !(self.x_input_stack.is_empty() &&
                      self.y_input_stack.is_empty())
            }
            pub fn deactivate_direction(&mut self,
                                        direction: DirectionInputScalar,
                                        axis: Axis) {
                self.get_input_stack(axis).retain(|element|
                                                      *element != direction);
            }
            pub fn activate_direction(&mut self,
                                      direction: DirectionInputScalar,
                                      axis: Axis) {
                if !self.get_input_stack(axis).contains(&direction) {
                    self.get_input_stack(axis).push(direction);
                }
            }
        }
        pub struct Input {
            action: Action,
            pub move_stack: DirectionInputStack,
            pub shoot_stack: DirectionInputStack,
        }
        impl Input {
            pub fn new() -> Self {
                Self{action: Action::None,
                     move_stack: DirectionInputStack::new(),
                     shoot_stack: DirectionInputStack::new(),}
            }
        }
        impl <'a> System<'a> for Input {
            type
            SystemData
            =
            (ReadStorage<'a, TakesInput>, WriteStorage<'a, MoveDirection>);
            fn run(&mut self,
                   (input_tag, mut move_direction): Self::SystemData) {
                use specs::Join;
                for (_, move_direction) in
                    (&input_tag, &mut move_direction).join() {
                    move_direction.set(self.move_stack.get_direction_recent());
                }
            }
        }
    }
    pub mod collision {
        use components::{collision::*, physics::*};
        use ggez::graphics::Vector2;
        use specs::ReadStorage;
        use specs::System;
        use specs::WriteStorage;
        pub struct UpdatePenetrations;
        impl <'a> System<'a> for UpdatePenetrations {
            type
            SystemData
            =
            (ReadStorage<'a, Hitbox>, ReadStorage<'a, BlocksMovement>,
             ReadStorage<'a, IsBlocked>, ReadStorage<'a, Position>,
             WriteStorage<'a, Collisions>);
            fn run(&mut self,
                   (hitbox, blocks_movement, is_blocked, position,
                    mut collisions): Self::SystemData) {
                use specs::Join;
                for (hitbox_1, position_1, collisions, _) in
                    (&hitbox, &position, &mut collisions, &is_blocked).join()
                    {
                    for (hitbox_2, position_2, _) in
                        (&hitbox, &position, &blocks_movement).join() {
                        collisions.recieve_collision(Collision::new(find_penetration(hitbox_1,
                                                                                     hitbox_2,
                                                                                     position_1.get(),
                                                                                     position_2.get()),
                                                                    CollisionType::Stop));
                    }
                }
            }
        }
        pub struct ResolveCollisions;
        impl <'a> System<'a> for ResolveCollisions {
            type
            SystemData
            =
            (WriteStorage<'a, Collisions>, WriteStorage<'a, Position>,
             WriteStorage<'a, Velocity>);
            fn run(&mut self,
                   (mut collisions, mut position, mut velocity):
                       Self::SystemData) {
                use specs::Join;
                for (collisions, position, _) in
                    (&mut collisions, &mut position, !&velocity).join() {
                    position.add(-1.0 *
                                     collisions.get_net_vector(CollisionType::Stop));
                    collisions.clear();
                }
                for (collisions, position, _velocity) in
                    (&mut collisions, &mut position, &mut velocity).join() {
                    let net_vector =
                        collisions.get_net_vector(CollisionType::Stop);
                    position.add(-1.0 * net_vector);
                    collisions.clear();
                }
            }
        }
        pub fn find_penetration(hitbox_1: &Hitbox, hitbox_2: &Hitbox,
                                position_1: Vector2, position_2: Vector2)
         -> Vector2 {
            match (hitbox_1, hitbox_2) {
                (Hitbox::Rectangle { dimensions: dimensions_1, angle: angle_1
                 }, Hitbox::Rectangle {
                 dimensions: dimensions_2, angle: angle_2 }) =>
                rect_rect_penetration(position_1, position_2, dimensions_1,
                                      dimensions_2, angle_1, angle_2),
                (Hitbox::Rectangle { dimensions, angle }, Hitbox::Circle {
                 radius }) =>
                -1.0 *
                    circle_rect_penetration(position_2, position_1, radius,
                                            dimensions, angle),
                (Hitbox::Circle { radius }, Hitbox::Rectangle {
                 dimensions, angle }) =>
                circle_rect_penetration(position_1, position_2, radius,
                                        dimensions, angle),
                (Hitbox::Circle { radius: radius_1 }, Hitbox::Circle {
                 radius: radius_2 }) => {
                    circle_circle_penetration(position_1, position_2,
                                              radius_1, radius_2)
                }
                (_, _) => Vector2::zeros(),
            }
        }
        fn circle_rect_penetration(circle_position: Vector2,
                                   rectangle_position: Vector2, radius: &f32,
                                   dimensions: &Vector2, angle: &f32)
         -> Vector2 {
            let circle_center = get_circle_center(circle_position, *radius);
            let rectangle_center =
                get_rectangle_center(rectangle_position, *dimensions);
            let displacement = circle_center - rectangle_center;
            let distance =
                Vector2::new(displacement.x.abs(), displacement.y.abs());
            let h = rectangle_position + dimensions;
            if (distance.x > ((dimensions.x / 2.0) + radius)) ||
                   (distance.y > ((dimensions.y / 2.0) + radius)) {
                return Vector2::zeros()
            } else if (distance.x <= (dimensions.x / 2.0)) ||
                          (distance.y <= (dimensions.y / 2.0)) {
                let circle_dimensions =
                    Vector2::new(2.0 * *radius, 2.0 * *radius);
                let h1 = circle_position + circle_dimensions;
                let h2 = rectangle_position + dimensions;
                let d1 = h2 - circle_position;
                let d2 = h1 - rectangle_position;
                if (d1.x > 0.0) & (d2.x > 0.0) & (d1.y > 0.0) & (d2.y > 0.0) {
                    let px =
                        match d1.x.abs() < d2.x.abs() {
                            true => -1.0 * d1.x,
                            false => d2.x,
                        };
                    let py =
                        match d1.y.abs() < d2.y.abs() {
                            true => -1.0 * d1.y,
                            false => d2.y,
                        };
                    return match px.abs() > py.abs() {
                               true => Vector2::new(0.0, py),
                               false => Vector2::new(px, 0.0),
                           }
                } else { return Vector2::zeros() }
            } else {
                let corner_location =
                    match (displacement.x > 0.0, displacement.y > 0.0) {
                        (true, true) => Vector2::new(h.x, h.y),
                        (true, false) =>
                        Vector2::new(h.x, rectangle_position.y),
                        (false, false) =>
                        Vector2::new(rectangle_position.x,
                                     rectangle_position.y),
                        (false, true) =>
                        Vector2::new(rectangle_position.x, h.y),
                    };
                circle_point_penetration(circle_position, radius,
                                         corner_location)
            }
        }
        fn circle_point_penetration(circle_position: Vector2, radius: &f32,
                                    point_position: Vector2) -> Vector2 {
            let circle_center = get_circle_center(circle_position, *radius);
            let displacement = circle_center - point_position;
            if displacement.norm().abs() >= *radius {
                Vector2::zeros()
            } else { displacement - (*radius * displacement.normalize()) }
        }
        fn circle_circle_penetration(position_1: Vector2, position_2: Vector2,
                                     radius_1: &f32, radius_2: &f32)
         -> Vector2 {
            let center_1 = get_circle_center(position_1, *radius_1);
            let center_2 = get_circle_center(position_2, *radius_2);
            let displacement = center_1 - center_2;
            if displacement.norm().abs() >= (*radius_1 + *radius_2) {
                Vector2::zeros()
            } else {
                1.0 *
                    (displacement -
                         ((radius_1 / displacement.norm()) * displacement) -
                         ((radius_2 / displacement.norm()) * displacement))
            }
        }
        fn rect_rect_penetration(position_1: Vector2, position_2: Vector2,
                                 dimensions_1: &Vector2,
                                 dimensions_2: &Vector2, angle_1: &f32,
                                 angle_2: &f32) -> Vector2 {
            let h1 = position_1 + dimensions_1;
            let h2 = position_2 + dimensions_2;
            let d1 = h2 - position_1;
            let d2 = h1 - position_2;
            if (d1.x > 0.0) & (d2.x > 0.0) & (d1.y > 0.0) & (d2.y > 0.0) {
                let px =
                    match d1.x.abs() < d2.x.abs() {
                        true => -1.0 * d1.x,
                        false => d2.x,
                    };
                let py =
                    match d1.y.abs() < d2.y.abs() {
                        true => -1.0 * d1.y,
                        false => d2.y,
                    };
                match px.abs() > py.abs() {
                    true => Vector2::new(0.0, py),
                    false => Vector2::new(px, 0.0),
                }
            } else { Vector2::zeros() }
        }
    }
    pub struct UpdatePos;
    impl <'a> System<'a> for UpdatePos {
        type
        SystemData
        =
        (ReadExpect<'a, DeltaTime>, WriteExpect<'a, DebugTable>,
         ReadStorage<'a, Name>, ReadStorage<'a, Velocity>,
         WriteStorage<'a, Position>);
        fn run(&mut self,
               (dt, mut debug_table, name, vel, mut pos): Self::SystemData) {
            use specs::Join;
            for (name, vel, pos) in (&name, &vel, &mut pos).join() {
                pos.add(vel.get() * dt.get());
                debug_table.load(name.read().to_owned(),
                                 ::fmt::format(::std::fmt::Arguments::new_v1_formatted(&["x: ",
                                                                                         ", y: "],
                                                                                       &match (&(pos.x()
                                                                                                     as
                                                                                                     i32),
                                                                                               &(pos.y()
                                                                                                     as
                                                                                                     i32))
                                                                                            {
                                                                                            (arg0,
                                                                                             arg1)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt),
                                                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        },
                                                                                       &[::std::fmt::rt::v1::Argument{position:
                                                                                                                          ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                                      format:
                                                                                                                          ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                             ' ',
                                                                                                                                                         align:
                                                                                                                                                             ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                         flags:
                                                                                                                                                             0u32,
                                                                                                                                                         precision:
                                                                                                                                                             ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                         width:
                                                                                                                                                             ::std::fmt::rt::v1::Count::Implied,},},
                                                                                         ::std::fmt::rt::v1::Argument{position:
                                                                                                                          ::std::fmt::rt::v1::Position::At(1usize),
                                                                                                                      format:
                                                                                                                          ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                             ' ',
                                                                                                                                                         align:
                                                                                                                                                             ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                         flags:
                                                                                                                                                             0u32,
                                                                                                                                                         precision:
                                                                                                                                                             ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                         width:
                                                                                                                                                             ::std::fmt::rt::v1::Count::Implied,},}])));
            }
        }
    }
    pub struct UpdateVel;
    impl <'a> System<'a> for UpdateVel {
        type
        SystemData
        =
        (ReadExpect<'a, DeltaTime>, ReadStorage<'a, Acceleration>,
         ReadStorage<'a, MoveDrag>, WriteStorage<'a, Velocity>);
        fn run(&mut self, (dt, acc, drag, mut vel): Self::SystemData) {
            use specs::Join;
            for (acc, vel) in (&acc, &mut vel).join() {
                vel.add(acc.get() * dt.get());
            }
            for (drag, vel) in (&drag, &mut vel).join() {
                let v_old = vel.get();
                vel.add(-1.0 * drag.get_constant() * v_old * dt.get());
            }
        }
    }
    pub struct Render<'c> {
        context: &'c mut Context,
    }
    impl <'c> Render<'c> {
        pub fn new(context: &'c mut Context) -> Self { Self{context,} }
    }
    impl <'a, 'c> System<'a> for Render<'c> {
        type
        SystemData
        =
        (WriteExpect<'a, DebugTable>, ReadExpect<'a, Camera>,
         WriteExpect<'a, Assets>, ReadStorage<'a, DrawableComponent>,
         ReadStorage<'a, Position>);
        fn run(&mut self,
               (mut table, camera, mut assets, drawable, position):
                   Self::SystemData) {
            use specs::Join;
            table.render(self.context);
            for (drawable, position) in (&drawable, &position).join() {
                drawable.render(self.context, &mut assets, position,
                                camera.get_draw_parameters());
            }
        }
    }
    pub struct HandleMoveDirection;
    impl <'a> System<'a> for HandleMoveDirection {
        type
        SystemData
        =
        (ReadStorage<'a, MoveDirection>, WriteStorage<'a, Acceleration>);
        fn run(&mut self, (dir, mut acc): Self::SystemData) {
            use specs::Join;
            for (dir, acc) in (&dir, &mut acc).join() {
                let magnitude = dir.get_move_acceleration();
                acc.set(dir.get() * magnitude);
            }
        }
    }
    pub struct UpdateCamera {
    }
    impl <'a> System<'a> for UpdateCamera {
        type
        SystemData
        =
        (WriteExpect<'a, Camera>, ReadStorage<'a, CameraFollows>,
         ReadStorage<'a, Position>);
        fn run(&mut self, (mut camera, follows, position): Self::SystemData) {
            use specs::Join;
            for (follows, position) in (&follows, &position).join() {
                camera.set_translation(position.get());
            }
        }
    }
}
fn main() {
    let (width, height) = (800, 450);
    let cb =
        ContextBuilder::new("revivi",
                            "ggez").window_setup(conf::WindowSetup::default().title("WINDOW_TITLE")).window_mode(conf::WindowMode::default().dimensions(width,
                                                                                                                                                        height));
    let ctx = &mut cb.build().unwrap();
    match main_state::MainState::new(ctx, width, height) {
        Err(e) => {
            ::io::_print(::std::fmt::Arguments::new_v1(&["Could not load game!\n"],
                                                       &match () {
                                                            () => [],
                                                        }));
            ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["Error: ",
                                                                   "\n"],
                                                                 &match (&e,)
                                                                      {
                                                                      (arg0,)
                                                                      =>
                                                                      [::std::fmt::ArgumentV1::new(arg0,
                                                                                                   ::std::fmt::Display::fmt)],
                                                                  },
                                                                 &[::std::fmt::rt::v1::Argument{position:
                                                                                                    ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                format:
                                                                                                    ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                       ' ',
                                                                                                                                   align:
                                                                                                                                       ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                   flags:
                                                                                                                                       0u32,
                                                                                                                                   precision:
                                                                                                                                       ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                   width:
                                                                                                                                       ::std::fmt::rt::v1::Count::Implied,},}]));
        }
        Ok(ref mut game) => {
            let result = run(ctx, game);
            if let Err(e) = result {
                ::io::_print(::std::fmt::Arguments::new_v1_formatted(&["Error encountered running game: ",
                                                                       "\n"],
                                                                     &match (&e,)
                                                                          {
                                                                          (arg0,)
                                                                          =>
                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                       ::std::fmt::Display::fmt)],
                                                                      },
                                                                     &[::std::fmt::rt::v1::Argument{position:
                                                                                                        ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                    format:
                                                                                                        ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                           ' ',
                                                                                                                                       align:
                                                                                                                                           ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                       flags:
                                                                                                                                           0u32,
                                                                                                                                       precision:
                                                                                                                                           ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                       width:
                                                                                                                                           ::std::fmt::rt::v1::Count::Implied,},}]));
            } else {
                ::io::_print(::std::fmt::Arguments::new_v1(&["Game exited cleanly.\n"],
                                                           &match () {
                                                                () => [],
                                                            }));
            }
        }
    }
}
