use crate::{hittable::hit_record::HitRecord, Color, Ray, Vec3};

use super::material::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub const fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }

    fn reflect(&self, ray: &mut Ray, hit_record: &HitRecord) {
        ray.pos = hit_record.point;
        ray.dir = ray.dir - hit_record.normal * 2.0 * (ray.dir * hit_record.normal)
            + Vec3::random_unit() * self.fuzz;
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = self.albedo;
        *scattered = *ray;
        self.reflect(scattered, hit_record);
        return scattered.dir * hit_record.normal > 0.0;
    }
}
