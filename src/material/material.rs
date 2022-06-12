use crate::{hittable::hit_record::HitRecord, Color, Rand, Ray, Vec3};

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        rand: &mut Rand,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn reflect(&self, rand: &mut Rand, ray: &mut Ray, hit_record: &HitRecord, fuzz: f32) {
        ray.pos = hit_record.point;
        ray.dir = ray.dir - hit_record.normal * 2.0 * (&ray.dir * &hit_record.normal)
            + Vec3::random_unit(rand) * fuzz;
    }
}
