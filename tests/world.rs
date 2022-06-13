#[cfg(test)]

mod tests {
    use rust_ray::{init_scene, Dielectric, Object, Ray, Sphere, Vec3, World};
    #[test]
    fn hit() {
        let mut world = World::new();
        world.push(Object::new(
            Box::new(Sphere::new(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                1.0,
            )),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(
                Vec3 {
                    x: 4.0,
                    y: 0.0,
                    z: 1.0,
                },
                1.0,
            )),
            Box::new(Dielectric::new(1.5)),
        ));

        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(world.hit(&ray).hit(), false);

        let ray = Ray::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(world.hit(&ray).hit(), true);
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
