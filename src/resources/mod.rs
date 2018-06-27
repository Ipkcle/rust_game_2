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
    view_dimensions: Vector2,
    translation: Point2,
    scale: Point2,
    rotation: f32,
    shear: Point2,
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            view_dimensions: Vector2::new(850.0, 450.0),
            translation: Point2::new(0.0, 0.0),
            scale: Point2::new(0.0, 0.0),
            rotation: 0.0,
            shear: Point2::new(0.0, 0.0),
        }
    }
}

impl Camera {
    fn new(view_dimensions: Vector2) -> Camera {
        Camera {
            view_dimensions,
            translation: Point2::new(0.0, 0.0),
            scale: Point2::new(0.0, 0.0),
            rotation: 0.0,
            shear: Point2::new(0.0, 0.0),
        }
    }

    pub fn new_with(view_dimensions: Vector2, translation: Point2, scale: Point2) -> Self {
        Self {
            view_dimensions,
            translation,
            scale,
            rotation: 0.0,
            shear: Point2::new(0.0, 0.0),
        }
    }

    pub fn get_view_dimensions(&self) -> Vector2 {
        self.view_dimensions
    }

    pub fn get_scale(&self) -> Point2 {
        self.scale
    }

    pub fn get_translation(&self) -> Point2 {
        self.translation
    }

    pub fn set_view_dimension(&mut self, view_dimensions: Vector2) {
        self.view_dimensions = view_dimensions;
    }

    pub fn set_scale(&mut self, scale: Vector2) {
        self.scale = Point2::new(scale.x, scale.y);
    }

    pub fn set_translation(&mut self, translation: Vector2) {
        self.translation = Point2::new(translation.x, translation.y);
    }

    pub fn set_center(&mut self, center_translation: Vector2) {
        self.translation = Point2::new(center_translation.x - self.view_dimensions.x * 0.5, center_translation.y - self.view_dimensions.y * 0.5);
    }

    pub fn get_draw_parameters(&self) -> DrawParam {
        DrawParam {
            dest: -1.0 * self.get_translation(),
            scale: self.get_scale(),
            rotation: self.rotation,
            shear: self.shear,
            ..Default::default()
        }
    }
}
