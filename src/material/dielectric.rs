use super::{color, material::Material};
use crate::{hittable::hit_record::HitRecord, Color, Rand, Ray};

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
        let incidence = &dir * &normal;
        if incidence < 0.0 {
            refraction_ratio = 1.0 / refraction_ratio;
        }
        if &normal * &dir > 0.0 {
            normal = -normal;
        }
        let cos_theta = f32::min(f32::abs(incidence), 1.0);
        let r_out_perp = refraction_ratio * (dir + cos_theta * normal);
        let r_out_parallel = -f32::abs(1.0 - r_out_perp.length_squared()).sqrt() * normal;
        ray.pos = hit_record.point;
        ray.dir = (r_out_perp + r_out_parallel).normalized();
    }

    fn reflectance(cosine: f32, refraction_ratio: f32) -> f32 {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5);
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        rand: &mut Rand,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = color::WHITE;
        *scattered = Ray {
            pos: ray.pos,
            dir: ray.dir,
        };
        let cos_theta = f32::min(f32::abs(&ray.dir * &hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut refraction_ratio = self.refraction_index;
        if &ray.dir * &hit_record.normal < 0.0 {
            refraction_ratio = 1.0 / refraction_ratio;
        }
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand.rand() {
            self.reflect(rand, scattered, hit_record, 0.0);
        } else {
            Dielectric::refract(scattered, hit_record, self.refraction_index);
        }
        return true;
    }
}
