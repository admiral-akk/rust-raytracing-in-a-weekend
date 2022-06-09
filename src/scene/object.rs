use crate::{
    hittable::{hit_record::HitRecord, hittable::Hittable},
    material::material::Material,
    Ray,
};

pub struct Object {
    hitbox: Box<dyn Hittable>,
    material: Box<dyn Material>,
}

impl Object {
    pub fn new(hittable: Box<dyn Hittable>, material: Box<dyn Material>) -> Object {
        Object {
            hitbox: hittable,
            material: material,
        }
    }
    pub fn hit(&self, ray: &Ray, hit_record: &mut HitRecord) {
        self.hitbox.hit(ray, hit_record);
    }
}
