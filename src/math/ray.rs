use crate::{hittable::hit_record::HitRecord, material::color, Color, Rand, Vec3, World};

#[derive(Debug)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(pos: Vec3, dir: Vec3) -> Self {
        Self { pos, dir }
    }

    pub fn project(&self, t: f32) -> Vec3 {
        return self.pos + t * self.dir;
    }

    pub fn color(ray: &mut Ray, world: &World, rand: &mut Rand, depth: u32) -> Color {
        if depth == 0 {
            return color::BLACK;
        }
        let hit_record: HitRecord = world.hit(&ray);
        if hit_record.hit() {
            let mut scattered = Ray::new(ray.pos, ray.dir);
            let mut attenuation = color::BLACK;
            if hit_record.object.unwrap().scatter(
                ray,
                rand,
                &hit_record,
                &mut attenuation,
                &mut scattered,
            ) {
                return attenuation * Ray::color(&mut scattered, world, rand, depth - 1);
            }
            return color::BLACK;
        } else {
            return Color::sky_color(&ray.dir);
        }
    }
}
