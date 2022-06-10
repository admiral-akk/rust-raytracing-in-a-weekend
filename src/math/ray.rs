use crate::{hittable::hit_record::HitRecord, material::color, Color, Vec3, World};
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn project(&self, t: f32) -> Vec3 {
        return self.pos + t * self.dir;
    }

    pub fn color(ray: &mut Ray, world: &World, depth: u32) -> Color {
        if depth == 0 {
            return color::BLACK;
        }
        let hit_record: HitRecord = world.hit(&ray);
        if hit_record.hit() {
            let mut scattered = *ray;
            let mut attenuation = color::BLACK;
            if hit_record.object.unwrap().scatter(
                ray,
                &hit_record,
                &mut attenuation,
                &mut scattered,
            ) {
                return attenuation * Ray::color(&mut scattered, world, depth - 1);
            }
            return color::BLACK;
        } else {
            return Color::sky_color(&ray.dir);
        }
    }
}
