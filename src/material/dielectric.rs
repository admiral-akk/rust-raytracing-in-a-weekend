use super::{color, material::Material};
use crate::{hittable::hit_record::HitRecord, Color, Ray};

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub const fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    fn refract(ray: &mut Ray, hit_record: &HitRecord, refraction_index: f32) {
        let mut normal = hit_record.normal.normalized();
        let dir = ray.dir.normalized();
        let mut refraction_ratio = refraction_index;
        let incidence = dir * normal;
        if incidence < 0.0 {
            refraction_ratio = 1.0 / refraction_ratio;
        }
        if normal * dir > 0.0 {
            normal = normal * -1.0;
        }
        let cos_theta = f32::min(f32::abs(incidence), 1.0);
        let r_out_perp = (dir + normal * cos_theta) * refraction_ratio;
        let r_out_parallel = normal * -(f32::abs(1.0 - r_out_perp.len_sq())).sqrt();
        ray.pos = hit_record.point;
        ray.dir = (r_out_perp + r_out_parallel).normalized();
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = color::WHITE;
        *scattered = *ray;
        Dielectric::refract(scattered, hit_record, self.refraction_index);
        return true;
    }
}
