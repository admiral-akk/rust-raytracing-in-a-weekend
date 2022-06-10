use crate::{
    math::vector::{self, DOWN, RIGHT},
    Color, Ray, Vec3, World,
};

pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    up: Vec3,
    focal_length: f32,
    viewport_height: f32,
    viewport_width: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, vertical_fov_degrees: f32) -> Camera {
        let rad_angle = std::f32::consts::PI * vertical_fov_degrees / 180.0;
        let h = 2.0 * f32::tan(rad_angle / 2.0);
        Camera {
            pos: vector::ZERO,
            dir: vector::FORWARD,
            up: vector::DOWN * -1.0,
            focal_length: 1.0,
            viewport_height: h,
            viewport_width: h * aspect_ratio,
        }
    }

    pub fn color(&self, x: f32, y: f32, world: &World) -> Color {
        let view_x = x - 0.5;
        let view_y = y - 0.5;
        let focal_center = self.focal_length * self.dir;
        let x_delta = self.viewport_width * view_x * RIGHT;
        let y_delta = self.viewport_height * view_y * DOWN;
        let mut ray = Ray {
            pos: self.pos,
            dir: (focal_center + y_delta + x_delta).normalized(),
        };

        return Ray::color(&mut ray, world, 10);
    }
}
