use crate::{
    color,
    hittable::{self, HitRecord, Hittable},
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

    fn dir_to_color(dir: &Vec3) -> Color {
        let t = 0.5 * (dir.y + 1.0);
        return Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        } * t
            + Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            } * (1.0 - t);
    }

    fn generate_new_dir(normal: &Vec3) -> Vec3 {
        return ((Vec3::random_unit() - *normal).normalized() + *normal).normalized();
    }

    pub fn color(ray: &mut Ray, world: &World, depth: u32) -> Color {
        if depth == 0 {
            return color::BLACK;
        }
        let mut temp: HitRecord = hittable::DEFAULT;
        world.hit(&ray, &mut temp);
        if temp.hit() {
            ray.dir = Ray::generate_new_dir(&temp.normal);
            ray.pos = temp.point;
            return Ray::color(ray, &world, depth - 1) * 0.5;
        } else {
            return Ray::dir_to_color(&ray.dir);
        }
    }
}
