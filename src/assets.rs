use std::collections::HashMap;
use ggez::nalgebra::core::Vector2;
use ggez::nalgebra::core::dimension::U2;
use ggez::graphics::{DrawMode, Drawable, Mesh, MeshBuilder};
use ggez::graphics::{Point2};
use ggez::Context;

#[derive(Clone, Copy, Debug)]
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

impl DrawableAsset {
    pub fn rect(x: u32, y: u32) -> Self {
        DrawableAsset::Rectangle {
            dimensions: Vector2::new(x, y),
        }
    }
    pub fn circle(radius: u32) -> Self {
        DrawableAsset::Circle {
            radius
        }
    }
}

pub struct Assets {
    drawable: DrawableAssets,
}

impl Assets {
    pub fn get_drawable(&mut self, ctx: &mut Context, drawable_asset: DrawableAsset) -> &Drawable {
        self.drawable.get_drawable(ctx, drawable_asset)
    }

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            drawable: DrawableAssets::new(ctx),
        }
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
        DrawableAssets {
            player: Self::circle(ctx, 10.0),
            block: Self::rect(ctx, 20.0, 20.0),
            wallh: Self::rect(ctx, 400.0, 20.0),
            wallv: Self::rect(ctx, 20.0, 400.0),
            bullet: Self::rect(ctx, 2.0, 2.0),
            rectangles: HashMap::new(),      
            circles: HashMap::new(),      
        }
    }

    fn circle(ctx: &mut Context, radius: f32) -> Mesh {
        let mut builder = MeshBuilder::new();
        builder.circle(
            DrawMode::Fill,
            Point2::new(1.0 * radius, 1.0 * radius),
            radius,
            radius / 200.0,
        );
        builder.build(ctx).unwrap()
    }

    fn rect(ctx: &mut Context, x: f32, y: f32) -> Mesh {
        let mut builder = MeshBuilder::new();
        builder.polygon(
            DrawMode::Fill,
            &[
                Point2::new(0.0, 0.0),
                Point2::new(x, 0.0),
                Point2::new(x, 1.0*y),
                Point2::new(0.0, 1.0*y),
            ],
        );
        builder.build(ctx).unwrap()
    }



    pub fn get_drawable(&mut self, ctx: &mut Context, drawable_asset: DrawableAsset) -> &Drawable {
        match drawable_asset {
            DrawableAsset::Player => &self.player,
            DrawableAsset::Block => &self.block,
            DrawableAsset::Wallh => &self.wallh,
            DrawableAsset::Wallv => &self.wallv,
            DrawableAsset::Bullet => &self.bullet,
            DrawableAsset::Rectangle{
                dimensions
            } => {
                if let None = self.rectangles.get(&dimensions) {
                    let new_rectangle = Self::rect(ctx, dimensions.x as f32, dimensions.y as f32);
                    let key = Vector2::new(dimensions.x, dimensions.y);
                    self.rectangles.insert(key, new_rectangle);
                    self.rectangles.get(&key).unwrap()
                } else {
                    self.rectangles.get(&dimensions).unwrap()
                }
                /*
                 * This SEEMS better but it doesn't work :(
                if let Some(rectangle) = self.rectangles.get(&dimensions) {
                    return rectangle;
                }
                let new_rectangle = Self::rect(ctx, dimensions.x as f32, dimensions.y as f32);
                let key = Vector2::new(dimensions.x, dimensions.y);
                self.rectangles.insert(key, new_rectangle);
                self.rectangles.get(&key).unwrap()
                */
            },
            DrawableAsset::Circle {
                radius
            } => {
                if let None = self.circles.get(&radius) {
                    let new_circle = Self::circle(ctx, radius as f32);
                    self.circles.insert(radius, new_circle);
                    self.circles.get(&radius).unwrap()
                } else {
                    self.circles.get(&radius).unwrap()
                }
                /*
                 * This SEEMS better but it doesn't work :(
                if let Some(circle) = self.circles.get(&radius) {
                    return circle;
                }
                let new_circle = Self::circle(ctx, radius as f32);
                self.circles.insert(radius, new_circle);
                self.circles.get(&radius).unwrap()
                */
            },
        }
    }
}
