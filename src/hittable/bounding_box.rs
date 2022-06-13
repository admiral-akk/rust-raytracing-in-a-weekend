use crate::{math::vector, Ray, Vec3};

use super::hittable::Hittable;

pub struct BoundingBox {
    min: Vec3,
    max: Vec3,
}

impl BoundingBox {
    pub fn new(min: Vec3, max: Vec3) -> BoundingBox {
        BoundingBox { min: min, max: max }
    }

    pub fn union(box1: &BoundingBox, box2: &BoundingBox) -> BoundingBox {
        BoundingBox::new(box1.min.min(&box2.min), box1.max.max(&box2.max))
    }

    fn time_range(slope: f32, start: f32, min: f32, max: f32) -> (f32, f32) {
        if slope == 0.0 {
            if start > max || start < min {
                return (f32::INFINITY, f32::NEG_INFINITY);
            }
            return (f32::NEG_INFINITY, f32::INFINITY);
        }
        let (min_t, max_t) = ((min - start) / slope, (max - start) / slope);

        return (f32::min(min_t, max_t), f32::max(min_t, max_t));
    }
}

impl Hittable for BoundingBox {
    fn hit(&self, ray: &Ray) -> f32 {
        let x_range = BoundingBox::time_range(ray.dir.x, ray.pos.x, self.min.x, self.max.x);
        let y_range = BoundingBox::time_range(ray.dir.y, ray.pos.y, self.min.y, self.max.y);
        let z_range = BoundingBox::time_range(ray.dir.z, ray.pos.z, self.min.z, self.max.z);
        let min_t = f32::max(x_range.0, f32::max(y_range.0, z_range.0));
        let max_t = f32::min(x_range.1, f32::min(y_range.1, z_range.1));
        if max_t < min_t || max_t < 0.0001 {
            return f32::INFINITY;
        }
        return min_t;
    }

    fn hit_normal(&self, _ray: &crate::Ray, _hit_point: &Vec3) -> Vec3 {
        if _hit_point.x == self.min.x {
            return vector::LEFT;
        }
        if _hit_point.x == self.max.x {
            return vector::RIGHT;
        }
        if _hit_point.y == self.min.y {
            return vector::DOWN;
        }
        if _hit_point.y == self.max.y {
            return vector::UP;
        }
        if _hit_point.z == self.min.z {
            return vector::BACK;
        }
        if _hit_point.z == self.max.z {
            return vector::FORWARD;
        }
        return vector::ZERO;
    }

    fn bounds(&self) -> BoundingBox {
        BoundingBox::new(self.min, self.max)
    }
}
