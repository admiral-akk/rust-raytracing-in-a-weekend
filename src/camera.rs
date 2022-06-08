use crate::{
    math::vector::{self, RIGHT, UP},
    Ray, Vec3,
};

use crate::hittable::Hittable;

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

    pub fn color(&self, x: f32, y: f32, hittables: &Vec<Box<dyn Hittable>>) -> (u8, u8, u8) {
        let view_x = x - 0.5;
        let view_y = y - 0.5;
        let ray = Ray {
            pos: self.pos,
            dir: self.dir * self.focal_length
                + UP * self.viewport_height * view_y
                + RIGHT * self.viewport_width * view_x,
        };
        for hittable in hittables {
            if hittable.is_hit(&ray) {
                let norm = hittable.hit_normal(&ray);
                return (
                    ((norm.x + 1.0) * 256.0 / 2.0) as u8,
                    ((norm.y + 1.0) * 256.0 / 2.0) as u8,
                    ((norm.z + 1.0) * 256.0 / 2.0) as u8,
                );
            }
        }
        let color = ray.dir.normalized() * 256.0;
        return (color.z as u8, color.z as u8, 255);
    }
}
