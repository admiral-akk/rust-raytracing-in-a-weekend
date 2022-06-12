#[cfg(test)]

mod tests {
    use rust_ray::{BoundingBox, Hittable, Ray, Vec3};

    #[test]
    fn hit() {
        let bb = BoundingBox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0));
        let miss = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        assert_eq!(bb.hit(&miss) == f32::INFINITY, true);
        let miss = Ray::new(Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        assert_eq!(bb.hit(&miss) == f32::INFINITY, true);

        let hit = Ray::new(Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        assert_eq!(bb.hit(&hit) < f32::INFINITY, true);
        let hit = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(bb.hit(&hit) < f32::INFINITY, true);
    }

    #[test]
    fn union() {
        let bb = BoundingBox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0));
        let bb2 = BoundingBox::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(0.0, 0.0, 0.0));

        let union = BoundingBox::union(&bb, &bb2);
        let ray = Ray::new(Vec3::new(0.5, 0.5, 0.0), Vec3::new(0.0, 0.0, 1.0));

        assert_eq!(bb.hit(&ray) == f32::INFINITY, true);
        assert_eq!(bb2.hit(&ray) == f32::INFINITY, true);
        assert_eq!(union.hit(&ray) < f32::INFINITY, true);
    }
}
