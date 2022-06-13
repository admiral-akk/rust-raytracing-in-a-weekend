#[cfg(test)]

mod tests {
    use rust_ray::{init_scene, Display, Ray, Vec3, World};

    #[test]
    fn tiny() {
        let mut display = Display::new(10, 10, 1, 90.0);
        display.tick(0);
    }

    #[test]
    fn empty_world() {
        let mut world = World::new();
        init_scene(&mut world, 0);

        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        for _i in 0..100000000 {
            world.hit(&ray);
        }
    }

    #[test]
    fn full_world() {
        let mut world = World::new();
        init_scene(&mut world, 11);
        let world = world;

        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        for _i in 0..2000000 {
            world.hit(&ray);
        }
    }
}
