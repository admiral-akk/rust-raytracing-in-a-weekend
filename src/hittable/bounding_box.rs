use crate::{math::vector, Ray, Vec3};

use super::hittable::Hittable;
#[derive(Debug)]
pub struct BoundingBox {
    pub min: Vec3,
    max: Vec3,
}

pub const DEFAULT: BoundingBox = BoundingBox::new(vector::ZERO, vector::ZERO);

impl BoundingBox {
    pub const fn new(min: Vec3, max: Vec3) -> BoundingBox {
        BoundingBox { min: min, max: max }
    }

    pub fn union(box1: &BoundingBox, box2: &BoundingBox) -> BoundingBox {
        BoundingBox::new(box1.min.min(&box2.min), box1.max.max(&box2.max))
    }

    #[inline(always)]
    pub fn effecient_hit(
        slope: f32,
        start: f32,
        min: f32,
        max: f32,
        t_min: &mut f32,
        t_max: &mut f32,
    ) -> bool {
        if slope == 0.0 {
            return min <= start && start <= max;
        }
        let inv_d = 1.0 / slope;
        let mut t0 = (min - start) * inv_d;
        let mut t1 = (max - start) * inv_d;
        if inv_d < 0.0 {
            let temp = t0;
            t0 = t1;
            t1 = temp;
        }
        if t0 > *t_min {
            *t_min = t0;
        }
        if t1 < *t_max {
            *t_max = t1;
        }
        return t_max >= t_min;
    }

    #[inline(always)]
    pub fn is_hit2(&self, ray: &Ray, curr_t: f32) -> bool {
        let (mut min_t, mut max_t) = (0.0001, curr_t);
        let x_range = BoundingBox::time_range(ray.dir.x, ray.pos.x, self.min.x, self.max.x);
        if x_range.0 > min_t {
            min_t = x_range.0;
        }
        if x_range.1 < max_t {
            max_t = x_range.1;
        }
        if max_t <= min_t {
            return false;
        }
        let y_range = BoundingBox::time_range(ray.dir.y, ray.pos.y, self.min.y, self.max.y);
        if y_range.0 > min_t {
            min_t = y_range.0;
        }
        if y_range.1 < max_t {
            max_t = y_range.1;
        }
        if max_t <= min_t {
            return false;
        }
        let z_range = BoundingBox::time_range(ray.dir.z, ray.pos.z, self.min.z, self.max.z);
        if z_range.0 > min_t {
            min_t = z_range.0;
        }
        if z_range.1 < max_t {
            max_t = z_range.1;
        }
        if max_t <= min_t {
            return false;
        }
        return true;
    }

    #[inline(always)]
    pub fn is_hit(&self, ray: &Ray, min_t: &mut f32, max_t: &mut f32) -> bool {
        BoundingBox::effecient_hit(ray.dir.z, ray.pos.z, self.min.z, self.max.z, min_t, max_t)
            && BoundingBox::effecient_hit(
                ray.dir.y, ray.pos.y, self.min.y, self.max.y, min_t, max_t,
            )
            && BoundingBox::effecient_hit(
                ray.dir.x, ray.pos.x, self.min.x, self.max.x, min_t, max_t,
            )
    }

    #[inline(always)]
    fn time_range(slope: f32, start: f32, min: f32, max: f32) -> (f32, f32) {
        if slope == 0.0 {
            if start > max || start < min {
                return (f32::INFINITY, f32::NEG_INFINITY);
            }
            return (f32::NEG_INFINITY, f32::INFINITY);
        }
        let (mut min_t, mut max_t) = ((min - start) / slope, (max - start) / slope);
        if min_t > max_t {
            let temp = min_t;
            min_t = max_t;
            max_t = temp;
        }

        return (min_t, max_t);
    }
}

impl Hittable for BoundingBox {
    #[inline(always)]
    fn hit(&self, ray: &Ray) -> f32 {
        let x_range = BoundingBox::time_range(ray.dir.x, ray.pos.x, self.min.x, self.max.x);
        let y_range = BoundingBox::time_range(ray.dir.y, ray.pos.y, self.min.y, self.max.y);
        let z_range = BoundingBox::time_range(ray.dir.z, ray.pos.z, self.min.z, self.max.z);
        let max_t = f32::min(x_range.1, f32::min(y_range.1, z_range.1));
        if max_t < 0.0001 {
            return f32::INFINITY;
        }

        let min_t = f32::max(x_range.0, f32::max(y_range.0, z_range.0));
        if max_t < min_t {
            return f32::INFINITY;
        }
        return min_t;
    }

    fn hit_normal(&self, _ray: &crate::Ray, hit_point: &Vec3) -> Vec3 {
        if hit_point.x == self.min.x {
            return vector::LEFT;
        }
        if hit_point.x == self.max.x {
            return vector::RIGHT;
        }
        if hit_point.y == self.min.y {
            return vector::DOWN;
        }
        if hit_point.y == self.max.y {
            return vector::UP;
        }
        if hit_point.z == self.min.z {
            return vector::BACK;
        }
        if hit_point.z == self.max.z {
            return vector::FORWARD;
        }
        return vector::ZERO;
    }

    fn bounds(&self) -> BoundingBox {
        BoundingBox::new(self.min, self.max)
    }
}
