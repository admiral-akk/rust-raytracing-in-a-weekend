use crate::{
    hittable::{hit_record::HitRecord, hittable::Hittable},
    Ray,
};

pub struct Object {
    hitbox: Box<dyn Hittable>,
}

impl Object {
    pub fn new(hittable: Box<dyn Hittable>) -> Object {
        Object { hitbox: hittable }
    }
    pub fn hit(&self, ray: &Ray, hit_record: &mut HitRecord) {
        self.hitbox.hit(ray, hit_record);
    }
}
