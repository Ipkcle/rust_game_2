use ggez::graphics::DrawParam;
use ggez::graphics::{Point2, Vector2};

#[derive(Default)]
pub struct DeltaTime(f32);

impl DeltaTime {
    pub fn new(seconds: f32) -> Self {
        DeltaTime(seconds)
    }
    pub fn get(&self) -> f32 {
        self.0
    }
    pub fn set(&mut self, val: f32) {
        self.0 = val;
    }
}

pub struct Camera {
    translation: Point2,
    scale: Point2,
    rotation: f32,
    shear: Point2,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            translation: Point2::new(0.0, 0.0),
            scale: Point2::new(0.0, 0.0),
            rotation: 0.0,
            shear: Point2::new(0.0, 0.0),
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            translation: Point2::new(0.0, 0.0),
            scale: Point2::new(1.0, 1.0),
            rotation: 0.0,
            shear: Point2::new(0.0, 0.0),
        }
    }

    pub fn new_with(translation: Point2, scale: Point2) -> Self {
        Self {
            translation,
            scale,
            rotation: 0.0,
            shear: Point2::new(0.0, 0.0),
        }
    }

    pub fn get_scale(&self) -> Point2 {
        self.scale
    }

    pub fn get_translation(&self) -> Point2 {
        self.translation
    }

    pub fn set_scale(&mut self, scale: Vector2) {
        self.scale = Point2::new(scale.x, scale.y);
    }

    pub fn set_translation(&mut self, translation: Vector2) {
        self.translation = Point2::new(translation.x, translation.y);
    }

    pub fn get_draw_parameters(&self) -> DrawParam {
        DrawParam {
            dest: self.get_translation(),
            scale: self.get_scale(),
            rotation: self.rotation,
            shear: self.shear,
            ..Default::default()
        }
    }
}
