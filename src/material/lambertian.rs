use crate::{hittable::hit_record::HitRecord, Color, Ray, Vec3};

use super::material::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir = hit_record.normal + Vec3::random_unit();
        if scatter_dir.len_sq() < 0.00001 {
            scatter_dir = hit_record.normal;
        }
        *scattered = Ray {
            pos: hit_record.point,
            dir: scatter_dir,
        };
        *attenuation = self.albedo;
        return true;
    }
}
