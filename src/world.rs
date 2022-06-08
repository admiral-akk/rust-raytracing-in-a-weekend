use crate::{hittable::Hittable, math::vector, Ray, Vec3};

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }
}

impl Hittable for World {
    fn is_hit(&self, ray: &Ray) -> bool {
        for hittable in &self.objects {
            if hittable.is_hit(ray) {
                return true;
            }
        }
        return false;
    }
    fn hit_normal(&self, ray: &Ray) -> Vec3 {
        for hittable in &self.objects {
            if hittable.is_hit(ray) {
                return hittable.hit_normal(ray);
            }
        }
        return vector::ZERO;
    }
}
