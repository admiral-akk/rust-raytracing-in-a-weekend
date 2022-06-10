use crate::{
    math::vector::{self, DOWN, RIGHT, UP},
    Color, Ray, Vec3, World,
};

pub struct Camera {
    pos: Vec3,
    up: Vec3,
    right: Vec3,
    forward: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, vertical_fov_degrees: f32) -> Camera {
        let rad_angle = std::f32::consts::PI * vertical_fov_degrees / 180.0;
        let h = 2.0 * f32::tan(rad_angle / 2.0);
        let w = h * aspect_ratio;
        let focal_length = 1.0;

        Camera {
            pos: vector::ZERO,
            up: h * vector::DOWN,
            right: w * vector::RIGHT,
            forward: focal_length * vector::FORWARD,
        }
    }

    pub fn color(&self, x: f32, y: f32, world: &World) -> Color {
        let view_x = x - 0.5;
        let view_y = y - 0.5;
        let focal_center = self.forward;
        let x_delta = view_x * self.right;
        let y_delta = view_y * self.up;
        let mut ray = Ray {
            pos: self.pos,
            dir: (focal_center + y_delta + x_delta).normalized(),
        };

        return Ray::color(&mut ray, world, 10);
    }
}
