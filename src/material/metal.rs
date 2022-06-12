use crate::{hittable::hit_record::HitRecord, Color, Rand, Ray};

use super::material::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub const fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        rand: &mut Rand,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = self.albedo;
        *scattered = Ray::new(ray.pos, ray.dir);
        self.reflect(rand, scattered, hit_record, self.fuzz);
        return &scattered.dir * &hit_record.normal > 0.0;
    }
}
