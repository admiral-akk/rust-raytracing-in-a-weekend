use crate::{
    math::vector::{self, DOWN, RIGHT},
    Color, Ray, Vec3, World,
};

pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    focal_length: f32,
    viewport_height: f32,
    viewport_width: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Camera {
        Camera {
            pos: vector::ZERO,
            dir: vector::FORWARD,
            focal_length: 1.0,
            viewport_height: 2.0,
            viewport_width: 2.0 * aspect_ratio,
        }
    }

    pub fn color(&self, x: f32, y: f32, world: &World) -> Color {
        let view_x = x - 0.5;
        let view_y = y - 0.5;
        let mut ray = Ray {
            pos: self.pos,
            dir: (self.dir * self.focal_length
                + DOWN * self.viewport_height * view_y
                + RIGHT * self.viewport_width * view_x)
                .normalized(),
        };

        return Ray::color(&mut ray, world, 3);
    }
}
