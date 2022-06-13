use rand::{thread_rng, Rng};

use crate::{
    material::{color, dielectric::Dielectric},
    math::vector,
    Color, Lambertian, Metal, Sphere, World,
};

use super::object::Object;

pub fn init_scene(world: &mut World, range: i32) {
    let mut rng = thread_rng();

    world.push(Object::new(
        Box::new(Sphere::new(1000.0 * vector::DOWN, 1000.0)),
        Box::new(Lambertian::new(color::GREY)),
    ));

    for a in -range..range {
        for b in -range..range {
            let x = (a as f32) + 0.9 * rng.gen_range(0.0..=1.0);
            let z = (b as f32) + 0.9 * rng.gen_range(0.0..=1.0);

            let center = x * vector::RIGHT + 0.2 * vector::UP + z * vector::FORWARD;
            let hit_box = Sphere::new(center, 0.2);

            let choose_mat = rng.gen_range(0.0..=1.0);
            if choose_mat < 0.8 {
                // diffuse
                let albedo = Color::random() * Color::random();
                let lambertian = Lambertian::new(albedo);
                world.push(Object::new(Box::new(hit_box), Box::new(lambertian)));
            } else if choose_mat < 0.95 {
                // metal
                let albedo = Color::random_range(0.5, 1.0);
                let fuzz = rng.gen_range(0.0..=0.5);
                let metal = Metal::new(albedo, fuzz);
                world.push(Object::new(Box::new(hit_box), Box::new(metal)));
            } else {
                // glass
                let dielectric = Dielectric::new(1.5);
                world.push(Object::new(Box::new(hit_box), Box::new(dielectric)));
            }
        }
    }
    world.push(Object::new(
        Box::new(Sphere::new(vector::UP, 1.0)),
        Box::new(Dielectric::new(1.5)),
    ));
    world.push(Object::new(
        Box::new(Sphere::new(4.0 * vector::LEFT + vector::UP, 1.0)),
        Box::new(Lambertian::new(Color {
            r: 0.4,
            g: 0.2,
            b: 0.1,
        })),
    ));
    world.push(Object::new(
        Box::new(Sphere::new(4.0 * vector::RIGHT + vector::UP, 1.0)),
        Box::new(Metal::new(
            Color {
                r: 0.7,
                g: 0.6,
                b: 0.5,
            },
            0.0,
        )),
    ));
    world.recalculate_heap();
}
