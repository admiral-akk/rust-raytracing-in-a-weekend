use crate::{
    hittable::{hit_record, hit_record::HitRecord, hittable::Hittable},
    material::color,
    Color, Vec3, World,
};
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn project(&self, t: f32) -> Vec3 {
        return self.pos + self.dir * t;
    }
    fn generate_new_dir(normal: &Vec3) -> Vec3 {
        return ((Vec3::random_unit() - *normal).normalized() + *normal).normalized();
    }

    pub fn color(ray: &mut Ray, world: &World, depth: u32) -> Color {
        if depth == 0 {
            return color::BLACK;
        }
        let mut temp: HitRecord = hit_record::DEFAULT;
        world.hit(&ray, &mut temp);
        if temp.hit() {
            ray.dir = Ray::generate_new_dir(&temp.normal);
            ray.pos = temp.point;
            return Ray::color(ray, &world, depth - 1) * 0.5;
        } else {
            return Color::dir_to_color(&ray.dir);
        }
    }
}
