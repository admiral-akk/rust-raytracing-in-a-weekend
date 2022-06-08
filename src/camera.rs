use crate::{
    hittable::HitRecord,
    math::vector::{self, DOWN, RIGHT},
    Ray, Vec3, World,
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

    pub fn color(&self, x: f32, y: f32, world: &World) -> (u8, u8, u8) {
        let view_x = x - 0.5;
        let view_y = y - 0.5;
        let ray = Ray {
            pos: self.pos,
            dir: self.dir * self.focal_length
                + DOWN * self.viewport_height * view_y
                + RIGHT * self.viewport_width * view_x,
        };

        let mut temp: HitRecord = HitRecord {
            point: vector::ZERO,
            normal: vector::ZERO,
            t: f32::INFINITY,
        };
        world.hit(&ray, &mut temp);
        if temp.t < f32::INFINITY {
            let norm = temp.normal;
            return (
                ((norm.x + 1.0) * 256.0 / 2.0) as u8,
                ((norm.y + 1.0) * 256.0 / 2.0) as u8,
                ((norm.z + 1.0) * 256.0 / 2.0) as u8,
            );
        }
        let color = ray.dir.normalized() * 256.0;
        return (color.z as u8, color.z as u8, 255);
    }
}
