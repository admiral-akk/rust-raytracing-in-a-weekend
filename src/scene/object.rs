use crate::{
    hittable::{hit_record::HitRecord, hittable::Hittable},
    material::material::Material,
    Color, Ray, Sphere,
};

pub struct Object {
    hitbox: Sphere,
    material: Box<dyn Material>,
}

impl Object {
    pub fn new(hittable: Box<Sphere>, material: Box<dyn Material>) -> Object {
        Object {
            hitbox: *hittable,
            material: material,
        }
    }

    pub fn hit(&self, ray: &Ray, hit_record: &mut HitRecord) {
        self.hitbox.hit(ray, hit_record);
    }

    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        return self
            .material
            .scatter(ray, hit_record, attenuation, scattered);
    }
}
