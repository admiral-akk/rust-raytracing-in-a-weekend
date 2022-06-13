#[cfg(test)]

mod tests {
    use rust_ray::*;
    #[test]
    fn hit() {
        let mut world = World::new();
        world.push(Object::new(
            Box::new(Sphere::new(vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(4.0 * vector::RIGHT + vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.recalculate_heap();

        let ray = Ray::new(vector::ZERO, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(2.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), false);

        let ray = Ray::new(3.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(4.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);
    }

    #[test]
    fn hit2() {
        let mut world = World::new();
        world.push(Object::new(
            Box::new(Sphere::new(vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.recalculate_heap();

        let ray = Ray::new(vector::ZERO, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(2.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), false);

        let ray = Ray::new(3.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), false);

        let ray = Ray::new(4.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), false);
    }

    #[test]
    fn hit3() {
        let mut world = World::new();
        world.push(Object::new(
            Box::new(Sphere::new(vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(4.0 * vector::RIGHT + vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(8.0 * vector::RIGHT + vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.recalculate_heap();

        let ray = Ray::new(vector::ZERO, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(2.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), false);

        let ray = Ray::new(3.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(4.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(5.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(6.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), false);

        let ray = Ray::new(7.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(8.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);
    }

    #[test]
    fn hit4() {
        let mut world = World::new();
        world.push(Object::new(
            Box::new(Sphere::new(8.0 * vector::UP + 10.0 * vector::BACK, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(4.0 * vector::DOWN + 10.0 * vector::BACK, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(4.0 * vector::RIGHT + vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(8.0 * vector::RIGHT + vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(8.0 * vector::RIGHT + 10.0 * vector::BACK, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(8.0 * vector::DOWN + 10.0 * vector::BACK, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.recalculate_heap();

        let ray = Ray::new(vector::ZERO, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(2.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), false);

        let ray = Ray::new(3.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(4.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(5.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(6.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), false);

        let ray = Ray::new(7.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);

        let ray = Ray::new(8.0 * vector::RIGHT, vector::FORWARD);
        assert_eq!(world.hit(&ray).hit(), true);
    }

    #[test]
    fn hit5() {
        let mut world = World::new();
        world.push(Object::new(
            Box::new(Sphere::new(8.0 * vector::UP + 10.0 * vector::BACK, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(2.0 * vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(5.0 * vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(10.0 * vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(4.0 * vector::DOWN + 10.0 * vector::BACK, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(4.0 * vector::RIGHT + vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(8.0 * vector::RIGHT + vector::FORWARD, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(8.0 * vector::RIGHT + 10.0 * vector::BACK, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.push(Object::new(
            Box::new(Sphere::new(8.0 * vector::DOWN + 10.0 * vector::BACK, 1.0)),
            Box::new(Dielectric::new(1.5)),
        ));
        world.recalculate_heap();

        let ray = Ray::new(vector::ZERO, vector::FORWARD);
        assert_eq!(world.hit(&ray).t, 1.0);
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
