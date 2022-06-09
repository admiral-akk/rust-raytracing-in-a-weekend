use crate::{
    hittable::{self, HitRecord, Hittable},
    Ray,
};

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
    fn hit(&self, ray: &Ray, hit_record: &mut HitRecord) {
        hit_record.t = f32::INFINITY;
        let mut temp: HitRecord = hittable::DEFAULT;
        for hittable in &self.objects {
            hittable.hit(ray, &mut temp);
            if !temp.hit() {
                continue;
            }
            if temp.t < hit_record.t {
                hit_record.t = temp.t;
                hit_record.normal = temp.normal;
                hit_record.point = temp.point;
            }
        }
    }
}
