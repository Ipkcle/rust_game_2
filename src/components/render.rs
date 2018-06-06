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
    fn get_drawable<'a>(&self, ctx: &mut Context, assets: &'a mut Assets) -> &'a Drawable;

    fn get_draw_parameters(&self) -> DrawParam;

    fn render(&self, ctx: &mut Context, assets: &mut Assets, position: &Position, draw_param: DrawParam) {
        let drawable = self.get_drawable(ctx, assets);
        graphics::draw_ex(
            ctx,
            drawable,
            graphics::DrawParam {
                src: self.get_draw_parameters().src,
                dest: add_point2_vec2(add_point2(draw_param.dest, self.get_draw_parameters().dest), position.get()),
                rotation: draw_param.rotation + self.get_draw_parameters().rotation,
                scale: mul_point2(draw_param.scale, self.get_draw_parameters().scale),
                offset: self.get_draw_parameters().offset,
                shear: mul_point2(draw_param.shear, self.get_draw_parameters().shear),
                color: self.get_draw_parameters().color,
            },
        ).unwrap();
    }
}
#[derive(Debug)]
pub struct DrawableComponent {
    asset: DrawableAsset,
    draw_parameters: graphics::DrawParam,
}

impl DrawableComponent {
    pub fn new(asset: DrawableAsset) -> DrawableComponent {
        DrawableComponent {
            asset,
            draw_parameters: graphics::DrawParam {
                ..Default::default()
            }
        }
    }
    pub fn new_with_color(asset: DrawableAsset, color: Color) -> DrawableComponent {
        DrawableComponent {
            asset,
            draw_parameters: graphics::DrawParam {
                color: Some(color),
                ..Default::default()
            }
        }
    }
}

impl Component for DrawableComponent {
    type Storage = VecStorage<Self>;
}


impl HasDrawable for DrawableComponent {
    fn get_drawable<'a>(&self, ctx: &mut Context, assets: &'a mut Assets) -> &'a Drawable {
        assets.get_drawable(ctx, self.asset)
    }

    fn get_draw_parameters(&self) -> DrawParam {
        self.draw_parameters
    }
}
