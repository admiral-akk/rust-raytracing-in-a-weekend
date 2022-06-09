use crate::{hittable::hit_record::HitRecord, Color, Ray};

use super::material::Material;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    fn reflect(ray: &mut Ray, hit_record: &HitRecord) {
        ray.pos = hit_record.point;
        ray.dir = ray.dir - hit_record.normal * 2.0 * (ray.dir * hit_record.normal);
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
        Metal::reflect(scattered, hit_record);
        return true;
    }
}
