use crate::{
    hittable::{hit_record::HitRecord, hittable::Hittable},
    material::material::Material,
    BoundingBox, Color, Rand, Ray, Sphere, Vec3,
};

pub struct Object {
    hitbox: Sphere,
    material: Box<dyn Material>,
}

impl Object {
    pub fn new(hittable: Sphere, material: Box<dyn Material>) -> Object {
        Object {
            hitbox: hittable,
            material: material,
        }
    }

    pub fn scatter(
        &self,
        ray: &Ray,
        rand: &mut Rand,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        return self
            .material
            .scatter(ray, rand, hit_record, attenuation, scattered);
    }
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray) -> f32 {
        return self.hitbox.hit(ray);
    }
    fn hit_normal(&self, ray: &Ray, hit_point: &Vec3) -> Vec3 {
        return self.hitbox.hit_normal(ray, hit_point);
    }
    fn bounds(&self) -> BoundingBox {
        return self.hitbox.bounds();
    }
}
