use rand::{thread_rng, Rng};

use crate::{
    hittable::{hit_record, hit_record::HitRecord},
    material::{color, dielectric::Dielectric},
    math::vector,
    Color, Lambertian, Metal, Ray, Sphere, Vec3,
};

use super::object::Object;

pub struct World {
    objects: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn random_scene(&mut self, range: i32) {
        let mut rng = thread_rng();

        self.push(Object::new(
            Box::new(Sphere::new(1000.0 * vector::DOWN, 1000.0)),
            Box::new(Lambertian::new(color::GREY)),
        ));

        for a in -range..range {
            for b in -range..range {
                let center = Vec3 {
                    x: (a as f32) + 0.9 * rng.gen_range(0.0..=1.0),
                    y: 0.2,
                    z: (b as f32) + 0.9 * rng.gen_range(0.0..=1.0),
                };
                let hit_box = Sphere::new(center, 0.2);

                let choose_mat = rng.gen_range(0.0..=1.0);
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let lambertian = Lambertian::new(albedo);
                    self.push(Object::new(Box::new(hit_box), Box::new(lambertian)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..=0.5);
                    let metal = Metal::new(albedo, fuzz);
                    self.push(Object::new(Box::new(hit_box), Box::new(metal)));
                } else {
                    // glass
                    let dielectric = Dielectric::new(1.5);
                    self.push(Object::new(Box::new(hit_box), Box::new(dielectric)));
                }
            }
        }
        self.push(Object::new(
            Box::new(Sphere::new(
                Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                1.0,
            )),
            Box::new(Dielectric::new(1.5)),
        ));
        self.push(Object::new(
            Box::new(Sphere::new(
                Vec3 {
                    x: -4.0,
                    y: 1.0,
                    z: 0.0,
                },
                1.0,
            )),
            Box::new(Lambertian::new(Color {
                r: 0.4,
                g: 0.2,
                b: 0.1,
            })),
        ));
        self.push(Object::new(
            Box::new(Sphere::new(
                Vec3 {
                    x: 4.0,
                    y: 1.0,
                    z: 0.0,
                },
                1.0,
            )),
            Box::new(Metal::new(
                Color {
                    r: 0.7,
                    g: 0.6,
                    b: 0.5,
                },
                0.0,
            )),
        ));
    }
}

impl<'a> World {
    pub fn hit(&'a self, ray: &Ray) -> HitRecord<'a> {
        let mut ret: HitRecord = hit_record::DEFAULT;
        let mut temp: HitRecord = hit_record::DEFAULT;
        for hittable in &self.objects {
            hittable.hit(ray, &mut temp);
            if !temp.hit() {
                continue;
            }
            if temp.t < ret.t {
                ret.t = temp.t;
                ret.normal = temp.normal;
                ret.point = temp.point;
                ret.object = Some(hittable);
            }
        }
        return ret;
    }
}
